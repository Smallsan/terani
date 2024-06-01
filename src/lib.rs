pub mod text_effects;

pub use text_effects::text_effect::TextEffect;
pub use text_effects::scattering::Scattering;
pub use text_effects::typewriter::Typewriter;
pub use text_effects::scrolling::{ScrollingLeft, ScrollingRight};
pub use text_effects::blinking::Blinking;
pub use text_effects::color_changing::ColorChanging;
pub use text_effects::climbing::Climbing;

pub mod structs;

pub use structs::animation::Animation;
pub use structs::frame::Frame;
pub use structs::scheduler::Scheduler;

