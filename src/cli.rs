use structopt;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    about = "CLI utility for cleaning up your historical tweets",
    author = "Timothy Perrett",
)]
pub struct Cli {
    #[structopt(short, long = "dry-run")]
    dry_run: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(long = "user", parse(from_str))]
    user: Option<String>,

    #[structopt(short, long = "min-age-days")]
    min_age_days: Option<u8>,

    #[structopt(short, long = "min-score")]
    min_score: Option<u8>,
}
