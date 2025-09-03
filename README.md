# fastcommit

`fastcommit` is a command-line tool designed to help developers quickly generate standardized commit messages. It supports multiple languages and detail levels, and can automatically generate commit messages based on file differences.

-- **中文文档**：[README_CN.md](README_CN.md)

## Installation

You can install `fastcommit` using the following method:

```bash
# Install using cargo
cargo install --git  https://github.com/fslongjin/fastcommit --tag v0.3.0
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
   fastcommit -b -m
   ```

7. Generate commit message for a specific diff range:

   ```bash
   # Generate commit message for the last commit
   fastcommit -r HEAD~1
   
   # Generate commit message for a range of commits
   fastcommit -r abc123..def456
   ```

## Contributing

 Contributions of code or suggestions are welcome! Please read the [Contributing Guide](CONTRIBUTING.md) first.

## License

This project is licensed under the [MIT License](LICENSE).
