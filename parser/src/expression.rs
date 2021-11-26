use crate::{matchers::text, IResult, Span};
use site::Renderable;

#[derive(Debug)]
pub enum Expression {
    Text(String),
    BoldText(Vec<Expression>),
}

pub(crate) fn expression<'a>(i: Span<'a>) -> IResult<Span<'a>, Expression> {
    text(i).map(|(r, t)| (r, Expression::Text(t.fragment().to_string())))
}

impl Renderable for Expression {
    fn render_html(&self) -> String {
        match self {
            Statement::Heading((level, expr)) => {
                format!("<h{}>{}</h{}>", level, expr.render_html(), level)
            }
            Statement::Paragraph(expr) => expr.render_html(),
            _ => unimplemented!(),
        }
    }
}
