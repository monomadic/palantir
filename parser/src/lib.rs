mod matchers;

pub use matchers::Statement;
use nom::{Finish, IResult};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T> = Result<T, String>;

pub fn parse<'a, S: Into<Span<'a>>>(i: S) -> Result<Vec<Statement>, String> {
    matchers::statements(i.into())
        .finish()
        .map(|(_, statements)| statements)
        .map_err(|e| e.to_string())
}
