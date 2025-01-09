use sideko_rest_api::models::Api;
use tabled::settings::{object::Rows, Color, Panel};

use crate::{
    cmds::DisplayOutput,
    result::CliResult,
    utils::{self, get_sideko_client},
};

#[derive(clap::Args)]
pub struct ApiListCommand {
    /// Display result as a raw json or prettified
    #[arg(long, default_value = "pretty")]
    display: DisplayOutput,
}
impl ApiListCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();
        let apis = client.api().list().await?;

        match &self.display {
            DisplayOutput::Raw => utils::logging::log_json_raw(&apis),
            DisplayOutput::Pretty => {
                let mut table = tabled::Table::new(apis.into_iter().map(TabledApi));
                utils::tabled::header_panel(&mut table, "APIs");
                table
                    .with(Panel::header("APIs"))
                    .modify(Rows::single(1), Color::BOLD);

                utils::logging::log_table(table);
            }
        }

        Ok(())
    }
}
struct TabledApi(Api);
impl tabled::Tabled for TabledApi {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        let inner = &self.0;
        vec![
            inner.name.as_str().into(),
            inner.version_count.to_string().into(),
            inner.id.as_str().into(),
            inner.created_at.as_str().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "Name".into(),
            "Versions".into(),
            "ID".into(),
            "Created At".into(),
        ]
    }
}
