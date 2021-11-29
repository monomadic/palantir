#[macro_use]
extern crate log;

// mod page;
mod config;
mod router;
mod site;

// pub use page::Page;
pub use config::Config;
pub use site::Site;

/*
- contains all renderable pages
- routes paths to local files
- monitors filesystem for changes
- notifies server of changes
*/

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

// pub trait Resource {
//     mut Content;
//     fn update(&mut self);
//     fn read();
// }

impl<R: Renderable> Renderable for Vec<R> {
    fn render_html(&self) -> String {
        self.iter().map(|i| i.render_html()).collect::<String>()
    }
}
