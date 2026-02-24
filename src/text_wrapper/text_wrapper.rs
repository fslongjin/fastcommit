use super::hybrid_wrapper::HybridWrapper;
use super::types::*;
use unicode_width::UnicodeWidthStr;

pub struct TextWrapper {
    config: WrapConfig,
    wrapper: Box<dyn WordWrapper>,
}

impl TextWrapper {
    pub fn new(config: WrapConfig) -> Self {
        let wrapper: Box<dyn WordWrapper> = match config.strategy {
            WrapStrategy::WordBoundary => Box::new(WordBoundaryWrapper::new()),
            WrapStrategy::Hybrid => Box::new(HybridWrapper::new()),
            WrapStrategy::Semantic => Box::new(SemanticWrapper::new()),
        };

        Self { config, wrapper }
    }

    pub fn wrap(&self, text: &str) -> String {
        self.wrapper.wrap_text(text, &self.config)
    }
}

impl Default for TextWrapper {
    fn default() -> Self {
        Self::new(WrapConfig::default())
    }
}

// 单词边界包装器
pub struct WordBoundaryWrapper {
    inner: HybridWrapper,
}

impl WordBoundaryWrapper {
    pub fn new() -> Self {
        Self {
            inner: HybridWrapper::new(),
        }
    }
}

impl WordWrapper for WordBoundaryWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String {
        let mut word_config = config.clone();
        word_config.strategy = WrapStrategy::WordBoundary;
        self.inner.wrap_text(text, &word_config)
    }

    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String {
        let mut word_config = config.clone();
        word_config.strategy = WrapStrategy::WordBoundary;
        self.inner.wrap_segments(segments, &word_config)
    }
}

// 字符包装器
#[allow(dead_code)]
pub struct CharacterWrapper;

impl WordWrapper for CharacterWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String {
        let max_width = config.max_width;
        let indent = &config.indent;
        let hanging = &config.hanging_indent;

        let mut lines = Vec::new();
        let mut current_line = indent.clone();

        for ch in text.chars() {
            if current_line.width() >= max_width && !current_line.trim().is_empty() {
                lines.push(current_line);
                current_line = hanging.clone();
            }
            current_line.push(ch);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines.join("\n")
    }

    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String {
        let text: String = segments
            .iter()
            .map(|seg| match seg {
                TextSegment::PlainText(s) => s.clone(),
                TextSegment::CodeBlock(s) => s.clone(),
                TextSegment::Link(_, text) => text.clone(),
                TextSegment::InlineCode(s) => s.clone(),
            })
            .collect();

        self.wrap_text(&text, config)
    }
}

// 语义包装器
pub struct SemanticWrapper {
    inner: HybridWrapper,
}

impl SemanticWrapper {
    pub fn new() -> Self {
        Self {
            inner: HybridWrapper::new(),
        }
    }
}

impl WordWrapper for SemanticWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String {
        let mut semantic_config = config.clone();
        semantic_config.strategy = WrapStrategy::Semantic;
        self.inner.wrap_text(text, &semantic_config)
    }

    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String {
        let mut semantic_config = config.clone();
        semantic_config.strategy = WrapStrategy::Semantic;
        self.inner.wrap_segments(segments, &semantic_config)
    }
}
