use clap::{builder::PossibleValue, ValueEnum};
use sideko_rest_api::models::{SdkLanguageEnum, SdkModuleStructureEnum};

use crate::result::CliResult;

mod config;
mod create;
mod init;
mod update;

#[derive(clap::Subcommand)]
pub enum SdkSubcommand {
    // ------------ INTERACTIVE COMMANDS ------------
    /// interactively configure and create suite of sdks (recommended command for getting started)
    Init(init::SdkInitCommand),

    // ------------ SUB-COMMANDS ------------
    /// manage sdk configs
    #[command(subcommand)]
    Config(config::SdkConfigSubcommand),

    // ------------ COMMANDS ------------
    /// create an sdk using an existing sdk config
    Create(create::SdkCreateCommand),

    /// update sdk to implement changes to apis
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
            SdkLanguageEnum::Csharp => "💠".into(),
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
            SdkLang(SdkLanguageEnum::Csharp),
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
            SdkLanguageEnum::Csharp => PossibleValue::new("csharp"),
        };

        Some(val)
    }
}

#[derive(Debug, Clone)]
pub struct SdkModuleStructure(SdkModuleStructureEnum);
impl ValueEnum for SdkModuleStructure {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            SdkModuleStructure(SdkModuleStructureEnum::Path),
            SdkModuleStructure(SdkModuleStructureEnum::Flat),
            SdkModuleStructure(SdkModuleStructureEnum::Tag),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let val = match &self.0 {
            SdkModuleStructureEnum::Path => PossibleValue::new("path"),
            SdkModuleStructureEnum::Flat => PossibleValue::new("flat"),
            SdkModuleStructureEnum::Tag => PossibleValue::new("tag"),
        };

        Some(val)
    }
}
