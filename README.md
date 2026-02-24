# fastcommit

`fastcommit` is a command-line tool designed to help developers quickly generate standardized commit messages. It supports multiple languages and detail levels, and can automatically generate commit messages based on file differences.

-- **中文文档**：[README_CN.md](README_CN.md)

## Installation

You can install `fastcommit` using the following method:

```bash
# Install using cargo
cargo install --git  https://github.com/fslongjin/fastcommit --tag v0.7.1
```


## Usage

### Basic Usage

```bash
git add .
fastcommit
```

### Options

NOTE: All common config can be configured via `~/.fastcommit/config.toml`

- `-d, --diff-file <DIFF_FILE>`: Specify the path to the file containing the differences.
- `--conventional <CONVENTIONAL>`: Enable or disable conventional commit style analysis. Acceptable values are `true` or `false`.
- `-l, --language <LANGUAGE>`: Specify the language for the commit message. Acceptable values are `en` (English) or `zh` (Chinese).
- `-b, --generate-branch`: Generate branch name.
   - `--branch-prefix`: prefix of the generated branch name
- `-m, --message`: Generate commit message (use with -b to output both)
- `-v, --verbosity <VERBOSITY>`: Set the detail level of the commit message. Acceptable values are `verbose` (detailed), `normal`, or `quiet` (concise). The default is `quiet`.
- `-p, --prompt <PROMPT>`: Additional prompt to help AI understand the commit context.
- `-r, --range <RANGE>`: Specify diff range for generating commit message (e.g. HEAD~1, abc123..def456).
- `--no-wrap`: Disable text wrapping for long lines.
- `--wrap-width <WIDTH>`: Set custom line width for text wrapping (default: config file setting or 80).
- `-c, --commit`: Automatically run `git commit` after generating the message.
- `--commit-args <ARG>`: Extra arguments to pass to `git commit` (can be specified multiple times, e.g. `--commit-args "-s" --commit-args "--no-verify"`).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Examples

1. Generate a commit message using default settings:

   ```bash
   fastcommit -d changes.diff
   ```

2. Enable conventional commit style and specify the Chinese language:

   ```bash
   fastcommit -d changes.diff --conventional true -l zh
   ```

3. Set the detail level to `verbose`:

   ```bash
   fastcommit -d changes.diff -v verbose
   ```

4. Provide additional context to help AI understand the commit:

   ```bash
   fastcommit -d changes.diff -p "Fixed login page styling issues, especially button alignment"
   ```

5. Generate branch name only:

   ```bash
   fastcommit -b
   ```

6. Generate both branch name and commit message:

   ```bash
   # Using separate flags
   fastcommit -b -m

   # Or using combined short flags
   fastcommit -bm
   ```

7. Generate commit message for a specific diff range:

   ```bash
   # Generate commit message for the last commit
   fastcommit -r HEAD~1

   # Generate commit message for a range of commits
   fastcommit -r abc123..def456
   ```

8. Control text wrapping behavior:

   ```bash
   # Disable text wrapping
   fastcommit --no-wrap

   # Set custom line width
   fastcommit --wrap-width 60

   # Combine with other options
   fastcommit -b -m --wrap-width 100
   ```

9. Auto-commit after generating the message:

   ```bash
   # Generate and auto-commit
   fastcommit -c

   # Auto-commit with signoff and skip hooks
   fastcommit -c --commit-args "-s" --commit-args "--no-verify"
   ```

## Development

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to run code quality checks before each commit.

```bash
# Install pre-commit
pip install pre-commit

# Install the git hooks
pre-commit install

# (Optional) Run all hooks manually
pre-commit run --all-files
```

The following checks will run automatically on `git commit`:

- **rustfmt** — Code formatting check
- **clippy** — Static analysis with warnings as errors
- **cargo-check** — Compilation check

## GitHub PR Integration

`fastcommit` can generate commit messages for GitHub Pull Requests, which is useful when merging PRs.

### Prerequisites

- [GitHub CLI (`gh`)](https://cli.github.com/) must be installed and authenticated

### Usage

```bash
# Auto-detect PR from current branch
fastcommit pr

# Generate commit message for a specific PR
fastcommit pr 123

# Specify repository (when not in a git directory)
fastcommit pr 123 --repo owner/repo

# Use conventional commit style
fastcommit pr 123 --conventional true

# Specify language
fastcommit pr 123 -l zh
```

### PR Command Options

- `[PR_NUMBER]`: PR number to generate commit message for. If not specified, auto-detects from current branch.
- `--repo <REPO>`: Specify repository in `owner/repo` format.
- `--conventional <CONVENTIONAL>`: Enable conventional commit style.
- `-l, --language <LANGUAGE>`: Specify language (`en` or `zh`).
- `-v, --verbosity <VERBOSITY>`: Set detail level (`verbose`, `normal`, `quiet`).
- `-p, --prompt <PROMPT>`: Additional context for AI.
- `--no-sanitize`: Disable sensitive info sanitizer.
- `--no-wrap`: Disable text wrapping.

For more details, see [GitHub PR Integration Guide](docs/github-pr-integration.md).

## Contributing

Contributions of code or suggestions are welcome! Please read the [Contributing Guide](CONTRIBUTING.md) first.

## License

This project is licensed under the [MIT License](LICENSE).
