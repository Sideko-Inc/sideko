use std::fs;

use camino::Utf8PathBuf;
use log::info;
use sideko_rest_api::{
    models::{ApiVersion, OpenApiExtensionEnum},
    resources::api::spec::GetOpenapiRequest,
};

use crate::{result::CliResult, styles::fmt_green, utils::get_sideko_client};

#[derive(clap::Args)]
pub struct ApiVersionDownloadCommand {
    /// api name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// version to update (e.g. `2.1.5` or `latest`)
    #[arg(long)]
    pub version: String,

    /// custom output path of sdk config (must be .yaml or .yml or .json)
    /// the command may alter the extension according to the format of the
    /// downloaded OpenAPI file
    #[arg(
        long,
        value_parser = crate::utils::validators::validate_file_json_yaml_allow_dne,
    )]
    pub output: Option<Utf8PathBuf>,
}
impl ApiVersionDownloadCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let openapi_res = client
            .api()
            .spec()
            .get_openapi(GetOpenapiRequest {
                api_name: self.name.clone(),
                api_version: ApiVersion::Str(self.version.clone()),
            })
            .await?;

        let default_stem = format!("{}-{}", &self.name, &self.version);

        let dest = match &self.output {
            Some(o) => {
                let stem = o.file_stem().unwrap_or(&default_stem);
                let ext = o.extension().unwrap_or_default();
                if matches!(&openapi_res.extension, OpenApiExtensionEnum::Json) && ext != "json" {
                    Utf8PathBuf::new().join(format!("./{stem}.json"))
                } else if matches!(&openapi_res.extension, OpenApiExtensionEnum::Yaml)
                    && ext != "yml"
                    && ext != "yaml"
                {
                    Utf8PathBuf::new().join(format!("./{stem}.yaml"))
                } else {
                    o.clone()
                }
            }
            None => Utf8PathBuf::new().join(format!("./{default_stem}.{}", &openapi_res.extension)),
        };

        fs::write(&dest, openapi_res.openapi)?;

        info!("{} OpenAPI saved to {dest}", fmt_green("✔"));

        Ok(())
    }
}
