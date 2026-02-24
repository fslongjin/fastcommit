# GitHub PR Integration

This document describes how to use `fastcommit` to generate commit messages for GitHub Pull Requests.

## Overview

The `fastcommit pr` command allows you to generate a commit message for an entire Pull Request. This is particularly useful when you're ready to merge a PR and need a comprehensive commit message that summarizes all the changes.

## Prerequisites

Before using the PR integration, ensure you have:

1. **GitHub CLI (`gh`) installed**

   ```bash
   # Check if gh is installed
   gh --version

   # Install gh on macOS
   brew install gh

   # Install gh on Linux
   # See: https://github.com/cli/cli/blob/trunk/docs/install_linux.md
   ```

2. **GitHub CLI authenticated**

   ```bash
   gh auth login
   ```

## Usage

### Basic Usage

```bash
# Auto-detect PR from current branch
fastcommit pr

# Generate commit message for a specific PR
fastcommit pr 123
```

### Specify Repository

When running `fastcommit pr` outside the target repository, use the `--repo` flag:

```bash
fastcommit pr 123 --repo owner/repo
```

### All Options

```
fastcommit pr [PR_NUMBER] [OPTIONS]

Arguments:
  [PR_NUMBER]  PR number to generate commit message for.
               If not specified, auto-detects from current branch.

Options:
      --repo <REPO>           Specify repository (format: owner/repo)
      --conventional <BOOL>   Enable conventional commit style (true/false)
  -l, --language <LANG>       Specify language (en/zh)
  -v, --verbosity <LEVEL>     Set detail level (verbose/normal/quiet)
  -p, --prompt <TEXT>         Additional context for AI
      --no-sanitize           Disable sensitive info sanitizer
      --no-wrap               Disable text wrapping
      --wrap-width <WIDTH>    Set custom line width for wrapping
```

## Examples

### Generate commit message for current branch's PR

```bash
# Assuming you're on a branch with an open PR
fastcommit pr
```

Output:
```
feat: add user authentication system

- Implement JWT-based authentication
- Add login/logout endpoints
- Integrate with existing user service
- Add rate limiting for auth endpoints
```

### Generate with conventional commit style

```bash
fastcommit pr 456 --conventional true
```

Output:
```
fix(auth): resolve token refresh issue

- Fix improper token validation on refresh
- Add proper error handling for expired tokens
- Update token storage to use secure cookies
```

### Add context for better results

```bash
fastcommit pr 789 -p "This PR fixes the performance issues reported in issue #123"
```

### Generate in Chinese

```bash
fastcommit pr 123 -l zh
```

Output:
```
feat: 添加用户认证系统

- 实现 JWT 认证机制
- 添加登录/登出接口
- 与现有用户服务集成
- 添加认证接口的速率限制
```

## How It Works

1. **PR Detection**: If no PR number is specified, `fastcommit` uses `gh pr view` to detect the PR associated with the current branch.

2. **Diff Retrieval**: The tool fetches the PR diff using `gh pr diff`.

3. **Message Generation**: The diff is processed by the AI to generate a commit message, using the same logic as the standard `fastcommit` command.

## Troubleshooting

### "GitHub CLI (gh) is not installed"

Install GitHub CLI:
- **macOS**: `brew install gh`
- **Linux**: See [installation guide](https://github.com/cli/cli/blob/trunk/docs/install_linux.md)
- **Windows**: `winget install GitHub.cli`

### "Failed to detect current PR"

This happens when:
- You're not on a branch with an open PR
- The PR is in a different repository

**Solution**: Specify the PR number explicitly:
```bash
fastcommit pr 123
```

### "gh pr diff failed"

This usually indicates:
- You're not authenticated with GitHub CLI

**Solution**: Run `gh auth login`

### "No diff found for PR"

This happens when the PR has no changes (empty PR).

## Tips

1. **Use with PR merge**: Generate the commit message before merging:
   ```bash
   # Generate the message
   fastcommit pr 123

   # Then merge with the generated message
   gh pr merge 123 --merge
   ```

2. **Combine with conventional commits**: For projects following conventional commit conventions:
   ```bash
   fastcommit pr 123 --conventional true
   ```

3. **Add merge context**: Provide additional context about the PR's purpose:
   ```bash
   fastcommit pr 123 -p "Closes #456, implements feature requested by users"
   ```

## Related Documentation

- [README.md](../README.md) - Main documentation
- [GitHub CLI Documentation](https://cli.github.com/manual/)
