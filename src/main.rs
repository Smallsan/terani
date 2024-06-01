use std::time::Duration;

use terani::{Animation, Frame, Climbing, Scheduler};

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
