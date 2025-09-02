use clap::Parser;
use log::error;

mod cli;
mod config;
mod constants;
mod generate;
mod template_engine;
mod update_checker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = cli::Args::parse();
    let mut config = config::load_config().await?;

    // 合并命令行参数和配置文件
    if let Some(c) = args.conventional {
        config.conventional = c;
    }
    if let Some(l) = args.language {
        config.language = l;
    }
    if let Some(v) = args.verbosity {
        config.verbosity = v;
    }

    run_update_checker().await;

    // 根据参数决定生成内容：
    // 1. --gb --m 同时：生成分支名 + 提交信息
    // 2. 仅 --gb：只生成分支名
    // 3. 默认（无 --gb 或仅 --m）：生成提交信息
    if args.generate_branch && args.generate_message {
        let (branch_name, msg) = generate::generate_both(&args, &config).await?;
        println!("Generated branch name: {}", branch_name);
        println!("{}", msg);
    } else if args.generate_branch {
        let branch_name = generate::generate_branch(&args, &config).await?;
        println!("Generated branch name: {}", branch_name);
    } else {
        // 包括：无参数 或 仅 --m
        let msg = generate::generate(&args, &config).await?;
        println!("{}", msg);
    }
    Ok(())
}

async fn run_update_checker() {
    match update_checker::check_for_updates().await {
        Ok(Some(update_info)) => {
            update_checker::display_update_info(&update_info);
        }
        Ok(None) => {
            // 没有新版本，无需处理
        }
        Err(e) => {
            // 忽略更新检查错误，不影响主功能
            error!("Error checking for updates: {}", e);
        }
    }
}
