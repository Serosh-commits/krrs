use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::{Mutex, Lazy};

use alloc::collections::VecDeque;

pub static KEYBOARD: Lazy<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Lazy::new(|| {
    Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore))
});

pub static KEY_QUEUE: Lazy<Mutex<VecDeque<char>>> = Lazy::new(|| {
    Mutex::new(VecDeque::new())
});

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    KEY_QUEUE.lock().push_back(character);
                }
                DecodedKey::RawKey(_) => {}
            }
        }
    }
}

pub fn pop_key() -> Option<char> {
    KEY_QUEUE.lock().pop_front()
}
