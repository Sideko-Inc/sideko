use sideko_rest_api::models::{Api, LintResult};

use crate::utils::url_builder::ApiUrl;

pub struct TabledApi {
    pub api: Api,
    pub subdomain: String,
}
impl tabled::Tabled for TabledApi {
    const LENGTH: usize = 3;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![
            self.api.name.as_str().into(),
            self.api.version_count.to_string().into(),
            ApiUrl::new(&self.api.name).build(&self.subdomain).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "versions".into(), "🔗 link".into()]
    }
}

pub struct TabledLintResult {
    pub filename: String,
    pub result: LintResult,
}

impl tabled::Tabled for TabledLintResult {
    const LENGTH: usize = 6;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let location = format!(
            "{filename}:{start_line}:{start_col}",
            filename = &self.filename,
            start_line = self.result.location.start_line,
            start_col = self.result.location.start_column
        );

        vec![
            location.into(),
            self.result.location.path.as_str().into(),
            self.result.rule.as_str().into(),
            self.result.category.as_str().into(),
            self.result.severity.to_string().into(),
            self.result.message.as_str().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "Location".into(),
            "Path".into(),
            "Rule".into(),
            "Category".into(),
            "Severity".into(),
            "Message".into(),
        ]
    }
}
