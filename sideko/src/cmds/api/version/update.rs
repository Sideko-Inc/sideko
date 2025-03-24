use camino::Utf8PathBuf;
use sideko_rest_api::{models::ApiVersion, resources::api::spec::PatchRequest, UploadFile};
use tabled::settings::{object::Rows, Color};

use crate::{
    cmds::DisplayOutput,
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

use super::tabled::TabledApiSpec;

#[derive(clap::Args)]
pub struct ApiVersionUpdateCommand {
    /// api name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// version to update (e.g. `2.1.5` or `latest`)
    #[arg(long)]
    pub version: String,

    /// version to update with (e.g. `2.1.5`)
    #[arg(long)]
    pub new_version: Option<String>,

    /// path to openapi spec (yaml or json format) to update with
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_json_yaml,
    )]
    pub spec: Option<Utf8PathBuf>,

    /// Allow linting errors to be present in the provided spec [default: false]
    ///
    /// By default using an OpenAPI that contains linting errors is disallowed.
    /// If you wish to allow linting errors you may experience issues later with SDK generation or mock servers.
    #[arg(long)]
    pub allow_lint_errors: bool,

    /// enable or disable the mock server
    #[arg(long)]
    pub mock: Option<bool>,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl ApiVersionUpdateCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let openapi = if let Some(path) = &self.spec {
            Some(UploadFile::from_path(path.as_str()).map_err(|e| {
                CliError::io_custom(format!("failed reading openapi from path: {path}"), e)
            })?)
        } else {
            None
        };

        let updated_version = client
            .api()
            .spec()
            .patch(PatchRequest {
                api_name: self.name.clone(),
                api_version: ApiVersion::Str(self.version.clone()),
                version: self.new_version.clone(),
                mock_server_enabled: self.mock,
                openapi,
                allow_lint_errors: Some(self.allow_lint_errors),
                ..Default::default()
            })
            .await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&updated_version),
            DisplayOutput::Pretty => {
                let org = client.org().get().await?;
                let mut table = tabled::Table::new([TabledApiSpec {
                    version: updated_version,
                    org_subdomain: org.subdomain.clone(),
                }]);
                utils::tabled::header_panel(&mut table, "updated api version");
                table.modify(Rows::single(1), Color::BOLD);

                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
