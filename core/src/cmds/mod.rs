mod login;
pub(crate) use login::LoginCommand;

mod api;
pub(crate) use api::ApiSubcommand;

mod sdk;
pub(crate) use sdk::SdkSubcommand;

mod doc;
pub(crate) use doc::DocSubcommand;

#[derive(clap::ValueEnum, serde::Serialize, Default, Debug, Clone)]
pub enum DisplayOutput {
    #[default]
    Raw,
    Pretty,
}
