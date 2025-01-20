use sideko_rest_api::models::Api;

pub struct TabledApi(pub Api);
impl tabled::Tabled for TabledApi {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let inner = &self.0;
        vec![
            inner.name.as_str().into(),
            inner.version_count.to_string().into(),
            inner.id.as_str().into(),
            inner.created_at.as_str().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "Name".into(),
            "Versions".into(),
            "ID".into(),
            "Created At".into(),
        ]
    }
}
