use nom_locate::LocatedSpan;
mod heading;
mod statement;

use nom::{Finish, IResult};

pub use statement::Statement;
pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T> = Result<T, String>;

pub(crate) fn text<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>> {
    nom::bytes::complete::is_not("\n")(i)
}

// fn parse_line<'a, S: Into<Span<'a>>>(i: S) -> IResult<Span<'a>, Statement> {
//     text(i.into()).map(|(r, s)| (r.into(), Statement::Text(String::from(&**s.fragment()))))
// }

fn parse_line<'a>(i: Span<'a>) -> IResult<Span<'a>, Statement> {
    heading::heading(i).map(|(r, h)| (r, Statement::Heading(h)))
}

pub fn parse<'a, S: Into<Span<'a>>>(i: S) -> Result<Statement, String> {
    parse_line(i.into())
        .finish()
        .map(|(_, doc)| doc)
        .map_err(|e| e.to_string())
}
