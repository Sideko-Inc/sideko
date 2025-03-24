use camino::Utf8PathBuf;
use sideko_rest_api::{models::VersionOrBump, resources::api::InitRequest, UploadFile};
use tabled::settings::{object::Rows, Color};

use crate::{
    cmds::DisplayOutput,
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

use super::{tabled::TabledApi, version::tabled::TabledApiSpec};

#[derive(clap::Args)]
pub struct ApiCreateCommand {
    /// name of api (only alphanumeric characters and dashes, e.g. `my-api`)
    #[arg(long)]
    pub name: String,

    /// semantic version of initial version (e.g. `0.1.0`)
    #[arg(long)]
    pub version: String,

    /// path to openapi spec of initial version (yaml or json format)
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_json_yaml,
    )]
    pub spec: Utf8PathBuf,

    /// Allow linting errors to be present in the provided spec [default: false]
    ///
    /// By default using an OpenAPI that contains linting errors is disallowed.
    /// If you wish to allow linting errors you may experience issues later with SDK generation or mock servers.
    #[arg(long)]
    pub allow_lint_errors: bool,

    /// disable mock server for initial version [default: enabled]
    #[arg(long)]
    pub disable_mock: bool,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl ApiCreateCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let version = client
            .api()
            .init(InitRequest {
                name: self.name.clone(),
                mock_server_enabled: Some(!self.disable_mock),
                openapi: UploadFile::from_path(self.spec.as_str()).map_err(|e| {
                    CliError::io_custom(
                        format!("failed reading openapi from path: {}", &self.spec),
                        e,
                    )
                })?,
                version: VersionOrBump::Str(self.version.clone()),
                allow_lint_errors: Some(self.allow_lint_errors),
                ..Default::default()
            })
            .await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&version),
            DisplayOutput::Pretty => {
                let org = client.org().get().await?;
                let mut api_table = tabled::Table::new([TabledApi {
                    api: version.api.clone(),
                    subdomain: org.subdomain.clone(),
                }]);
                utils::tabled::header_panel(&mut api_table, "api");
                api_table.modify(Rows::single(1), Color::BOLD);
                utils::logging::log_table(api_table);

                let mut version_table = tabled::Table::new([TabledApiSpec {
                    version,
                    org_subdomain: org.subdomain.clone(),
                }]);
                utils::tabled::header_panel(&mut version_table, "initial version");
                version_table.modify(Rows::single(1), Color::BOLD);
                utils::logging::log_table(version_table);
            }
        }

        Ok(())
    }
}
