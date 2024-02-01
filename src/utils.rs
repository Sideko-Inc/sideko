use url::Url;

use crate::CliResult;

pub fn validate_url(val: &str) -> CliResult<Url> {
    url::Url::parse(val).map_err(|e| crate::CliError::ArgumentError(format!("Invalid URL: {}", e)))
}
