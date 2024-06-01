use std::io::{self, stdout, Write};

use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::text_effects::text_effect::TextEffect;

pub struct Scattering;

/// This implementation of the TextEffect trait will scatter the text.
impl TextEffect for Scattering {
    fn apply(&self, text: &str) -> Vec<String> {
        let mut rng = thread_rng();
        let mut frames = vec![];
        let chars: Vec<char> = text.chars().collect();
        let mut output: Vec<Option<char>> = vec![None; text.len()];
        let mut indices: Vec<usize> = (0..text.len()).collect();
        indices.shuffle(&mut rng);
        for &idx in &indices {
            output[idx] = Some(chars[idx]);
            frames.push(
                output.iter().map(|&c| c.unwrap_or(' ')).collect()
            );
        }
        frames
    }

    fn display(&self, text: &str) -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H"); 
        stdout().write_all(text.as_bytes())?;
        stdout().flush()
    }
}