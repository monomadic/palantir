use clap::Parser;
use std::path::PathBuf;

mod build;
mod serve;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Command::parse() {
        Command::Build { input } => build::build(input),
        Command::Serve { url } => serve::serve(&url),
    }
}

#[derive(Parser)]
#[clap(name = "palantir")]
enum Command {
    #[clap()]
    Build {
        /// Input file
        #[clap(
            parse(from_os_str),
            short = 'i',
            long = "input",
            default_value = "./examples/index.md"
        )]
        input: PathBuf,
    },
    #[clap()]
    Serve {
        /// Server host resource
        #[clap(short = 'u', long = "url", default_value = "127.0.0.1:3030")]
        url: String,
    },
}
