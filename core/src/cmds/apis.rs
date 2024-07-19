use crate::{
    config,
    result::{Error, Result},
    utils::check_for_updates,
};
use prettytable::Table;
use prettytable::{format, row};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use sideko_api::{
    request_types::{CreateApiProjectRequest, CreateApiVersionRequest, ListApiVersionsRequest},
    schemas::{NewApiProject, NewApiVersion},
    Client as SidekoClient,
};

fn extract_title(input: &str) -> String {
    if let Ok(json_value) = serde_json::from_str::<JsonValue>(input) {
        if let Some(title) = json_value.pointer("/info/title").and_then(|v| v.as_str()) {
            return title.to_string();
        }
    }
    if let Ok(yaml_value) = serde_yaml::from_str::<YamlValue>(input) {
        if let Some(title) = yaml_value["info"]["title"].as_str() {
            return title.to_string();
        }
    }
    panic!("Could not find info.title in the supplied openapi")
}

pub async fn handle_list_apis() -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let api_projects = client.list_api_projects().await.map_err(|e| {
        Error::api_with_debug(
            "Failed listing API projects. Re-run the command with -v to debug.",
            &format!("{e}"),
        )
    })?;

    log::info!("Listing API Projects...");
    println!("\n");
    for api_project in api_projects.clone().into_iter() {
        let id = api_project.id;
        let title = api_project.title;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        let versions = client
            .list_api_versions(ListApiVersionsRequest {
                project_id: id.clone(),
            })
            .await
            .map_err(|e| {
                Error::api_with_debug(
                    "Failed listing API project versions. Re-run the command with -v to debug.",
                    &format!("{e}"),
                )
            })?;

        if versions.is_empty() {
            table.add_row(row!["No versions available"]);
        } else {
            table.add_row(row![b -> "Semver" , b -> "Version ID"]);
            for version in &versions {
                table.add_row(row![version.semver, version.id]);
            }
        }

        println!("{}\nID: {}", title, id);
        table.printstd();
        println!("\n");
    }

    Ok(())
}

pub async fn create_new_api_project(params: &NewApiVersion, title: Option<String>) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let title = title.unwrap_or(extract_title(&params.openapi));
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
            project_id: api_project.id,
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

pub async fn create_new_api_project_version(
    project_id: uuid::Uuid,
    params: &NewApiVersion,
) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;
    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let new_version = client
        .create_api_version(CreateApiVersionRequest {
            project_id: project_id.into(),
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
