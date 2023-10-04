use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use url::Url;

use flate2::read::GzDecoder;
use std::io::Cursor;
use std::path::PathBuf;
use tar::Archive;

mod generate;
mod styles;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ProgrammingLanguage {
    Python,
    Ruby,
    Typescript,
    Rust,
    Go,
}

impl ToString for ProgrammingLanguage {
    fn to_string(&self) -> String {
        match self {
            ProgrammingLanguage::Python => "python".to_string(),
            ProgrammingLanguage::Ruby => "ruby".to_string(),
            ProgrammingLanguage::Typescript => "typescript".to_string(),
            ProgrammingLanguage::Rust => "rust".to_string(),
            ProgrammingLanguage::Go => "go".to_string(),
        }
    }
}

#[derive(Subcommand)]
#[command(styles=styles::get_styles())]
enum Commands {
    Generate {
        openapi_path: PathBuf,
        language: ProgrammingLanguage,
        output_dir: Utf8PathBuf,
        base_url: Option<String>,
        name: Option<String>,
    },
}

fn is_valid_url(val: &str) -> Result<Url, String> {
    url::Url::parse(val).map_err(|e| format!("Invalid URL: {}", e))
}

#[derive(Debug)]
pub enum GenerateError {
    ReqwestError(reqwest::Error),
    FailedResponse(reqwest::StatusCode, String),
    FileError(String),
    DownloadError(String),
    NetworkError(String),
    ArgumentError(String),
}

fn main() -> Result<(), GenerateError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            openapi_path,
            language,
            output_dir,
            base_url,
            name,
        } => {
            println!(
                "Generating Sideko SDK in {}",
                &language.to_string().to_uppercase()
            );

            // Input checks
            if let Some(base_url) = &base_url {
                if is_valid_url(base_url).is_err() {
                    return Err(GenerateError::ArgumentError(format!(
                        "Invalid base url: {base_url}"
                    )));
                };
            }
            if !output_dir.is_dir() {
                return Err(GenerateError::ArgumentError(
                    "Please specify a directory for the output to save to".to_string(),
                ));
            }
            let ext = &openapi_path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or(GenerateError::ArgumentError(
                    "Invalid file extension".to_string(),
                ))?;

            // generate call
            let bytes = generate::generate(openapi_path, ext, language, base_url, name)?;

            // save to output path
            let gz_decoder = GzDecoder::new(Cursor::new(&bytes));
            let mut archive = Archive::new(gz_decoder);
            if let Err(e) = archive.unpack(output_dir) {
                return Err(GenerateError::ArgumentError(format!(
                    "Failed to unpack archive: {}",
                    e
                )));
            }
            println!(
                "Successfully generated SDK. Saving to {}",
                &output_dir.to_string()
            );
            Ok(())
        }
    }
}
