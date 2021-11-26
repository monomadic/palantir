use crate::{matchers::text, NomResult, Span};
use site::Renderable;

#[derive(Debug)]
pub enum Expression {
    Text(String),
    BoldText(Vec<Expression>),
}

pub(crate) fn expression<'a>(i: Span<'a>) -> NomResult<Expression> {
    text(i).map(|(r, t)| (r, Expression::Text(t.fragment().to_string())))
}

impl Renderable for Expression {
    fn render_html(&self) -> String {
        match self {
            Expression::Text(s) => s.into(),
            Expression::BoldText(expr) => expr.render_html(),
        }
    }
}
