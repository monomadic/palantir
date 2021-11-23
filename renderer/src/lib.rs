use parser::Statement;
use std::fmt::Formatter;

pub trait Renderable {
    fn render_html(&self) -> String;
}

impl Renderable for Statement {
    fn render_html(&self) -> String {
        String::from("rrr")
    }
}

impl std::fmt::Debug for dyn Renderable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "<Renderable>");
        Ok(())
    }
}
