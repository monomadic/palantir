#[macro_use]
extern crate log;

mod args;

fn main() {
    // set up a logger with default level 'info'
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    match args::run() {
        Ok(_) => info!("done"),
        Err(e) => error!("{}", e), // todo: print error properly
    }
}
