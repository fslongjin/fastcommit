# Text Auto-Wrapping Feature

## Overview

`fastcommit` now supports intelligent text auto-wrapping functionality, ensuring AI-generated commit messages and branch names are displayed properly in the terminal, enhancing user experience. This feature includes rich text processing capabilities such as code block protection, link preservation, inline code handling, and other advanced features.

## Feature Highlights

### 🎯 Smart Wrapping Strategy
- **Word boundary protection**: Automatically wraps at word boundaries to avoid breaking word integrity
- **Chinese-English mixed support**: Intelligently recognizes Chinese characters and handles mixed text correctly
- **Long word handling**: Optionally force-wrap at appropriate positions for extremely long words
- **Special text protection**: Intelligently recognizes and protects special formats like code blocks and links

### 🏗️ Text Segmentation Processing
- **Multi-layer parsing architecture**: Uses layered parsing to handle complex nested text structures
- **Intelligent segmentation recognition**: Automatically identifies different text types like code blocks, links, and inline code
- **Priority processing**: Handles different text segment types in order of importance

### 🔗 Code Block Processing
- **Code block detection**: Automatically recognizes ```code``` format code blocks
- **Format protection**: Code blocks maintain original format without being broken by word wrapping
- **Smart wrapping**: Automatically adds line breaks before and after code blocks to ensure readability
- **Configuration control**: Can be enabled or disabled through configuration

### 🔗 Link Preservation
- **Multi-format support**: Supports direct URLs and Markdown link formats
- **Smart conversion**: Automatically converts URLs to Markdown format to maintain readability
- **Link protection**: Ensures URL integrity, avoiding being split by line breaks
- **Text extraction**: Optionally display only link text, hiding full URLs

### ⚡ Inline Code Processing
- **Format recognition**: Automatically recognizes `code` format inline code
- **Protection mechanism**: Inline code maintains original format without being split by line breaks
- **Display optimization**: Ensures inline code readability when wrapping

### ⚙️ Flexible Configuration
- **Configuration file control**: Control default behavior via `~/.fastcommit/config.toml`
- **Command line arguments**: Override configuration at runtime via CLI arguments
- **Terminal adaptation**: Automatically detects terminal width or uses user-specified width
- **Priority management**: CLI arguments have higher priority than configuration file settings

## Configuration Options

### Configuration File Settings

Add the following configuration to `~/.fastcommit/config.toml`:

```toml
[text_wrap]
# Enable text wrapping (default: true)
enabled = true
# Default wrapping width (default: 80)
default_width = 80
# Protect word boundaries (default: true)
preserve_words = true
# Break long words when necessary (default: true)
break_long_words = true
# Handle code blocks (default: true)
handle_code_blocks = true
# Preserve link integrity (default: true)
preserve_links = true
```

### Configuration Option Details

#### `enabled` - Master Switch
- **Type**: `bool`
- **Default**: `true`
- **Description**: Controls whether text wrapping functionality is enabled. When set to `false`, all text wrapping processing is disabled.

#### `default_width` - Default Width
- **Type**: `usize`
- **Default**: `80`
- **Description**: Sets the default width for text wrapping (number of characters). Used when `--wrap-width` is not specified.

#### `preserve_words` - Word Protection
- **Type**: `bool`
- **Default**: `true`
- **Description**: Whether to wrap at word boundaries. When `true`, wrapping won't occur in the middle of words; when `false`, wrapping can occur at any position.

#### `break_long_words` - Long Word Handling
- **Type**: `bool`
- **Default**: `true`
- **Description**: Whether to force-break long words at appropriate positions when word length exceeds wrapping width. Only effective when `preserve_words = true`.

#### `handle_code_blocks` - Code Block Processing
- **Type**: `bool`
- **Default**: `true`
- **Description**: Whether to specially handle code blocks. When enabled, code blocks are separated and line breaks are added before and after them.

#### `preserve_links` - Link Preservation
- **Type**: `bool`
- **Default**: `true`
- **Description**: Whether to preserve complete link format. When enabled, URLs are converted to Markdown format; when disabled, only link text is displayed.

