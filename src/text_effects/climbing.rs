use textwrap::fill;
use std::io::{self, stdout, Write};
use crate::text_effects::text_effect::TextEffect;
pub struct Climbing;

impl TextEffect for Climbing {
    fn apply(&self, text: &str) -> Vec<String> {
        let mut frames = vec![];
        for width in (10..=text.len()).chain((10..text.len()).rev()) {
            let frame = fill(text, width);
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