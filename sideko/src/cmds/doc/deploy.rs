use std::time::Duration;

use log::{debug, info};
use serde_json::json;
use sideko_rest_api::{
    models::{Deployment, DeploymentStatusEnum, DeploymentTargetEnum},
    resources::doc::{
        self,
        deployment::{GetRequest, TriggerRequest},
    },
};

use crate::{
    result::{CliError, CliResult},
    styles::fmt_yellow,
    utils::{get_sideko_client, spinner::Spinner},
};

#[derive(clap::Args)]
pub struct DocDeployCommand {
    /// doc project name or id e.g. my-docs
    #[arg(long)]
    pub name: String,

    /// deploy to production [default: preview]
    #[arg(long)]
    pub prod: bool,

    /// exit command after successful trigger [default: waits until deployment completes]
    #[arg(long)]
    pub no_wait: bool,
}
impl DocDeployCommand {
    fn is_terminal_status(&self, status: &DeploymentStatusEnum) -> bool {
        match status {
            DeploymentStatusEnum::Generated
            | DeploymentStatusEnum::Created
            | DeploymentStatusEnum::Building => false,
            DeploymentStatusEnum::Cancelled
            | DeploymentStatusEnum::Complete
            | DeploymentStatusEnum::Error => true,
        }
    }

    async fn poll_deployment(&self, mut deployment: Deployment) -> CliResult<Deployment> {
        let mut client = get_sideko_client();
        let mut status = deployment.status.clone();
        let mut sp = Spinner::new(format!("📖 deployment {}", fmt_yellow(&status.to_string())));

        while !self.is_terminal_status(&status) {
            // poll for new status every 2 secs
            tokio::time::sleep(Duration::from_secs(2)).await;

            // check for update
            deployment = client
                .doc()
                .deployment()
                .get(GetRequest {
                    deployment_id: deployment.id.clone(),
                    doc_name: deployment.doc_version.doc_project_id.clone(),
                })
                .await?;

            // update spinner on status change
            if deployment.status.to_string() != status.to_string() {
                status = deployment.status.clone();
                sp.update_text(format!("status:{}", fmt_yellow(&status.to_string())));
            }
        }

        let deployment_details = serde_json::to_string_pretty(&deployment)
            .unwrap_or_else(|_| json!(deployment).to_string());

        match &deployment.status {
            DeploymentStatusEnum::Complete => {
                sp.stop_success("📖 deployment complete.");
            }
            DeploymentStatusEnum::Cancelled => {
                sp.stop_warn("deployment has been cancelled");
                return Err(CliError::general_debug(
                    format!(
                        "deployment polling terminated in `{}` status",
                        deployment.status
                    ),
                    format!("deployment: {deployment_details}"),
                ));
            }
            DeploymentStatusEnum::Error => {
                sp.stop_error("deployment failed");
                return Err(CliError::general_debug(
                    format!(
                        "deployment polling terminated in `{}` status",
                        deployment.status
                    ),
                    format!("deployment: {deployment_details}"),
                ));
            }
            DeploymentStatusEnum::Created
            | DeploymentStatusEnum::Building
            | DeploymentStatusEnum::Generated => {
                sp.stop_warn("polling terminated in non-terminal status");
                return Err(CliError::general_debug(
                    format!("deployment polling terminated in `{}` status. polling should continue until terminal status", deployment.status),
                    format!("deployment: {deployment_details}"),
                ));
            }
        }

        Ok(deployment)
    }

    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let target = if self.prod {
            DeploymentTargetEnum::Production
        } else {
            DeploymentTargetEnum::Preview
        };

        let doc_project = client
            .doc()
            .get(doc::GetRequest {
                doc_name: self.name.clone(),
            })
            .await?;
        let deployment = client
            .doc()
            .deployment()
            .trigger(TriggerRequest {
                doc_name: self.name.clone(),
                target: target.clone(),
                doc_version_id: None,
            })
            .await?;

        let target = target.to_string().to_lowercase();
        info!("{target} deployment triggered");
        debug!(
            "deployment (id={}) metadata: {}",
            &deployment.id,
            serde_json::to_string_pretty(&deployment.metadata)
                .unwrap_or_else(|_| deployment.metadata.to_string())
        );

        if self.no_wait {
            info!("--no-wait specified, not polling until completion");
            return Ok(());
        }

        let start = chrono::Utc::now();
        let poll_future = self.poll_deployment(deployment);
        match tokio::time::timeout(Duration::from_secs(600), poll_future).await {
            Ok(deployment_res) => {
                debug!(
                    "deployment took {}s",
                    (chrono::Utc::now() - start).num_seconds()
                );

                let deployment = deployment_res?;
                let url = match &deployment.target {
                    DeploymentTargetEnum::Preview => {
                        format!(
                            "https://{}",
                            doc_project.domains.preview.unwrap_or_default()
                        )
                    }
                    DeploymentTargetEnum::Production => {
                        format!(
                            "https://{}",
                            doc_project.domains.production.unwrap_or_default()
                        )
                    }
                };

                info!("site available at: {url}");
                Ok(())
            }
            Err(_) => Err(CliError::general(
                "timeout: deployment did not complete within 10min",
            )),
        }
    }
}
