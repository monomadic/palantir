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
    // /// Converts a file path to an url path.
    // pub(crate) fn route(&self, path: impl AsRef<Path>) -> String {
    //     "/".into()
    // }

    /// Returns the default file output location for a given url path.
    pub(crate) fn output_path(&self, route: &str) -> PathBuf {
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

        PathBuf::from(route)
            .strip_prefix("/")
            .unwrap_or(&PathBuf::from(route));

        PathBuf::from(self.base_path.clone()).join("index.html")

        // self.base_path.join(path).join("index.html")
    }

    pub(crate) fn templates_glob(&self) -> String {
        self.base_path
            .join(self.page_glob.clone())
            .to_str()
            .unwrap_or("content/*.md")
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // fn assert_route(base_path: &str, path: &str, local_path: &str) {
    //     let config = Config {
    //         base_path: PathBuf::from(base_path),
    //         ..Config::default()
    //     };

    //     assert_eq!(config.route(path), local_path);
    // }

    fn assert_output_path(base_path: &str, path: &str, local_path: &str) {
        let config = Config {
            base_path: PathBuf::from(base_path),
            ..Config::default()
        };

        assert_eq!(config.output_path(path).to_str().unwrap(), local_path);
    }

    #[test]
    fn test_output_path() {
        assert_output_path("", "", "index.html");
        assert_output_path("", "/", "index.html");
        assert_output_path("", "../", "index.html");
        assert_output_path("", "../../posts/../", "index.html");
        assert_output_path("base", "/", "base/index.html");
        assert_output_path("base/posts", "../", "base/posts/index.html");
        // assert_get_output_path("base", "/posts.html", "base/posts/index.html"); // todo: sophisticated routes
    }

    // #[test]
    // fn test_route() {
    //     assert_route("base", "base/index.html", "/");
    // }
}
