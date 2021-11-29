use std::fmt;
type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub enum ParserError {
    UnexpectedData(String),
    Other(BoxError),
    Message(String),
}

impl ParserError {
    pub(crate) fn new<E: Into<BoxError>>(err: E) -> Self {
        ParserError::Other(err.into())
    }
}

impl std::error::Error for ParserError {}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Skip showing worthless `Error { .. }` wrapper.
        fmt::Debug::fmt("blah", f)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("self.inner", f)
    }
}
