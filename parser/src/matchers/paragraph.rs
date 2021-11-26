use crate::{NomResult, Span};

pub(crate) fn paragraph<'a>(i: Span<'a>) -> NomResult<Vec<crate::expression::Expression>> {
    nom::multi::many0(crate::expression::expression)(i)
}
