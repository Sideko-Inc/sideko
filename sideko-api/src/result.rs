///Generated by Sideko (sideko.dev)
#[derive(Debug, thiserror::Error)]
pub enum Error<T> {
    #[error("Failed serializing json: {0} ")]
    Serialize(serde_json::Error),
    #[error("Failed sending http request: {0}")]
    Dispatch(reqwest::Error),
    #[error("Failed deserializing json into {expected_signature:?}: {serde_err:?} ")]
    UnexpectedResponseBody {
        status_code: u16,
        method: String,
        url: String,
        response_text: String,
        expected_signature: String,
        serde_err: serde_json::Error,
    },
    #[error(
        "Unexpected status {status_code:?}, handlers set up for {expected_status_codes:?}"
    )]
    BlockingUnexpectedStatus {
        status_code: u16,
        method: String,
        url: String,
        response: reqwest::blocking::Response,
        expected_status_codes: Vec<String>,
    },
    #[error(
        "Unexpected status {status_code:?}, handlers set up for {expected_status_codes:?}"
    )]
    UnexpectedStatus {
        status_code: u16,
        method: String,
        url: String,
        response: reqwest::Response,
        expected_status_codes: Vec<String>,
    },
    #[error("Response returned unsuccessful status code: {status_code:?}")]
    Response { status_code: u16, method: String, url: String, data: T },
    #[error("Failed extracting bytes from response: {0}")]
    ResponseBytes(reqwest::Error),
    #[error("Failed reading file: {0}")]
    File(std::io::Error),
}
pub type Result<T, E> = std::result::Result<T, Error<E>>;
