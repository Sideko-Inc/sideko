use crate::{
    config,
    result::{Error, Result},
    utils::check_for_updates,
};
use log::info;
use prettytable::Table;
use prettytable::{format, row};
use sideko_api::{
    request_types::{ListDocVersionsRequest, TriggerDeploymentRequest},
    schemas::{DocVersionStatusEnum, NewDeployment},
    Client as SidekoClient,
};

pub async fn handle_list_docs() -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let doc_projects = client.list_doc_projects().await.map_err(|e| {
        Error::api_with_debug(
            "Failed listing Doc projects. Re-run the command with -v to debug.",
            &format!("{e}"),
        )
    })?;

    log::info!("Listing Doc Projects...");
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    if doc_projects.is_empty() {
        table.add_row(row!["No doc projects"]);
    } else {
        table.add_row(row![b -> "Name", b -> "Preview URL", b -> "Production URL",]);
        for doc_project in &doc_projects {
            table.add_row(row![
                doc_project.title,
                doc_project.domains.preview.clone().unwrap_or_default(),
                doc_project
                    .domains
                    .production
                    .clone()
                    .unwrap_or_default()
                    .clone()
            ]);
        }
    }
    table.printstd();

    Ok(())
}

pub async fn handle_deploy_docs(name: &str, prod: &bool) -> Result<()> {
    // check for updates after all other validation passed
    check_for_updates().await?;

    // make request
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);

    // get the (one) draft version for the project
    let doc_versions = client.list_doc_versions(ListDocVersionsRequest {
        project_id_or_name: name.to_string()
    }).await.map_err(|e| {
        Error::api_with_debug(
            "Could not find doc versions for the given project id. Re-run the command with -v to debug.",
            &format!("{e}"),
        )
    })?;
    let draft_version = doc_versions
        .iter()
        .find(|d| match d.status {
            DocVersionStatusEnum::Draft => true,
            DocVersionStatusEnum::Publishing => false,
            DocVersionStatusEnum::Published => false,
        })
        .expect("A draft version always exists");
    let target = match prod {
        true => {
            info!("Creating product deployment...");
            sideko_api::schemas::DeploymentTargetEnum::Production
        }
        false => {
            info!("Creating preview deployment...");
            sideko_api::schemas::DeploymentTargetEnum::Preview
        }
    };
    client
        .trigger_deployment(TriggerDeploymentRequest {
            project_id_or_name: name.to_string(),
            data: NewDeployment {
                doc_version_id: draft_version.id.clone(),
                target,
            },
        })
        .await
        .map_err(|e| {
            Error::api_with_debug(
                "Failed triggering deployment. Re-run the command with -v to debug.",
                &format!("{e}"),
            )
        })?;

    info!("A new documentation deployment has been triggered",);

    Ok(())
}
