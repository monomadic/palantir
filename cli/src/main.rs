#[macro_use]
extern crate log;

mod args;

fn main() {
    env_logger::init();
    match args::run() {
        Ok(_) => info!("done"),
        Err(e) => error!("{}", e), // todo: print error properly
    }
}
