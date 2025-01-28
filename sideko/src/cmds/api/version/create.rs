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
    /// API name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// Semantic version (e.g. `2.1.5`) or version bump (`patch`, `minor`, `major`, `rc`)
    #[arg(long)]
    pub version: String,

    /// Path to OpenAPI spec (YAML or JSON format)
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_json_yaml,
    )]
    pub spec: Utf8PathBuf,

    /// Disable mock server for new version [default: enabled]
    #[arg(long)]
    pub disable_mock: bool,

    /// Display result as a raw json or prettified
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
                        format!("Failed reading OpenAPI from path: {}", &self.spec),
                        e,
                    )
                })?,
                notes: None,
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
                utils::tabled::header_panel(&mut table, "New API Version");
                table.modify(Rows::single(1), Color::BOLD);

                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
