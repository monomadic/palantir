use nom_locate::LocatedSpan;

mod matchers;

use nom::{Finish, IResult};

use matchers::{statement, Statement};
pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
pub type ParserResult<T> = Result<T, String>;

pub fn parse<'a, S: Into<Span<'a>>>(i: S) -> Result<Statement, String> {
    statement(i.into())
        .finish()
        .map(|(_, doc)| doc)
        .map_err(|e| e.to_string())
}
