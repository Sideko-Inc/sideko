use std::fs;

use camino::Utf8PathBuf;
use clap::{builder::PossibleValue, ValueEnum};
use log::debug;
use sideko_rest_api::models::{SdkLanguageEnum, SdkModuleStructureEnum};

use crate::result::{CliError, CliResult};

mod config;
mod create;
mod init;
mod released;
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

    /// mark an sdk as released
    Released(released::SdkReleasedCommand),
}

impl SdkSubcommand {
    pub async fn handle(&self) -> CliResult<()> {
        match self {
            SdkSubcommand::Config(cmd) => cmd.handle().await,
            SdkSubcommand::Init(cmd) => cmd.handle().await,
            SdkSubcommand::Create(cmd) => cmd.handle().await,
            SdkSubcommand::Update(cmd) => cmd.handle().await,
            SdkSubcommand::Released(cmd) => cmd.handle().await,
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

#[derive(Debug, serde::Deserialize)]
struct SdkMetadata {
    pub id: String,
}

impl SdkMetadata {
    /// loads sdk metadata from the the .sdk.json file in the root of the repo
    fn load_from_repo(repo: &Utf8PathBuf) -> CliResult<Self> {
        let md_path = repo.join(".sdk.json");
        if !(md_path.is_file() && md_path.exists()) {
            return Err(CliError::general_debug(
                format!("could not determine sdk id of this repository. are you sure {repo} is the root of a sideko sdk?"),
                format!("sdk metadata path does not exist in repo: {md_path}"),
            ));
        }

        let md_str = fs::read_to_string(&md_path).map_err(|e| {
            CliError::general_debug(
                format!("could not determine sdk id of this repository. are you sure {repo} is the root of a sideko sdk?"),
                format!("unable to read sdk metadata path to string {md_path}: {e:?}"),
            )
        })?;
        debug!("Found sdk metadata: {md_str}");

        let md: SdkMetadata = serde_json::from_str(&md_str).map_err(|e| {
            CliError::general_debug(
                "could not determine sdk id of this repository. are you sure this a sideko sdk?",
                format!("unable to deserialize sdk metadata path to string {md_path}: {e:?}"),
            )
        })?;
        Ok(md)
    }
}
