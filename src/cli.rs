use clap::{Parser, Subcommand};

use crate::config::{CommitLanguage, Verbosity};

#[derive(Parser, Debug)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    about = concat!(
        "AI-based command line tool to quickly generate standardized commit messages.\n\n",
        "Version: ", env!("CARGO_PKG_VERSION")
    ),
    subcommand_required = false,
    arg_required_else_help = false
)]
pub struct Args {
    #[clap(flatten)]
    pub commit_args: CommitArgs,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate commit message for staged changes (default behavior)
    Commit(CommitArgs),

    /// Generate commit message for a GitHub PR
    Pr(PrArgs),
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Commit(CommitArgs::default())
    }
}

/// Common arguments shared by commit and pr commands
#[derive(Parser, Debug, Default)]
pub struct CommonArgs {
    #[clap(long, help = "Enable conventional commit style analysis")]
    pub conventional: Option<bool>,

    #[clap(short, long, help = "Specify the language for commit messages")]
    pub language: Option<CommitLanguage>,

    #[clap(short, long, help = "Set the verbosity level")]
    pub verbosity: Option<Verbosity>,

    #[clap(
        short,
        long,
        help = "Additional prompt to help AI understand the commit context"
    )]
    pub prompt: Option<String>,

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
        allow_hyphen_values = true
    )]
    pub commit_args: Vec<String>,
}

#[derive(Parser, Debug, Default)]
pub struct CommitArgs {
    #[clap(short, long, help = "Path to the file containing the diff to analyze")]
    pub diff_file: Option<String>,

    #[clap(
        long = "generate-branch",
        short = 'b',
        help = "Generate a branch name based on changes (optionally with prefix)"
    )]
    pub generate_branch: bool,

    #[clap(long, help = "Override branch prefix (default from config)")]
    pub branch_prefix: Option<String>,

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

    #[clap(flatten)]
    pub common: CommonArgs,
}

#[derive(Parser, Debug)]
pub struct PrArgs {
    /// PR number, auto-detect from current branch if not specified
    #[clap(name = "PR_NUMBER")]
    pub pr_number: Option<u32>,

    /// Specify repository (format: owner/repo)
    #[clap(long)]
    pub repo: Option<String>,

    #[clap(flatten)]
    pub common: CommonArgs,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to parse args from iterator
    fn parse_args<I, T>(iter: I) -> Result<Args, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Args::try_parse_from(iter)
    }

    #[test]
    fn test_top_level_short_options_combined() {
        // Test: fastcommit -bm (generate both branch and message)
        let args = parse_args(["fastcommit", "-bm"]).unwrap();
        assert!(args.command.is_none(), "No subcommand should be set");
        assert!(
            args.commit_args.generate_branch,
            "-b should set generate_branch"
        );
        assert!(
            args.commit_args.generate_message,
            "-m should set generate_message"
        );
    }

    #[test]
    fn test_top_level_short_options_separate() {
        // Test: fastcommit -b -m (generate both branch and message)
        let args = parse_args(["fastcommit", "-b", "-m"]).unwrap();
        assert!(args.command.is_none());
        assert!(args.commit_args.generate_branch);
        assert!(args.commit_args.generate_message);
    }

    #[test]
    fn test_top_level_branch_only() {
        // Test: fastcommit -b (generate branch only)
        let args = parse_args(["fastcommit", "-b"]).unwrap();
        assert!(args.command.is_none());
        assert!(args.commit_args.generate_branch);
        assert!(!args.commit_args.generate_message);
    }

    #[test]
    fn test_top_level_message_only() {
        // Test: fastcommit -m (generate message only - default behavior)
        let args = parse_args(["fastcommit", "-m"]).unwrap();
        assert!(args.command.is_none());
        assert!(!args.commit_args.generate_branch);
        assert!(args.commit_args.generate_message);
    }

    #[test]
    fn test_no_args_uses_default() {
        // Test: fastcommit (no args - default commit behavior)
        let args = parse_args(["fastcommit"]).unwrap();
        assert!(args.command.is_none());
        assert!(!args.commit_args.generate_branch);
        assert!(!args.commit_args.generate_message);
    }

    #[test]
    fn test_commit_subcommand_with_options() {
        // Test: fastcommit commit -bm (using subcommand explicitly)
        let args = parse_args(["fastcommit", "commit", "-bm"]).unwrap();
        assert!(args.command.is_some(), "Subcommand should be set");
        if let Some(Commands::Commit(commit_args)) = args.command {
            assert!(commit_args.generate_branch);
            assert!(commit_args.generate_message);
        } else {
            panic!("Expected Commit subcommand");
        }
    }

    #[test]
    fn test_pr_subcommand() {
        // Test: fastcommit pr 123
        let args = parse_args(["fastcommit", "pr", "123"]).unwrap();
        if let Some(Commands::Pr(pr_args)) = args.command {
            assert_eq!(pr_args.pr_number, Some(123));
        } else {
            panic!("Expected Pr subcommand");
        }
    }

    #[test]
    fn test_top_level_range_option() {
        // Test: fastcommit -r HEAD~1
        let args = parse_args(["fastcommit", "-r", "HEAD~1"]).unwrap();
        assert!(args.command.is_none());
        assert_eq!(args.commit_args.range, Some("HEAD~1".to_string()));
    }

    #[test]
    fn test_top_level_with_common_args() {
        // Test: fastcommit -bm --no-wrap --language en
        let args = parse_args(["fastcommit", "-bm", "--no-wrap", "--language", "en"]).unwrap();
        assert!(args.command.is_none());
        assert!(args.commit_args.generate_branch);
        assert!(args.commit_args.generate_message);
        assert!(args.commit_args.common.no_wrap);
        assert_eq!(
            args.commit_args.common.language,
            Some(CommitLanguage::English)
        );
    }

    #[test]
    fn test_commit_subcommand_with_range() {
        // Test: fastcommit commit -r HEAD~1 -b
        let args = parse_args(["fastcommit", "commit", "-r", "HEAD~1", "-b"]).unwrap();
        if let Some(Commands::Commit(commit_args)) = args.command {
            assert!(commit_args.generate_branch);
            assert_eq!(commit_args.range, Some("HEAD~1".to_string()));
        } else {
            panic!("Expected Commit subcommand");
        }
    }

    #[test]
    fn test_diff_file_option() {
        // Test: fastcommit --diff-file /path/to/diff
        let args = parse_args(["fastcommit", "--diff-file", "/path/to/diff"]).unwrap();
        assert!(args.command.is_none());
        assert_eq!(
            args.commit_args.diff_file,
            Some("/path/to/diff".to_string())
        );
    }

    #[test]
    fn test_auto_commit_option() {
        // Test: fastcommit -c (auto commit after generating)
        let args = parse_args(["fastcommit", "-c"]).unwrap();
        assert!(args.command.is_none());
        assert!(args.commit_args.common.commit);
    }

    #[test]
    fn test_combined_all_options() {
        // Test: fastcommit -bmc --no-sanitize (all flags combined)
        let args = parse_args(["fastcommit", "-bmc", "--no-sanitize"]).unwrap();
        assert!(args.command.is_none());
        assert!(args.commit_args.generate_branch);
        assert!(args.commit_args.generate_message);
        assert!(args.commit_args.common.commit);
        assert!(args.commit_args.common.no_sanitize);
    }
}
