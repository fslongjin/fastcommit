use super::types::*;
use regex::Regex;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub struct HybridWrapper {
    code_block_regex: Regex,
    link_regex: Regex,
    inline_code_regex: Regex,
}

impl HybridWrapper {
    pub fn new() -> Self {
        Self {
            code_block_regex: Regex::new(r"```[\s\S]*?```").unwrap(),
            link_regex: Regex::new(r"https?://[^\s]+|\[([^\]]+)\]\(([^)]+)\)").unwrap(),
            inline_code_regex: Regex::new(r"`[^`]+`").unwrap(),
        }
    }
}

impl WordWrapper for HybridWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String {
        if text.is_empty() {
            return String::new();
        }

        // 解析文本段
        let segments = self.parse_segments(text);

        // 处理分段文本
        self.wrap_segments(&segments, config)
    }

    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String {
        let mut result = String::new();
        let mut current_line = String::new();
        let mut current_width = config.indent.width();

        for segment in segments {
            let processed = self.process_segment(segment, config);

            if current_width + processed.width() <= config.max_width {
                current_line.push_str(&processed);
                current_width += processed.width();
            } else {
                // 当前行放不下，需要换行
                if !current_line.is_empty() {
                    result.push_str(&current_line);
                    result.push('\n');
                }

                // 新行处理
                current_line = config.indent.clone();
                if !current_line.is_empty() {
                    current_line.push_str(&config.hanging_indent);
                }
                current_line.push_str(&processed);
                current_width = current_line.width();
            }
        }

        if !current_line.is_empty() {
            result.push_str(&current_line);
        }

        result
    }
}

impl HybridWrapper {
    fn parse_segments(&self, text: &str) -> Vec<TextSegment> {
        let mut segments = Vec::new();
        let mut remaining = text.to_string();

        // 处理代码块
        while let Some(mat) = self.code_block_regex.find(&remaining) {
            let before = &remaining[..mat.start()];
            if !before.is_empty() {
                let sub_segments = self.parse_links_and_code(before);
                segments.extend(sub_segments);
            }

            segments.push(TextSegment::CodeBlock(mat.as_str().to_string()));
            remaining = remaining[mat.end()..].to_string();
        }

        // 处理剩余文本中的链接和行内代码
        let final_segments = self.parse_links_and_code(&remaining);
        segments.extend(final_segments);

        segments
    }

    fn parse_links_and_code(&self, text: &str) -> Vec<TextSegment> {
        let mut segments = Vec::new();
        let mut remaining = text.to_string();

        // 处理链接
        while let Some(mat) = self.link_regex.find(&remaining) {
            let before = &remaining[..mat.start()];
            if !before.is_empty() {
                let code_segments = self.parse_inline_code(before);
                segments.extend(code_segments);
            }

            let link_text = mat.as_str();
            if link_text.starts_with('[') {
                // Markdown 链接 [text](url)
                if let Some(caps) = self.link_regex.captures(link_text) {
                    let text = caps.get(1).unwrap().as_str();
                    let url = caps.get(2).unwrap().as_str();
                    segments.push(TextSegment::Link(url.to_string(), text.to_string()));
                }
            } else {
                // 直接 URL
                segments.push(TextSegment::Link(
                    link_text.to_string(),
                    link_text.to_string(),
                ));
            }

            remaining = remaining[mat.end()..].to_string();
        }

        // 处理剩余文本中的行内代码
        let code_segments = self.parse_inline_code(&remaining);
        segments.extend(code_segments);

        segments
    }

    fn parse_inline_code(&self, text: &str) -> Vec<TextSegment> {
        let mut segments = Vec::new();
        let mut remaining = text.to_string();

        // 处理行内代码
        while let Some(mat) = self.inline_code_regex.find(&remaining) {
            let before = &remaining[..mat.start()];
            if !before.is_empty() {
                segments.push(TextSegment::PlainText(before.to_string()));
            }

            segments.push(TextSegment::InlineCode(mat.as_str().to_string()));
            remaining = remaining[mat.end()..].to_string();
        }

        if !remaining.is_empty() {
            segments.push(TextSegment::PlainText(remaining));
        }

        segments
    }

