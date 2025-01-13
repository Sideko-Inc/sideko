use std::io::ErrorKind;

use camino::Utf8PathBuf;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use inquire::validator::StringValidator;
use log::{debug, info};
use regex::Regex;
use serde_json::json;
use sideko_rest_api::{
    models::{Api, ApiSpec, OrganizationFeatures, SdkLanguageEnum, VersionOrBump},
    resources::api::{self, spec},
    UploadFile,
};

use crate::{
    cmds::sdk::{config::init::SdkConfigInitCommand, create::SdkCreateCommand, SdkLang},
    result::{CliError, CliResult},
    styles::fmt_green,
    utils::{self, get_sideko_client, validators::PathKind},
};

#[derive(clap::Args)]
pub struct SdkInitCommand;

impl SdkInitCommand {
    async fn prompt_create_api(&self) -> CliResult<Api> {
        let name = inquire::Text::new("API Name:")
            .with_help_message(
                "API name must only include lower-case alphanumeric characters and dashes",
            )
            .with_placeholder("my-api")
            .with_validator(ApiNameValidator)
            .prompt()?;

        let mut client = get_sideko_client();
        let new_api = client.api().create(api::CreateRequest { name }).await?;
        info!("{} API created", fmt_green("✓"));
        debug!("New API with id: {}", &new_api.id);

        Ok(new_api)
    }

    async fn prompt_create_version(&self, api: &Api) -> CliResult<ApiSpec> {
        let oas_path = inquire::Text::new("OpenAPI:")
            .with_help_message("Enter path to OpenAPI (≥3.0) specification for the new version")
            .with_placeholder("path/to/spec.yml")
            .with_validator(PathValidator::file().with_extensions(&[".json", ".yaml", ".yml"]))
            .with_autocomplete(FilePathCompleter::default())
            .prompt()?;
        let version = inquire::Text::new("Version:")
            .with_help_message(
                "Enter the version of this API following the semantic versioning format",
            )
            .with_placeholder("0.1.0")
            .with_validator(SemverValidator)
            .prompt()?;

        let mut client = get_sideko_client();
        let new_version = client
            .api()
            .spec()
            .create(spec::CreateRequest {
                api_name: api.name.clone(),
                openapi: UploadFile::from_path(&oas_path).map_err(|e| {
                    CliError::io_custom(format!("Failed reading OpenAPI from path: {oas_path}"), e)
                })?,
                version: VersionOrBump::Str(version),
                mock_server_enabled: Some(true),
                ..Default::default()
            })
            .await?;
        info!("{} Version created", fmt_green("✓"));
        debug!(
            "New API version in `{}` with id: {}",
            &api.name, &new_version.id
        );

        Ok(new_version)
    }

    async fn select_api(&self, options: &[Api]) -> CliResult<Api> {
        if options.is_empty() {
            self.prompt_create_api().await
        } else {
            // select one of the existing APIs or create a new one
            let create_new_option = "<Create New API>";
            let mut names: Vec<String> = options.iter().map(|a| a.name.clone()).collect();
            names.insert(0, create_new_option.into());

            let choice = inquire::Select::new("Select API:", names).prompt()?;

            if choice == create_new_option {
                self.prompt_create_api().await
            } else {
                Ok(options
                    .iter()
                    .find(|a| a.name == choice)
                    .cloned()
                    .expect("Invalid option chosen"))
            }
        }
    }

    async fn select_version(&self, api: &Api, options: &[ApiSpec]) -> CliResult<ApiSpec> {
        if options.is_empty() {
            self.prompt_create_version(api).await
        } else {
            // select one of the existing versions or create a new one
            let create_new_option = "<Create New Version>";
            let mut versions: Vec<String> = options.iter().map(|v| v.version.clone()).collect();
            versions.insert(0, create_new_option.to_string());

            let choice = inquire::Select::new("Select Version:", versions).prompt()?;

            if choice == create_new_option {
                self.prompt_create_version(api).await
            } else {
                Ok(options
                    .iter()
                    .find(|v| v.version == choice)
                    .cloned()
                    .expect("Invalid option chosen"))
            }
        }
    }

    async fn select_config(&self, api: &Api, version: &ApiSpec) -> CliResult<(Utf8PathBuf, bool)> {
        let generate_new = inquire::Confirm::new("Create new Sideko SDK config?")
            .with_default(true)
            .prompt()?;
        if generate_new {
            Ok((self.create_config(api, version).await?, true))
        } else {
            let config_path = inquire::Text::new("Config:")
                .with_help_message("Enter path Sideko SDK config")
                .with_placeholder("./sdk-config.yml")
                .with_validator(PathValidator::file().with_extensions(&[".yaml", ".yml"]))
                .with_autocomplete(FilePathCompleter::default())
                .prompt()?;

            Ok((Utf8PathBuf::new().join(config_path), false))
        }
    }

