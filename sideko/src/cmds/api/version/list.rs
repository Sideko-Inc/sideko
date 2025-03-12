use sideko_rest_api::resources::api::spec::ListRequest;
use tabled::settings::{object::Rows, Color};

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};

use super::tabled::TabledApiSpec;

#[derive(clap::Args)]
pub struct ApiVersionListCommand {
    /// api name or id e.g. my-api
    #[arg(long)]
    pub name: String,

    /// limit results to most recent N versions
    #[arg(long)]
    pub limit: Option<usize>,

    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl ApiVersionListCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let mut versions = client
            .api()
            .spec()
            .list(ListRequest {
                api_name: self.name.clone(),
            })
            .await?;

        if let Some(limit) = self.limit {
            versions = versions[0..versions.len().min(limit)].to_vec();
        }

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&versions),
            DisplayOutput::Pretty => {
                let org = client.org().get().await?;
                let iter = versions.into_iter().map(|version| TabledApiSpec {
                    version,
                    org_subdomain: org.subdomain.clone(),
                });
                let mut table = tabled::Table::new(iter);
                utils::tabled::header_panel(&mut table, "api versions");
                table.modify(Rows::single(1), Color::BOLD);

                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
