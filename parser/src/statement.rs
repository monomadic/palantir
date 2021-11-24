use crate::heading::Heading;
use site::Renderable;

#[derive(Debug)]
pub enum Statement {
    Text(String),
    BoldText(Vec<Statement>),
    Heading(Heading),
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
            Statement::Heading(h) => h.render_html(),
            _ => unimplemented!(),
        }
    }
}
