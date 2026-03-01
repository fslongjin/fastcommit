use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::UPDATE_CHECKER_URL;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub name: String,
    pub repo: String,
    pub version: String,
    pub tag: String,
    pub title: String,
    pub published_at: String,
    #[serde(default)]
    pub body_markdown: String,
    #[serde(default)]
    pub body_excerpt: String,
    pub url: String,
    #[serde(default)]
    pub assets: Vec<Asset>,
    pub cache: Cache,
    pub upstream_status: String,
    #[serde(default)]
    pub commit_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Asset {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub download_url: String,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub fetched_at: String,
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub swr_expiry: String,
    #[serde(default)]
    pub stale: bool,
}

// 检查更新的主函数
pub async fn check_for_updates() -> Result<Option<UpdateInfo>> {
    // 获取当前版本
    let current_version = env!("CARGO_PKG_VERSION");
    debug!("当前版本: {current_version}");

    // 获取缓存路径
    let cache_path = get_cache_path()?;
    debug!("缓存路径: {cache_path}");

    // 检查是否有有效的缓存
    if let Some(cached_info) = load_cached_info(&cache_path)? {
        debug!("找到缓存的更新信息");
        if is_cache_valid(&cached_info) {
            debug!("缓存有效");
            // 如果有新版本，返回更新信息
            if is_newer_version(&cached_info.version, current_version) {
                debug!("发现新版本 (缓存): {}", cached_info.version);
                return Ok(Some(cached_info));
            }
            debug!("没有发现新版本");
            return Ok(None);
        } else {
            debug!("缓存已过期");
        }
    } else {
        debug!("未找到缓存文件");
    }

    // 从网络获取最新版本信息
    debug!("从网络获取最新版本信息");
    let update_info = fetch_latest_version().await?;
    debug!("获取到最新版本信息: {:?}", update_info.version);

    // 缓存更新信息
    save_cached_info(&cache_path, &update_info)?;
    debug!("更新信息已缓存");

    // 检查是否有新版本
    if is_newer_version(&update_info.version, current_version) {
        debug!("发现新版本: {}", update_info.version);
        Ok(Some(update_info))
    } else {
        debug!("当前版本已是最新");
        Ok(None)
    }
}

// 获取缓存文件路径
fn get_cache_path() -> Result<String> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("无法获取home目录"))?;
    let cache_dir = home_dir.join(".fastcommit");
    fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join("update_cache.json");
    Ok(cache_file.to_string_lossy().to_string())
}

// 加载缓存的更新信息
fn load_cached_info(path: &str) -> Result<Option<UpdateInfo>> {
    debug!("尝试加载缓存文件: {path}");
    if !Path::new(path).exists() {
        debug!("缓存文件不存在");
        return Ok(None);
    }

    debug!("读取缓存文件内容");
    let content = fs::read_to_string(path)?;
    debug!("解析缓存文件内容");
    let update_info: UpdateInfo = serde_json::from_str(&content)?;
    debug!("成功加载缓存信息: {:?}", update_info.version);
    Ok(Some(update_info))
}

// 保存更新信息到缓存
fn save_cached_info(path: &str, info: &UpdateInfo) -> Result<()> {
    debug!("保存更新信息到缓存: {path}");
    let content = serde_json::to_string_pretty(info)?;
    fs::write(path, content)?;
    debug!("缓存保存成功");
    Ok(())
}

// 检查缓存是否有效
fn is_cache_valid(info: &UpdateInfo) -> bool {
    debug!("检查缓存是否有效");
    let expired_time = parse_time(&info.cache.expires_at);
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if let Some(expired_time) = expired_time {
        let is_valid = current_time < expired_time;
        debug!("过期时间: {expired_time}, 当前时间: {current_time}, 缓存有效: {is_valid}");
        is_valid
    } else {
        debug!("无法解析缓存时间，缓存无效");
        false
    }
}

// 解析时间字符串
fn parse_time(time_str: &str) -> Option<u64> {
    debug!("解析时间字符串: {time_str}");
    // 处理两种时间格式
    if let Ok(ts) = time_str.parse::<u64>() {
        debug!("解析为时间戳: {ts}");
        return Some(ts);
    }

    // 尝试解析RFC3339格式的时间
    if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(time_str) {
        let timestamp = datetime.timestamp() as u64;
        debug!("解析为RFC3339时间: {timestamp}");
        Some(timestamp)
    } else {
        debug!("无法解析时间字符串");
        None
    }
}

// 从网络获取最新版本信息
async fn fetch_latest_version() -> Result<UpdateInfo> {
    let url = UPDATE_CHECKER_URL;
    debug!("发送请求到: {url}");

    let response = reqwest::get(url)
        .await
        .map_err(|e| anyhow::anyhow!("请求更新服务器失败: {:?}", e.source()))?;
    debug!("收到响应状态: {}", response.status());

    // 检查响应状态
    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            // 如果是404，返回默认的更新信息（表示没有可用的更新）
            debug!("服务器返回404，使用默认更新信息");
            return Ok(create_default_update_info());
        }
        return Err(anyhow::anyhow!(
            "服务器返回错误状态码: {}",
            response.status()
        ));
    }

    // 解析响应体
    let update_info: UpdateInfo = response.json().await?;
    debug!("解析更新信息成功: {:?}", update_info.version);
    Ok(update_info)
}

