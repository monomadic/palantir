use crate::{matchers, NomResult, Span};
use site::Renderable;

#[derive(Debug)]
pub enum Expression {
    Text(String),
    BoldText(Vec<Expression>),
}

pub(crate) fn expression<'a>(i: Span<'a>) -> NomResult<Expression> {
    nom::branch::alt((
        nom::combinator::map(matchers::bold, Expression::BoldText),
        nom::combinator::map(matchers::text, |t| {
            Expression::Text(t.fragment().to_string())
        }),
    ))(i)
}

impl Renderable for Expression {
    fn render_html(&self) -> String {
        match self {
            Expression::Text(s) => s.into(),
            Expression::BoldText(expr) => format!("<strong>{}</strong>", expr.render_html()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heading() {
        assert!(expression(Span::from("")).is_err());
        assert_eq!(
            expression(Span::from("**hello**")).unwrap().1.render_html(),
            "<strong>hello</strong>"
        );
    }
}
