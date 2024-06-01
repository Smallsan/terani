use colored::*;
use std::io::{self, stdout, Write};
use crate::text_effects::text_effect::TextEffect;
pub struct ColorChanging;

/// This implementation of the TextEffect trait will change the color of the text.
impl TextEffect for ColorChanging {
    fn apply(&self, text: &str) -> Vec<String> {
        let colors = ["red", "green", "yellow", "blue", "magenta", "cyan"];
        let mut frames = vec![];
        for &color in &colors {
            let frame = match color {
                "red" => text.red().to_string(),
                "green" => text.green().to_string(),
                "yellow" => text.yellow().to_string(),
                "blue" => text.blue().to_string(),
                "magenta" => text.magenta().to_string(),
                "cyan" => text.cyan().to_string(),
                _ => text.to_string(),
            };
            frames.push(frame);
        }
        frames
    }

    fn display(&self, text: &str) -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H"); 
        stdout().write_all(text.as_bytes())?;
        stdout().flush()
    }
}