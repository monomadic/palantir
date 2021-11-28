use crate::{expression::Expression, NomResult, Span};

fn text_characters(i: Span) -> NomResult<Span> {
    nom::bytes::complete::is_not("*#{}\n")(i)
}

pub(crate) fn text<'a>(i: Span<'a>) -> NomResult<Span<'a>> {
    text_characters(i)
    // nom::character::complete::alphanumeric1(i)
    // nom::sequence::tuple((
    //     nom::character::complete::alphanumeric0,
    // ))(i)
    // nom::bytes::complete::is_not("\n")(i)
}

pub(crate) fn bold<'a>(i: Span<'a>) -> NomResult<Vec<Expression>> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("**"),
        // nom::character::complete::alphanumeric0,
        // nom::multi::many0(crate::expression::expression),
        text_characters,
        nom::bytes::complete::tag("**"),
    ))(i)
    // .map(|(r, (_, exprs, _))| (r, exprs))
    .map(|(r, (_, text, _))| (r, vec![Expression::Text(text.fragment().to_string())]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_text_characters() {
        assert!(text_characters(Span::from("")).is_err());
        assert!(text_characters(Span::from("\n")).is_err());
        assert!(text_characters(Span::from("*")).is_err());
        assert_eq!(
            text_characters(Span::from("a*")).unwrap().1.to_string(),
            "a"
        );
        assert!(text_characters(Span::from("aaaa")).is_ok());
        assert!(text_characters(Span::from("oh hello there!")).is_ok());
    }

    #[test]
    fn test_bold() {
        assert!(bold(Span::new_extra("", "")).is_err());
        assert_eq!(
            &format!("{:?}", bold(Span::from("**hi**")).unwrap().1),
            "[Text(\"hi\")]"
        );
        assert_eq!(bold(Span::from("**hi**")).unwrap().0.to_string(), "");
        assert_eq!(bold(Span::from("**hi**\n")).unwrap().0.to_string(), "\n");
    }
}
