use crate::matchers;
use crate::{IResult, Span};
use site::Renderable;

#[derive(Debug, Default)]
pub struct Heading {
    level: usize,
    content: String,
}

pub(crate) fn heading<'a>(
    i: Span<'a>,
) -> IResult<Span<'a>, (usize, Vec<crate::expression::Expression>)> {
    nom::sequence::tuple((
        nom::multi::many0(nom::bytes::complete::tag("#")),
        nom::character::complete::space1,
        nom::multi::many0(crate::expression::expression),
    ))(i)
    .map(|(r, (hash, _, expr))| (r, (hash.len(), expr)))
}

impl Renderable for Heading {
    fn render_html(&self) -> String {
        format!("<h{}>{}</h{}>", self.level, self.content, self.level)
    }
}
