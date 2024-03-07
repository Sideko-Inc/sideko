#[derive(Debug)]
pub enum Error {
    General { msg: String, debug: Option<String> },
    Argument { msg: String, debug: Option<String> },
    Api { msg: String, debug: Option<String> },
    Io { msg: String, err: std::io::Error },
}

impl Error {
    pub fn general(msg: &str) -> Self {
        Error::General {
            msg: msg.into(),
            debug: None,
        }
    }
    pub fn general_with_debug(msg: &str, debug: &str) -> Self {
        Error::General {
            msg: msg.into(),
            debug: Some(debug.into()),
        }
    }

    pub fn arg(msg: &str) -> Self {
        Error::Argument {
            msg: msg.into(),
            debug: None,
        }
    }
    pub fn arg_with_debug(msg: &str, debug: &str) -> Self {
        Error::Argument {
            msg: msg.into(),
            debug: Some(debug.into()),
        }
    }

    pub fn api(msg: &str) -> Self {
        Error::Api {
            msg: msg.into(),
            debug: None,
        }
    }
    pub fn api_with_debug(msg: &str, debug: &str) -> Self {
        Error::Api {
            msg: msg.into(),
            debug: Some(debug.into()),
        }
    }

    pub fn error_msg(&self) -> String {
        match self {
            Error::General { msg, .. } => format!("Error: {msg}"),
            Error::Argument { msg, .. } => format!("Argument Error: {msg}"),
            Error::Api { msg, .. } => format!("API Error: {msg}"),
            Error::Io { msg, .. } => format!("IO Error: {msg}"),
        }
    }

    pub fn debug_msg(&self) -> Option<String> {
        match self {
            Error::General { debug, .. }
            | Error::Argument { debug, .. }
            | Error::Api { debug, .. } => debug.clone(),
            Error::Io { err, .. } => Some(format!("{err}")),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
