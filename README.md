# fastcommit

`fastcommit` is a command-line tool designed to help developers quickly generate standardized commit messages. It supports multiple languages and detail levels, and can automatically generate commit messages based on file differences.

-- **中文文档**：[README_CN.md](README_CN.md)

## Installation

You can install `fastcommit` using the following method:

```bash
# Install using cargo
cargo install fastcommit
```

## Usage

### Basic Usage

```bash
git add .
fastcommit
```

### Options

- `-d, --diff-file <DIFF_FILE>`: Specify the path to the file containing the differences.
- `--conventional <CONVENTIONAL>`: Enable or disable conventional commit style analysis. Acceptable values are `true` or `false`.
- `-l, --language <LANGUAGE>`: Specify the language for the commit message. Acceptable values are `en` (English) or `zh` (Chinese).
- `-v, --verbosity <VERBOSITY>`: Set the detail level of the commit message. Acceptable values are `verbose` (detailed), `normal`, or `quiet` (concise). The default is `quiet`.
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

## Contributing

 Contributions of code or suggestions are welcome! Please read the [Contributing Guide](CONTRIBUTING.md) first.

## License

This project is licensed under the [MIT License](LICENSE).