    pub async fn create_config(&self, api: &Api, version: &ApiSpec) -> CliResult<Utf8PathBuf> {
        let output_str = inquire::Text::new("Config Output Path:")
            .with_help_message("Enter path to save Sideko SDK config")
            .with_default("./sdk-config.yml")
            .with_validator(
                PathValidator::file()
                    .with_extensions(&[".yaml", ".yml"])
                    .with_allow_dne(),
            )
            .with_autocomplete(FilePathCompleter::default())
            .prompt()?;
        let output = Utf8PathBuf::new().join(&output_str);

        // TODO: accept x_mods? is it too complex/too many steps?
        debug!("Running `sideko sdk config init` with prompted input...");
        let init_cmd = SdkConfigInitCommand {
            api_name: api.name.clone(),
            api_version: version.version.clone(),
            x_mods: false,
            output: output.clone(),
        };
        init_cmd.handle().await?;
        info!("{} Config generated", fmt_green("✓"));

        Ok(output)
    }

    async fn select_language(&self) -> CliResult<SdkLanguageEnum> {
        // confirm feature flags for language generation
        let mut client = get_sideko_client();
        let org = client.org().get().await?;
        let validator = SdkLanguageValidator::new(&org.features);
        let mut lang_input = None;
        while lang_input.is_none() {
            // inquire::Select API does not allow for disabled options, rolled our own simple one
            // TODO: clear previous selection?
            let lang_str = inquire::Select::new("Language:", validator.options())
                .with_help_message("Select language to generate SDK")
                .prompt()?;
            if let Ok(inquire::validator::Validation::Valid) = validator.validate(&lang_str) {
                lang_input = Some(lang_str);
            }
        }

        // the validator ensures the casting of options to SdkLanguageEnum passes so we can use .expect here
        let lang = validator
            .to_lang(&lang_input.unwrap_or_default())
            .expect("Failed casting language selection");
        Ok(lang)
    }

    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let api_options = client.api().list().await?;
        debug!("Found {} APIs to choose from", &api_options.len());
        let api = self.select_api(&api_options).await?;

        let version_options = client
            .api()
            .spec()
            .list(spec::ListRequest {
                api_name: api.name.clone(),
            })
            .await?;
        debug!("Found {} versions to choose from", &version_options.len());
        let api_version = self.select_version(&api, &version_options).await?;

        let (config, newly_generated) = self.select_config(&api, &api_version).await?;
        let generate_now = if newly_generated {
            inquire::Confirm::new("Generate now?")
                .with_default(true)
                .with_help_message("Generate with the default configuration?")
                .prompt()?
        } else {
            true
        };
        if generate_now {
            let lang = self.select_language().await?;
            let version = inquire::Text::new("SDK Version:")
                .with_help_message(
                    "Enter the version of this SDK following the semantic versioning format",
                )
                .with_default("0.1.0")
                .with_validator(SemverValidator)
                .prompt()?;
            debug!("Running `sideko sdk create` with prompted input...");
            let create_sdk_cmd = SdkCreateCommand {
                config,
                lang: SdkLang(lang),
                version: version.parse().expect("failed parsing sdk semver"),
                api_version: api_version.version,
                gh_actions: true,
                output: Utf8PathBuf::new().join("."),
            };
            create_sdk_cmd.handle().await?;

            info!("{} Done.", fmt_green("✓"))
        } else {
            info!("Make any alterations to {config} (https://docs.sideko.dev/sdk-generation/customizing-sdks) then run `sideko sdk create` to generate your first SDK!")
        }

        Ok(())
    }
}

#[derive(Clone)]
struct ApiNameValidator;
impl inquire::validator::StringValidator for ApiNameValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        if input.len() < 3 {
            return Ok(inquire::validator::Validation::Invalid(
                "API name must be at least 3 characters".into(),
            ));
        }

        let api_name_pattern =
            Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").expect("invalid API Name regex pattern");
        if api_name_pattern.is_match(input) {
            Ok(inquire::validator::Validation::Valid)
        } else {
            Ok(inquire::validator::Validation::Invalid(
                "Invalid API Name".into(),
            ))
        }
    }
}

#[derive(Clone)]
struct SemverValidator;
impl inquire::validator::StringValidator for SemverValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        let valid = match semver::Version::parse(input) {
            Ok(_) => inquire::validator::Validation::Valid,
            Err(e) => inquire::validator::Validation::Invalid(
                format!("Invalid semantic version: {e}").into(),
            ),
        };

        Ok(valid)
    }
}