### Command Line Arguments

```bash
# Disable text wrapping
fastcommit --no-wrap

# Specify wrapping width
fastcommit --wrap-width 60

# Force enable wrapping (overrides disabled settings in config file)
fastcommit --force-wrap

# Combined usage
fastcommit --wrap-width 100 --force-wrap
```

### Priority Explanation

Parameter priority (from highest to lowest):
1. Command line `--force-wrap` parameter (force enable)
2. Command line `--no-wrap` parameter (force disable)
3. Command line `--wrap-width` parameter
4. `default_width` setting in configuration file
5. Auto-detected terminal width
6. Default value 80

## Usage Examples

### Basic Usage

```bash
# Use default configuration text wrapping
fastcommit

# Long commit messages will automatically wrap when generated
```

### Custom Width

```bash
# Set narrower display width
fastcommit --wrap-width 60

# Set wider display width
fastcommit --wrap-width 120
```

### Disable Wrapping

```bash
# Completely disable text wrapping
fastcommit --no-wrap

# Only disables for current execution, configuration file settings remain unchanged
```

### Generate Branch Names

```bash
# Generate branch name and apply wrapping
fastcommit -b

# Generate branch name and commit message simultaneously
fastcommit -b -m
```

### Complex Scenario Examples

```bash
# Handle commit messages containing code blocks
fastcommit -m "Fixed database connection issues and added retry mechanism

```sql
SELECT * FROM users WHERE status = 'active'
```

Also optimized API response time"

# Handle technical documentation containing links
fastcommit -m "Updated README documentation, see https://github.com/example/project for more info, and refactored code following [Best Practices Guide](https://docs.example.com/best-practices)"

# Handle mixed content technical commits
fastcommit -m "Added caching mechanism in `UserService`, using Redis for data storage, see ```redis_client.rs``` file for details"
```

### Configuration File Example

```toml
# ~/.fastcommit/config.toml
[text_wrap]
enabled = true
default_width = 80
preserve_words = true
break_long_words = true
handle_code_blocks = true
preserve_links = true
```

## Technical Implementation

### Core Algorithms

1. **Hybrid Wrapping Strategy**:
   - Prioritize wrapping at word boundaries
   - For Chinese text, wrap at punctuation marks
   - Force wrap at character boundaries when necessary

2. **Unicode Width Calculation**:
   - Correctly handle display width of Chinese characters
   - Support CJK character width calculation
   - Compatible with various terminal fonts

3. **Special Text Recognition**:
   - Code block recognition: ```code```
   - Link recognition: http://... and [text](url)
   - Inline code: `code`

### Architecture Design

```
TextWrapper (High-level Interface)
├── WrapConfig (Configuration Management)
├── HybridWrapper (Hybrid Strategy Implementation)
├── WordBoundaryWrapper (Word Boundary Strategy)
├── SemanticWrapper (Semantic Awareness Strategy)
├── TextSegmentProcessor (Text Segment Processing)
└── OutputRenderer (Output Rendering)
```

### Performance Optimization

- **Lazy initialization**: Regular expressions compiled on demand
- **Memory efficient**: Avoid unnecessary string copying
- **Fast algorithm**: Linear time complexity wrapping algorithm

### Code Structure

```rust
// Core type definitions
pub struct WrapConfig {
    pub max_width: usize,
    pub preserve_words: bool,
    pub break_long_words: bool,
    pub handle_code_blocks: bool,
    pub preserve_links: bool,
    pub strategy: WrapStrategy,
    pub indent: String,
    pub hanging_indent: String,
}

// Text segment types
pub enum TextSegment {
    PlainText(String),
    CodeBlock(String),
    Link(String, String),
    InlineCode(String),
}

// Wrapper trait
pub trait WordWrapper {
    fn wrap_text(&self, text: &str, config: &WrapConfig) -> String;
    fn wrap_segments(&self, segments: &[TextSegment], config: &WrapConfig) -> String;
}
```

