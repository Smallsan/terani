use std::{io, thread};
use std::time::Duration;

use crate::text_effects::text_effect::TextEffect;
use crate::structs::frame::Frame;

pub struct Animation {
    frames: Vec<Frame>,
    current_frame: usize,
    current_subframe: usize,
    paused: bool,
    speed: Duration,
}

/// Represents an animation with a sequence of frames and a speed.
/// The animation can be paused, resumed, and advanced to the next frame.
impl Animation {
    /// Creates a new animation with the given frames and speed.
    ///
    /// # Arguments
    ///
    /// * `frames` - A vector of frames that make up the animation.
    /// * `speed` - The duration between each frame.
    ///
    /// # Returns
    ///
    /// The newly created animation.
    pub fn new(frames: Vec<Frame>, speed: Duration) -> Self {
        Self {
            frames,
            current_frame: 0,
            current_subframe: 0,
            paused: false,
            speed,
        }
    }

    /// Advances the animation to the next frame.
    ///
    /// # Errors
    ///
    /// Returns an `io::Result` if there is an error displaying the frame.
    pub fn next_frame(&mut self) -> io::Result<()> {
        if !self.paused {
            self.advance_frame();
            thread::sleep(self.speed);
        }
        let frame = &self.frames[self.current_frame];
        frame.display(self.current_subframe)
    }

    /// Advances the animation to the next subframe within the current frame. You shouldn't use this, use `next_frame` instead.
    pub fn advance_frame(&mut self) {
        let frame = &self.frames[self.current_frame];
        self.current_subframe = (self.current_subframe + 1) % frame.content.len();
        if self.current_subframe == 0 {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    /// Checks if the animation has finished.
    pub fn is_finished(&self) -> bool {
        self.current_frame >= self.frames.len()
    }

    /// Pauses the animation.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes the animation.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Checks if the animation is currently paused.
    ///
    /// # Returns
    ///
    /// `true` if the animation is paused, `false` otherwise.
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

