mod login;
pub(crate) use login::LoginCommand;

mod logout;
pub(crate) use logout::LogoutCommand;

mod api;
pub(crate) use api::ApiSubcommand;

mod account;
pub(crate) use account::AccountSubcommand;

mod sdk;
pub(crate) use sdk::SdkSubcommand;

mod doc;
pub(crate) use doc::DocSubcommand;

mod config;
pub(crate) use config::ConfigSubcommand;

#[derive(clap::ValueEnum, serde::Serialize, Default, Debug, Clone)]
pub enum DisplayOutput {
    #[default]
    Raw,
    Pretty,
}
