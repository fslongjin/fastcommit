use crate::{
    config::{CommitLanguage, Verbosity},
    constants::PromptTemplateReplaceLabel,
};

pub struct TemplateContext<'a> {
    pub conventional: bool,
    pub language: CommitLanguage,
    pub verbosity: Verbosity,
    pub diff_content: &'a str,
}

impl<'a> TemplateContext<'a> {
    pub fn new(
        conventional: bool,
        language: CommitLanguage,
        verbosity: Verbosity,
        diff_content: &'a str,
    ) -> Self {
        Self {
            conventional,
            language,
            verbosity,
            diff_content,
        }
    }
}

pub fn render_template(template: &str, context: TemplateContext) -> anyhow::Result<String> {
    let rendered = template
        .replace(
            PromptTemplateReplaceLabel::ConventionalCommit.get_label(),
            &context.conventional.to_string(),
        )
        .replace(
            PromptTemplateReplaceLabel::Language.get_label(),
            &context.language.to_string(),
        )
        .replace(
            PromptTemplateReplaceLabel::VerbosityLevel.get_label(),
            context.verbosity.to_template_level(),
        )
        .replace(
            PromptTemplateReplaceLabel::Diff.get_label(),
            context.diff_content,
        );

    Ok(rendered)
}
