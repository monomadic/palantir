pub use site::{Document, Renderable};
use std::path::{Path, PathBuf};

pub struct Page {
    path: PathBuf,
    memo: String,
}

impl Document for Page {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        Page {
            path: path.as_ref().to_path_buf(),
            memo: String::new(),
        }
    }
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.memo = std::fs::read_to_string(&self.path)?;
        Ok(())
    }
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn render(&self) -> String {
        String::new()
    }
}

impl Renderable for Page {
    fn render_html(&self) -> String {
        String::from("I'm html!")
    }
}
