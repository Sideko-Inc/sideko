use crate::{GenerateError, ProgrammingLanguage};
use bytes::Bytes;
use reqwest::blocking::multipart;
use std::path::PathBuf;

#[allow(clippy::ptr_arg)]
pub fn generate(
    openapi_path: &PathBuf,
    ext: &str,
    language: &ProgrammingLanguage,
    base_url: &Option<String>,
    name: &Option<String>,
) -> Result<Bytes, GenerateError> {
    let data = FormData {
        extension: ext.to_string(),
        file_path: openapi_path.display().to_string(),
        language: language.to_string(),
        base_url: base_url.clone(),
        name: name.clone(),
    };
    let form = data.to_multipart()?;
    Ok(generate_request(form))?
}

struct FormData {
    extension: String,
    file_path: String,
    language: String,
    base_url: Option<String>,
    name: Option<String>,
}

impl FormData {
    fn to_multipart(&self) -> Result<multipart::Form, GenerateError> {
        let base_url = &self.base_url.clone().unwrap_or_default();
        let name = &self.name.clone().unwrap_or(String::from("sdk"));
        Ok(multipart::Form::new()
            .text("extension", self.extension.clone())
            .text("language", self.language.clone())
            .file("file", &self.file_path)
            .map_err(|e| GenerateError::FileError(format!("Unable to attach file: {e}")))?
            .text("base_url", base_url.clone())
            .text("name", name.clone()))
    }
}

fn generate_request(form: multipart::Form) -> Result<Bytes, GenerateError> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://api.sideko.dev/v1/sdk/generate/")
        .multipart(form)
        .send()
        .map_err(|e| GenerateError::NetworkError(format!("Failed to make network request: {e}")))?;

    if !response.status().is_success() {
        eprintln!("Failed to make network request");
        return Err(GenerateError::FailedResponse(
            response.status(),
            response.text().unwrap_or_default(),
        ));
    }
    let bytes = response
        .bytes()
        .map_err(|e| GenerateError::DownloadError(format!("Could not download file: {e}")))?;

    Ok(bytes)
}
