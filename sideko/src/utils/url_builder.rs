pub struct ApiUrl {
    name: String,
    version: Option<String>,
}
impl ApiUrl {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = Some(version.into());
        self
    }
    pub fn build(&self, subdomain: &str) -> String {
        let mut url = format!(
            "https://{subdomain}.sideko.dev/apis/{name}",
            name = &self.name
        );
        if let Some(version) = &self.version {
            url += &format!("/version/{version}");
        }

        url
    }
}

pub struct DocUrl {
    name: String,
    version: Option<i64>,
}
impl DocUrl {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }
    pub fn with_version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    pub fn build(&self, subdomain: &str) -> String {
        let mut url = format!(
            "https://{subdomain}.sideko.dev/docs/{name}",
            name = &self.name
        );
        if let Some(version) = self.version {
            url += &format!("/{version}");
        }

        url
    }
}
