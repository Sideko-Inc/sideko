use sideko_rest_api::models::DocProject;

pub struct TabledDocProject(pub DocProject);
impl tabled::Tabled for TabledDocProject {
    const LENGTH: usize = 6;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let inner = &self.0;
        vec![
            inner.name.as_str().into(),
            inner
                .domains
                .production
                .as_ref()
                .map(|u| format!("https://{u}"))
                .unwrap_or_default()
                .into(),
            inner
                .domains
                .preview
                .as_ref()
                .map(|u| format!("https://{u}"))
                .unwrap_or_default()
                .into(),
            inner.current_version.version.to_string().into(),
            inner.id.as_str().into(),
            inner.created_at.as_str().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "Name".into(),
            "Production URL".into(),
            "Preview URL".into(),
            "Current Version".into(),
            "ID".into(),
            "Created At".into(),
        ]
    }
}
