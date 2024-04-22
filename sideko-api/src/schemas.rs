pub struct BinaryResponse {
    pub content: bytes::Bytes,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiKey {
    pub api_key: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliUpdate {
    pub message: String,
    pub severity: CliUpdateSeverityEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StatelessGenerateSdk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub language: GenerationLanguageEnum,
    pub openapi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Error {
    pub description: String,
    pub error: ErrorCodeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum CliUpdateSeverityEnum {
    #[default]
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "suggested")]
    Suggested,
    #[serde(rename = "required")]
    Required,
}
impl std::fmt::Display for CliUpdateSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            CliUpdateSeverityEnum::Info => "info",
            CliUpdateSeverityEnum::Suggested => "suggested",
            CliUpdateSeverityEnum::Required => "required",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GenerationLanguageEnum {
    #[default]
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "typescript")]
    Typescript,
}
impl std::fmt::Display for GenerationLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GenerationLanguageEnum::Python => "python",
            GenerationLanguageEnum::Go => "go",
            GenerationLanguageEnum::Rust => "rust",
            GenerationLanguageEnum::Ruby => "ruby",
            GenerationLanguageEnum::Typescript => "typescript",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ErrorCodeEnum {
    #[default]
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "internal_server_error")]
    InternalServerError,
    #[serde(rename = "Bad Request")]
    BadRequest,
    #[serde(rename = "unavailable_subdomain")]
    UnavailableSubdomain,
    #[serde(rename = "invalid_openapi")]
    InvalidOpenapi,
    #[serde(rename = "invalid_url")]
    InvalidUrl,
}
impl std::fmt::Display for ErrorCodeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ErrorCodeEnum::Forbidden => "forbidden",
            ErrorCodeEnum::Unauthorized => "unauthorized",
            ErrorCodeEnum::NotFound => "not_found",
            ErrorCodeEnum::InternalServerError => "internal_server_error",
            ErrorCodeEnum::BadRequest => "Bad Request",
            ErrorCodeEnum::UnavailableSubdomain => "unavailable_subdomain",
            ErrorCodeEnum::InvalidOpenapi => "invalid_openapi",
            ErrorCodeEnum::InvalidUrl => "invalid_url",
        };
        write!(f, "{}", str_val)
    }
}
