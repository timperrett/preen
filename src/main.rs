
mod cli;
mod commands;

use cli::Cli;
use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let cli: Cli = Cli::from_args();

    Ok(())
}