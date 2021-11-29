mod matchers;
// pub use matchers::Statement;
mod error;
mod expression;
mod statement;

pub use error::ParserError;
use nom::Finish;
use nom_locate::LocatedSpan;
pub use site::{Parser, Renderable};

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub(crate) type NomResult<'a, T> = nom::IResult<Span<'a>, T>;

pub struct MarkdownParser;

impl Parser<AST> for MarkdownParser {
    fn parse(&self, i: &str) -> Result<AST, Box<dyn std::error::Error>> {
        parse(i)
    }
}

#[derive(Debug)]
pub struct AST {
    nodes: Vec<statement::Statement>, // don't leak this
}

impl Renderable for AST {
    fn render_html(&self) -> String {
        self.nodes.render_html()
    }
}

pub fn parse<'a>(i: impl Into<Span<'a>>) -> Result<AST, Box<dyn std::error::Error>> {
    match statement::statements(i.into()).finish() {
        Ok((r, nodes)) => {
            if r.len() != 0 {
                return Err(ParserError::UnexpectedData(r.to_string()).into());
            } else {
                Ok(AST { nodes })
            }
        }
        Err(e) => Err(ParserError::Message(format!("{:?}", e)).into()),
    }
}
