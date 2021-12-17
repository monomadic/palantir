#[macro_use]
extern crate log;

// mod page;
mod config;
mod site;
mod template;

// pub use page::Page;
pub use crate::{config::Config, site::Site};

pub trait Renderable {
    fn render_html(&self) -> String;
}

// todo: get rid of the boxes
pub trait Parser<R: Renderable> {
    fn parse(&self, i: &str) -> Result<R, Box<dyn std::error::Error>>;
}

pub trait Servable {
    fn request(path: &str);
}

impl<R: Renderable> Renderable for Vec<R> {
    fn render_html(&self) -> String {
        self.iter().map(|i| i.render_html()).collect::<String>()
    }
}
