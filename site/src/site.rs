use crate::{Config, Parser, Renderable};
use std::collections::HashMap;

#[derive(Default)]
pub struct Site<R: Renderable, P: Parser<R>> {
    // pub base_path: PathBuf,
    // pub output_path: PathBuf,
    // page_cache: HashMap<String, String>,
    ast_cache: HashMap<String, R>, // localpath, ast
    config: Config,
    parser: P,
}

impl<R: Renderable, P: Parser<R>> Site<R, P> {
    pub fn new(parser: P) -> Self {
        Self {
            ast_cache: HashMap::new(),
            config: Config::default(),
            parser,
        }
    }

    pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for file in glob::glob(&self.config.page_glob)? {
            match file {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(())
    }

    pub fn request(&self, path: &str) -> &R {
        let local_path = crate::router::route(path);

        info!("Requested {}", local_path);

        match self.ast_cache.get(&local_path) {
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
