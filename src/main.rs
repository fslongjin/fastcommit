use clap::Parser;

#[macro_use]
extern crate lazy_static;

mod cli;
mod config;
mod constants;
mod generate;
mod template_engine;

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

    let msg = generate::generate(&args, &config).await?;
    println!("{}", msg);
    Ok(())
}
