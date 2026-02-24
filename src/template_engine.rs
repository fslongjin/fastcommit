use crate::{
    config::{CommitLanguage, Verbosity},
    constants::PromptTemplateReplaceLabel,
};

pub struct TemplateContext<'a> {
    pub conventional: bool,
    pub language: CommitLanguage,
    pub verbosity: Verbosity,
    pub diff_content: &'a str,
    pub user_description: Option<&'a str>,
}

impl<'a> TemplateContext<'a> {
    pub fn new(
        conventional: bool,
        language: CommitLanguage,
        verbosity: Verbosity,
        diff_content: &'a str,
        user_description: Option<&'a str>,
    ) -> Self {
        Self {
            conventional,
            language,
            verbosity,
            diff_content,
            user_description,
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
            context.verbosity.as_template_level(),
        )
        .replace(
            PromptTemplateReplaceLabel::Diff.get_label(),
            context.diff_content,
        )
        .replace(
            PromptTemplateReplaceLabel::UserDescription.get_label(),
            context.user_description.unwrap_or(""),
        );

    Ok(rendered)
}