// 创建默认的更新信息（当服务器不可用时使用）
fn create_default_update_info() -> UpdateInfo {
    UpdateInfo {
        name: "fastcommit".to_string(),
        repo: "https://github.com/fslongjin/fastcommit".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        tag: "".to_string(),
        title: "".to_string(),
        published_at: "".to_string(),
        body_markdown: "".to_string(),
        body_excerpt: "".to_string(),
        url: "".to_string(),
        assets: Vec::new(),
        cache: Cache {
            fetched_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
            expires_at: "".to_string(),
            swr_expiry: "".to_string(),
            stale: false,
        },
        upstream_status: "error".to_string(),
        commit_hash: "".to_string(),
    }
}

// 比较版本号
fn is_newer_version(remote_version: &str, current_version: &str) -> bool {
    debug!("比较版本号: 远程版本={remote_version}, 当前版本={current_version}");
    // 移除版本号前的'v'字符（如果存在）
    let remote = remote_version.strip_prefix('v').unwrap_or(remote_version);
    let current = current_version.strip_prefix('v').unwrap_or(current_version);

    // 简单的版本比较（按分段比较）
    let remote_parts: Vec<&str> = remote.split('.').collect();
    let current_parts: Vec<&str> = current.split('.').collect();

    debug!(
        "解析后的版本号: 远程版本={:?}, 当前版本={:?}",
        remote_parts, current_parts
    );

    for (r, c) in remote_parts.iter().zip(current_parts.iter()) {
        match (r.parse::<u32>(), c.parse::<u32>()) {
            (Ok(r_num), Ok(c_num)) => {
                debug!("比较数字版本段: {r_num} vs {c_num}");
                if r_num > c_num {
                    debug!("远程版本更新");
                    return true;
                } else if r_num < c_num {
                    debug!("当前版本更新或相等");
                    return false;
                }
                // 如果相等，继续比较下一段
                debug!("版本段相等，继续比较下一段");
            }
            _ => {
                // 如果无法解析为数字，按字典序比较
                debug!("按字典序比较版本段: {r} vs {c}");
                if r > c {
                    debug!("远程版本更新");
                    return true;
                } else if r < c {
                    debug!("当前版本更新或相等");
                    return false;
                }
            }
        }
    }

    // 如果所有段都相等，检查是否有更多段
    let result = remote_parts.len() > current_parts.len();
    debug!("版本段比较完成，远程版本是否更新: {result}");
    result
}

// Display update information
pub fn display_update_info(update_info: &UpdateInfo) {
    let border = "─";
    let corner_tl = "╭";
    let corner_tr = "╮";
    let corner_bl = "╰";
    let corner_br = "╯";
    let vertical = "│";

    // Sanitize the tag to prevent command injection: only allow alphanumeric, '.', '-', '_'
    let sanitized_tag = update_info
        .tag
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect::<String>();

    let content = vec![
        "✨ fastcommit has a new version available!".to_string(),
        String::new(),
        format!("📦 New version: {}", update_info.version),
        format!("📅 Release date: {}", update_info.published_at),
        String::new(),
        "🚀 Install the new version with the following command:".to_string(),
        format!(
            "  cargo install --git https://github.com/fslongjin/fastcommit --tag {}",
            sanitized_tag
        ),
    ];

    // Calculate display width properly (accounting for emojis)
    let max_width = content
        .iter()
        .map(|s| {
            let mut width = 0;
            for c in s.chars() {
                // Emojis and wide characters take up more space
                if c as u32 >= 0x1F600 || c as u32 >= 0x1000 {
                    // Emojis and CJK characters
                    width += 2;
                } else {
                    width += 1;
                }
            }
            width
        })
        .max()
        .unwrap_or(0);
    let box_width = max_width + 4; // 2 spaces padding on each side

    // Top border
    println!("{corner_tl}{}{corner_tr}", border.repeat(box_width));

    // Content lines
    for line in content {
        let mut display_width = 0;
        for c in line.chars() {
            // Emojis and wide characters take up more space
            if c as u32 >= 0x1F600 || c as u32 >= 0x1000 {
                // Emojis and CJK characters
                display_width += 2;
            } else {
                display_width += 1;
            }
        }

        print!("{vertical}  ");
        print!("{line}");
        // Add padding to align the right border
        for _ in 0..(max_width - display_width) {
            print!(" ");
        }
        println!("  {vertical}");
    }

    // Bottom border
    println!("{corner_bl}{}{corner_br}", border.repeat(box_width));
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(is_newer_version("v0.1.8", "v0.1.7"));
        assert!(is_newer_version("v0.2.0", "v0.1.7"));
        assert!(is_newer_version("v1.0.0", "v0.1.7"));
        assert!(!is_newer_version("v0.1.7", "v0.1.7"));
        assert!(!is_newer_version("v0.1.6", "v0.1.7"));
    }
}
