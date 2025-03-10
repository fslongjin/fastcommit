pub const DEFAULT_OPENAI_API_BASE: &str = "https://api.openai.com/v1";

pub const DEFAULT_OPENAI_MODEL: &str = "gpt-3.5-turbo";

pub const DEFAULT_PROMPT_TEMPLATE: &str = r#"
# 角色

作为代码版本控制专家，请分析以下变更并生成commit message。

要求：
- 使用{{language}}编写
- 详细程度：{{verbosity_level}}
- 只输出最终message
- 使用约定式提交规范? {{conventional_commit}}

# 输出格式案例

message内容要使用<aicommit>标签包裹，例如：
<aicommit>

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
}

impl PromptTemplateReplaceLabel {
    pub fn get_label(&self) -> &str {
        match self {
            PromptTemplateReplaceLabel::Language => "{{language}}",
            PromptTemplateReplaceLabel::VerbosityLevel => "{{verbosity_level}}",
            PromptTemplateReplaceLabel::ConventionalCommit => "{{conventional_commit}}",
            PromptTemplateReplaceLabel::Diff => "{{diff}}",
        }
    }
}

lazy_static! {
    pub static ref STOP_WORDS: Vec<String> = vec![String::from("</aicommit>")];
}