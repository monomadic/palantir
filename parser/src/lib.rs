mod matchers;
// pub use matchers::Statement;
mod expression;
mod statement;

use nom::Finish;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T> = Result<T, String>;
pub(crate) type NomResult<'a, T> = nom::IResult<Span<'a>, T>;

pub fn parse<'a>(i: impl Into<Span<'a>>) -> Result<Vec<statement::Statement>, String> {
    statement::statements(i.into())
        .finish()
        .map(|(_, statements)| statements)
        .map_err(|e| e.to_string())
}
