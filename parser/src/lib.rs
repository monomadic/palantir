mod matchers;

pub use matchers::Statement;
use nom::{Finish, IResult};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T> = Result<T, String>;
pub(crate) type NomResult<T> = IResult<Span<'a>, T>;

pub fn parse<'a>(i: impl Into<Span<'a>) -> Result<Vec<Statement>, String> {
    matchers::statements(i.into())
        .finish()
        .map(|(_, statements)| statements)
        .map_err(|e| e.to_string())
}
