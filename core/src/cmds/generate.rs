use crate::{
    config,
    result::{Error, Result},
    utils::{self, check_for_updates},
};
use flate2::read::GzDecoder;
use log::debug;

use serde::{Deserialize, Serialize};
use sideko_api::{
    request_types as sideko_request_types, schemas as sideko_schemas, Client as SidekoClient,
};
use std::{fs, io::Cursor, path::PathBuf};
use tar::Archive;

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

pub struct GenerateSdkParams {
    pub source: OpenApiSource,
    pub destination: PathBuf,
    pub language: sideko_schemas::GenerationLanguageEnum,
    // options
    pub base_url: Option<String>,
    pub package_name: Option<String>,
    pub tests_mock_server_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SidekoApiErr {
    message: String,
    data: Option<serde_json::Value>,
}

pub async fn load_openapi(source: &OpenApiSource) -> Result<String> {
    match source {
        OpenApiSource::Url(url) => {
            debug!("Loading OpenAPI spec from url: {url}");
            let response = reqwest::get(url.to_string()).await.map_err(|e| {
                Error::arg_with_debug(
                    &format!("Failed loading OpenAPI spec from {url}"),
                    &format!("Send err: {e}"),
                )
            })?;
            if !response.status().is_success() {
                return Err(Error::arg_with_debug(
                    &format!(
                        "Failed loading OpenAPI spec from {url}: {}",
                        response.status()
                    ),
                    &format!("response: {:#?}", &response),
                ));
            }

            let openapi = response.text().await.map_err(|e| {
                Error::arg_with_debug(
                    "Could not extract OpenAPI text from response",
                    &format!("error: {:#?}", e),
                )
            })?;
            Ok(openapi)
        }
        OpenApiSource::Path(buf) => {
            let path_str = buf.to_str().unwrap_or_default();
            debug!("Reading OpenAPI from path: {path_str}");
            let openapi = fs::read_to_string(buf.clone()).map_err(|err| Error::Io {
                msg: format!("Failed reading OpenAPI from path {path_str}"),
                err,
            })?;

            Ok(openapi)
        }
        OpenApiSource::Raw(raw) => {
            debug!("OpenAPI provided as raw string");
            Ok(raw.clone())
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
    let openapi = load_openapi(&params.source).await?;

    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let gen_response = client
        .stateless_generate_sdk(sideko_request_types::StatelessGenerateSdkRequest {
            data: sideko_schemas::StatelessGenerateSdk {
                openapi,
                language: params.language.clone(),
                package_name: params.package_name.clone(),
                base_url: params.base_url.clone(),
                tests_mock_server_url: params.tests_mock_server_url.clone(),
            },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed generating SDK. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;

    // unpack into destination
    let dest_str = params.destination.to_str().unwrap_or_default();
    let gz_decoder = GzDecoder::new(Cursor::new(&gen_response.content));
    let mut archive = Archive::new(gz_decoder);
    archive
        .unpack(&params.destination)
        .map_err(|err| Error::Io {
            msg: format!("Failed to unpack archive into {dest_str}"),
            err,
        })?;

    log::info!(
        "Successfully generated SDK in {}, saved to {dest_str}",
        params.language.to_string(),
    );
    Ok(())
}
