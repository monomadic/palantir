use nom_locate::LocatedSpan;
mod statement;

pub use statement::Statement;
pub type Span<'a> = LocatedSpan<&'a str, &'a str>;

pub fn parse_line<'a, S: Into<Span<'a>>>(
    _line: S,
) -> Result<Statement, Box<dyn std::error::Error>> {
    Ok(Statement::Text("example text".into()))
}
