use crate::{NomResult, Span};

pub(crate) fn text<'a>(i: Span<'a>) -> NomResult<Span<'a>> {
    nom::bytes::complete::is_not("\n")(i)
}
