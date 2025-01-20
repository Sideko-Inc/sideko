use sideko_rest_api::{models::ApiVersion, resources::api::spec::GetStatsRequest};
use tabled::settings::{object::Rows, Remove};

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};

#[derive(clap::Args, Debug)]
pub struct ApiStatsCommand {
    /// API name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// API name or id e.g. my-api
    #[arg(long, default_value = "latest")]
    pub version: String,

    /// Display result as a raw json or prettified
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
                    SummaryRow::new("Total Endpoints (paths)", stats.endpoints),
                    SummaryRow::new("Total Methods (operation)", stats.methods),
                    SummaryRow::new("Authenticated Methods", stats.authenticated_methods),
                    SummaryRow::new("Public Methods", stats.public_methods),
                    SummaryRow::new("Authentication Schemes", auth_schemes),
                ];
                let mut table = tabled::Table::new(summary_data);
                table.with(Remove::row(Rows::first()));
                utils::tabled::header_panel(&mut table, "Stats");

                utils::logging::log_table(table);

                // TODO: LINTING
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
