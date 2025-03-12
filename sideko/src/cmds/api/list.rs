use tabled::settings::{object::Rows, Color};

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};

use super::tabled::TabledApi;

#[derive(clap::Args)]
pub struct ApiListCommand {
    /// display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    pub display: DisplayOutput,
}
impl ApiListCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let apis = client.api().list().await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&apis),
            DisplayOutput::Pretty => {
                let org = client.org().get().await?;

                let mut table = tabled::Table::new(apis.into_iter().map(|api| TabledApi {
                    api,
                    subdomain: org.subdomain.clone(),
                }));
                utils::tabled::header_panel(&mut table, "apis");
                table.modify(Rows::single(1), Color::BOLD);
                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
