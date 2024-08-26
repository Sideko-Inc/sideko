use crate::{
    config,
    result::{Error, Result},
    utils::{self, check_for_updates},
};
use flate2::{write::GzEncoder, Compression};
use tempfile::TempDir;

use flate2::read::GzDecoder;
use log::debug;

use prettytable::{format, row, Table};
use serde::{Deserialize, Serialize};
use sideko_rest_api::{
    request_types::{self as sideko_request_types, ListSdksRequest, UpdateSdkRequest},
    schemas::{self as sideko_schemas, File as SidekoFile, GenerationLanguageEnum},
    Client as SidekoClient,
};
use std::{
    fs::{self, File},
    io::Cursor,
    path::{Path, PathBuf},
    process::Command,
};
use tar::{Archive, Builder};
use walkdir::WalkDir;

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
    pub language: GenerationLanguageEnum,
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

pub async fn handle_try(params: &GenerateSdkParams) -> Result<()> {
    log::info!(
        "Generating unmanaged Sideko SDK in {}",
        &params.language.to_string()
    );

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

pub async fn handle_create(
    language: &GenerationLanguageEnum,
    api_project_version_id: &str,
    repo_name: &str,
    semver: &str,
    destination: &PathBuf,
) -> Result<()> {
    check_for_updates().await?;
    let dest_str = destination
        .to_str()
        .expect("could not create destination path");

    log::info!(
        "Creating the initial version of a Sideko Managed SDK in {}",
        &language.to_string().to_uppercase()
    );
    log::info!("The SDK will be saved here: {}", &dest_str);

    utils::validate_path(destination.clone(), &utils::PathKind::Dir, true)?;
    // check for updates after all other validation passed

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let gen_response = client
        .create_sdk(sideko_request_types::CreateSdkRequest {
            data: sideko_schemas::SdkProject {
                language: language.clone(),
                api_project_version_id: api_project_version_id.into(),
                name: repo_name.into(),
                semver: semver.into(),
            },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating SDK. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    // unpack into destination
    let gz_decoder = GzDecoder::new(Cursor::new(&gen_response.content));
    let mut archive = Archive::new(gz_decoder);
    archive
        .unpack(destination)
        .map_err(|err| Error::Io {
            msg: format!("Failed to unpack archive into {dest_str}"),
            err,
        })
        .expect("could not unpack archive");

    log::info!("Successfully generated SDK. Saved to {dest_str}",);
    Ok(())
}

pub async fn handle_list_sdks(name: &str) -> Result<()> {
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    let sdks = client
        .list_sdks(ListSdksRequest {
            api_id_or_name: name.to_string(),
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed listing SDKs. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    log::info!("Listing SDKs for the {} API Project...", name);

    if sdks.is_empty() {
        table.add_row(row!["No sdks available"]);
    } else {
        table.add_row(row![b -> "Name" , b -> "Language", b -> "Semver"]);
        for sdk in sdks {
            table.add_row(row![sdk.name, sdk.language, sdk.semver]);
        }
    }
    table.printstd();
    Ok(())
}

pub async fn handle_update(
    repo_path: &Path,
    name: &str,
    semver: &str,
    api_project_semver: Option<String>,
) -> Result<()> {
    log::info!("Creating patch for the new version of {}", name);

    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);

    // Create a temporary directory for the .git contents
    let temp_dir = TempDir::new().map_err(|e| Error::Io {
        msg: "Failed to create temporary directory".into(),
        err: e,
    })?;

    // Copy .git directory to temporary directory
    let git_path = repo_path.join(".git");
    copy_dir_all(&git_path, temp_dir.path())?;

    // Create tar.gz file
    let patch_dir = TempDir::new().map_err(|e| Error::Io {
        msg: "Failed to create patch directory".into(),
        err: e,
    })?;
    let tar_gz_path = patch_dir.path().join("git_patch.tar.gz");
    let tar_gz = File::create(&tar_gz_path).map_err(|e| Error::Io {
        msg: format!("Failed to create tar.gz file at {:?}", tar_gz_path),
        err: e,
    })?;

    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    for entry in WalkDir::new(temp_dir.path()) {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.strip_prefix(temp_dir.path()).unwrap();

        if path.is_file() {
            let mut file = File::open(path).unwrap();
            tar.append_file(name, &mut file).unwrap();
        } else if path.is_dir() && !name.as_os_str().is_empty() {
            tar.append_dir(name, path).unwrap();
        }
    }

    tar.finish().unwrap();
    let enc = tar.into_inner().unwrap(); // Finalize the gzip stream
    enc.finish().unwrap();
    let git_patch_tar_path = tar_gz_path.to_string_lossy().into_owned();

    // Send the request
    let patch_response = client
        .update_sdk(UpdateSdkRequest {
            name: name.into(),
            semver: semver.into(),
            api_version_id_or_semver: api_project_semver.clone(),
            data: SidekoFile {
                file: git_patch_tar_path,
            },
        })
        .await
        .unwrap();

    let patch_content = patch_response.patch;
    if patch_content.is_empty() {
        log::warn!("No updates to apply");
        return Ok(());
    }
    let file_path = repo_path.join("update.patch");
    fs::write(&file_path, patch_content.as_bytes()).expect("could not write file");
    let output = Command::new("git")
        .current_dir(repo_path)
        .arg("apply")
        .arg("update.patch")
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        log::info!("Patch applied successfully with git");
        fs::remove_file(&file_path).expect("failed to delete patch file");
    } else {
        return Err(Error::general(
            "Failed to apply git patch. The patch has been saved.",
        ));
    }

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst).map_err(|e| Error::Io {
        msg: format!("Failed to create directory: {:?}", dst.as_ref()),
        err: e,
    })?;

    for entry in fs::read_dir(src.as_ref()).map_err(|e| Error::Io {
        msg: format!("Failed to read directory: {:?}", src.as_ref()),
        err: e,
    })? {
        let entry = entry.unwrap();
        let ty = entry.file_type().map_err(|e| Error::Io {
            msg: "Failed to get file type".into(),
            err: e,
        })?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name())).map_err(|e| {
                Error::Io {
                    msg: format!("Failed to copy file: {:?}", entry.path()),
                    err: e,
                }
            })?;
        }
    }
    Ok(())
}
