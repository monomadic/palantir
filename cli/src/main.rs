use page::{Document, Page, Renderable};
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("./examples/index.md");
    let mut page = Page::new(path);
    page.read()?;
    page.parse()?;
    println!("{}", page.render_html());
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("done."),
        Err(e) => println!("{}", e), // todo: print error properly
    }
}
