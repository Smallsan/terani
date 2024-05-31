use std::io;
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};

mod text_effects;

use text_effects::text_effect::TextEffect;
use text_effects::scattering::Scattering;
use text_effects::typewriter::Typewriter;
use text_effects::scrolling::{ScrollingLeft, ScrollingRight};

mod structs;

use structs::animation;
use structs::frame;

pub struct Animation<TE: TextEffect> {
    frames: Vec<Frame<TE>>,
    current_frame: usize,
    current_subframe: usize,
    paused: bool,
}

impl<TE: TextEffect> Animation<TE> {
    fn new(frames: Vec<Frame<TE>>) -> Self {
        Self {
            frames,
            current_frame: 0,
            current_subframe: 0,
            paused: false,
        }
    }

    fn next_frame(&mut self) -> (&Frame<TE>, usize) {
        if !self.paused {
            let frame = &self.frames[self.current_frame];
            self.current_subframe = (self.current_subframe + 1) % frame.content.len();
            if self.current_subframe == 0 {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
            }
            (frame, self.current_subframe)
        } else {
            (&self.frames[self.current_frame], self.current_subframe)
        }
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

pub struct Frame<TE: TextEffect> {
    content: Vec<String>,
    text_effect: TE,
}

impl<TE: TextEffect> Frame<TE> {
    pub fn new(text: &str, text_effect: TE) -> Self {
        Self {
            content: text_effect.apply(text),
            text_effect,
        }
    }

    fn display(&self, frame_number: usize) -> io::Result<()> {
        if frame_number >= self.content.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid frame number"));
        }
        self.text_effect.display(&self.content[frame_number])
    }
}

fn main() {
    let frames = vec![
        Frame::new("I am in love with you please marry me!", ScrollingLeft),
        Frame::new("Alice alice alice alice alice alice", ScrollingLeft),
        Frame::new("I want to be with you forever!", ScrollingLeft),
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