//! Main crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("PointError: {0}")]
    PointError(String),

    #[error("ReqwestError: {0}")]
    Reqwest(String),

    #[error("Error while Parsing Json: {0}")]
    Json(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error.to_string())
    }
}
