use crate::{
    config,
    result::{Error, Result},
    utils::check_for_updates,
};
use prettytable::Table;
use prettytable::{format, row};
use sideko_rest_api::{
    request_types::{
        CreateApiProjectRequest, CreateApiVersionRequest, GetApiProjectRequest,
        ListApiVersionsRequest,
    },
    schemas::{ApiProject, ApiVersion, NewApiProject, NewApiVersion},
    Client as SidekoClient,
};

pub async fn data_get_api_project(name: String) -> Result<ApiProject> {
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    client
        .get_api_project(GetApiProjectRequest {
            project_id_or_name: name,
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed finding API project with the given name. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })
}

pub async fn data_list_versions(name: String) -> Result<Vec<ApiVersion>> {
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    client
        .list_api_versions(ListApiVersionsRequest {
            project_id_or_name: name,
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed finding listing API versions for the API with the provided name. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })
}

pub async fn handle_list_apis(name: &Option<String>) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let api_projects = {
        if let Some(name) = name {
            vec![data_get_api_project(name.clone()).await?]
        } else {
            log::info!("Listing API Projects...");

            client.list_api_projects().await.map_err(|e| {
                Error::api_with_debug(
                    "Failed listing API projects. Re-run the command with -v to debug.",
                    &format!("{e}"),
                )
            })?
        }
    };

    for api_project in api_projects.clone().into_iter() {
        let name = api_project.title;
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        let versions = data_list_versions(name.clone()).await?;
        if versions.is_empty() {
            table.add_row(row!["No versions available"]);
        } else {
            table.add_row(row![b -> "Semver" , b -> "Notes"]);
            for version in &versions {
                table.add_row(row![version.semver, version.notes]);
            }
        }
        println!("{}", name);
        table.printstd();
    }
    Ok(())
}

pub async fn create_new_api_project(params: &NewApiVersion, title: String) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let api_project = client
        .create_api_project(CreateApiProjectRequest {
            data: NewApiProject { title },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating API. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    let first_version = client
        .create_api_version(CreateApiVersionRequest {
            project_id_or_name: api_project.id,
            data: params.clone(),
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating API project version. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    log::info!(
        "Created API Project with title: {} with initial version: {}",
        api_project.title,
        first_version.semver
    );

    Ok(())
}

pub async fn create_new_api_project_version(name: String, params: &NewApiVersion) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;
    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let new_version = client
        .create_api_version(CreateApiVersionRequest {
            project_id_or_name: name,
            data: params.clone(),
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating API project version. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    log::info!(
        "Updated API Project with new version: {} ",
        new_version.semver
    );

    Ok(())
}
