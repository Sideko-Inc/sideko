use std::{env, fmt::Display, str::FromStr};

use camino::Utf8PathBuf;
use log::{debug, warn};

use crate::result::{CliError, CliResult};

pub enum ConfigKey {
    ConfigPath,
    ApiKey,
    ApiBaseUrl,
}
impl ConfigKey {
    pub fn get(&self) -> Option<String> {
        env::var(self.to_string()).ok()
    }

    /// Updates dotenv by replacing exiting config key entry
    /// or appending a new line
    pub fn set<S: ToString>(&self, val: S) -> CliResult<()> {
        let sh_safe = shlex::try_quote(&val.to_string())
            .map(String::from)
            .unwrap_or_else(|_| val.to_string());
        let dotenv_entry = format!("{self}={sh_safe}");

        let cfg_path = get_config_path()?;
        let curr_dotenv= if cfg_path.exists() {
            let dotenv_string = std::fs::read_to_string(cfg_path.clone()).map_err(|e| {
                CliError::io_custom(format!("Failed loading sideko config file to update {self}: {cfg_path}"), e)
            })?;
            dotenv_string.split("\n").map(String::from).collect()
        } else {
            vec![]
        };

        // append or replace cfg var
        let mut replaced = false;
        let mut new_dotenv: Vec<String> = curr_dotenv.into_iter().map(|l| {
            if l.starts_with(&format!("{self}=")) {
                replaced = true;
                dotenv_entry.clone()
            } else {
                l
            }
        }
        ).collect();

        if !replaced {
            // append
            new_dotenv.push(dotenv_entry);
        }


    std::fs::write(&cfg_path, new_dotenv.join("\n")).map_err(|e| {
        CliError::io_custom(format!("Failed updating sideko config {self}: {cfg_path}"), e)
    })?;

        debug!("Set config {self}: {cfg_path}");

        Ok(())
    }
}
impl Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let env_var = match self {
            ConfigKey::ApiKey => "SIDEKO_API_KEY",
            ConfigKey::ApiBaseUrl => "SIDEKO_BASE_URL",
            ConfigKey::ConfigPath => "SIDEKO_CONFIG_PATH",
        };

        write!(f, "{env_var}")
    }
}

pub(crate) fn load() -> CliResult<()> {
    let cfg_path = get_config_path()?;
    if cfg_path.exists() {
        dotenvy::from_path(&cfg_path).map_err(|e| CliError::general_debug(format!("Failed loading sideko config: {cfg_path}"), e))?;
        debug!("Loaded config: {cfg_path}");
    }
    Ok(())
}


/// Retrieves the config path from user-set ConfigKey::ConfigPath,
/// defaulting to $HOME/.sideko if not set
pub(crate) fn get_config_path() -> CliResult<Utf8PathBuf> {
    if let Some(p) = ConfigKey::ConfigPath.get() {
        let path = Utf8PathBuf::from_str(&p).map_err(|e| {
            CliError::general_debug(
                format!(
                    "Unable to build default config path: ${} is set to an ill-formatted path: {p}",
                    ConfigKey::ConfigPath
                ),
                format!("{e:?}"),
            )
        })?;
        Ok(path)
    } else {
        get_default_config_path()
    }

}
pub(crate) fn get_default_config_path() -> CliResult<Utf8PathBuf> {
    let home = env::var("HOME")
            .map_err(|_| CliError::general("Unable to build default config path: $HOME is not set"))?;
        let mut default_path = Utf8PathBuf::from_str(&home).map_err(|e| {
            CliError::general_debug(
                format!(
                "Unable to build default config path: $HOME is set to an ill-formatted path: {home}"
    
            ),
                format!("{e:?}"),
            )
        })?;
        default_path.push(".sideko");
    
        Ok(default_path)
}

/// Retrieves Sideko API base url from user-set ConfigKey::ApiBaseUrl,
/// defaulting to production environment if not set
pub(crate) fn get_base_url() -> String {
    let url = ConfigKey::ApiBaseUrl.get()
        .unwrap_or(sideko_rest_api::environment::Environment::default().to_string());

    if !url.ends_with("/v1") {
        warn!("Sideko API base url does not end with `/v1`, this probably means it is wrong")
    }

    url
}
