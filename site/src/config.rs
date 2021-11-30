use std::path::PathBuf;

pub struct Config {
    pub base_path: PathBuf,
    pub page_glob: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("examples"),
            page_glob: String::from("**/*.md"),
        }
    }
}

impl Config {
    pub(crate) fn to_local_path(&self, path: &str) -> PathBuf {
        self.base_path.join(path)
    }
    pub(crate) fn from_local_path(&self, path: &str) -> String {
        format!("/{}", path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_to_local_path(base_path: &str, path: &str, local_path: &str) {
        let config = Config {
            base_path: PathBuf::from(base_path),
            ..Config::default()
        };

        assert_eq!(config.to_local_path(path).to_str(), local_path);
    }

    #[test]
    fn test_to_local_path() {
        assert_to_local_path("base", "/", "base/index.md");
    }
}
