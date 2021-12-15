use clap::Parser;
use std::path::PathBuf;

mod build;
mod serve;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Command::parse() {
        Command::Build { input } => build::build(input),
        Command::Serve => serve::serve(),
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
    Serve,
}
