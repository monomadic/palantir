use renderer::Renderable;
use site::Document;
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Document::read(PathBuf::from("./"))?.render_html());
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("done."),
        Err(e) => println!("{}", e), // todo: print error properly
    }
}
