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
                print!("> ");
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
            _ => println!("Unknown command: '{}'", command),
        }
        self.buffer.clear();
    }
}
