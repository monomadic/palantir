use clap::Parser;
use std::path::PathBuf;

mod build;
mod serve;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Build { input } => build::build(input),
        Command::Serve => serve::serve(),
    }
}

#[derive(Parser)]
#[clap(name = "Palantir")]
pub struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
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
