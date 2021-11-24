use site::Document;
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::new(PathBuf::from("./"))?;
    doc.read()?;

    println!("{}", doc.render_html());
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("done."),
        Err(e) => println!("{}", e), // todo: print error properly
    }
}
