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
