mod document;
// mod site;

pub use document::Document;
// pub use site::Site;

pub trait Renderable {
    fn render_html(&self) -> String;
}

// todo: get rid of the boxes
pub trait Parseable {
    fn parse(&self) -> Result<Box<dyn Parseable>, Box<dyn std::error::Error>>;
}

pub trait Servable {
    fn request(path: &str);
}

// pub trait Resource {
//     mut Content;
//     fn update(&mut self);
//     fn read();
// }

impl<T> Renderable for Vec<T> {
    fn render_html(&self) -> String {
        self.iter().map(|i| i.render_html()).collect::<String>()s
    }
}
