use crate::text;
use crate::{IResult, Span};
use site::Renderable;

#[derive(Debug, Default)]
pub struct Heading {
    level: u8,
    content: String,
}

pub(crate) fn heading<'a>(i: Span<'a>) -> IResult<Span<'a>, Heading> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("#"),
        nom::character::complete::space1,
        text,
    ))(i)
    .map(|(r, (_, _, text))| {
        (
            r,
            Heading {
                level: 1,
                content: String::from(&**text.fragment()),
            },
        )
    })
}

impl Renderable for Heading {
    fn render_html(&self) -> String {
        format!("<h1>{}</h1>", self.content)
    }
}

// fn matcher<'a>(i: Span<'a>) -> IResult<Span<'a>, Statement<'a>, ParserError<Span<'a>>> {
//     nom::sequence::tuple((
//         nom::bytes::complete::tag("#"),
//         nom::character::complete::space1,
//         nom::character::complete::alphanumeric1,
//     ))
// }

// fn render_html(statements: Vec<Statement>) -> String {
//     format!("<h1>{}</h1>", statements.iter().map(render_html))
// }

// fn render_markdown()
