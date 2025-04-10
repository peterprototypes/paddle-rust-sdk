//! Error handling module for Paddle API client

use std::error;
use std::fmt;

use serde::Deserialize;

use crate::ErrorResponse;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    RequestError,
    ApiError,
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PaddleError {
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub code: String,
    pub detail: String,
    pub documentation_url: String,
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Url(url::ParseError),
    Paddle(ErrorResponse),
    QueryString(serde_qs::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Request(err) => write!(f, "Request error: {}", err),
            Self::Url(err) => write!(f, "URL error: {}", err),
            Self::Paddle(err) => write!(f, "Paddle error: {}", err.error.detail),
            Self::QueryString(err) => write!(f, "Query string error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Request(err) => Some(err),
            Self::Url(err) => Some(err),
            Self::Paddle(_) => None,
            Self::QueryString(err) => Some(err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::Url(err)
    }
}

impl From<serde_qs::Error> for Error {
    fn from(err: serde_qs::Error) -> Self {
        Self::QueryString(err)
    }
}
