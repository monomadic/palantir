use renderer::Renderable;
use std::path::Path;

#[derive(Default, Debug)]
pub struct Document {
    // title: String,
    pub nodes: Vec<Box<dyn Renderable>>,
}

impl Document {
    pub fn read<P: AsRef<Path>>(_path: P) -> Result<Document, Box<dyn std::error::Error>> {
        Ok(Document::default())
    }
}

impl Renderable for Document {
    fn render_html(&self) -> String {
        self.nodes
            .iter()
            .map(|doc| doc.render_html())
            .collect::<String>()
    }
}
