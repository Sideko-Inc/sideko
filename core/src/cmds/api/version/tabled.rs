use sideko_rest_api::models::ApiSpec;

pub struct TabledApiSpec(pub ApiSpec);
impl tabled::Tabled for TabledApiSpec {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let inner = &self.0;
        let mock_enabled = if inner.mock_server.enabled {
            "🟢"
        } else {
            "🔴"
        };
        vec![
            inner.version.as_str().into(),
            format!("{mock_enabled} {url}", url = &inner.mock_server.url).into(),
            inner.id.as_str().into(),
            inner.created_at.as_str().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "Version".into(),
            "Mock Server".into(),
            "ID".into(),
            "Created At".into(),
        ]
    }
}
