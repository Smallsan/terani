use std::io;
use crate::text_effects::text_effect::TextEffect;

pub struct Frame {
    pub content: Vec<String>,
    pub text_effect: Box<dyn TextEffect>,
}

impl Frame {
    /// Creates a new `Frame` with the given `text` and `text_effect`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content of the frame.
    /// * `text_effect` - The text effect to apply to the frame.
    ///
    /// # Returns
    ///
    /// A new `Frame` instance.
    pub fn new(text: &str, text_effect: Box<dyn TextEffect>) -> Self {
        Self {
            content: text_effect.apply(text),
            text_effect,
        }
    }

    /// Displays the frame at the specified `frame_number`.
    ///
    /// # Arguments
    ///
    /// * `frame_number` - The index of the frame to display.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating the success or failure of the operation.
    /// If the `frame_number` is invalid, an `InvalidInput` error is returned.
    pub fn display(&self, frame_number: usize) -> io::Result<()> {
        if frame_number >= self.content.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid frame number"));
        }
        self.text_effect.display(&self.content[frame_number])
    }
}