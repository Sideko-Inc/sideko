use std::io::ErrorKind;

use camino::Utf8PathBuf;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use log::{debug, info};
use regex::Regex;
use semver::Version;
use serde_json::json;
use sideko_rest_api::{
    models::{
        Api, ApiSpec, ApiVersion, OrganizationFeatures, SdkLanguageEnum, SdkModuleStructureEnum,
        VersionOrBump,
    },
    resources::api::{self, spec},
    UploadFile,
};

use crate::{
    cmds::sdk::{
        config::init::SdkConfigInitCommand, create::SdkCreateCommand, SdkLang, SdkModuleStructure,
    },
    result::{CliError, CliResult},
    styles::{fmt_green, fmt_grey},
    utils::{self, get_sideko_client, validators::PathKind},
};

#[derive(clap::Args)]
pub struct SdkInitCommand;

#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorBody {
    description: String,
}

impl SdkInitCommand {
    async fn prompt_create_api(&self) -> CliResult<Api> {
        let name = inquire::Text::new("api name:")
            .with_help_message(
                "api name must only include lower-case alphanumeric characters and dashes",
            )
            .with_placeholder("my-api")
            .with_validator(ApiNameValidator)
            .prompt()?;

        let mut client = get_sideko_client();
        let new_api = client.api().create(api::CreateRequest { name }).await?;
        info!("{} api created", fmt_green("✔"));
        debug!("api with id: {}", &new_api.id);

        Ok(new_api)
    }

