use sideko_rest_api::models::DocProject;

use crate::utils::url_builder::DocUrl;

pub struct TabledDocProject {
    pub doc: DocProject,
    pub org_subdomain: String,
}
impl tabled::Tabled for TabledDocProject {
    const LENGTH: usize = 5;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![
            self.doc.name.as_str().into(),
            self.doc.current_version.version.to_string().into(),
            self.doc
                .domains
                .production
                .as_ref()
                .map(|u| format!("https://{u}"))
                .unwrap_or_default()
                .into(),
            self.doc
                .domains
                .preview
                .as_ref()
                .map(|u| format!("https://{u}"))
                .unwrap_or_default()
                .into(),
            DocUrl::new(&self.doc.name)
                .with_version(self.doc.current_version.version)
                .build(&self.org_subdomain)
                .into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "current version".into(),
            "production url".into(),
            "preview url".into(),
            "🔗 link".into(),
        ]
    }
}
