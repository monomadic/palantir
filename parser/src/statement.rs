use crate::{expression::Expression, NomResult, Span};
use site::Renderable;

#[derive(Debug)]
pub enum Statement {
    Heading((usize, Vec<Expression>)),
    Paragraph(Vec<Expression>),
    // UnorderedList(Vec<Statement>),
    // UnorderedListItem(Vec<Expression>),
    // OrderedList(Vec<Statement>),
    // TodoList(Vec<TodoListItem>)
}

pub(crate) fn statements<'a>(i: Span<'a>) -> NomResult<Vec<Statement>> {
    nom::multi::many0(statement)(i)
    // indent(i.fragment(), "<filename>")
    //     .map(|(r, lines)| {
    //         lines
    //             .into_iter()
    //             .map(|node| statement(node.borrow().clone().into()))
    //     })
    //     .collect()
}

pub(crate) fn statement<'a>(i: Span<'a>) -> NomResult<Statement> {
    nom::sequence::tuple((
        nom::branch::alt((
            nom::combinator::map(crate::matchers::heading, Statement::Heading),
            nom::combinator::map(crate::matchers::paragraph, Statement::Paragraph),
        )),
        nom::character::complete::multispace1,
    ))(i)
    .map(|(r, (statement, _eol))| (r, statement))
}

// pub(crate) fn statement<'a>(i: Span<'a>) -> IResult<Span<'a>, Statement> {
//     nom::sequence::tuple((
//         crate::expression::expression,
//         nom::character::complete::space0,
//         nom::character::complete::newline,
//     ))(i)
//     .map(|(r, (expr, _, _))| (r, expr))
// }

impl Renderable for Statement {
    fn render_html(&self) -> String {
        match self {
            Statement::Heading((level, expr)) => {
                format!("<h{}>{}</h{}>", level, expr.render_html(), level)
            }
            Statement::Paragraph(expr) => format!("<p>{}</p>", expr.render_html()),
        }
    }
}

// impl Into<Statement> for String {
//     fn into(self) -> Statement {
//         Statement::Text(self)
//     }
// }

// impl<'a> Into<Statement> for Span<'a> {
//     fn into(self) -> Statement {
//         Statement::Text(self.fragment().to_string())
//     }
// }
