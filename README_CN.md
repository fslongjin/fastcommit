# fastcommit

`fastcommit` 是一个命令行工具，旨在帮助开发者快速生成规范的提交信息。它支持多种语言和详细级别，并且可以根据文件差异自动生成提交信息。

## 安装

你可以通过以下方式安装 `fastcommit`：

```bash
# 使用 cargo 安装
cargo install --git  https://github.com/fslongjin/fastcommit --tag v0.1.4
```

## 使用

### 基本用法

```bash
git add .
fastcommit 
```

### 选项

NOTE: All common config can be configured via `~/.fastcommit/config.toml`

- `-d, --diff-file <DIFF_FILE>`: 指定包含差异的文件路径。
- `--conventional <CONVENTIONAL>`: 启用或禁用规范提交风格分析。可选值为 `true` 或 `false`。
- `-l, --language <LANGUAGE>`: 指定提交信息的语言。可选值为 `en`（英文）或 `zh`（中文）。
- `-gb, --generate-branch`: 模式：生成分支名
   - `--branch-prefix`: 生成的分支名的前缀 
- `-v, --verbosity <VERBOSITY>`: 设置提交信息的详细级别。可选值为 `verbose`（详细）、`normal`（正常）或 `quiet`（简洁）。 默认为 `quiet`。
- `-h, --help`: 打印帮助信息。
- `-V, --version`: 打印版本信息。

### 示例

1. 使用默认设置生成提交信息：

   ```bash
   fastcommit -d changes.diff
   ```

2. 启用规范提交风格并指定中文语言：

   ```bash
   fastcommit -d changes.diff --conventional true -l zh
   ```

3. 设置详细级别为 `verbose`：

   ```bash
   fastcommit -d changes.diff -v verbose
   ```

## 贡献

欢迎贡献代码或提出建议！请先阅读 [贡献指南](CONTRIBUTING.md)。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
