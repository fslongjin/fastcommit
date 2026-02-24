# fastcommit

`fastcommit` 是一个命令行工具，旨在帮助开发者快速生成规范的提交信息。它支持多种语言和详细级别，并且可以根据文件差异自动生成提交信息。

## 安装

你可以通过以下方式安装 `fastcommit`：

```bash
# 使用 cargo 安装
cargo install --git  https://github.com/fslongjin/fastcommit --tag v0.6.0
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
- `-b, --generate-branch`: 生成分支名
   - `--branch-prefix`: 生成的分支名的前缀
- `-m, --message`: 生成提交信息（与 -b 一起使用可同时输出）
- `-v, --verbosity <VERBOSITY>`: 设置提交信息的详细级别。可选值为 `verbose`（详细）、`normal`（正常）或 `quiet`（简洁）。 默认为 `quiet`。
- `-p, --prompt <PROMPT>`: 额外的提示信息，帮助 AI 理解提交上下文。
- `-r, --range <RANGE>`: 指定差异范围以生成提交信息（例如：HEAD~1, abc123..def456）。
- `--no-wrap`: 禁用长行文本换行。
- `--wrap-width <WIDTH>`: 设置文本换行的自定义行宽度（默认：配置文件设置或 80）。
- `-c, --commit`: 生成提交信息后自动执行 `git commit`。
- `--commit-args <ARG>`: 传递给 `git commit` 的额外参数（可多次指定，例如 `--commit-args "-s" --commit-args "--no-verify"`）。
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

4. 提供额外上下文帮助 AI 理解提交：

   ```bash
   fastcommit -d changes.diff -p "修复了登录页面的样式问题，特别是按钮对齐"
   ```

5. 仅生成分支名：

   ```bash
   fastcommit -b
   ```

6. 同时生成分支名和提交信息：

   ```bash
   fastcommit -b -m
   ```

7. 为特定的差异范围生成提交信息：

   ```bash
   # 为最近一次提交生成提交信息
   fastcommit -r HEAD~1

   # 为指定提交范围生成提交信息
   fastcommit -r abc123..def456
   ```

8. 控制文本换行行为：

   ```bash
   # 禁用文本换行
   fastcommit --no-wrap

   # 设置自定义行宽度
   fastcommit --wrap-width 60

   # 与其他选项组合使用
   fastcommit -b -m --wrap-width 100
   ```

9. 生成提交信息后自动提交：

   ```bash
   # 生成并自动提交
   fastcommit -c

   # 自动提交并签名、跳过 hook
   fastcommit -c --commit-args "-s" --commit-args "--no-verify"
   ```

## GitHub PR 集成

`fastcommit` 可以为 GitHub Pull Request 生成提交信息，适用于合并 PR 时使用。

### 前置条件

- 需要安装并登录 [GitHub CLI (`gh`)](https://cli.github.com/)

### 使用方法

```bash
# 自动检测当前分支关联的 PR
fastcommit pr

# 为指定 PR 生成提交信息
fastcommit pr 123

# 指定仓库（不在 git 目录中时）
fastcommit pr 123 --repo owner/repo

# 使用约定式提交风格
fastcommit pr 123 --conventional true

# 指定语言
fastcommit pr 123 -l zh
```

### PR 命令选项

- `[PR_NUMBER]`: PR 编号，不指定则自动检测当前分支关联的 PR
- `--repo <REPO>`: 指定仓库，格式为 `owner/repo`
- `--conventional <CONVENTIONAL>`: 启用约定式提交风格
- `-l, --language <LANGUAGE>`: 指定语言（`en` 或 `zh`）
- `-v, --verbosity <VERBOSITY>`: 设置详细级别（`verbose`、`normal`、`quiet`）
- `-p, --prompt <PROMPT>`: 额外的提示信息
- `--no-sanitize`: 禁用敏感信息清理
- `--no-wrap`: 禁用文本换行

更多详情请参阅 [GitHub PR 集成指南](docs/github-pr-integration.md)。

## 开发

### Pre-commit Hooks

本项目使用 [pre-commit](https://pre-commit.com/) 在每次提交前自动运行代码质量检查。

```bash
# 安装 pre-commit
pip install pre-commit

# 安装 git hooks
pre-commit install

# （可选）手动运行所有检查
pre-commit run --all-files
```

以下检查会在 `git commit` 时自动执行：

- **rustfmt** — 代码格式化检查
- **clippy** — 静态分析，警告视为错误
- **cargo-check** — 编译检查

## 贡献

欢迎贡献代码或提出建议！请先阅读 [贡献指南](CONTRIBUTING.md)。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
