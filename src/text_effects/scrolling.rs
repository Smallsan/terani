use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;
use crate::text_effects::text_effect::TextEffect;
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::queue;

/// BROKEN BROKEN BROKEN
pub struct ScrollingLeft;

impl TextEffect for ScrollingLeft {
    fn apply(&self, text: &str) -> Vec<String> {
        let mut frames = vec![];
        let spaces = " ".repeat(text.len());
        let extended_text = format!("{}{}{}", spaces, text, spaces);
        for i in 0..=(extended_text.len() - text.len()) {
            frames.push(extended_text[i..i + text.len()].to_string());
        }
        frames
    }

    fn display(&self, text: &str) -> io::Result<()> {
        for frame in self.apply(text) {
            queue!(stdout(), Clear(ClearType::All))?;
            stdout().write_all(frame.as_bytes())?;
            stdout().flush()?;
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}

pub struct ScrollingRight;

impl TextEffect for ScrollingRight {
    fn apply(&self, text: &str) -> Vec<String> {
        let mut frames = vec![];
        let spaces = " ".repeat(text.len());
        let extended_text = format!("{}{}{}", text, spaces, spaces);
        for i in (0..=(extended_text.len() - text.len())).rev() {
            frames.push(extended_text[i..i + text.len()].to_string());
        }
        frames
    }

    fn display(&self, text: &str) -> io::Result<()> {
        for frame in self.apply(text) {
            let (terminal_width, _) = terminal::size()?;
            let spaces = " ".repeat(terminal_width as usize - frame.len());
            queue!(stdout(), Clear(ClearType::All))?;
            stdout().write_all(format!("{}{}", spaces, frame).as_bytes())?;
            stdout().flush()?;
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}