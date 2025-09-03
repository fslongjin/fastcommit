use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs};

use crate::constants::{DEFAULT_MAX_TOKENS, DEFAULT_OPENAI_API_BASE, DEFAULT_OPENAI_MODEL};

fn default_true() -> bool {
    true
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
