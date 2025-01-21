use clap::{builder::PossibleValue, ValueEnum};
use sideko_rest_api::models::SdkLanguageEnum;

use crate::result::CliResult;

mod config;
mod create;
mod init;
mod update;

#[derive(clap::Subcommand)]
pub enum SdkSubcommand {
    // ------------ INTERACTIVE COMMANDS ------------
    /// Interactively configure and create suite of SDKs (recommended command for getting started)
    Init(init::SdkInitCommand),

    // ------------ SUB-COMMANDS ------------
    /// Manage SDK configs
    #[command(subcommand)]
    Config(config::SdkConfigSubcommand),

    // ------------ COMMANDS ------------
    /// Create an SDK using an SDK Config
    Create(create::SdkCreateCommand),

    /// Update SDK to implement changes to APIs
    Update(update::SdkUpdateCommand),
}

impl SdkSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            SdkSubcommand::Config(cmd) => cmd.handle().await,
            SdkSubcommand::Init(cmd) => cmd.handle().await,
            SdkSubcommand::Create(cmd) => cmd.handle().await,
            SdkSubcommand::Update(cmd) => cmd.handle().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SdkLang(SdkLanguageEnum);
impl SdkLang {
    pub fn emoji(&self) -> String {
        match &self.0 {
            SdkLanguageEnum::Go => "🐹".into(),
            SdkLanguageEnum::Java => "☕️".into(),
            SdkLanguageEnum::Python => "🐍".into(),
            SdkLanguageEnum::Rust => "🦀".into(),
            SdkLanguageEnum::Typescript => "🟦".into(),
        }
    }
}

impl ValueEnum for SdkLang {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            SdkLang(SdkLanguageEnum::Python),
            SdkLang(SdkLanguageEnum::Typescript),
            SdkLang(SdkLanguageEnum::Rust),
            SdkLang(SdkLanguageEnum::Go),
            SdkLang(SdkLanguageEnum::Java),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let val = match &self.0 {
            SdkLanguageEnum::Python => PossibleValue::new("python"),
            SdkLanguageEnum::Typescript => PossibleValue::new("typescript"),
            SdkLanguageEnum::Rust => PossibleValue::new("rust"),
            SdkLanguageEnum::Go => PossibleValue::new("go"),
            SdkLanguageEnum::Java => PossibleValue::new("java"),
        };

        Some(val)
    }
}
