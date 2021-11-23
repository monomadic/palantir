use nom_locate::LocatedSpan;
mod statement;

pub use statement::Statement;
pub type Span<'a> = LocatedSpan<&'a str, &'a str>;

pub fn parse(_lines: Span) -> Result<Statement, Box<dyn std::error::Error>> {
    Ok(Statement {})
}
