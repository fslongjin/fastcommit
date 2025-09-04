use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use std::time::Duration;

const RANDOM_PHRASES: &[&str] = &[
    "🤔 正在思考如何优雅地描述这次变更...",
    "✨ 魔法正在发生，请稍候...",
    "🚀 加速代码提交中...",
    "🧠 AI大脑正在疯狂运转...",
    "🌈 生成彩虹般绚丽的提交信息...",
    "⚡ 闪电般快速分析代码...",
    "🎨 为你的代码添加艺术气息...",
    "🕺 Gee Kee Tai May! Oh Baby！ ",
    "💃 Gee Kee 实在是太美！ ",
    "🎪 代码马戏团表演中...",
    "🍵 喝杯茶，马上就好...",
    "🎸 为代码变更谱写乐章...",
    "🏎️  极速代码分析中...",
    "🎭 戏剧化地描述你的变更...",
    "🌌 在代码宇宙中探索ing...",
];

pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg:.dim}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));

        Self { pb }
    }

    pub async fn start_with_random_messages(&self) {
        let pb = self.pb.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                let mut rng = rand::thread_rng();
                if let Some(phrase) = RANDOM_PHRASES.choose(&mut rng) {
                    pb.set_message(phrase.to_string());
                }
            }
        });
    }

    #[allow(dead_code)]
    pub fn finish_with_message(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }

    pub fn finish(&self) {
        self.pb.finish_and_clear();
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.finish();
    }
}
