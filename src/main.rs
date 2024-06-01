use std::io;
use std::thread;
use std::time::Duration;

mod text_effects;

use text_effects::text_effect::TextEffect;
use text_effects::scattering::Scattering;
use text_effects::typewriter::Typewriter;
use text_effects::scrolling::{ScrollingLeft, ScrollingRight};
use text_effects::blinking::Blinking;
use text_effects::color_changing::ColorChanging;
use text_effects::climbing::Climbing;

mod structs;

use structs::animation;
use structs::frame;


pub struct Animation<TE: TextEffect> {
    frames: Vec<Frame<TE>>,
    current_frame: usize,
    current_subframe: usize,
    paused: bool,
    speed: Duration,
}

impl<TE: TextEffect> Animation<TE> {
    pub fn new(frames: Vec<Frame<TE>>, speed: Duration) -> Self {
        Self {
            frames,
            current_frame: 0,
            current_subframe: 0,
            paused: false,
            speed,
        }
    }
    pub fn next_frame(&mut self) -> io::Result<()> {
        if !self.paused {
            self.advance_frame();
            thread::sleep(self.speed);
        }
        let frame = &self.frames[self.current_frame];
        frame.display(self.current_subframe)
    }

    fn advance_frame(&mut self) {
        let frame = &self.frames[self.current_frame];
        self.current_subframe = (self.current_subframe + 1) % frame.content.len();
        if self.current_subframe == 0 {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_paused(&self) -> bool {
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
        Frame::new("I am in love with you please marry me!", Climbing),
        Frame::new("Alice alice alice alice alice alice", Climbing),
        Frame::new("I want to be with you forever!", Climbing),
    ];
    let mut animation = Animation::new(frames, Duration::from_millis(100));
    loop {
        if let Err(e) = animation.next_frame() {
            eprintln!("Error displaying frame: {}", e);
            break;
        }
    }
}