## Troubleshooting

### Common Issues

**Q: Why isn't my text automatically wrapping?**
A: Please check:
1. Whether `text_wrap.enabled` in configuration file is `true`
2. Whether `--no-wrap` parameter is used
3. Whether terminal width is sufficient to accommodate text
4. Whether there are conflicting `--force-wrap` parameters

**Q: Chinese characters displaying incorrectly?**
A: Ensure:
1. Terminal supports Unicode characters
2. Font settings are correct
3. `preserve_words` option is not disabled
4. Using terminal font that supports CJK characters

**Q: Code blocks being unexpectedly wrapped?**
A: Check:
1. Whether `handle_code_blocks` setting is correct
2. Whether code block markers are complete (three backticks)
3. Whether code block content contains special characters

**Q: Links being split across multiple lines?**
A: Check:
1. Whether `preserve_links` is `true`
2. Whether link length exceeds `max_width`
3. Whether `break_long_words` settings are affecting it

**Q: Long words or URLs not being handled correctly?**
A: Check:
1. Whether `break_long_words` is `true`
2. Whether `preserve_words` setting is appropriate
3. Whether `max_width` setting is too small

### Debugging Methods

Enable verbose logging:

```bash
RUST_LOG=debug fastcommit
```

View current configuration:

```bash
cat ~/.fastcommit/config.toml
```

Test different configurations:

```bash
# Test different widths
fastcommit --wrap-width 60
fastcommit --wrap-width 100

# Test disabling wrapping
fastcommit --no-wrap

# Test force enabling
fastcommit --force-wrap
```

Check terminal support:

```bash
echo $COLUMNS
echo $TERM
```

## Best Practices

### Recommended Configuration

For most users, the following configuration is recommended:

```toml
[text_wrap]
enabled = true
default_width = 80
preserve_words = true
break_long_words = true
handle_code_blocks = true
preserve_links = true
```

### Adjustments for Different Scenarios

**Narrow terminals (mobile devices)**:
```toml
[text_wrap]
enabled = true
default_width = 50
preserve_words = true
break_long_words = false
```

**Wide screens (desktop development)**:
```toml
[text_wrap]
enabled = true
default_width = 100
preserve_words = true
break_long_words = true
```

**Code repositories (with lots of technical details)**:
```toml
[text_wrap]
enabled = true
default_width = 80
handle_code_blocks = true
preserve_links = true
preserve_words = true
```

**Plain text repositories (minimal formatting)**:
```toml
[text_wrap]
enabled = true
default_width = 80
handle_code_blocks = false
preserve_links = false
```

### Performance Optimization Tips

1. **Set reasonable width**: Avoid setting too small `max_width`, this causes frequent wrapping affecting performance
2. **Disable unnecessary features**: If code block or link processing is not needed, disable corresponding options
3. **Use appropriate strategies**: Choose suitable wrapping strategies based on text type
4. **Avoid frequent configuration changes**: Configuration changes trigger re-initialization, affecting performance

### Maintenance Tips

1. **Regularly update configuration**: Adjust configuration promptly as terminal environment changes
2. **Monitor performance**: If performance issues are found, check for unnecessary enabled features
3. **Backup configuration**: Important configuration files should have backups
4. **Test new features**: Verify new features in test environment before using in production

## Future Improvements

### Planned Features

- [ ] **Semantic-aware wrapping**: More intelligent wrapping based on sentence structure
- [ ] **Multi-language support**: Enhanced support for Japanese, Korean and other languages
- [ ] **Color themes**: Support for different color theme outputs
- [ ] **Real-time preview**: Interactive wrapping preview functionality
- [ ] **Configuration validation**: Configuration file syntax checking and validation

### Contribution Guidelines

Contributions are welcome! Please refer to:

1. Code style: Follow project's existing Rust code style
2. Test coverage: New features need corresponding unit tests
3. Documentation updates: Update related documentation and examples
4. Performance considerations: Ensure new features don't significantly impact performance

## License

This feature follows the project's MIT license.