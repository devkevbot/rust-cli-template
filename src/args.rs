use crate::commands;
use crate::Result;
use clap::{Parser, Subcommand};

pub fn run() -> Result<()> {
    execute(Args::parse())
}

fn execute(args: Args) -> Result<()> {
    match args.command {
        Command::Action => commands::action(),
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Action,
}
