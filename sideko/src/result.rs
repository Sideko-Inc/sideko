use std::io;

use log::{debug, error};

#[derive(Debug)]
pub enum CliError {
    General {
        msg: String,
        debug: Option<String>,
    },
    Io {
        err: io::Error,
        override_msg: Option<String>,
    },
    Api {
        err: sideko_rest_api::Error,
        override_msg: Option<String>,
    },
    Inquire {
        err: inquire::InquireError,
        override_msg: Option<String>,
    },
    Keyring {
        err: keyring::Error,
        override_msg: Option<String>,
    },
    Arboard {
        err: arboard::Error,
        override_msg: Option<String>,
    },
}

impl CliError {
    pub fn general<S: ToString>(msg: S) -> Self {
        CliError::General {
            msg: msg.to_string(),
            debug: None,
        }
    }
    pub fn general_debug<S: ToString, D: ToString>(msg: S, debug: D) -> Self {
        CliError::General {
            msg: msg.to_string(),
            debug: Some(debug.to_string()),
        }
    }

    pub fn io_custom<S: ToString>(msg: S, err: io::Error) -> Self {
        CliError::Io {
            override_msg: Some(msg.to_string()),
            err,
        }
    }
    pub fn api_custom<S: ToString>(msg: S, err: sideko_rest_api::Error) -> Self {
        CliError::Api {
            override_msg: Some(msg.to_string()),
            err,
        }
    }
    pub fn inquire_custom<S: ToString>(msg: S, err: inquire::InquireError) -> Self {
        CliError::Inquire {
            err,
            override_msg: Some(msg.to_string()),
        }
    }
    pub fn keyring_custom<S: ToString>(msg: S, err: keyring::Error) -> Self {
        CliError::Keyring {
            err,
            override_msg: Some(msg.to_string()),
        }
    }
    pub fn arboard_custom<S: ToString>(msg: S, err: arboard::Error) -> Self {
        CliError::Arboard {
            err,
            override_msg: Some(msg.to_string()),
        }
    }

    pub fn log(&self) {
        let err_log = match self {
            CliError::General { msg, debug } => {
                if let Some(d) = debug {
                    debug!("{d}")
                }
                msg.clone()
            }
            CliError::Io { override_msg, err } => {
                debug!("{err:?}");
                override_msg.clone().unwrap_or_else(|| err.to_string())
            }
            CliError::Keyring { override_msg, err } => {
                debug!("{err:?}");
                override_msg.clone().unwrap_or_else(|| err.to_string())
            }
            CliError::Arboard { override_msg, err } => {
                debug!("{err:?}");
                override_msg.clone().unwrap_or_else(|| err.to_string())
            }
            CliError::Inquire { err, override_msg } => {
                debug!("{err:?}");
                override_msg.clone().unwrap_or_else(|| err.to_string())
            }
            CliError::Api { override_msg, err } => {
                match err {
                    sideko_rest_api::Error::Io(e) => debug!("SDK IO Error: {e:?}"),
                    sideko_rest_api::Error::Request(e) => debug!("SDK Request Error: {e:?}"),
                    sideko_rest_api::Error::DeserializeJson(e, json_str) => {
                        let res_json = serde_json::to_string_pretty(
                            &serde_json::from_str::<serde_json::Value>(json_str)
                                .unwrap_or_default(),
                        )
                        .unwrap_or_else(|_| json_str.to_string());
                        debug!("Deserializer Error: {e:?}");
                        debug!("Raw JSON: {res_json}");
                    }
                    sideko_rest_api::Error::Api(e) | sideko_rest_api::Error::ContentType(e) => {
                        debug!("Response headers: {:#?}", &e.headers);
                        if let Ok(val) = e.json::<serde_json::Value>() {
                            log::debug!(
                                "Body: {}",
                                serde_json::to_string_pretty(&val)
                                    .unwrap_or_else(|_| val.to_string())
                            );
                            if let Some(serde_json::Value::String(description)) =
                                val.get("description")
                            {
                                error!("{description}");
                            }
                        } else if let Ok(text) = std::str::from_utf8(&e.content) {
                            log::debug!("Body: {text}",);
                        } else {
                            log::debug!("Unable to display body ({} bytes)", e.content.len())
                        }
                    }
                    sideko_rest_api::Error::Custom(msg) => debug!("{msg}"),
                }

                override_msg.clone().unwrap_or_else(|| err.to_string())
            }
        };

        error!("{err_log}");
    }
}

impl From<sideko_rest_api::Error> for CliError {
    fn from(err: sideko_rest_api::Error) -> Self {
        Self::Api {
            err,
            override_msg: None,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        Self::Io {
            err,
            override_msg: None,
        }
    }
}

impl From<inquire::InquireError> for CliError {
    fn from(err: inquire::InquireError) -> Self {
        Self::Inquire {
            err,
            override_msg: None,
        }
    }
}

impl From<keyring::Error> for CliError {
    fn from(err: keyring::Error) -> Self {
        Self::Keyring {
            err,
            override_msg: None,
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;
