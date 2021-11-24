use crate::{IResult, Span};

pub(crate) fn text<'a>(i: Span<'a>) -> IResult<Span<'a>, Span<'a>> {
    nom::bytes::complete::is_not("\n")(i)
}