    fn process_segment(&self, segment: &TextSegment, config: &WrapConfig) -> String {
        match segment {
            TextSegment::PlainText(text) => {
                if config.handle_code_blocks {
                    self.wrap_plain_text(text, config)
                } else {
                    text.clone()
                }
            }
            TextSegment::CodeBlock(code) => {
                if config.handle_code_blocks {
                    format!("\n{}\n", code)
                } else {
                    code.clone()
                }
            }
            TextSegment::Link(url, text) => {
                if config.preserve_links {
                    format!("[{}]({})", text, url)
                } else {
                    text.clone()
                }
            }
            TextSegment::InlineCode(code) => {
                format!("`{}`", code)
            }
        }
    }

    fn wrap_plain_text(&self, text: &str, config: &WrapConfig) -> String {
        if config.preserve_paragraphs {
            // 保留段落格式的处理方式
            self.wrap_with_paragraphs(text, config)
        } else {
            // 原有的处理方式
            self.wrap_without_paragraphs(text, config)
        }
    }

    fn wrap_with_paragraphs(&self, text: &str, config: &WrapConfig) -> String {
        let mut result = String::new();
        let paragraphs: Vec<&str> = text.split("\n\n").collect();

        for (i, paragraph) in paragraphs.iter().enumerate() {
            if i > 0 {
                result.push_str("\n\n"); // 段落之间保留空行
            }

            // 检查段落内是否有换行符，如果有则保留
            if paragraph.contains('\n') {
                let lines: Vec<&str> = paragraph.lines().collect();
                for (j, line) in lines.iter().enumerate() {
                    if j > 0 {
                        result.push('\n');
                    }
                    if !line.trim().is_empty() {
                        let wrapped_line = self.wrap_without_paragraphs(line.trim(), config);
                        result.push_str(&wrapped_line);
                    } else {
                        result.push('\n');
                    }
                }
            } else {
                let wrapped_paragraph = self.wrap_without_paragraphs(paragraph.trim(), config);
                result.push_str(&wrapped_paragraph);
            }
        }

        result
    }

    fn wrap_without_paragraphs(&self, text: &str, config: &WrapConfig) -> String {
        let words: Vec<&str> = if config.preserve_words {
            text.split_whitespace().collect()
        } else {
            text.split(' ').collect()
        };

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = config.indent.width();

        for word in words {
            let word_width = word.width();
            let separator_width = if current_line.is_empty() { 0 } else { 1 };

            if current_width + separator_width + word_width <= config.max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
                current_width += separator_width + word_width;
            } else {
                // 当前单词放不下，需要换行
                if config.break_long_words && word_width > config.max_width {
                    // 长单词强制换行
                    if !current_line.is_empty() {
                        lines.push(current_line);
                        current_line = String::new();
                        current_width = config.indent.width();
                    }

                    let mut remaining = word;
                    while !remaining.is_empty() {
                        let available = config.max_width - current_width;
                        let (part, rest) = if remaining.width() <= available {
                            (remaining, "")
                        } else {
                            self.break_word_at_width(remaining, available)
                        };

                        if !current_line.is_empty() {
                            current_line.push(' ');
                        }
                        current_line.push_str(part);
                        current_width = current_line.width();

                        if !rest.is_empty() {
                            lines.push(current_line);
                            current_line = config.hanging_indent.clone();
                            current_width = current_line.width();
                            remaining = rest;
                        } else {
                            break;
                        }
                    }
                } else {
                    // 普通换行
                    if !current_line.is_empty() {
                        lines.push(current_line);
                    }
                    current_line = config.hanging_indent.clone();
                    current_line.push_str(word);
                    current_width = current_line.width();
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines.join("\n")
    }

    fn break_word_at_width<'a>(&self, word: &'a str, max_width: usize) -> (&'a str, &'a str) {
        let mut current_width = 0;
        for (i, ch) in word.char_indices() {
            current_width += ch.width_cjk().unwrap_or(1);
            if current_width > max_width {
                return (&word[..i], &word[i..]);
            }
        }
        (word, "")
    }
}
