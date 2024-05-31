use std::time::Duration;
use std::io::{self, stdout, Write};
use std::thread;

mod structs;
use structs::animation;
use structs::frame;


pub struct Animation {
    frames: Vec<Frame>,
    current_frame: usize,
}

impl Animation {
    fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            current_frame: 0,
        }
    }

    fn next_frame(&mut self) -> &Frame {
        let frame = &self.frames[self.current_frame];
        self.current_frame = (self.current_frame + 1) % self.frames.len();
        frame
    }
}


struct Frame {
    content: String,
}

impl Frame {
    fn display(&self) -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H");
        stdout().write_all(self.content.as_bytes())?;
        stdout().flush()
    }
}




fn main() {
    let frames = vec![
        Frame { content: String::from("Frame 1") },
        Frame { content: String::from("Frame 2") },
        Frame { content: String::from("Frame 3") },
    ];
    let mut animation = Animation::new(frames);
    loop {
        let frame = animation.next_frame();
        frame.display().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}


