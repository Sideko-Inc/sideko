use crate::{
    config,
    result::{Error, Result},
    utils::check_for_updates,
};
use prettytable::Table;
use prettytable::{format, row};
use sideko_rest_api::{
    models::{Api, ApiSpec, NewApi, NewApiSpec},
    resources::api::{
        spec::{CreateRequest as SpecCreate, ListRequest},
        CreateRequest, GetRequest,
    },
    Client as SidekoClient,
};

pub async fn data_get_api_project(id: String) -> Result<Api> {
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    client
        .api()
        .get(GetRequest { api_name: id })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed finding API with the given id. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })
}

pub async fn data_list_versions(name: String) -> Result<Vec<ApiSpec>> {
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    client
        .api().spec().list(ListRequest {
            api_name: name,
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

            client.api().list().await.map_err(|e| {
                Error::api_with_debug(
                    "Failed listing API projects. Re-run the command with -v to debug.",
                    &format!("{e}"),
                )
            })?
        }
    };

    for api_project in api_projects.clone().into_iter() {
        let name = api_project.name;
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        let versions = data_list_versions(name.clone()).await?;
        if versions.is_empty() {
            table.add_row(row!["No versions available"]);
        } else {
            table.add_row(row![b -> "Semver" , b -> "Notes"]);
            for version in &versions {
                table.add_row(row![version.version, version.notes]);
            }
        }
        println!("{}", name);
        table.printstd();
    }
    Ok(())
}

pub async fn create_new_api_project(params: &NewApiSpec, title: String) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let api_project = client
        .api()
        .create(CreateRequest {
            data: NewApi { name: title },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating API. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    let first_version = client
        .api()
        .spec()
        .create(SpecCreate {
            api_name: api_project.name.clone(),
            data: params.clone(),
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating API spec. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    log::info!(
        "Created API Project with id: {} with initial semantic version: {}",
        api_project.id,
        first_version.version
    );

    Ok(())
}

pub async fn create_new_api_project_version(name: String, params: &NewApiSpec) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;
    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let new_version = client
        .api()
        .spec()
        .create(SpecCreate {
            api_name: name,
            data: params.clone(),
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed creating API spec. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;
    log::info!(
        "Updated API with new specification as version: {} ",
        new_version.version
    );

    Ok(())
}
