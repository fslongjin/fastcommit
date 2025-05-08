use clap::Parser;

use crate::config::{CommitLanguage, Verbosity};

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, help = "Path to the file containing the diff to analyze")]
    pub diff_file: Option<String>,

    #[clap(long, help = "Enable conventional commit style analysis")]
    pub conventional: Option<bool>,

    #[clap(short, long, help = "Specify the language for commit messages")]
    pub language: Option<CommitLanguage>,

    #[clap(short, long, help = "Set the verbosity level")]
    pub verbosity: Option<Verbosity>,

    #[clap(
        long,
        help = "Maximum line length for commit message (0 means no limit)"
    )]
    pub max_line_length: Option<u32>,
}
