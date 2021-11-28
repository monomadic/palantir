use crate::{NomResult, Span};

pub(crate) fn heading<'a>(i: Span<'a>) -> NomResult<(usize, Vec<crate::expression::Expression>)> {
    nom::sequence::tuple((
        nom::multi::many0(nom::bytes::complete::tag("#")),
        nom::character::complete::space1,
        nom::multi::many0(crate::expression::expression),
    ))(i)
    .map(|(r, (hash, _, expr))| (r, (hash.len(), expr)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heading() {
        assert!(heading(Span::from("")).is_err());
        assert!(heading(Span::from("# hello")).is_ok());
    }
}
