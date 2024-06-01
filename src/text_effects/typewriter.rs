use std::io::{self, stdout, Write};

use crate::text_effects::text_effect::TextEffect;
pub struct Typewriter;

/// This implementation of the TextEffect trait will make the text appear as if it was being typed.
impl TextEffect for Typewriter {
    fn apply(&self, text: &str) -> Vec<String> {
        let mut frames = vec![];
        let mut current = String::new();
        for character in text.chars() {
            current.push(character);
            frames.push(current.clone());
        }
        frames
    }

    fn display(&self, text: &str) -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H"); 
        stdout().write_all(text.as_bytes())?;
        stdout().flush()
    }
}
