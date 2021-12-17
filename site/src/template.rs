use askama::Template;

#[derive(Default, Template)]
#[template(path = "../templates/frame.html", escape = "none")]
pub(crate) struct HomeTemplate {
    pub title: String,
    pub head: String,
    pub body: String,
}
