use crate::{Animation, TextEffect};

pub struct Scheduler<TE: TextEffect> {
    pub animations: Vec<Animation<TE>>,
}

impl<TE: TextEffect> Scheduler<TE> {
    pub fn new(animations: Vec<Animation<TE>>) -> Self {
        Self { animations }
    }

    pub fn update(&mut self) {
        for animation in &mut self.animations {
            animation.advance_frame();
        }
        self.animations.retain(|animation| !animation.is_finished());
    }
}