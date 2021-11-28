use std::path::Path;

pub trait Document {
    fn new(path: impl AsRef<Path>) -> Self;
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
