use sideko_rest_api::models::ApiSpec;

use crate::utils::url_builder::ApiUrl;

pub struct TabledApiSpec {
    pub version: ApiSpec,
    pub org_subdomain: String,
}
impl tabled::Tabled for TabledApiSpec {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let mock_enabled = if self.version.mock_server.enabled {
            "🟢"
        } else {
            "🔴"
        };
        vec![
            self.version.version.as_str().into(),
            self.version.api.name.as_str().into(),
            format!("{mock_enabled} {url}", url = &self.version.mock_server.url).into(),
            ApiUrl::new(&self.version.api.name)
                .with_version(&self.version.version)
                .build(&self.org_subdomain)
                .into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "api".into(),
            "mock server".into(),
            "🔗 link".into(),
        ]
    }
}
