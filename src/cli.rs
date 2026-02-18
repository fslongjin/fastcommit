use clap::Parser;

use crate::config::{CommitLanguage, Verbosity};

#[derive(Parser, Debug)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    about = concat!(
        "AI-based command line tool to quickly generate standardized commit messages.\n\n",
        "Version: ", env!("CARGO_PKG_VERSION")
    )
)]
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
        long = "generate-branch",
        short = 'b',
        help = "Generate a branch name based on changes (optionally with prefix)"
    )]
    pub generate_branch: bool,

    #[clap(long, help = "Override branch prefix (default from config)")]
    pub branch_prefix: Option<String>,

    #[clap(
        short,
        long,
        help = "Additional prompt to help AI understand the commit context"
    )]
    pub prompt: Option<String>,

    #[clap(
        short = 'r',
        long,
        help = "Specify diff range (e.g. HEAD~1, abc123..def456)"
    )]
    pub range: Option<String>,

    #[clap(
        short = 'm',
        long = "message",
        help = "Generate commit message (use with -b to output both)"
    )]
    pub generate_message: bool,

    #[clap(
        long = "no-sanitize",
        help = "Temporarily disable sensitive info sanitizer for this run"
    )]
    pub no_sanitize: bool,

    #[clap(long = "no-wrap", help = "Disable text wrapping for long lines")]
    pub no_wrap: bool,

    #[clap(
        long = "wrap-width",
        help = "Set custom line width for text wrapping (default: terminal width)"
    )]
    pub wrap_width: Option<usize>,

    #[clap(
        short = 'c',
        long = "commit",
        help = "Automatically run git commit after generating the message"
    )]
    pub commit: bool,

    #[clap(
        long = "commit-args",
        help = "Extra arguments to pass to git commit (can be specified multiple times)",
        num_args = 1,
        allow_hyphen_values = true,
    )]
    pub commit_args: Vec<String>,
}
