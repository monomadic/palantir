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
            // page_cache: HashMap::new(),
            ast_cache: HashMap::new(),
            config: Config::default(),
            parser,
        }
    }

    pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for file in glob::glob(&self.config.page_glob)? {
            match file {
                Ok(path) => {
                    self.update_cache(&self.config.from_local_path(path.to_str().unwrap().into()))?
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(())
    }

    pub fn update_cache(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match self.ast_cache.get(path) {
            Some(_) => (),
            None => {
                warn!("File {} is not in AST cache, attempting to parse it", path);
                let local_path = self.config.to_local_path(path);
                let file = std::fs::read_to_string(&local_path)?;
                // self.page_cache.insert(local_path, file);

                let ast = self.parser.parse(&*file)?;
                info!("Parsed {} as {}", local_path, path);
                self.ast_cache.insert(path.into(), ast);
                ()
            }
        })
    }

    pub fn get_renderer(&self, path: &str) -> Result<&R, Box<dyn std::error::Error>> {
        info!("Requested {}", path);

        // self.update_cache(path)?;
        Ok(self.ast_cache.get(path).expect(&format!(
            "cache error for: {:?}\ncache keys:{:?}",
            path,
            self.ast_cache.keys()
        )))
    }

    pub fn render_html(&self, path: &str) -> String {
        match self.ast_cache.get(path) {
            Some(doc) => doc.render_html(),
            None => format!(
                "cache error for: {}\ncache keys: {:?}",
                path,
                self.ast_cache.keys()
            ),
        }
    }

    /// request a html page for a given url path
    pub fn request_html(&self, path: &str) -> String {
        match self.get_renderer(path) {
            Ok(page) => page.render_html(),
            Err(e) => format!("error {:?}", e),
        }
    }

    pub fn write_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
