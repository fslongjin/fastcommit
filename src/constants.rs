pub const DEFAULT_OPENAI_API_BASE: &str = "https://api.openai.com/v1";

pub const DEFAULT_OPENAI_MODEL: &str = "gpt-3.5-turbo";

pub const DEFAULT_PROMPT_TEMPLATE: &str = r#"
# 角色

作为代码版本控制专家，请分析以下变更并生成commit message。

# 要求：
- **使用约定式提交规范?： {{conventional_commit}}**
- **只输出最终message**
- 使用{{language}}编写
- 详细程度：{{verbosity_level}}

{{user_description}}

# 什么是约定式提交规范？

约定式提交 1.0.0
概述
约定式提交规范是一种基于提交信息的轻量级约定。 它提供了一组简单规则来创建清晰的提交历史； 这更有利于编写自动化工具。 通过在提交信息中描述功能、修复和破坏性变更， 使这种惯例与 SemVer 相互对应。

提交说明的结构如下所示：

原文：

<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
译文：

<类型>[可选 范围]: <描述>

[可选 正文]

[可选 脚注]
提交说明包含了下面的结构化元素，以向类库使用者表明其意图：

fix: 类型 为 fix 的提交表示在代码库中修复了一个 bug（这和语义化版本中的 PATCH 相对应）。
feat: 类型 为 feat 的提交表示在代码库中新增了一个功能（这和语义化版本中的 MINOR 相对应）。
BREAKING CHANGE: 在脚注中包含 BREAKING CHANGE: 或 <类型>(范围) 后面有一个 ! 的提交，表示引入了破坏性 API 变更（这和语义化版本中的 MAJOR 相对应）。 破坏性变更可以是任意 类型 提交的一部分。
除 fix: 和 feat: 之外，也可以使用其它提交 类型 ，例如 @commitlint/config-conventional（基于 Angular 约定）中推荐的 build:、chore:、 ci:、docs:、style:、refactor:、perf:、test:，等等。
build: 用于修改项目构建系统，例如修改依赖库、外部接口或者升级 Node 版本等；
chore: 用于对非业务性代码进行修改，例如修改构建流程或者工具配置等；
ci: 用于修改持续集成流程，例如修改 Travis、Jenkins 等工作流配置；
docs: 用于修改文档，例如修改 README 文件、API 文档等；
style: 用于修改代码的样式，例如调整缩进、空格、空行等；
refactor: 用于重构代码，例如修改代码结构、变量名、函数名等但不修改功能逻辑；
perf: 用于优化性能，例如提升代码的性能、减少内存占用等；
test: 用于修改测试用例，例如添加、删除、修改代码的测试用例等。
脚注中除了 BREAKING CHANGE: <description> ，其它条目应该采用类似 git trailer format 这样的惯例。
其它提交类型在约定式提交规范中并没有强制限制，并且在语义化版本中没有隐式影响（除非它们包含 BREAKING CHANGE）。 可以为提交类型添加一个围在圆括号内的范围，以为其提供额外的上下文信息。例如 feat(parser): adds ability to parse arrays.。

# 输出格式案例

message内容要使用<aicommit>标签包裹，例如：
<aicommit>
（这里是commit标题）

(这里是commit message内容)

</aicommit>

变更内容：

{{diff}}
"#;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PromptTemplateReplaceLabel {
    Language,
    VerbosityLevel,
    ConventionalCommit,
    Diff,
    UserDescription,
}

impl PromptTemplateReplaceLabel {
    pub fn get_label(&self) -> &str {
        match self {
            PromptTemplateReplaceLabel::Language => "{{language}}",
            PromptTemplateReplaceLabel::VerbosityLevel => "{{verbosity_level}}",
            PromptTemplateReplaceLabel::ConventionalCommit => "{{conventional_commit}}",
            PromptTemplateReplaceLabel::Diff => "{{diff}}",
            PromptTemplateReplaceLabel::UserDescription => "{{user_description}}",
        }
    }
}

lazy_static! {
    pub static ref STOP_WORDS: Vec<String> = vec![String::from("</aicommit>")];
}

pub const DEFAULT_MAX_TOKENS: u32 = 2048;

pub const BRANCH_NAME_PROMPT: &str = r#"
# 角色
作为代码版本控制专家，请根据以下变更生成一个简洁、描述性的分支名。

# 要求：
1. 使用英文小写字母和连字符
2. 长度不超过40个字符
3. 能准确反映变更内容
4. 如果有前缀，请在前面加上前缀
5. 直接返回分支名，不要包含其他内容或解释
6. branch name内容要使用<aicommit></aicommit>标签包裹

# 示例：

branch name内容要使用<aicommit>标签包裹，例如：

## 示例1：

<aicommit>
fix-login-issue
</aicommit>

## 示例2：

<aicommit>
feat-add-user-auth
</aicommit>

## 示例3：

<aicommit>
username-refactor-payment-module
</aicommit>

变更内容：

{{diff}}
"#;

pub const UPDATE_CHECKER_URL: &str =
    "http://update-checker.longjin666.cn/v1/updates/fastcommit/latest";
