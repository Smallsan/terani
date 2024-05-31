use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

mod structs;
use structs::animation;
use structs::frame;

pub struct Animation {
    frames: Vec<Frame>,
    current_frame: usize,
    current_subframe: usize,
    paused: bool,
}

impl Animation {
    fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            current_frame: 0,
            current_subframe: 0,
            paused: false,
        }
    }

    fn next_frame(&mut self) -> (&Frame, usize) {
        if !self.paused {
            // If the animation is not paused, get the current frame
            let frame = &self.frames[self.current_frame];
    
            // Increment the current subframe, wrapping around to 0 if it exceeds the number of subframes
            self.current_subframe = (self.current_subframe + 1) % frame.content.len();
    
            // If the current subframe is 0 (which means we've just wrapped around), increment the current frame,
            // wrapping around to 0 if it exceeds the number of frames
            if self.current_subframe == 0 {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
            }
    
            // Return the current frame
            (frame, self.current_subframe)
        } else {
            // If the animation is paused, simply return the current frame without advancing the animation
            (&self.frames[self.current_frame], self.current_subframe)
        }
    }

    fn current_subframe(&self) -> usize {
        self.current_subframe
    }

    fn pause(&mut self) {
        self.paused = true;
    }

    fn resume(&mut self) {
        self.paused = false;
    }
    fn is_paused(&self) -> bool {
        self.paused
    }
}

struct Frame {
    /// The content of the frame.
    content: Vec<String>,
}

impl Frame {
        /// Split the text into individual characters.
        /// For the typewriter effect.
    fn new (text: &str) -> Self {
        let mut content = vec![];
        let mut current = String::new();
        for character in text.chars() {
            current.push(character);
            content.push(current.clone());
        }
        Self { content }
    }
    /// Display the frame.
    fn display(&self, frame_number: usize) -> io::Result<()> {
        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H"); 
        stdout().write_all(self.content[frame_number].as_bytes())?;
        stdout().flush()
    }
}

fn main() {
    let frames = vec![
        Frame::new("I am in love with you please marry me!"),
        Frame::new("Alice alice alice alice alice alice"),
        Frame::new("I want to be with you forever!"),
    ];
    let mut animation = Animation::new(frames);
    loop {
        let (current_frame, current_subframe) = animation.next_frame();
        current_frame.display(current_subframe).unwrap();
        thread::sleep(Duration::from_millis(100));
        if !animation.is_paused() {
            animation.pause();
            animation.resume();
        } 
    }
}