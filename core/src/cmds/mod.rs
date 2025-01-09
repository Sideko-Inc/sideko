mod login;
pub(crate) use login::LoginCommand;

mod api;
pub(crate) use api::ApiSubcommand;

#[derive(clap::ValueEnum, serde::Serialize, Default, Debug, Clone)]
enum DisplayOutput {
    #[default]
    Raw,
    Pretty,
}
