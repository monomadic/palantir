use crate::Renderable;
use std::collections::HashMap;
use std::path::Path;
/*
- contains all renderable pages
- monitors filesystem for changes
- notifies server of changes
*/

#[derive(Default)]
pub struct Site<R: Renderable> {
    // pub base_path: PathBuf,
    // pub output_path: PathBuf,
    pub documents: HashMap<String, R>, // hashmap
                                       // server: S,
                                       // parser: P,
}

impl<R: Renderable> Site<R> {
    pub fn read<P: AsRef<Path>>(_path: P) -> Result<Site<R>, Box<dyn std::error::Error>> {
        // pretend to read all the files in the dir here

        Ok(Self {
            documents: HashMap::new(),
        })
    }

    pub fn request(&self, path: &str) -> String {
        format!("requested: {}", path)
    }
}

// impl Servable for Site {
//     fn request(path: &str) {
//         println!("requested: {}", path);
//     }
// }
