use std::io::{self, stdout, Write};

use crate::text_effects::text_effect::TextEffect;

pub struct Blinking;

/// This implementation of the TextEffect trait will make the text blink.
impl TextEffect for Blinking {
    fn apply(&self, text: &str) -> Vec<String> {
        let mut frames = vec![];
        for _ in 0..10 {
            frames.push(text.to_string());
            frames.push(" ".repeat(text.len()));
        }
        frames
    }

    fn display(&self, text: &str) -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H"); 
        stdout().write_all(text.as_bytes())?;
        stdout().flush()
    }
}