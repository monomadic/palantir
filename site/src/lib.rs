use server::Servable;
use std::path::Path;

#[derive(Debug)]
pub struct Site {
    // pub base_path: PathBuf,
// pub output_path: PathBuf,
}

impl Site {
    pub fn read_from_path<P: AsRef<Path>>(_path: P) -> Result<Site, Box<dyn std::error::Error>> {
        Ok(Site {})
    }
}

impl Servable for Site {
    fn request(path: &str) {
        println!("requested: {}", path);
    }
}
