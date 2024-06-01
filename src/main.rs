use std::time::Duration;

use terani::{Animation, Blinking, Climbing, Frame, TextEffect};

fn main() {
    let frames = vec![
        Frame::new("I am in love with you please marry me!", Box::new(Climbing)),
        Frame::new("Alice alice alice alice alice alice", Box::new(Blinking)),
        Frame::new("I want to be with you forever!", Box::new(Climbing)),
    ];
    let mut animation = Animation::new(frames, Duration::from_millis(100));
    loop {
        if let Err(e) = animation.next_frame() {
            eprintln!("Error displaying frame: {}", e);
            break;
        }
    }
}
