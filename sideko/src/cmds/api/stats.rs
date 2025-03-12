use sideko_rest_api::{models::ApiVersion, resources::api::spec::GetStatsRequest};
use tabled::settings::{object::Rows, Remove};

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};

#[derive(clap::Args, Debug)]
pub struct ApiStatsCommand {
    /// api name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// api name or id e.g. my-api
    #[arg(long, default_value = "latest")]
    pub version: String,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl ApiStatsCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let stats = client
            .api()
            .spec()
            .get_stats(GetStatsRequest {
                api_name: self.name.clone(),
                api_version: ApiVersion::Str(self.version.clone()),
            })
            .await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&stats),
            DisplayOutput::Pretty => {
                let auth_schemes = if stats.authentication_schemes.is_empty() {
                    "None".to_string()
                } else {
                    stats.authentication_schemes.join("; ")
                };
                let summary_data = vec![
                    SummaryRow::new("total endpoints (paths)", stats.endpoints),
                    SummaryRow::new("total methods (operations)", stats.methods),
                    SummaryRow::new("authenticated methods", stats.authenticated_methods),
                    SummaryRow::new("public methods", stats.public_methods),
                    SummaryRow::new("authentication schemes", auth_schemes),
                ];
                let mut table = tabled::Table::new(summary_data);
                table.with(Remove::row(Rows::first()));
                utils::tabled::header_panel(&mut table, "stats");

                utils::logging::log_table(table);
            }
        }
        Ok(())
    }
}

#[derive(tabled::Tabled)]
struct SummaryRow {
    name: String,
    val: String,
}
impl SummaryRow {
    pub fn new<N: ToString, V: ToString>(name: N, val: V) -> Self {
        Self {
            name: name.to_string(),
            val: val.to_string(),
        }
    }
}
