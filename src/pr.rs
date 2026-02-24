use std::process::Command;

use crate::cli::PrArgs;
use crate::config::Config;
use crate::generate::generate_commit_message;

/// Get PR diff using gh CLI
fn get_pr_diff_from_gh(pr_number: Option<u32>, repo: Option<&str>) -> anyhow::Result<String> {
    let mut cmd = Command::new("gh");
    cmd.args(["pr", "diff"]);

    if let Some(num) = pr_number {
        cmd.arg(num.to_string());
    }
    if let Some(r) = repo {
        cmd.args(["--repo", r]);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("gh pr diff failed: {}", stderr.trim()));
    }

    let diff = String::from_utf8_lossy(&output.stdout).into_owned();
    if diff.trim().is_empty() {
        return Err(anyhow::anyhow!("No diff found for PR"));
    }

    Ok(diff)
}

/// Detect current branch's associated PR number
fn detect_current_pr(repo: Option<&str>) -> anyhow::Result<u32> {
    let mut cmd = Command::new("gh");
    cmd.args(["pr", "view", "--json", "number"]);

    if let Some(r) = repo {
        cmd.args(["--repo", r]);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Failed to detect current PR. Please specify PR number explicitly. Error: {}",
            stderr.trim()
        ));
    }

    let json_output = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&json_output)
        .map_err(|e| anyhow::anyhow!("Failed to parse gh output: {}", e))?;

    let pr_number = parsed["number"]
        .as_u64()
        .ok_or(anyhow::anyhow!("Could not find PR number in gh output"))?;

    Ok(pr_number as u32)
}

/// Check if gh CLI is available
fn is_gh_available() -> bool {
    Command::new("gh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Generate commit message for a PR
pub async fn generate_pr_message(args: &PrArgs, config: &Config) -> anyhow::Result<String> {
    // Check if gh is available
    if !is_gh_available() {
        return Err(anyhow::anyhow!(
            "GitHub CLI (gh) is not installed or not in PATH. Please install it from https://cli.github.com/"
        ));
    }

    // Detect PR number if not specified
    let pr_number = match args.pr_number {
        Some(num) => num,
        None => detect_current_pr(args.repo.as_deref())?,
    };

    log::info!("Getting diff for PR #{}...", pr_number);

    // Get PR diff
    let diff = get_pr_diff_from_gh(Some(pr_number), args.repo.as_deref())?;

    log::info!("Generating commit message...");

    // Generate commit message using existing logic
    let message = generate_commit_message(&diff, config, args.common.prompt.as_deref()).await?;

    Ok(message)
}
