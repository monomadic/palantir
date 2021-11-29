pub(crate) fn to_local_path(_path: &str) -> String {
    format!("examples/index.md")
}

pub(crate) fn from_local_path(path: &str) -> String {
    format!("/{}", path)
}
