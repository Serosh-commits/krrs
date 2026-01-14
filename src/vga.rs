use core::fmt;
use spin::Mutex;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const BUFFER_ADDR: usize = 0xb8000;

#[derive(Clone, Copy)]
#[repr(transparent)]
struct ColorCode(u8);

#[derive(Clone, Copy)]
#[repr(C)]
struct Char {
    ascii: u8,
    color: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Char; WIDTH]; HEIGHT],
}

pub struct Writer {
    col: usize,
    color: ColorCode,
    buffer: *mut Buffer,
}

unsafe impl Send for Writer {}
unsafe impl Sync for Writer {}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            c => {
                if self.col >= WIDTH {
                    self.newline();
                }
                let row = HEIGHT - 1;
                unsafe {
                    (*self.buffer).chars[row][self.col] = Char {
                        ascii: c,
                        color: self.color,
                    };
                }
                self.col += 1;
            }
        }
    }

    fn newline(&mut self) {
        for r in 1..HEIGHT {
            for c in 0..WIDTH {
                unsafe {
                    let character = (*self.buffer).chars[r][c];
                    (*self.buffer).chars[r - 1][c] = character;
                }
            }
        }
        self.clear_row(HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii: b' ',
            color: self.color,
        };
        for c in 0..WIDTH {
            unsafe {
                (*self.buffer).chars[row][c] = blank;
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    col: 0,
    color: ColorCode(0x07),
    buffer: BUFFER_ADDR as *mut Buffer,
});

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        core::fmt::Write::write_fmt(&mut *$crate::vga::WRITER.lock(), format_args!($($arg)*)).unwrap()
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => { $crate::print!("{}\n", format_args!($($arg)*)) };
}

pub fn init() {
    let mut w = WRITER.lock();
    w.col = 0;
    for r in 0..HEIGHT {
        w.clear_row(r);
    }
}
