use alloc::string::String;

pub struct Shell {
    buffer: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn input(&mut self, c: char) {
        match c {
            '\n' => {
                print!("\n");
                self.execute();
                crate::vga::set_colors(crate::vga::Color::Cyan, crate::vga::Color::Black);
                print!("> ");
                crate::vga::set_colors(crate::vga::Color::White, crate::vga::Color::Black);
            }
            '\x08' => { // Backspace
                if self.buffer.pop().is_some() {
                    print!("\x08 \x08"); // Erase char on screen
                }
            }
            c => {
                print!("{}", c);
                self.buffer.push(c);
            }
        }
    }

    fn execute(&mut self) {
        let command = self.buffer.trim();
        match command {
            "" => {},
            "help" => {
                println!("Available commands:");
                println!("  help - Show this menu");
                println!("  ver  - Show OS version");
                println!("  cls  - Clear screen");
            }
            "ver" => println!("Rust OS v0.1.0"),
            "cls" => crate::vga::init(),
            "uptime" => {
                let ticks = crate::idt::TICKS.load(core::sync::atomic::Ordering::Relaxed);
                println!("Uptime: {} ticks", ticks);
            }
            "free" => {
                let used = crate::heap::used();
                let free = crate::heap::free();
                println!("Heap Memory:");
                println!("  Used: {} bytes", used);
                println!("  Free: {} bytes", free);
                println!("  Total: {} bytes", used + free);
            }
            "color" => {
                println!("Available colors: 0-15 (e.g., color 14 0)");
                println!("Usage: color <fg> <bg>");
            }
            c if c.starts_with("color ") => {
                let parts: alloc::vec::Vec<&str> = c.split_whitespace().collect();
                if parts.len() == 3 {
                    if let (Ok(fg), Ok(bg)) = (parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                        unsafe {
                            let fg_color: crate::vga::Color = core::mem::transmute(fg % 16);
                            let bg_color: crate::vga::Color = core::mem::transmute(bg % 16);
                            crate::vga::set_colors(fg_color, bg_color);
                        }
                    }
                }
            }
            _ => println!("Unknown command: '{}'", command),
        }
        self.buffer.clear();
    }
}