    async fn prompt_create_version(&self, api: &Api) -> CliResult<(ApiSpec, bool)> {
        let oas_path = inquire::Text::new("openapi:")
            .with_help_message("enter path to openapi (≥3.0) specification for the new version")
            .with_placeholder("path/to/spec.yml")
            .with_validator(PathValidator::file().with_extensions(&[".json", ".yaml", ".yml"]))
            .with_autocomplete(FilePathCompleter::default())
            .prompt()?;
        let version = inquire::Text::new("version:")
            .with_help_message("enter the version of this api in the semver format")
            .with_default("0.1.0")
            .with_validator(SemverValidator)
            .prompt()?;

        let mut client = get_sideko_client();

        // Try to create the spec with strict linting first
        let result = client
            .api()
            .spec()
            .create(spec::CreateRequest {
                api_name: api.name.clone(),
                openapi: UploadFile::from_path(&oas_path).map_err(|e| {
                    CliError::io_custom(format!("failed reading openapi from path: {oas_path}"), e)
                })?,
                version: VersionOrBump::Str(version.clone()),
                mock_server_enabled: Some(true),
                allow_lint_errors: Some(false),
                ..Default::default()
            })
            .await;

        match result {
            Ok(v) => {
                info!("{} version created", fmt_green("✔"));
                debug!("new api version in `{}` with id: {}", &api.name, &v.id);
                Ok((v, false))
            }
            Err(sideko_rest_api::Error::Api(e)) => {
                let err_msg: ApiErrorBody = serde_json::from_slice(&e.content)
                    .map_err(|_| sideko_rest_api::Error::Api(e.clone()))?;

                if err_msg.description.contains("linting errors") {
                    // Ask user if they want to continue with linting errors
                    let continue_with_errors = inquire::Confirm::new(
                        "OpenAPI spec has linting errors. Continue anyway?",
                    )
                    .with_help_message(
                        "Continuing will allow the spec to be uploaded despite linting errors. This may result in poor SDK quality!",
                    )
                    .with_default(false)
                    .prompt()?;

                    if continue_with_errors {
                        // Retry with allow_lint_errors = true
                        let retry_result = client
                            .api()
                            .spec()
                            .create(spec::CreateRequest {
                                api_name: api.name.clone(),
                                openapi: UploadFile::from_path(&oas_path).map_err(|e| {
                                    CliError::io_custom(
                                        format!("failed reading openapi from path: {oas_path}"),
                                        e,
                                    )
                                })?,
                                version: VersionOrBump::Str(version.clone()),
                                mock_server_enabled: Some(true),
                                allow_lint_errors: Some(true),
                                ..Default::default()
                            })
                            .await?;
                        info!(
                            "{} version created (please fix the linting errors later by running: sideko api lint)",
                            fmt_green("✔")
                        );
                        Ok((retry_result, true))
                    } else {
                        // User chose not to continue, return the original error
                        Err(sideko_rest_api::Error::Api(e).into())
                    }
                } else {
                    // Different error, return it
                    Err(sideko_rest_api::Error::Api(e).into())
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn select_api(&self, options: &[Api]) -> CliResult<Api> {
        if options.is_empty() {
            self.prompt_create_api().await
        } else {
            // select one of the existing APIs or create a new one
            let create_new_option = "<create new api>";
            let mut names: Vec<String> = options.iter().map(|a| a.name.clone()).collect();
            names.insert(0, create_new_option.into());

            let choice = inquire::Select::new("select api:", names).prompt()?;

            if choice == create_new_option {
                self.prompt_create_api().await
            } else {
                Ok(options
                    .iter()
                    .find(|a| a.name == choice)
                    .cloned()
                    .expect("invalid option chosen"))
            }
        }
    }

    async fn select_version(&self, api: &Api, options: &[ApiSpec]) -> CliResult<(ApiSpec, bool)> {
        if options.is_empty() {
            self.prompt_create_version(api).await
        } else {
            // select one of the existing versions or create a new one
            let create_new_option = "<create new version>";
            let mut versions: Vec<String> = options.iter().map(|v| v.version.clone()).collect();
            versions.insert(0, create_new_option.to_string());

            let choice = inquire::Select::new("select version:", versions).prompt()?;

            if choice == create_new_option {
                self.prompt_create_version(api).await
            } else {
                Ok((
                    options
                        .iter()
                        .find(|v| v.version == choice)
                        .cloned()
                        .expect("invalid option chosen"),
                    false,
                ))
            }
        }
    }

    async fn select_config(&self, api: &Api, version: &ApiSpec) -> CliResult<Utf8PathBuf> {
        let generate_new = inquire::Confirm::new("create new sdk config? (need one to generate)")
            .with_default(true)
            .prompt()?;
        if generate_new {
            let mod_struct_options = vec![
                format!(
                    "path (recommended) {}",
                    fmt_grey("-- e.g. /store/order -> store.order.list()")
                ),
                format!(
                    "tag {}",
                    fmt_grey("-- uses OpenAPI tag to generate modules")
                ),
                format!(
                    "flat {}",
                    fmt_grey("-- all SDK functions available at the root")
                ),
            ];

            let res = inquire::Select::new(
                "generate SDK modules from:",
                mod_struct_options,
            )
            .with_help_message("select default SDK module/function name generation technique. learn more at: https://docs.sideko.dev/sdk-generation/customizing-sdks#module--function-customizations")
            .prompt()?;

            let mod_struct = if res.starts_with("tags") {
                SdkModuleStructureEnum::Tag
            } else if res.starts_with("flat") {
                SdkModuleStructureEnum::Flat
            } else {
                SdkModuleStructureEnum::Path
            };

            self.create_config(api, version, mod_struct).await
        } else {
            let config_path = inquire::Text::new("config:")
                .with_help_message("enter path to sdk config")
                .with_placeholder("./sdk-config.yml")
                .with_validator(PathValidator::file().with_extensions(&[".yaml", ".yml"]))
                .with_autocomplete(FilePathCompleter::default())
                .prompt()?;

            Ok(Utf8PathBuf::new().join(config_path))
        }
    }

    pub async fn create_config(
        &self,
        api: &Api,
        version: &ApiSpec,
        mod_struct: SdkModuleStructureEnum,
    ) -> CliResult<Utf8PathBuf> {
        let mut output = Utf8PathBuf::new().join("./sdk-config.yml");
        let mut path_modifier = 1;
        while output.exists() {
            output = Utf8PathBuf::new().join(format!("./sdk-config-{path_modifier}.yml"));
            debug!("default config output exists, trying {output}...");
            path_modifier += 1;
        }
        debug!("running `sideko sdk config init` with prompted input...");
        let init_cmd = SdkConfigInitCommand {
            api_name: api.name.clone(),
            api_version: version.version.clone(),
            module_structure: Some(SdkModuleStructure(mod_struct)),
            output: output.clone(),
        };
        init_cmd.handle().await?;
        info!("{} sdk config generated", fmt_green("✔"));

        Ok(output)
    }

    async fn select_languages(&self) -> CliResult<Vec<SdkLanguageEnum>> {
        // confirm feature flags for language generation
        let mut client = get_sideko_client();
        let org = client.org().get().await?;

        let mut langs = vec![];
        let validator = SdkLanguageValidator::new(&org.features);

        while langs.is_empty() {
            let input = inquire::MultiSelect::new("select languages:", validator.options())
                .with_validator(validator.clone())
                .prompt()?;

            // the validator ensures the casting of options to SdkLanguageEnum passes so we can use .expect here
            langs = input
                .into_iter()
                .map(|l| {
                    validator
                        .to_lang(&l)
                        .expect("failed casting lang selection")
                })
                .collect()
        }

        Ok(langs)
    }

    pub async fn handle(&self) -> CliResult<()> {
        let mut client = get_sideko_client();

        let api_options = client.api().list().await?;
        debug!("found {} apis to choose from", &api_options.len());
        let api = self.select_api(&api_options).await?;

        let version_options = client
            .api()
            .spec()
            .list(spec::ListRequest {
                api_name: api.name.clone(),
            })
            .await?;
        debug!("found {} versions to choose from", &version_options.len());
        let api_version = self.select_version(&api, &version_options).await?;
        let max_sdk_methods = client.org().get().await?.features.max_sdk_api_methods;

        // Check API stats for operation count
        let stats = client
            .api()
            .spec()
            .get_stats(spec::GetStatsRequest {
                api_name: api.name.clone(),
                api_version: ApiVersion::Str(api_version.0.version.clone()),
            })
            .await?;

        if stats.methods > max_sdk_methods && !max_sdk_methods == -1 {
            info!(
                "⚠️ ⚠️ ⚠️ api has {} operations, which exceeds your current limit of {}.",
                stats.methods, max_sdk_methods
            );
            info!("⚠️ ⚠️ ⚠️ consider using the SDK config to hide unused operations: https://docs.sideko.dev/sdk-generation/customizing-sdks");
        }

        let config = self.select_config(&api, &api_version.0).await?;
        let langs = self.select_languages().await?;
        for lang in langs {
            debug!(
                "running `sideko sdk create --lang {} ...` with prompted input",
                json!(&lang)
            );
            let create_sdk_cmd = SdkCreateCommand {
                config: config.clone(),
                lang: SdkLang(lang),
                version: Version::new(0, 1, 0),
                api_version: api_version.0.version.clone(),
                gh_actions: true,
                output: Utf8PathBuf::new().join("."),
                allow_lint_errors: api_version.1,
            };
            create_sdk_cmd.handle().await?;
        }

        info!("\n{} sdks generated successfully.", fmt_green("✔"));
        info!("\nlearn about setting up automatic updates here: https://docs.sideko.dev/sdk-generation/managed-sdks\n");

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
                "api name must be at least 3 characters".into(),
            ));
        }

        let api_name_pattern =
            Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").expect("invalid api name regex pattern");
        if api_name_pattern.is_match(input) {
            Ok(inquire::validator::Validation::Valid)
        } else {
            Ok(inquire::validator::Validation::Invalid(
                "invalid api name".into(),
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
                format!("invalid semantic version: {e}").into(),
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
    pub fn with_extensions<S: ToString>(mut self, extensions: &[S]) -> Self {
        self.extensions = Some(extensions.iter().map(|s| s.to_string()).collect());
        self
    }
    #[allow(unused)]
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
            SdkLanguageEnum::Csharp => self.features.allow_sdk_csharp,
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
            self.option(&SdkLanguageEnum::Csharp),
            self.option(&SdkLanguageEnum::Rust),
            self.option(&SdkLanguageEnum::Java),
        ]
    }

    pub fn to_lang(&self, input: &str) -> Result<SdkLanguageEnum, serde_json::Error> {
        let lang_str = format!("\"{}\"", input.replace(" (requires upgrade)", ""));
        serde_json::from_str(&lang_str)
    }
}
impl inquire::validator::MultiOptionValidator<String> for SdkLanguageValidator {
    fn validate(
        &self,
        input: &[inquire::list_option::ListOption<&String>],
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        let mut langs = vec![];
        for l in input {
            let lang = match self.to_lang(l.value) {
                Ok(l) => l,
                Err(_) => {
                    return Ok(inquire::validator::Validation::Invalid(
                        "invalid language selected".into(),
                    ))
                }
            };
            langs.push(lang);
        }

        let disallowed_langs: Vec<&SdkLanguageEnum> =
            langs.iter().filter(|l| !self.allowed(l)).collect();
        if !disallowed_langs.is_empty() {
            Ok(inquire::validator::Validation::Invalid(
                format!(
                    "the selected language(s) is not available in your plan: {}",
                    json!(disallowed_langs)
                )
                .into(),
            ))
        } else {
            Ok(inquire::validator::Validation::Valid)
        }
    }
}

/// lifted from https://github.com/mikaelmello/inquire/blob/main/inquire/examples/complex_autocompletion.rs
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
