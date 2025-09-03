use crate::config::Config;
use regex::Regex; // added for sanitize_with_config

/// Represents a redacted secret occurrence.
#[derive(Debug, Clone)]
pub struct Redaction {
    pub _kind: &'static str,
    pub _placeholder: String,
}

impl Redaction {
    pub fn new(kind: &'static str, placeholder: impl Into<String>) -> Self {
        Self {
            _kind: kind,
            _placeholder: placeholder.into(),
        }
    }
}

/// Runtime redaction entry for custom user patterns.
#[derive(Debug, Clone)]
pub struct CustomRedactionMeta {
    pub name: String,
    pub regex: Regex,
}

/// Sanitize text before sending to the model provider with builtin + custom patterns.
pub fn sanitize(
    text: &str,
    enabled: bool,
    custom_patterns: &[CustomRedactionMeta],
) -> (String, Vec<Redaction>) {
    if !enabled || text.is_empty() {
        return (text.to_string(), Vec::new());
    }

    let mut redactions: Vec<Redaction> = Vec::new();
    let mut counter = 0usize;

    // Built-in patterns (ordered) – static lifetime kinds
    let builtin: Vec<(&'static str, Regex)> = vec![
        (
            "PRIVATE_KEY_BLOCK",
            Regex::new(
                r"-----BEGIN [A-Z ]+PRIVATE KEY-----[\s\S]*?-----END [A-Z ]+PRIVATE KEY-----",
            )
            .unwrap(),
        ),
        (
            "GITHUB_TOKEN",
            Regex::new(r"\bgh[pousr]_[A-Za-z0-9]{36}\b").unwrap(),
        ),
        (
            "AWS_ACCESS_KEY_ID",
            Regex::new(r"\bAKIA[0-9A-Z]{16}\b").unwrap(),
        ),
        (
            "JWT",
            Regex::new(r"\b[A-Za-z0-9_-]{10,}\.([A-Za-z0-9_-]{10,})\.[A-Za-z0-9_-]{10,}\b")
                .unwrap(),
        ),
        (
            "BEARER_TOKEN",
            Regex::new(r"(?i)bearer\s+[A-Za-z0-9\-_.=]+\b").unwrap(),
        ),
        (
            "GENERIC_API_KEY",
            Regex::new(
                r#"(?i)(api_?key|secret|token|authorization)[\s:=\"']+([A-Za-z0-9_\-]{8,})"#,
            )
            .unwrap(),
        ),
    ];

    let mut sanitized = text.to_string();

    // Apply builtin patterns
    for (kind, re) in builtin.iter() {
        loop {
            if let Some(m) = re.find(&sanitized) {
                counter += 1;
                let placeholder = format!("[REDACTED:{}#{}]", kind, counter);
                sanitized.replace_range(m.start()..m.end(), &placeholder);
                redactions.push(Redaction::new(kind, placeholder));
            } else {
                break;
            }
        }
    }

    // Apply custom patterns – use their provided name (converted to static via leak for simplicity)
    for meta in custom_patterns {
        loop {
            if let Some(m) = meta.regex.find(&sanitized) {
                counter += 1;
                let placeholder = format!("[REDACTED:{}#{}]", meta.name, counter);
                sanitized.replace_range(m.start()..m.end(), &placeholder);
                // We leak the string to get a 'static str; acceptable given tiny count and CLI nature
                let leaked: &'static str = Box::leak(meta.name.clone().into_boxed_str());
                redactions.push(Redaction::new(leaked, placeholder));
            } else {
                break;
            }
        }
    }

    (sanitized, redactions)
}

/// Convenience: sanitize multiple text components and return redaction info combined.
pub fn sanitize_for_model(
    diff: &str,
    user_prompt: Option<&str>,
    enabled: bool,
    custom_patterns: &[CustomRedactionMeta],
) -> (String, Option<String>, Vec<Redaction>) {
    let (sdiff, mut r1) = sanitize(diff, enabled, custom_patterns);
    let (sprompt, r2) = match user_prompt {
        Some(p) => {
            let (s, rs) = sanitize(p, enabled, custom_patterns);
            (Some(s), rs)
        }
        None => (None, Vec::new()),
    };
    r1.extend(r2);
    (sdiff, sprompt, r1)
}

/// Compile custom patterns from config; invalid ones are logged and skipped.
fn compile_custom_patterns(
    items: &[crate::config::CustomSanitizePattern],
) -> Vec<CustomRedactionMeta> {
    // made private
    let mut out = Vec::new();
    for item in items {
        match Regex::new(&item.regex) {
            Ok(re) => out.push(CustomRedactionMeta {
                name: item.name.clone(),
                regex: re,
            }),
            Err(e) => {
                log::warn!("Skip invalid custom sanitize regex '{}': {}", item.regex, e);
            }
        }
    }
    out
}

/// High-level helper: directly use full Config, hiding compilation logic from callers.
pub fn sanitize_with_config(
    diff: &str,
    user_prompt: Option<&str>,
    config: &Config,
) -> (String, Option<String>, Vec<Redaction>) {
    let compiled = compile_custom_patterns(&config.custom_sanitize_patterns);
    sanitize_for_model(diff, user_prompt, config.sanitize_secrets, &compiled)
}
