# Sanitizer Configuration Guide

To prevent leaking sensitive information when sending diffs/user descriptions to model providers, fastcommit includes a built-in secret sanitization mechanism. This mechanism replaces matched sensitive content with placeholders before generating commit messages or branch names, for example:

```
AKIAIOSFODNN7EXAMPLE  -> [REDACTED:AWS_ACCESS_KEY_ID#1]
-----BEGIN PRIVATE KEY----- ... -> [REDACTED:PRIVATE_KEY_BLOCK#2]
Bearer abcdef123456 ....        -> [REDACTED:BEARER_TOKEN#3]
```

## 1. Basic Toggle

Configuration file: `~/.fastcommit/config.toml`

Field:
```
sanitize_secrets = true
```
Set to `false` to completely disable sanitization.

## 2. Built-in Matching Rules
Current built-in rules (name -> regex description):

| Name | Description |
|------|-------------|
| PRIVATE_KEY_BLOCK | Matches private key blocks from `-----BEGIN ... PRIVATE KEY-----` to `-----END ... PRIVATE KEY-----` |
| GITHUB_TOKEN | Matches tokens with prefixes like `ghp_` / `ghs_` / `gho_` / `ghr_` / `ghu_` + 36 alphanumeric characters |
| AWS_ACCESS_KEY_ID | Starts with `AKIA` + 16 uppercase alphanumeric characters |
| JWT | Typical 3-segment Base64URL JWT structure |
| BEARER_TOKEN | Bearer token headers (`Bearer xxx`) |
| GENERIC_API_KEY | Common field names: `api_key` / `apikey` / `apiKey` / `secret` / `token` / `authorization` followed by separator and value |

Matched content will be replaced with `[REDACTED:<name>#sequence_number]`.

## 3. Custom Rules
You can add custom rules in the configuration file to capture team-specific sensitive string formats.

Example:
```
[[custom_sanitize_patterns]]
name = "INTERNAL_URL"
regex = "https://internal\\.corp\\.example\\.com/[A-Za-z0-9/_-]+"

[[custom_sanitize_patterns]]
name = "UUID_TOKEN"
regex = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}"
```

Notes:
- `name`: Identifier in the placeholder; recommended to use uppercase underscore style.
- `regex`: Rust regex (ECMAScript-like, but without backtracking support); please escape backslashes appropriately.
- All custom rules are executed after built-in rules.
- If a regex is invalid, it will be skipped and a warning will be output in the logs.

## 4. Viewing Sanitization Statistics
The current version outputs the following when running with `RUST_LOG=debug`:
```
Sanitized N potential secrets from diff/prompt
```
In the future, `--show-redactions` can be added to display more detailed tables (planned feature).

## 5. Performance and Notes
- There may be minor performance overhead for very large diffs (multiple find-replace passes). If performance is sensitive, reduce the number of custom rules.
- Custom regex should not be overly broad, otherwise it may falsely match normal code context, affecting model understanding.
- The model cannot see the original replaced content. If context hints are needed, design semantically expressive tags with `name`, for example: `DB_PASSWORD`/`INTERNAL_ENDPOINT`.

## 6. Common Custom Pattern Examples
```
[[custom_sanitize_patterns]]
name = "SLACK_WEBHOOK"
regex = "https://hooks\\.slack\\.com/services/[A-Za-z0-9/_-]+"

[[custom_sanitize_patterns]]
name = "DISCORD_WEBHOOK"
regex = "https://discord(?:app)?\\.com/api/webhooks/[0-9]+/[A-Za-z0-9_-]+"

[[custom_sanitize_patterns]]
name = "GCP_SERVICE_ACCOUNT"
regex = "[0-9]{12}-compute@developer\\.gserviceaccount\\.com"

[[custom_sanitize_patterns]]
name = "STRIPE_KEY"
regex = "sk_(live|test)_[A-Za-z0-9]{10,}"
```

## 7. Complete Example Configuration Snippet
```
sanitize_secrets = true

[[custom_sanitize_patterns]]
name = "INTERNAL_URL"
regex = "https://internal\\.corp\\.example\\.com/[A-Za-z0-9/_-]+"

[[custom_sanitize_patterns]]
name = "STRIPE_KEY"
regex = "sk_(live|test)_[A-Za-z0-9]{10,}"
```

## 8. Future Plans
- Report mode: Output table statistics of match categories and counts
- Allow listing redacted placeholder hints at the end of commit messages (configurable)

For adding new default built-in rules or improvements, welcome to submit Issues / PRs.
