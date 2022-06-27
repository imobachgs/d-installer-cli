mod commands;
mod language;
mod software;
mod storage;

use clap::Parser;
use commands::Commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Storage(command) => storage::handle(&command),
        Commands::Language(command) => language::handle(&command),
        Commands::Software(command) => software::handle(&command),
    }
}
