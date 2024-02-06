use crate::{
    config,
    result::{Error, Result},
    utils,
};
use flate2::read::GzDecoder;
use log::debug;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{fs, io::Cursor, path::PathBuf, str::FromStr};
use tar::Archive;

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

pub enum OpenApiSource {
    Url(url::Url),
    Path(PathBuf),
    Raw(String),
}

impl From<&String> for OpenApiSource {
    fn from(value: &String) -> Self {
        if let Ok(u) = url::Url::parse(value) {
            return OpenApiSource::Url(u);
        }

        let buf = PathBuf::from(value);
        if utils::validate_path(buf.clone(), &utils::PathKind::File, false).is_ok() {
            return OpenApiSource::Path(buf);
        }

        OpenApiSource::Raw(value.clone())
    }
}

pub enum OpenAPIExtension {
    Json,
    Yaml,
}

impl FromStr for OpenAPIExtension {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let lowered = s.to_lowercase();
        match lowered.as_str() {
            "json" => Ok(Self::Json),
            "yml" | "yaml" => Ok(Self::Yaml),
            _ => Err(Error::General(format!(
                "Invalid extension {lowered}, expecting: json || yaml || yml"
            ))),
        }
    }
}

impl ToString for OpenAPIExtension {
    fn to_string(&self) -> String {
        match self {
            OpenAPIExtension::Json => "json".into(),
            OpenAPIExtension::Yaml => "yaml".into(),
        }
    }
}

impl OpenAPIExtension {
    pub fn from_content(val: &str) -> Result<Self> {
        if serde_json::from_str::<serde_json::Value>(val).is_ok() {
            Ok(Self::Json)
        } else if serde_yaml::from_str::<serde_json::Value>(val).is_ok() {
            Ok(Self::Yaml)
        } else {
            Err(Error::General(
                "Provided OpenAPI is neither json nor yaml object".to_string(),
            ))
        }
    }
}

pub struct GenerateSdkParams {
    pub source: OpenApiSource,
    pub destination: PathBuf,
    pub language: ProgrammingLanguage,
    // options
    pub base_url: Option<String>,
    pub package_name: Option<String>,
}

struct GenerateForm {
    extension: OpenAPIExtension,
    openapi_bytes: Vec<u8>,
    language: String,
    base_url: Option<String>,
    package_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SidekoApiErr {
    message: String,
    data: Option<serde_json::Value>,
}

impl GenerateForm {
    fn to_multipart(&self) -> Result<Form> {
        let mut form = Form::new()
            .text("extension", self.extension.to_string())
            .text("language", self.language.clone());
        let file_part = Part::stream(self.openapi_bytes.clone())
            .file_name(format!("openapi.{}", &self.extension.to_string()))
            .mime_str(
                mime_guess::from_ext(&self.extension.to_string())
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

async fn load_and_validate_openapi(source: &OpenApiSource) -> Result<(Vec<u8>, OpenAPIExtension)> {
    match source {
        OpenApiSource::Url(url) => {
            debug!("Loading OpenAPI spec from url: {url}");
            let response = reqwest::get(url.to_string()).await.map_err(|e| {
                Error::ReqwestError(format!("Failed loading OpenAPI spec from {url}"), e)
            })?;
            if !response.status().is_success() {
                return Err(Error::ResponseError(
                    format!(
                        "Failed loading OpenAPI spec from {url}: {}",
                        response.status()
                    ),
                    format!("{:?}", &response),
                ));
            }

            let openapi = response.text().await.map_err(|e| {
                Error::ReqwestError(
                    "Could not extract OpenAPI text from response".to_string(),
                    e,
                )
            })?;
            let ext = OpenAPIExtension::from_content(&openapi)?;
            Ok((openapi.into_bytes(), ext))
        }
        OpenApiSource::Path(buf) => {
            let path_str = buf.to_str().unwrap_or_default();
            debug!("Reading OpenAPI from path: {path_str}");
            let openapi = fs::read_to_string(buf.clone()).map_err(|e| {
                Error::IoError(format!("Failed reading OpenAPI from path {path_str}"), e)
            })?;
            let ext_err = format!("Unable to read file extension from path {path_str}");
            let ext = buf
                .extension()
                .ok_or(Error::ArgumentError(ext_err.clone()))?
                .to_str()
                .ok_or(Error::ArgumentError(ext_err.clone()))?;
            let openapi_ext = OpenAPIExtension::from_str(ext)?;

            Ok((openapi.into_bytes(), openapi_ext))
        }
        OpenApiSource::Raw(raw) => {
            debug!("OpenAPI provided as raw string");
            let ext = OpenAPIExtension::from_content(raw)?;
            Ok((raw.clone().into_bytes(), ext))
        }
    }
}

pub async fn handle_generate(params: &GenerateSdkParams) -> Result<()> {
    log::info!("Generating Sideko SDK in {}", &params.language.to_string());

    // validate input
    if let Some(base) = &params.base_url {
        utils::validate_url(base)?;
    }
    utils::validate_path(params.destination.clone(), &utils::PathKind::Dir, true)?;

    let (openapi_bytes, extension) = load_and_validate_openapi(&params.source).await?;
    let api_key = config::get_api_key()?;

    // make request
    let client = Client::new();

    let url = format!("{}/v1/sdk/generate/", config::get_base_url());
    let form = GenerateForm {
        extension,
        openapi_bytes,
        language: params.language.to_string(),
        base_url: params.base_url.clone(),
        package_name: params.package_name.clone(),
    }
    .to_multipart()?;
    let response = client
        .post(url)
        .multipart(form)
        .header("x-api-key", &api_key)
        .send()
        .await
        .map_err(|e| Error::ReqwestError("Failed to make network request".to_string(), e))?;

    let status = response.status();
    if !status.is_success() {
        let debug_res = format!("{:?}", &response);
        let mut server_msg = String::default();
        if let Ok(err) =
            serde_json::from_str::<SidekoApiErr>(&response.text().await.unwrap_or_default())
        {
            server_msg = format!(" - {}", err.message)
        }
        return Err(Error::ResponseError(
            format!("Generate SDK request failed: {status}{server_msg}"),
            debug_res,
        ));
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|e| Error::ReqwestError("Could not extract file from response".to_string(), e))?;

    // unpack into destination
    let dest_str = params.destination.to_str().unwrap_or_default();
    let gz_decoder = GzDecoder::new(Cursor::new(&bytes));
    let mut archive = Archive::new(gz_decoder);
    archive
        .unpack(&params.destination)
        .map_err(|e| Error::IoError(format!("Failed to unpack archive into {dest_str}"), e))?;

    log::info!(
        "Successfully generated SDK in {}, saved to {dest_str}",
        params.language.to_string(),
    );
    Ok(())
}
