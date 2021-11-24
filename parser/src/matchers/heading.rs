use crate::matchers;
use crate::{IResult, Span};
use site::Renderable;

#[derive(Debug, Default)]
pub struct Heading {
    level: usize,
    content: String,
}

pub(crate) fn heading<'a>(i: Span<'a>) -> IResult<Span<'a>, Heading> {
    nom::sequence::tuple((
        nom::multi::many0(nom::bytes::complete::tag("#")),
        nom::character::complete::space1,
        matchers::text,
    ))(i)
    .map(|(r, (hash, _, text))| {
        (
            r,
            Heading {
                level: hash.len(),
                content: String::from(&**text.fragment()),
            },
        )
    })
}

impl Renderable for Heading {
    fn render_html(&self) -> String {
        format!("<h{}>{}</h{}>", self.level, self.content, self.level)
    }
}
