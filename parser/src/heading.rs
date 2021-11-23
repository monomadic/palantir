use crate::statement::Statement;

#[derive(Debug, Default)]
pub(crate) struct Heading {
    level: u8,
    content: Vec<Statement>,
}

fn matcher<'a>(i: Span<'a>) -> IResult<Span<'a>, Statement<'a>, ParserError<Span<'a>>> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("#"),
        nom::character::complete::space1,
        nom::character::complete::alphanumeric1,
    ))
}

fn render_html(statements: Vec<Statement>) -> String {
    format!("<h1>{}</h1>", statements.iter().map(render_html))
}

// fn render_markdown()
