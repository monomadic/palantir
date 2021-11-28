// use crate::{matchers, NomResult, Span};
// use site::Renderable;

// #[derive(Debug)]
// pub enum Statement {
//     Text(String),
//     BoldText(Vec<Statement>),
//     Heading(matchers::Heading),
// }

// pub(crate) fn statement<'a>(i: Span<'a>) -> NomResult<Statement> {
//     nom::multi::tuple((
//         nom::branch::alt((
//             nom::combinator::map(matchers::heading, Statement::Heading),
//             nom::combinator::map(matchers::text, |s| {
//                 Statement::Text(s.fragment().to_string())
//             }),
//         )),

//     ))(i)
// }

// pub(crate) fn statements<'a>(i: Span<'a>) -> NomResult<Vec<Statement>> {
//     nom::multi::many0(statement)(i)
// }

// impl Renderable for Statement {
//     fn render_html(&self) -> String {
//         match self {
//             Statement::Text(s) => String::from(s),
//             Statement::Heading(h) => h.render_html(),
//             _ => unimplemented!(),
//         }
//     }
// }

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