#[derive(Clone)]
struct PathValidator {
    kind: PathKind,
    allow_dne: bool,
    extensions: Option<Vec<String>>,
}
impl PathValidator {
    pub fn file() -> Self {
        Self {
            kind: PathKind::File,
            allow_dne: false,
            extensions: None,
        }
    }
    pub fn dir() -> Self {
        Self {
            kind: PathKind::Dir,
            allow_dne: false,
            extensions: None,
        }
    }
    pub fn with_extensions<S: ToString>(mut self, extensions: &[S]) -> Self {
        self.extensions = Some(extensions.iter().map(|s| s.to_string()).collect());
        self
    }
    pub fn with_allow_dne(mut self) -> Self {
        self.allow_dne = true;
        self
    }
}
impl inquire::validator::StringValidator for PathValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        let res = if let Some(extensions) = &self.extensions {
            utils::validators::validate_file_with_extension(
                input,
                self.allow_dne,
                &extensions.iter().map(|e| e.as_str()).collect::<Vec<&str>>(),
            )
        } else {
            utils::validators::validate_path(input, self.kind.clone(), self.allow_dne)
        };
        let valid = if let Err(e) = res {
            inquire::validator::Validation::Invalid(e.into())
        } else {
            inquire::validator::Validation::Valid
        };

        Ok(valid)
    }
}

#[derive(Clone)]
struct SdkLanguageValidator {
    features: OrganizationFeatures,
}
impl SdkLanguageValidator {
    pub fn new(features: &OrganizationFeatures) -> Self {
        Self {
            features: features.clone(),
        }
    }

    fn allowed(&self, lang: &SdkLanguageEnum) -> bool {
        match lang {
            SdkLanguageEnum::Go => self.features.allow_sdk_go,
            SdkLanguageEnum::Java => self.features.allow_sdk_java,
            SdkLanguageEnum::Python => self.features.allow_sdk_python,
            SdkLanguageEnum::Rust => self.features.allow_sdk_rust,
            SdkLanguageEnum::Typescript => self.features.allow_sdk_typescript,
        }
    }

    fn option(&self, lang: &SdkLanguageEnum) -> String {
        let postfix = if self.allowed(lang) {
            ""
        } else {
            " (requires upgrade)"
        };

        format!("{}{postfix}", json!(lang).to_string().replace("\"", ""))
    }

    pub fn options(&self) -> Vec<String> {
        vec![
            self.option(&SdkLanguageEnum::Python),
            self.option(&SdkLanguageEnum::Typescript),
            self.option(&SdkLanguageEnum::Go),
            self.option(&SdkLanguageEnum::Rust),
            self.option(&SdkLanguageEnum::Java),
        ]
    }

    pub fn to_lang(&self, input: &str) -> Result<SdkLanguageEnum, serde_json::Error> {
        let lang_str = format!("\"{}\"", input.replace(" (requires upgrade)", ""));
        serde_json::from_str(&lang_str)
    }
}
impl inquire::validator::StringValidator for SdkLanguageValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        let lang = match self.to_lang(input) {
            Ok(l) => l,
            Err(_) => {
                return Ok(inquire::validator::Validation::Invalid(
                    "Invalid language selected".into(),
                ))
            }
        };

        if self.allowed(&lang) {
            Ok(inquire::validator::Validation::Valid)
        } else {
            Ok(inquire::validator::Validation::Invalid(
                "The selected language is not available in your plan".into(),
            ))
        }
    }
}

/// Lifted from https://github.com/mikaelmello/inquire/blob/main/inquire/examples/complex_autocompletion.rs
#[derive(Clone, Default)]
pub struct FilePathCompleter {
    input: String,
    paths: Vec<String>,
}
impl FilePathCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), inquire::CustomUserError> {
        if input == self.input && !self.paths.is_empty() {
            return Ok(());
        }

        self.input = input.to_owned();
        self.paths.clear();

        let input_path = std::path::PathBuf::from(input);

        let fallback_parent = input_path
            .parent()
            .map(|p| {
                if p.to_string_lossy() == "" {
                    std::path::PathBuf::from(".")
                } else {
                    p.to_owned()
                }
            })
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let scan_dir = if input.ends_with('/') {
            input_path
        } else {
            fallback_parent.clone()
        };

        let entries = match std::fs::read_dir(scan_dir) {
            Ok(read_dir) => Ok(read_dir),
            Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_dir(fallback_parent),
            Err(err) => Err(err),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let path = entry.path();
            let path_str = if path.is_dir() {
                format!("{}/", path.to_string_lossy())
            } else {
                path.to_string_lossy().to_string()
            };

            self.paths.push(path_str);
        }

        Ok(())
    }

    fn fuzzy_sort(&self, input: &str) -> Vec<(String, i64)> {
        let mut matches: Vec<(String, i64)> = self
            .paths
            .iter()
            .filter_map(|path| {
                SkimMatcherV2::default()
                    .smart_case()
                    .fuzzy_match(path, input)
                    .map(|score| (path.clone(), score))
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        matches
    }
}
impl inquire::Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        self.update_input(input)?;

        let matches = self.fuzzy_sort(input);
        Ok(matches.into_iter().take(15).map(|(path, _)| path).collect())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        self.update_input(input)?;

        Ok(if let Some(suggestion) = highlighted_suggestion {
            inquire::autocompletion::Replacement::Some(suggestion)
        } else {
            let matches = self.fuzzy_sort(input);
            matches
                .first()
                .map(|(path, _)| inquire::autocompletion::Replacement::Some(path.clone()))
                .unwrap_or(inquire::autocompletion::Replacement::None)
        })
    }
}
