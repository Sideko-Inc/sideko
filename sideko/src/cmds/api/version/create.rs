use camino::Utf8PathBuf;
use sideko_rest_api::{models::VersionOrBump, resources::api::spec::CreateRequest, UploadFile};
use tabled::settings::{object::Rows, Color};

use crate::{
    cmds::DisplayOutput,
    result::{CliError, CliResult},
    utils::{self, get_sideko_client},
};

use super::tabled::TabledApiSpec;

#[derive(clap::Args)]
pub struct ApiVersionCreateCommand {
    /// api name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// semantic version (e.g. `2.1.5`) or version bump (`patch`, `minor`, `major`, `rc`)
    #[arg(long)]
    pub version: String,

    /// path to openapi specification (YAML or JSON format)
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_json_yaml,
    )]
    pub spec: Utf8PathBuf,

    /// Allow linting errors to be present in the provided spec [default: false]
    ///
    /// By default creating a new version with an OpenAPI that contains linting errors is disallowed.
    /// If you wish to allow linting errors you may experience issues later with SDK generation or mock servers.
    #[arg(long)]
    pub allow_lint_errors: bool,

    /// disable mock server for new version [default: enabled]
    #[arg(long)]
    pub disable_mock: bool,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl ApiVersionCreateCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let new_version = client
            .api()
            .spec()
            .create(CreateRequest {
                api_name: self.name.clone(),
                version: VersionOrBump::Str(self.version.clone()),
                mock_server_enabled: Some(!self.disable_mock),
                openapi: UploadFile::from_path(self.spec.as_str()).map_err(|e| {
                    CliError::io_custom(
                        format!("failed reading openapi from path: {}", &self.spec),
                        e,
                    )
                })?,
                notes: None,
                allow_lint_errors: Some(self.allow_lint_errors),
            })
            .await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&new_version),
            DisplayOutput::Pretty => {
                let org = client.org().get().await?;
                let mut table = tabled::Table::new([TabledApiSpec {
                    version: new_version,
                    org_subdomain: org.subdomain.clone(),
                }]);
                utils::tabled::header_panel(&mut table, "new api version");
                table.modify(Rows::single(1), Color::BOLD);

                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
