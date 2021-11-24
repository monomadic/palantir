use site::Renderable;

#[derive(Debug)]
pub enum Statement {
    Text(String),
    BoldText(Vec<Statement>),
    Heading(Vec<Statement>),
}

impl Default for Statement {
    fn default() -> Self {
        Statement::Text(String::default())
    }
}

impl Renderable for Statement {
    fn render_html(&self) -> String {
        match self {
            Statement::Text(s) => String::from(s),
            _ => unimplemented!(),
        }
    }
}
