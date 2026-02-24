pub mod hybrid_wrapper;
#[allow(clippy::module_inception)]
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
        let config = WrapConfig {
            max_width: 20,
            ..Default::default()
        };
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
        let config = WrapConfig {
            max_width: 10,
            break_long_words: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Thisisaverylongwordthatneedstobebroken";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_code_block_preservation() {
        let config = WrapConfig {
            max_width: 20,
            handle_code_blocks: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Here is some text ```code block``` and more text";
        let result = wrapper.wrap(text);

        assert!(result.contains("code block"));
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_code_block_disabled() {
        let config = WrapConfig {
            max_width: 20,
            handle_code_blocks: false,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Here is some text ```code block``` and more text";
        let result = wrapper.wrap(text);

        // 代码块不应该被特殊处理
        assert!(!result.contains("\ncode block\n"));
    }

    #[test]
    fn test_link_preservation() {
        let config = WrapConfig {
            max_width: 30,
            preserve_links: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Check out this link: https://example.com/some/long/url for more info";
        let result = wrapper.wrap(text);

        assert!(result.contains("https://example.com"));
        assert!(result.contains("["));
        assert!(result.contains("]"));
    }

    #[test]
    fn test_link_disabled() {
        let config = WrapConfig {
            max_width: 30,
            preserve_links: false,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Check out this link: https://example.com/some/long/url for more info";
        let result = wrapper.wrap(text);

        // 链接不应该被转换为 markdown 格式
        assert!(!result.contains("]("));
    }

    #[test]
    fn test_markdown_links() {
        let config = WrapConfig {
            max_width: 30,
            preserve_links: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "See [Example](https://example.com) for details";
        let result = wrapper.wrap(text);

        assert!(result.contains("[Example](https://example.com)"));
    }

    #[test]
    fn test_inline_code() {
        let config = WrapConfig {
            max_width: 20,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Use the `command` to run it";
        let result = wrapper.wrap(text);

        assert!(result.contains("`command`"));
    }

    #[test]
    fn test_inline_code_no_double_backticks() {
        // 测试修复：行内代码不应该重复添加反引号
        let config = WrapConfig {
            max_width: 80,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Change the default path from ``config.json`` to ``settings.json``";
        let result = wrapper.wrap(text);

        // 验证没有重复的反引号（不应该出现`` `code` ``这样的格式）
        assert!(!result.contains("`` `"));
        assert!(!result.contains("` ``"));
        // 验证行内代码格式正确
        assert!(result.contains("`config.json`"));
        assert!(result.contains("`settings.json`"));
    }

    #[test]
    fn test_inline_code_wrapping() {
        // 测试行内代码的换行处理：应该作为整体，不应该被拆分
        let config = WrapConfig {
            max_width: 30,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);
        let text = "Change the default path from ``config.json`` to ``settings.json``";
        let result = wrapper.wrap(text);

        // 验证行内代码没有被拆分（每个行内代码应该完整出现）
        let lines: Vec<&str> = result.lines().collect();
        // 检查每一行，确保行内代码是完整的
        for line in &lines {
            // 如果一行包含反引号，应该成对出现
            let backtick_count = line.matches('`').count();
            assert_eq!(
                backtick_count % 2,
                0,
                "行内代码的反引号应该成对出现: {line}"
            );
        }
    }

    #[test]
    fn test_mixed_content() {
        let config = WrapConfig {
            max_width: 25,
            handle_code_blocks: true,
            preserve_links: true,
            ..Default::default()
        };
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
        let config = WrapConfig {
            max_width: 80,
            handle_code_blocks: true,
            preserve_links: true,
            preserve_words: true,
            ..Default::default()
        };
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
                assert!(line.width() <= 80, "Line '{line}' exceeds width 80");
            }
        }
    }

    #[test]
    fn test_list_item_no_wrap_after_dash() {
        // 测试列表项在 `-` 后不换行
        let config = WrapConfig {
            max_width: 80,
            preserve_paragraphs: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);

        // 测试用例：列表项以 `-` 开头，后续文本很长
        let text = "- This is a very long list item that should wrap properly without breaking after the dash";
        let result = wrapper.wrap(text);

        // 验证 `-` 不在单独一行
        let lines: Vec<&str> = result.lines().collect();
        // 第一行应该包含 `-` 和后续文本，不应该只有 `-`
        assert!(!lines[0].trim().eq("-"), "列表项不应该在 `-` 后立即换行");
        assert!(lines[0].contains("-"), "第一行应该包含 `-`");
        assert!(lines[0].len() > 1, "第一行不应该只有 `-`");
    }

    #[test]
    fn test_list_item_with_inline_code_as_whole() {
        // 测试列表项包含行内代码时作为整体处理
        let config = WrapConfig {
            max_width: 80,
            preserve_paragraphs: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);

        // 测试用例：列表项包含行内代码，不应该在行内代码前后换行
        let text = "- Use the `Config` class to initialize the system and then process the data";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();
        // 验证第一行包含 `-`、行内代码和后续文本
        assert!(lines[0].contains("-"), "第一行应该包含 `-`");
        assert!(
            lines[0].contains("`Config`"),
            "第一行应该包含行内代码 `Config`"
        );
        assert!(lines[0].contains("class"), "第一行应该包含后续文本");

        // 验证行内代码前后不换行（即第一行应该包含 `-` 和 `Use` 以及 `Config`）
        assert!(
            lines[0].contains("-") && lines[0].contains("Use") && lines[0].contains("`Config`"),
            "列表项应该在行内代码前后保持整体，第一行: '{}'",
            lines[0]
        );
    }

    #[test]
    fn test_list_item_with_multiple_inline_codes() {
        // 测试列表项包含多个行内代码时作为整体处理
        let config = WrapConfig {
            max_width: 80,
            preserve_paragraphs: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);

        // 测试用例：列表项包含多个行内代码
        let text = "- Add `getUser` method to `UserService` class for retrieving user information";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();
        // 验证第一行包含 `-` 和第一个行内代码
        assert!(lines[0].contains("-"), "第一行应该包含 `-`");
        assert!(
            lines[0].contains("`getUser`"),
            "第一行应该包含第一个行内代码"
        );

        // 验证列表项作为整体处理，行内代码前后不换行
        assert!(
            lines[0].contains("-") && lines[0].contains("Add") && lines[0].contains("`getUser`"),
            "列表项应该在第一个行内代码前后保持整体，第一行: '{}'",
            lines[0]
        );
    }

    #[test]
    fn test_list_item_preserve_paragraphs() {
        // 测试列表项在保留段落格式时的换行行为
        let config = WrapConfig {
            max_width: 80,
            preserve_paragraphs: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);

        // 测试用例：包含多个列表项的段落
        let text = "Feature: Add new functionality\n\n- Implement `FeatureA` class with `method1` and `method2`\n- Add `FeatureB` module to handle data processing\n- Create unit tests for all new components";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();

        // 验证每个列表项都不应该在 `-` 后立即换行
        let mut found_list_items = 0;
        for line in &lines {
            if line.trim().starts_with("-") {
                found_list_items += 1;
                // 验证列表项的第一行不应该只有 `-`
                assert!(
                    line.trim().len() > 1,
                    "列表项不应该在 `-` 后立即换行: {line}"
                );
            }
        }
        assert!(found_list_items >= 3, "应该找到至少3个列表项");
    }

    #[test]
    fn test_list_item_long_text_wrapping() {
        // 测试列表项文本很长时的换行行为
        let config = WrapConfig {
            max_width: 50,
            preserve_paragraphs: true,
            ..Default::default()
        };
        let wrapper = TextWrapper::new(config);

        // 测试用例：列表项文本很长，应该换行，但 `-` 不应该单独一行
        let text = "- This is a very long list item that contains multiple words and should wrap to multiple lines when the width is limited";
        let result = wrapper.wrap(text);

        let lines: Vec<&str> = result.lines().collect();
        // 验证第一行包含 `-` 和部分文本
        assert!(lines[0].contains("-"), "第一行应该包含 `-`");
        assert!(!lines[0].trim().eq("-"), "第一行不应该只有 `-`");
        assert!(lines[0].len() > 1, "第一行应该包含文本内容");

        // 验证后续行使用悬挂缩进（如果有）
        if lines.len() > 1 {
            // 后续行不应该以 `-` 开头（除非是新的列表项）
            for line in lines.iter().skip(1) {
                if !line.trim().is_empty() {
                    assert!(
                        !line.trim().starts_with("-") || line.trim().starts_with("- "),
                        "后续行不应该意外包含列表标记: {line}"
                    );
                }
            }
        }
    }
}
