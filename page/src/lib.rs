pub use site::{Document, Renderable};
use std::path::{Path, PathBuf};

pub struct Page {
    path: PathBuf,
    data: Option<String>,
}

impl Document for Page {
    fn new(path: impl AsRef<Path>) -> Self {
        Page {
            path: path.as_ref().to_path_buf(),
            data: None,
        }
    }
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.data = Some(std::fs::read_to_string(&self.path)?);
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
        let data: &str = &(self.data.clone().unwrap()).clone(); // fix this garbage
        let html = parser::parse(data)
            .unwrap()
            .iter()
            .map(|s| s.render_html())
            .collect::<String>();
        String::from(html)
    }
}
