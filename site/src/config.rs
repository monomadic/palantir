use std::path::{Path, PathBuf};

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
    /// Converts a local file path into a url path.
    pub(crate) fn get_route(&self, path: impl AsRef<Path>) -> String {
        "/".into()
    }

    /// Returns the local file output path for a given url.
    pub(crate) fn get_output_path(&self, path: &str) -> PathBuf {
        // let path: String = path.strip_prefix("/").unwrap_or(path).into();
        // let mut local_path = PathBuf::new().join(".").join(self.base_path.clone());
        // local_path.push(path);
        // local_path.push("index.html");

        // //

        // // let mut path: String = path.into();
        // // path.insert(0, '.');
        // // path.to_string.push_str("/index.md");
        // local_path
        // println!("config {:?}", self.base_path);

        PathBuf::from(path)
            .strip_prefix("/")
            .unwrap_or(&PathBuf::from(path));

        PathBuf::from(self.base_path.clone()).join("index.html")

        // self.base_path.join(path).join("index.html")
    }

    pub(crate) fn glob_templates(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut glob_dir = self.base_path.clone();
        glob_dir.push(self.page_glob.clone());
        println!("{:?}", glob_dir);

        // glob::glob(&glob_dir.to_str().unwrap())?.filter_map(Result::ok)
        // .collect()
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_get_route(base_path: &str, path: &str, local_path: &str) {
        let config = Config {
            base_path: PathBuf::from(base_path),
            ..Config::default()
        };

        assert_eq!(config.get_route(path), local_path);
    }

    fn assert_get_output_path(base_path: &str, path: &str, local_path: &str) {
        let config = Config {
            base_path: PathBuf::from(base_path),
            ..Config::default()
        };

        assert_eq!(config.get_output_path(path).to_str().unwrap(), local_path);
    }

    #[test]
    fn test_get_output_path() {
        assert_get_output_path("", "", "index.html");
        assert_get_output_path("", "/", "index.html");
        assert_get_output_path("", "../", "index.html");
        assert_get_output_path("", "../../posts/../", "index.html");
        assert_get_output_path("base", "/", "base/index.html");
        assert_get_output_path("base/posts", "../", "base/posts/index.html");
        // assert_get_output_path("base", "/posts.html", "base/posts/index.html"); // todo: sophisticated routes
    }

    #[test]
    fn test_get_route() {
        assert_get_route("base", "base/index.html", "/base/");
    }
}
