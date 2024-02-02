use crate::{CliError, CliResult};
use bytes::Bytes;
use camino::Utf8PathBuf;
use reqwest::blocking::multipart;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ProgrammingLanguage {
    Python,
    Ruby,
    Typescript,
    Rust,
    Go,
}

impl ToString for ProgrammingLanguage {
    fn to_string(&self) -> String {
        match self {
            ProgrammingLanguage::Python => "python".to_string(),
            ProgrammingLanguage::Ruby => "ruby".to_string(),
            ProgrammingLanguage::Typescript => "typescript".to_string(),
            ProgrammingLanguage::Rust => "rust".to_string(),
            ProgrammingLanguage::Go => "go".to_string(),
        }
    }
}

struct GenerateFormData {
    extension: String,
    file_path: String,
    language: String,
    base_url: Option<String>,
    name: Option<String>,
}

impl GenerateFormData {
    fn to_multipart(&self) -> CliResult<multipart::Form> {
        let base_url = &self.base_url.clone().unwrap_or_default();
        let name = &self.name.clone().unwrap_or(String::from("sdk"));
        Ok(multipart::Form::new()
            .text("extension", self.extension.clone())
            .text("language", self.language.clone())
            .file("file", &self.file_path)
            .map_err(|e| CliError::FileError(format!("Unable to attach file: {e}")))?
            .text("base_url", base_url.clone())
            .text("name", name.clone()))
    }
}

pub fn handle_generate(
    openapi_path: &Utf8PathBuf,
    ext: &str,
    language: &ProgrammingLanguage,
    base_url: &Option<String>,
    name: &Option<String>,
) -> CliResult<Bytes> {
    let data = GenerateFormData {
        extension: ext.to_string(),
        file_path: openapi_path.to_string(),
        language: language.to_string(),
        base_url: base_url.clone(),
        name: name.clone(),
    };
    let form = data.to_multipart()?;

    Ok(generate_request(form))?
}

fn generate_request(form: multipart::Form) -> CliResult<Bytes> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://api.sideko.dev/v1/sdk/generate/")
        .multipart(form)
        .send()
        .map_err(|e| CliError::NetworkError(format!("Failed to make network request: {e}")))?;

    if !response.status().is_success() {
        eprintln!("Failed to make network request");
        return Err(CliError::FailedResponse(
            response.status(),
            response.text().unwrap_or_default(),
        ));
    }
    let bytes = response
        .bytes()
        .map_err(|e| CliError::DownloadError(format!("Could not download file: {e}")))?;

    Ok(bytes)
}
