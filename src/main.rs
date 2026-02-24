use clap::Parser;
use log::error;
use text_wrapper::{TextWrapper, WrapConfig};

mod animation;
mod cli;
mod config;
mod constants;
mod generate;
mod sanitizer;
mod template_engine;
mod text_wrapper;
mod update_checker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // 启动spinner动画
    let spinner = animation::Spinner::new();
    spinner.start_with_random_messages().await;

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
    if args.no_sanitize {
        // CLI override to disable sanitizer
        config.sanitize_secrets = false;
    }

    // 合并 auto_commit 参数
    let auto_commit = args.commit || config.auto_commit;
    let commit_args = if args.commit_args.is_empty() {
        &config.commit_args
    } else {
        &args.commit_args
    };

    // 确定是否启用文本包装 (CLI 参数优先级高于配置)
    let enable_wrapping = !args.no_wrap && config.text_wrap.enabled;

    // 预创建统一的包装配置和包装器 (如果需要)
    let wrapper = if enable_wrapping {
        let wrap_config =
            WrapConfig::from_config_and_args(&config.text_wrap, args.wrap_width, false);
        Some(TextWrapper::new(wrap_config))
    } else {
        None
    };

    run_update_checker().await;

    // 创建提交消息专用的包装器（启用段落保留）
    let commit_wrapper = if enable_wrapping {
        let wrap_config =
            WrapConfig::from_config_and_args(&config.text_wrap, args.wrap_width, true);
        Some(TextWrapper::new(wrap_config))
    } else {
        None
    };

    // 根据参数决定生成内容
    if args.generate_branch && args.generate_message {
        // 生成分支名 + 提交信息
        let (branch_name, msg) = generate::generate_both(&args, &config).await?;
        spinner.finish();
        print_wrapped_content(&wrapper, &branch_name, Some("Generated branch name:"));
        print_wrapped_content(&commit_wrapper, &msg, None);
        if auto_commit {
            generate::execute_git_commit(&msg, commit_args)?;
        }
    } else if args.generate_branch {
        // 仅生成分支名
        let branch_name = generate::generate_branch(&args, &config).await?;
        spinner.finish();
        print_wrapped_content(&wrapper, &branch_name, Some("Generated branch name:"));
    } else {
        // 仅生成提交信息（默认行为）
        let msg = generate::generate(&args, &config).await?;
        spinner.finish();
        print_wrapped_content(&commit_wrapper, &msg, None);
        if auto_commit {
            generate::execute_git_commit(&msg, commit_args)?;
        }
    }
    Ok(())
}

fn print_wrapped_content(wrapper: &Option<TextWrapper>, content: &str, prefix: Option<&str>) {
    if let Some(wrapper) = wrapper {
        if let Some(p) = prefix {
            println!("{} {}", p, wrapper.wrap(content));
        } else {
            println!("{}", wrapper.wrap(content));
        }
    } else {
        if let Some(p) = prefix {
            println!("{} {}", p, content);
        } else {
            println!("{}", content);
        }
    }
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
