use std::io;

pub trait TextEffect {
    fn apply(&self, text: &str) -> Vec<String>;
    fn display(&self, text: &str) -> io::Result<()>;
}