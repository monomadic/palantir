use std::path::PathBuf;

pub struct Config {
    pub base_path: PathBuf,
    pub page_glob: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("."),
            page_glob: String::from("examples/**/*.md"),
        }
    }
}
