use site;
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    site::Site::read_from_path(PathBuf::from("./")).map(|_| ())
}

fn main() {
    match run() {
        Ok(_) => println!("done."),
        Err(e) => println!("{}", e), // todo: print error properly
    }
}
