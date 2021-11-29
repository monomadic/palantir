use glob::Paths;

pub(crate) fn glob_files<S: ToString>(pattern: S) -> Result<(), Box<dyn std::error::Error>> {
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut files = Vec::new();
    let globs: Paths = glob::glob_with(&pattern.to_string(), options)
        .map_err(|_| AstryxError::Generic("glob error".into()))?;

    for file in globs {
        // TODO wrap unwrap in error
        let path = file.expect("file to unwrap");
        let filepath: String = path.as_os_str().to_str().unwrap().into();

        files.push(Node::new(Object::Path(filepath)));
    }

    Ok(Object::Array(files))
}
