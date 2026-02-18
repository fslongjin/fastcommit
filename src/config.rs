use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs};

use crate::constants::{DEFAULT_MAX_TOKENS, DEFAULT_OPENAI_API_BASE, DEFAULT_OPENAI_MODEL};

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextWrapConfig {
    /// Enable text wrapping for long lines
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Default line width for text wrapping
    #[serde(default = "default_wrap_width")]
    pub default_width: usize,
    /// Preserve word boundaries when wrapping
    #[serde(default = "default_true")]
    pub preserve_words: bool,
    /// Break long words when necessary
    #[serde(default = "default_true")]
    pub break_long_words: bool,
    /// Handle code blocks specially
    #[serde(default = "default_true")]
    pub handle_code_blocks: bool,
    /// Preserve links in text
    #[serde(default = "default_true")]
    pub preserve_links: bool,
    /// Hanging indent for wrapped lines (empty string for no indent)
    #[serde(default = "default_hanging_indent")]
    pub hanging_indent: String,
}

fn default_wrap_width() -> usize {
    80
}

fn default_hanging_indent() -> String {
    String::new() // 默认无悬挂缩进
}

impl Default for TextWrapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_width: 80,
            preserve_words: true,
            break_long_words: true,
            handle_code_blocks: true,
            preserve_links: true,
            hanging_indent: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomSanitizePattern {
    /// A short name/identifier for the pattern. e.g. "INTERNAL_URL"
    pub name: String,
    /// The regex pattern string. It should be a valid Rust regex.
    pub regex: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    api_base: Option<String>,
    pub api_key: String,
    pub model: Option<String>,
    /// The maximum number of tokens to generate in the commit message.
    pub max_tokens: Option<u32>,
    /// Whether to use conventional commit message format.
    pub conventional: bool,
    pub language: CommitLanguage,
    pub verbosity: Verbosity,
    /// Prefix for generated branch names (e.g. username in monorepo)
    pub branch_prefix: Option<String>,
    /// Enable sanitizing sensitive information (API keys, tokens, secrets) before sending diff to AI provider.
    #[serde(default = "default_true")]
    pub sanitize_secrets: bool,
    /// User defined extra regex patterns for sanitizer.
    #[serde(default)]
    pub custom_sanitize_patterns: Vec<CustomSanitizePattern>,
    /// Text wrapping configuration
    #[serde(default)]
    pub text_wrap: TextWrapConfig,
    /// Automatically run git commit after generating the message
    #[serde(default)]
    pub auto_commit: bool,
    /// Extra arguments to pass to git commit
    #[serde(default)]
    pub commit_args: Vec<String>,
}

impl Config {
    pub fn api_base(&self) -> String {
        let api_base = self
            .api_base
            .as_deref()
            .unwrap_or("https://api.openai.com/v1");

        let api_base = if api_base.ends_with("/") {
            api_base.to_owned()
        } else {
            format!("{}/", api_base)
        };

        api_base
    }
}

/// Commit message verbosity level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, clap::ValueEnum)]
pub enum Verbosity {
    /// Detailed commit message.
    #[serde(rename = "verbose")]
    #[clap(name = "verbose")]
    Verbose,
    /// Normal commit message.
    #[serde(rename = "normal")]
    #[clap(name = "normal")]
    Normal,
    /// Quiet commit message.
    #[serde(rename = "quiet")]
    #[clap(name = "quiet")]
    Quiet,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Quiet
    }
}

impl Verbosity {
    pub fn to_template_level(&self) -> &'static str {
        match self {
            Verbosity::Verbose => "详细",
            Verbosity::Normal => "中等",
            Verbosity::Quiet => "简洁",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum CommitLanguage {
    #[clap(name = "en")]
    #[serde(rename = "en")]
    English,
    #[clap(name = "zh")]
    #[serde(rename = "zh")]
    Chinese,
}

impl Default for CommitLanguage {
    fn default() -> Self {
        CommitLanguage::Chinese
    }
}

impl Display for CommitLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitLanguage::English => write!(f, "English"),
            CommitLanguage::Chinese => write!(f, "中文"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_base: Some(DEFAULT_OPENAI_API_BASE.into()),
            api_key: "".to_owned(),
            model: Some(DEFAULT_OPENAI_MODEL.into()),
            max_tokens: Some(DEFAULT_MAX_TOKENS),
            conventional: true,
            language: CommitLanguage::default(),
            verbosity: Verbosity::default(),
            branch_prefix: None,
            sanitize_secrets: true,
            custom_sanitize_patterns: Vec::new(),
            text_wrap: TextWrapConfig::default(),
            auto_commit: false,
            commit_args: Vec::new(),
        }
    }
}

pub async fn load_config() -> anyhow::Result<Config> {
    let config_path = dirs::home_dir()
        .map(|p| p.join(".fastcommit/config.toml"))
        .ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?;

    if !config_path.exists() {
        create_default_config(&config_path)?;
        println!(
            "Created default configuration file at {}. Please customize your api base and api key.",
            config_path.display()
        );
        std::process::exit(0);
    }

    let config_str = tokio::fs::read_to_string(config_path).await?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}

fn create_default_config(config_path: &std::path::Path) -> anyhow::Result<()> {
    let config_dir = config_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?;
    fs::create_dir_all(config_dir)?;

    let default_config = Config::default();
    let config_str = toml::to_string(&default_config)?;
    fs::write(config_path, config_str)?;

    Ok(())
}
