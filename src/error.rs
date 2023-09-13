//! Main crate Error

//! Error class that provide error message on a unexpected behaviour
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("PointError: {0}")]
    PointError(String),

    #[error("PathError: {0}")]
    PathError(String),

    #[error("ReqwestError: {0}")]
    Reqwest(String),

    #[error("Error while Parsing Json: {0}")]
    Json(String),

    #[error("Error: Bad Parameter")]
    BadParameter(),
}

/// Modify the Error class for the reqwest crate
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error.to_string())
    }
}

//. Modify the Error class for the serde_json crate
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error.to_string())
    }
}
