use page::{Document, Page, Renderable};
use std::path::Path;

pub(crate) fn build<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut page = Page::new(path.as_ref());
    page.read()?;
    page.parse()?;
    println!("{}", page.render_html());
    Ok(())
}
