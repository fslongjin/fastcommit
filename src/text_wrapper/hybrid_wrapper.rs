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
            // 支持单反引号和双反引号的行内代码（如 `code` 或 ``code``）
            inline_code_regex: Regex::new(r"`{1,3}[^`]+?`{1,3}").unwrap(),
        }
    }
}

impl WordWrapper for HybridWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String {
        if text.is_empty() {
            return String::new();
        }

        // 如果 preserve_paragraphs 为 true，需要先处理段落，然后再处理段
        if config.preserve_paragraphs {
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
                            // 对每一行单独处理，但不在段级别使用 preserve_paragraphs
                            let mut line_config = config.clone();
                            line_config.preserve_paragraphs = false;
                            let segments = self.parse_segments(line.trim());
                            let wrapped_line = self.wrap_segments(&segments, &line_config);
                            result.push_str(&wrapped_line);
                        } else {
                            result.push('\n');
                        }
                    }
                } else {
                    // 段落内没有换行符，直接处理整个段落
                    // 但不在段级别使用 preserve_paragraphs，因为段落处理已经在更高层级完成
                    let mut para_config = config.clone();
                    para_config.preserve_paragraphs = false;
                    let segments = self.parse_segments(paragraph.trim());
                    let wrapped_paragraph = self.wrap_segments(&segments, &para_config);
                    result.push_str(&wrapped_paragraph);
                }
            }

            result
        } else {
            // 解析文本段
            let segments = self.parse_segments(text);

            // 处理分段文本
            self.wrap_segments(&segments, config)
        }
    }

    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String {
        let mut result = String::new();
        let mut current_line = String::new();
        let mut current_width = config.indent.width();

        // 检查是否是列表项（第一个段是 PlainText 且以 "- " 开头）
        let is_list_item = segments
            .first()
            .and_then(|s| {
                if let TextSegment::PlainText(text) = s {
                    Some(text.trim_start().starts_with("-"))
                } else {
                    None
                }
            })
            .unwrap_or(false);

        for (idx, segment) in segments.iter().enumerate() {
            // 对于行内代码，需要特殊处理：作为不可分割的单元
            // 行内代码已经包含反引号（从正则匹配中），不需要通过process_segment重复添加
            let processed = match segment {
                TextSegment::InlineCode(code) => {
                    // 行内代码已经包含反引号，直接使用，作为不可分割的单元
                    code.clone()
                }
                _ => self.process_segment(segment, config),
            };

            let processed_width = processed.width();

            // 检查当前行是否能放下这个段
            // 对于行内代码，如果当前行已经有内容且放不下，需要整体换行
            // 对于普通文本，可以继续处理
            // 特殊处理：如果是列表项，且当前行只包含列表标记（"-"），不要换行
            let is_list_marker_only = is_list_item && idx == 0 && current_line.trim() == "-";
            let should_wrap =
                current_width + processed_width > config.max_width && !is_list_marker_only;

            if !should_wrap {
                current_line.push_str(&processed);
                current_width += processed_width;
            } else {
                // 当前行放不下，需要换行
                // 特殊处理：如果是列表项，且当前行只包含列表标记（"-"），不要换行
                if !current_line.is_empty() && !is_list_marker_only {
                    result.push_str(&current_line);
                    result.push('\n');
                }

                // 新行处理
                if is_list_marker_only {
                    // 当前行只有 "-"，不要换行，继续在当前行处理
                    if !current_line.ends_with(' ') {
                        current_line.push(' ');
                    }
                    current_line.push_str(&processed);
                    current_width = current_line.width();
                } else {
                    current_line = config.indent.clone();
                    if !current_line.is_empty() {
                        current_line.push_str(&config.hanging_indent);
                    }
                    current_line.push_str(&processed);
                    current_width = current_line.width();
                }
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
                    // 在 wrap_segments 中，PlainText 段不应该使用 wrap_with_paragraphs
                    // 因为段落处理应该在更高层级（wrap_text）进行
                    // 这里只处理单词级别的换行，不处理段落
                    let mut no_paragraph_config = config.clone();
                    no_paragraph_config.preserve_paragraphs = false;
                    self.wrap_plain_text(text, &no_paragraph_config)
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
                // InlineCode段已经包含反引号（从正则匹配中），直接返回
                // 行内代码应该作为不可分割的单元，不进行额外的包装处理
                code.clone()
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
                        let trimmed = line.trim();
                        let wrapped_line = self.wrap_without_paragraphs(trimmed, config);
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

            // 特殊处理：如果当前行只包含 "-"（列表项标记），不要换行
            let is_list_marker_only = current_line.trim() == "-";

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
                    // 特殊处理：如果当前行只包含 "-"，不要换行，而是继续在当前行处理
                    if !is_list_marker_only && !current_line.is_empty() {
                        lines.push(current_line);
                        current_line = String::new();
                        current_width = config.indent.width();
                    } else if is_list_marker_only {
                        // 当前行只有 "-"，不要换行，继续在当前行处理长单词
                        if !current_line.ends_with(' ') {
                            current_line.push(' ');
                            current_width += 1;
                        }
                    }

                    let mut remaining = word;
                    while !remaining.is_empty() {
                        let available = config.max_width - current_width;
                        let (part, rest) = if remaining.width() <= available {
                            (remaining, "")
                        } else {
                            self.break_word_at_width(remaining, available)
                        };

                        if !current_line.is_empty() && !current_line.ends_with(' ') {
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
                    // 特殊处理：如果当前行只包含 "-"，不要换行，而是继续在当前行处理
                    if is_list_marker_only {
                        // 当前行只有 "-"，不要换行，继续在当前行处理
                        if !current_line.ends_with(' ') {
                            current_line.push(' ');
                        }
                        current_line.push_str(word);
                        current_width = current_line.width();
                    } else {
                        // 正常换行
                        if !current_line.is_empty() {
                            lines.push(current_line);
                        }
                        current_line = config.hanging_indent.clone();
                        current_line.push_str(word);
                        current_width = current_line.width();
                    }
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
