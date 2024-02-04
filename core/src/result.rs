#[derive(Debug)]
pub enum Error {
    General(String),
    ArgumentError(String),
    ReqwestError(String, reqwest::Error),
    ResponseError(String, reqwest::Response),
    IoError(String, std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;