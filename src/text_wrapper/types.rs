#[derive(Debug, Clone, PartialEq)]
pub enum WrapStrategy {
    WordBoundary, // 单词边界换行
    Hybrid,       // 混合策略
    Semantic,     // 语义感知换行
}

#[derive(Debug, Clone)]
pub struct WrapConfig {
    pub max_width: usize,
    pub preserve_words: bool,
    pub break_long_words: bool,
    pub handle_code_blocks: bool,
    pub preserve_links: bool,
    pub preserve_paragraphs: bool,
    pub strategy: WrapStrategy,
    pub indent: String,
    pub hanging_indent: String,
}

impl WrapConfig {
    pub fn from_config_and_args(
        text_wrap_config: &crate::config::TextWrapConfig,
        wrap_width: Option<usize>,
        preserve_paragraphs: bool,
    ) -> Self {
        Self {
            max_width: wrap_width.unwrap_or(text_wrap_config.default_width),
            preserve_words: text_wrap_config.preserve_words,
            break_long_words: text_wrap_config.break_long_words,
            handle_code_blocks: text_wrap_config.handle_code_blocks,
            preserve_links: text_wrap_config.preserve_links,
            preserve_paragraphs,
            strategy: WrapStrategy::Hybrid,
            indent: String::new(),
            hanging_indent: text_wrap_config.hanging_indent.clone(),
        }
    }
}

impl Default for WrapConfig {
    fn default() -> Self {
        Self {
            max_width: 80,
            preserve_words: true,
            break_long_words: true,
            handle_code_blocks: true,
            preserve_links: true,
            preserve_paragraphs: false,
            strategy: WrapStrategy::Hybrid,
            indent: String::new(),
            hanging_indent: String::new(), // 默认无悬挂缩进，更适合大多数场景
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextSegment {
    PlainText(String),
    CodeBlock(String),
    Link(String, String), // (url, text)
    InlineCode(String),
}

pub trait WordWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String;
    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String;
}
