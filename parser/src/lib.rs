mod matchers;
// pub use matchers::Statement;
mod expression;
mod statement;

use nom::Finish;
use nom_locate::LocatedSpan;
pub use site::Renderable;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub(crate) type NomResult<'a, T> = nom::IResult<Span<'a>, T>;

#[derive(Debug)]
pub struct AST {
    nodes: Vec<statement::Statement>, // don't leak this
}

impl Renderable for AST {
    fn render_html(&self) -> String {
        self.nodes.render_html()
    }
}

pub fn parse<'a>(i: impl Into<Span<'a>>) -> Result<AST, String> {
    match statement::statements(i.into()).finish() {
        Ok((r, nodes)) => {
            if r.len() != 0 {
                return Err(format!("unexpected EOF: {}", r));
            } else {
                Ok(AST { nodes })
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
