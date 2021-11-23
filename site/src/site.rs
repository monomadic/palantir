use crate::Document;
use parser;
use server::Servable;
use std::path::Path;

#[derive(Default)]
pub struct Site {
    // pub base_path: PathBuf,
    // pub output_path: PathBuf,
    pub documents: Vec<Document>, // hashmap
}

impl Site {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Site, Box<dyn std::error::Error>> {
        // let file = std::fs::read_to_string(path)?;
        // let line = parser::parse_line(file.as_str())?;

        std::fs::read_to_string(path)
            .and_then(|file| parser::parse_line(file.as_str()))
            .map(|statement| Site {
                documents: vec![statement],
            })
            .map_err(|err| err.into())
    }
}

impl Servable for Site {
    fn request(path: &str) {
        println!("requested: {}", path);
    }
}
