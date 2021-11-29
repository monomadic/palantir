use crate::{Config, Renderable};
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct Site<R: Renderable> {
    // pub base_path: PathBuf,
    // pub output_path: PathBuf,
    // page_cache: HashMap<String, String>,
    documents: HashMap<String, R>, // localpath, ast
    _config: Config,
}

impl<R: Renderable> Site<R> {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            // page_cache: HashMap::new(),
            _config: Config::default(),
        }
    }

    pub fn read<P: AsRef<Path>>(_path: P) -> Result<Site<R>, Box<dyn std::error::Error>> {
        Ok(Self::new())
    }

    pub fn request(&self, path: &str) -> &R {
        let local_path = crate::router::route(path);

        info!("Requested {}", local_path);

        match self.documents.get(&local_path) {
            Some(doc) => doc,
            None => todo!("check for the file"),
        }
    }

    pub fn request_html(&self, path: &str) -> String {
        self.request(path).render_html()

        // let local_path = crate::router::route(path);
        // info!("Requested {}", local_path);

        // match self.documents.get(&local_path) {
        //     Some(doc) => doc.render_html(),
        //     None => String::from("404"),
        // }
    }

    pub fn write_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
