use openai_api_rust::chat::*;
use openai_api_rust::*;
use std::process::Command;

use crate::cli;
use crate::config::{self, Config};

use crate::constants::BRANCH_NAME_PROMPT;
use crate::constants::{DEFAULT_MAX_TOKENS, DEFAULT_OPENAI_MODEL, DEFAULT_PROMPT_TEMPLATE};
use crate::template_engine::{render_template, TemplateContext};

async fn generate_commit_message(
    diff: &str,
    config: &config::Config,
    user_description: Option<&str>,
) -> anyhow::Result<String> {
    let auth = Auth::new(config.api_key.as_str());

    let openai = OpenAI::new(auth, &config.api_base());

    // Add "commit message: " prefix to user description if provided
    let prefixed_user_description = user_description.map(|desc| {
        if desc.trim().is_empty() {
            desc.to_string()
        } else {
            format!("commit message: {}", desc)
        }
    });

    let template_ctx = TemplateContext::new(
        config.conventional,
        config.language,
        config.verbosity,
        diff,
        prefixed_user_description.as_deref(),
    );

    let messages = vec![
        Message {
            role: Role::System,
            content: "你是一个代码版本控制专家，请分析以下变更并生成commit message。".to_string(),
        },
        Message {
            role: Role::User,
            content: render_template(DEFAULT_PROMPT_TEMPLATE, template_ctx)?,
        },
    ];

    let chat = ChatBody {
        model: config
            .model
            .as_deref()
            .unwrap_or(DEFAULT_OPENAI_MODEL)
            .to_owned(),
        messages,
        temperature: Some(0.30f32),
        top_p: None,
        n: None,
        stream: Some(false),
        stop: None, // 移除 stop words 以避免思考过程中的干扰
        max_tokens: Some(DEFAULT_MAX_TOKENS as i32),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };
    let response = openai
        .chat_completion_create(&chat)
        .map_err(|e| anyhow::anyhow!("Failed to create chat completion: {}", e))?;
    let msg = &response.choices[0]
        .message
        .as_ref()
        .ok_or(anyhow::anyhow!("No message in response"))?
        .content;
    // Extract content between <aicommit> tags
    let commit_message = extract_aicommit_message(msg)?;
    Ok(commit_message)
}

fn delete_thinking_contents(orig: &str) -> String {
    let start_tag = "<think>";
    let end_tag = "</think>";

    let start_idx = orig.find(start_tag).unwrap_or(orig.len());
    let end_idx = orig.find(end_tag).unwrap_or_else(|| 0);
    let s = if start_idx < end_idx {
        let mut result = orig[..start_idx].to_string();
        result.push_str(&orig[end_idx..]);
        log::debug!(
            "Delete thinking contents, start_idx: {}, end_idx: {}: {:?} => {:?}",
            start_idx,
            end_idx,
            orig,
            result
        );
        result
    } else {
        orig.to_string()
    };
    s
}

fn extract_aicommit_message(response: &str) -> anyhow::Result<String> {
    let response = delete_thinking_contents(response);

    // 查找所有 <aicommit>...</aicommit> 块
    let mut matches = Vec::new();
    let mut pos = 0;

    while let Some(start_idx) = response[pos..].find("<aicommit>") {
        let absolute_start = pos + start_idx;
        let content_start = absolute_start + "<aicommit>".len();

        if let Some(end_idx) = response[content_start..].find("</aicommit>") {
            let absolute_end = content_start + end_idx;
            let content = &response[content_start..absolute_end];
            matches.push(content.trim());
            pos = absolute_end + "</aicommit>".len();
        } else {
            break;
        }
    }

    // 返回第一个匹配的内容
    matches
        .into_iter()
        .next()
        .map(|s| s.to_string())
        .ok_or(anyhow::anyhow!("Start tag <aicommit> not found"))
}

fn get_diff(diff_file: Option<&str>, range: Option<&str>) -> anyhow::Result<String> {
    match diff_file {
        Some(path) => std::fs::read_to_string(path).map_err(Into::into),
        None => {
            let mut cmd = Command::new("git");
            cmd.arg("diff");

            if let Some(range_str) = range {
                cmd.arg(range_str);
            } else {
                cmd.arg("--cached");
            }

            let output = cmd.output()?;
            let diff_str = String::from_utf8_lossy(&output.stdout).into_owned();
            if diff_str.trim().is_empty() {
                Err(anyhow::anyhow!("No changes to commit"))
            } else {
                Ok(diff_str)
            }
        }
    }
}

pub async fn generate(args: &cli::Args, config: &Config) -> anyhow::Result<String> {
    let diff = get_diff(args.diff_file.as_deref(), args.range.as_deref())?;
    let message = generate_commit_message(&diff, config, args.prompt.as_deref()).await?;
    Ok(message)
}

async fn generate_branch_name_with_ai(
    diff: &str,
    prefix: Option<&str>,
    config: &Config,
) -> anyhow::Result<String> {
    let auth = Auth::new(config.api_key.as_str());

    let openai = OpenAI::new(auth, &config.api_base());

    let prompt = BRANCH_NAME_PROMPT.replace("{{diff}}", diff);
    let messages = vec![
        Message {
            role: Role::System,
            content: "你是一个代码版本控制专家，擅长创建描述性的分支名。".to_string(),
        },
        Message {
            role: Role::User,
            content: prompt,
        },
    ];

    let chat = ChatBody {
        model: config
            .model
            .as_deref()
            .unwrap_or(DEFAULT_OPENAI_MODEL)
            .to_owned(),
        messages,
        temperature: Some(0.2f32),
        top_p: None,
        n: None,
        stream: Some(false),
        stop: None, // 移除 stop words 以避免思考过程中的干扰
        max_tokens: Some(DEFAULT_MAX_TOKENS as i32),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };

    let response = openai
        .chat_completion_create(&chat)
        .map_err(|e| anyhow::anyhow!("Failed to create chat completion: {}", e))?;
    let msg = response
        .choices
        .first()
        .ok_or(anyhow::anyhow!("No choices in response"))?
        .message
        .as_ref()
        .ok_or(anyhow::anyhow!("No message in response"))?
        .content
        .trim()
        .to_string();

    let branch_name = extract_aicommit_message(&msg)?;

    // Clean up the branch name
    let branch_name = if let Some(prefix) = prefix {
        format!("{}{}", prefix.trim(), branch_name.trim())
    } else {
        branch_name.trim().to_string()
    };

    if branch_name.is_empty() {
        return Err(anyhow::anyhow!("Failed to generate valid branch name"));
    }

    Ok(branch_name)
}

pub async fn generate_branch(args: &cli::Args, config: &Config) -> anyhow::Result<String> {
    let diff = get_diff(args.diff_file.as_deref(), args.range.as_deref())?;
    let prefix = args
        .branch_prefix
        .as_deref()
        .or(config.branch_prefix.as_deref());
    let branch_name = generate_branch_name_with_ai(&diff, prefix, config).await?;
    Ok(branch_name)
}

pub async fn generate_both(args: &cli::Args, config: &Config) -> anyhow::Result<(String, String)> {
    let diff = get_diff(args.diff_file.as_deref(), args.range.as_deref())?;
    let prefix = args
        .branch_prefix
        .as_deref()
        .or(config.branch_prefix.as_deref());
    let branch_name = generate_branch_name_with_ai(&diff, prefix, config).await?;
    let commit_message = generate_commit_message(&diff, config, args.prompt.as_deref()).await?;
    Ok((branch_name, commit_message))
}
