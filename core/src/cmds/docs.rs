use std::time::Duration;

use crate::{
    config,
    result::{Error, Result},
    utils::check_for_updates,
};
use log::{debug, info, warn};
use prettytable::Table;
use prettytable::{format, row};
use sideko_rest_api::{
    request_types::{
        GetDeploymentRequest, GetDocProjectRequest, ListDocVersionsRequest,
        TriggerDeploymentRequest,
    },
    schemas::{
        Deployment, DeploymentStatusEnum, DeploymentTargetEnum, DocVersionStatusEnum, NewDeployment,
    },
    Client as SidekoClient,
};
use spinners::{Spinner, Spinners};

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

pub async fn handle_deploy_docs(name: &str, prod: bool, no_wait: bool) -> Result<()> {
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
            sideko_rest_api::schemas::DeploymentTargetEnum::Production
        }
        false => {
            info!("Creating preview deployment...");
            sideko_rest_api::schemas::DeploymentTargetEnum::Preview
        }
    };
    let deployment = client
        .trigger_deployment(TriggerDeploymentRequest {
            project_id_or_name: draft_version.doc_project_id.clone(),
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

    if matches!(deployment.status, DeploymentStatusEnum::Error) {
        return Err(Error::general_with_debug(
            "Deployment has failed to trigger",
            &format!(
                "Deployment details: {}",
                serde_json::to_string(&deployment)
                    .unwrap_or("failed displaying deployment object".to_string())
            ),
        ));
    }

    info!("A new documentation deployment has been triggered");
    debug!(
        "Deployment metadata:\n{}",
        serde_json::to_string_pretty(&deployment.metadata).unwrap_or_default()
    );

    if no_wait {
        info!("User specified --no-wait, not polling until completion");
        return Ok(());
    }

    info!("Polling for completion, this may take a few minutes...");

    // poll
    let poll_future = poll_deployment(&deployment);
    match tokio::time::timeout(Duration::from_secs(600), poll_future).await {
        Err(_timeout) => {
            return Err(Error::general(
                "Deployment did not complete within time out (10min)",
            ));
        }
        Ok(poll_result) => {
            let terminal_deployment = poll_result?;
            if terminal_deployment.status.to_string() == DeploymentStatusEnum::Complete.to_string()
            {
                if let Ok(doc_project) = client
                    .get_doc_project(GetDocProjectRequest {
                        project_id_or_name: name.to_string(),
                    })
                    .await
                {
                    let url = match &terminal_deployment.target {
                        DeploymentTargetEnum::Preview => {
                            doc_project.domains.preview.unwrap_or_default()
                        }
                        DeploymentTargetEnum::Production => {
                            doc_project.domains.production.unwrap_or_default()
                        }
                    };

                    info!(
                        "{} deployment complete, available at https://{url}",
                        &terminal_deployment.target
                    );
                } else {
                    info!("{} deployment complete", &terminal_deployment.target)
                }
            }
        }
    }

    Ok(())
}

async fn poll_deployment(deployment: &Deployment) -> Result<Deployment> {
    let api_key = config::get_api_key()?;
    let client = SidekoClient::default()
        .with_base_url(&config::get_base_url())
        .with_api_key_auth(&api_key);
    let mut current_status: DeploymentStatusEnum = deployment.status.clone();

    let mut sp = Spinner::new(Spinners::Dots, "Polling deployment...".into());

    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;

        let d = client
            .get_deployment(GetDeploymentRequest {
                project_id_or_name: deployment.doc_version.doc_project_id.clone(),
                deployment_id: deployment.id.clone(),
            })
            .await
            .map_err(|e| {
                Error::api_with_debug(
                    "Failed retrieving deployment for polling",
                    &format!(
                        "Retrieving deployment with id {} encotered error: {e}",
                        &deployment.id
                    ),
                )
            })?;

        let status_str = d.status.to_string();
        if current_status.to_string() != status_str {
            sp.stop_with_newline();
            debug!("Deployment status updated to {}", &d.status);
            sp = Spinner::new(Spinners::Dots, "Polling deployment...".into());
            current_status = d.status.clone();
        }

        if status_str == DeploymentStatusEnum::Cancelled.to_string() {
            sp.stop_with_newline();
            warn!("Deployment has been cancelled");
            return Ok(d);
        } else if status_str == DeploymentStatusEnum::Error.to_string() {
            sp.stop_with_newline();
            return Err(Error::general_with_debug(
                "Deployment failed",
                &format!(
                    "Deployment details: {}",
                    serde_json::to_string(&deployment)
                        .unwrap_or("failed displaying deployment object".to_string())
                ),
            ));
        } else if status_str == DeploymentStatusEnum::Complete.to_string() {
            sp.stop_with_newline();
            return Ok(d);
        }
    }
}
