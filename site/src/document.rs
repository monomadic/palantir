use crate::{Parseable, Renderable};
use std::path::{Path, PathBuf};

#[derive(Default, Debug)]
pub struct Document {
    // title: Option<String>,
    // pub nodes: Node<Box<dyn Renderable>>, // needs to be a Node
    path: PathBuf,
    data: Option<String>,
    // data: T
}

impl Document {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Document, Box<dyn std::error::Error>> {
        Ok(Document {
            path: path.as_ref().to_path_buf(),
            data: None,
            // ..Self::default()
        })
    }

    // file changed; don't parse
    pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.data = Some(std::fs::read_to_string(&self.path)?);
        Ok(())
    }
}

// impl Parseable for Document {
//     // read and parse, return renderable (probably the ast of a supported language)
//     pub fn parse(&self) -> Result<Box<dyn Renderable>, Box<dyn std::error::Error>> {
//         // for now, assume we have an astryx document
//         Ok(self.parse())
//     }
// }

impl Renderable for Document {
    fn render_html(&self) -> String {
        self.data
            .iter()
            .map(|doc| doc.render_html())
            .collect::<String>()
    }
}
