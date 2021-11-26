use crate::{IResult, Span};

pub(crate) fn paragraph<'a>(i: Span<'a>) -> IResult<Span<'a>, Vec<crate::expression::Expression>> {
    nom::multi::many0(crate::expression::expression)(i)
}
