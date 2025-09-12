pub mod hybrid_wrapper;
pub mod text_wrapper;
pub mod types;

pub use text_wrapper::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn test_basic_wrapping() {
        let mut config = WrapConfig::default();
        config.max_width = 20;
        let wrapper = TextWrapper::new(config);
        let text = "This is a long line of text that should be wrapped";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1);

        for line in lines {
            assert!(line.width() <= 20);
        }
    }

    #[test]
    fn test_long_word_handling() {
        let mut config = WrapConfig::default();
        config.max_width = 10;
        config.break_long_words = true;
        let wrapper = TextWrapper::new(config);
        let text = "Thisisaverylongwordthatneedstobebroken";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_code_block_preservation() {
        let mut config = WrapConfig::default();
        config.max_width = 20;
        config.handle_code_blocks = true;
        let wrapper = TextWrapper::new(config);
        let text = "Here is some text ```code block``` and more text";
        let result = wrapper.wrap(text);

        assert!(result.contains("code block"));
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_code_block_disabled() {
        let mut config = WrapConfig::default();
        config.max_width = 20;
        config.handle_code_blocks = false;
        let wrapper = TextWrapper::new(config);
        let text = "Here is some text ```code block``` and more text";
        let result = wrapper.wrap(text);

        // 代码块不应该被特殊处理
        assert!(!result.contains("\ncode block\n"));
    }

    #[test]
    fn test_link_preservation() {
        let mut config = WrapConfig::default();
        config.max_width = 30;
        config.preserve_links = true;
        let wrapper = TextWrapper::new(config);
        let text = "Check out this link: https://example.com/some/long/url for more info";
        let result = wrapper.wrap(text);

        assert!(result.contains("https://example.com"));
        assert!(result.contains("["));
        assert!(result.contains("]"));
    }

    #[test]
    fn test_link_disabled() {
        let mut config = WrapConfig::default();
        config.max_width = 30;
        config.preserve_links = false;
        let wrapper = TextWrapper::new(config);
        let text = "Check out this link: https://example.com/some/long/url for more info";
        let result = wrapper.wrap(text);

        // 链接不应该被转换为 markdown 格式
        assert!(!result.contains("]("));
    }

    #[test]
    fn test_markdown_links() {
        let mut config = WrapConfig::default();
        config.max_width = 30;
        config.preserve_links = true;
        let wrapper = TextWrapper::new(config);
        let text = "See [Example](https://example.com) for details";
        let result = wrapper.wrap(text);

        assert!(result.contains("[Example](https://example.com)"));
    }

    #[test]
    fn test_inline_code() {
        let mut config = WrapConfig::default();
        config.max_width = 20;
        let wrapper = TextWrapper::new(config);
        let text = "Use the `command` to run it";
        let result = wrapper.wrap(text);

        assert!(result.contains("`command`"));
    }

    #[test]
    fn test_mixed_content() {
        let mut config = WrapConfig::default();
        config.max_width = 25;
        config.handle_code_blocks = true;
        config.preserve_links = true;
        let wrapper = TextWrapper::new(config);
        let text =
            "Here is a link: https://example.com and some ```code block``` with `inline code`";
        let result = wrapper.wrap(text);

        assert!(result.contains("https://example.com"));
        assert!(result.contains("code block"));
        assert!(result.contains("`inline code`"));
    }

    #[test]
    fn test_complex_scenario() {
        let mut config = WrapConfig::default();
        config.max_width = 80; // 更合理的宽度
        config.handle_code_blocks = true;
        config.preserve_links = true;
        config.preserve_words = true;
        let wrapper = TextWrapper::new(config);
        let text = "This is a long commit message that contains https://example.com and some ```rust\nfn main() {\n    println!(\"Hello\");\n}\n``` plus `inline_code` for reference.";
        let result = wrapper.wrap(text);

        // 验证各种元素都被正确处理
        assert!(result.contains("https://example.com"));
        assert!(result.contains("```rust"));
        assert!(result.contains("`inline_code`"));

        // 验证换行
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1);

        // 验证每行不超过指定宽度
        for line in lines {
            // 跳过代码块行，因为它们可能很长
            if !line.trim().starts_with("```")
                && !line.trim().starts_with("fn")
                && !line.trim().starts_with("println")
            {
                assert!(line.width() <= 80, "Line '{}' exceeds width 80", line);
            }
        }
    }
}
