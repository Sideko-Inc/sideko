use crate::{utils, CliError, CliResult};
use bytes::Bytes;
use camino::Utf8PathBuf;
use reqwest::{
    multipart::{Form, Part},
    Client,
};

#[derive(clap::ValueEnum, Clone, Debug)]
pub(crate) enum ProgrammingLanguage {
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

struct GenerateData {
    api_key: String,
    extension: String,
    file_path: Utf8PathBuf,
    language: String,
    base_url: Option<String>,
    package_name: Option<String>,
}

impl GenerateData {
    fn to_multipart(&self) -> CliResult<Form> {
        let mut form = Form::new()
            .text("extension", self.extension.clone())
            .text("language", self.language.clone());
        let file_bytes = std::fs::read(&self.file_path)
            .map_err(|e| CliError::FileError(format!("Unable to read file: {e}")))?;
        let file_part = Part::stream(file_bytes)
            .file_name(format!("openapi.{}", &self.extension))
            .mime_str(
                mime_guess::from_ext(&self.extension)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .unwrap();
        form = form.part("file", file_part);
        if let Some(b) = &self.base_url {
            form = form.text("base_url", b.clone());
        }
        if let Some(name) = &self.package_name {
            form = form.text("package_name", name.clone());
        }

        Ok(form)
    }
}

pub(crate) async fn handle_generate(
    openapi_path: &Utf8PathBuf,
    ext: &str,
    language: &ProgrammingLanguage,
    base_url: &Option<String>,
    package_name: &Option<String>,
) -> CliResult<Bytes> {
    let api_key = utils::get_api_key()?;
    let data = GenerateData {
        api_key,
        extension: ext.to_string(),
        file_path: openapi_path.clone(),
        language: language.to_string(),
        base_url: base_url.clone(),
        package_name: package_name.clone(),
    };

    Ok(generate_request(data).await)?
}

async fn generate_request(data: GenerateData) -> CliResult<Bytes> {
    let form = data.to_multipart()?;

    let client = Client::new();
    let url = format!("{}/v1/sdk/generate/", utils::sideko_base_url());

    let response = client
        .post(url)
        .multipart(form)
        .header("x-api-key", &data.api_key)
        .send()
        .await
        .map_err(|e| CliError::NetworkError(format!("Failed to make network request: {e}")))?;

    if !response.status().is_success() {
        eprintln!("Failed to make network request");
        return Err(CliError::FailedResponse(
            response.status(),
            response.text().await.unwrap_or_default(),
        ));
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|e| CliError::DownloadError(format!("Could not download file: {e}")))?;

    Ok(bytes)
}
