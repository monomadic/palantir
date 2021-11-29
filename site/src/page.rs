pub use crate::{Document, Renderable};
use log::{info, warn};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Page<R: Renderable> {
    file_path: PathBuf,
    file_content: Option<String>,
    ast: Option<R>,
}

pub enum PageError {
    IO(std::io::Error),
}

impl Document for Page {
    fn new(path: impl AsRef<Path>) -> Self {
        info!("reading {}", path.as_ref().display());
        Self {
            file_path: path.as_ref().to_path_buf(),
            ..Self::default()
        }
    }
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::read_to_string(&self.file_path)
            .map(|content| self.file_content = Some(content))?;
        Ok(())
    }
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.file_content {
            Some(file_content) => {
                self.ast = Some(parser::parse(data.clone().as_str())?);
            }
            None => {
                self.read()?;
                self.parse()?;
            }
        }
        Ok(())
    }
}

// impl Renderable for Page {
//     fn render_html(&self) -> String {
//         match &self.ast {
//             Some(ast) => ast.render_html(),
//             None => unimplemented!("ast not present (implement fallback)"),
//         }
//     }
// }
