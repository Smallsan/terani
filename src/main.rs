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

use structs::animation::Animation;
use structs::frame::Frame;


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