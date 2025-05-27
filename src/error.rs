//! Error handling module for Paddle API client

use std::error;
use std::fmt;

use chrono::Duration;
use serde::Deserialize;

use crate::ErrorResponse;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    RequestError,
    ApiError,
}

#[derive(Debug)]
pub enum SignatureError {
    Empty,
    InvalidFormat,
    InvalidPartFormat,
    ParseError,
    MaxVarianceExceeded(Duration),
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty string provided"),
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidPartFormat => write!(f, "invalid signature part format"),
            Self::ParseError => write!(f, "unable to extract timestamp or signature"),
            Self::MaxVarianceExceeded(dur) => write!(f, "request was made more than {dur} ago"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PaddleApiError {
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
    PaddleApi(ErrorResponse),
    QueryString(serde_qs::Error),
    PaddleSignature(SignatureError),
    ParseIntError(std::num::ParseIntError),
    MacError(hmac::digest::MacError),
    JsonError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Request(err) => write!(f, "Request error: {}", err),
            Self::Url(err) => write!(f, "URL error: {}", err),
            Self::PaddleApi(err) => write!(f, "Paddle error: {}", err.error.detail),
            Self::QueryString(err) => write!(f, "Query string error: {}", err),
            Self::PaddleSignature(err) => write!(f, "Paddle signature error: {}", err),
            Self::ParseIntError(err) => write!(f, "Integer parsing error: {}", err),
            Self::MacError(err) => write!(f, "Hmac error: {}", err),
            Self::JsonError(err) => write!(f, "Serde json error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Request(err) => Some(err),
            Self::Url(err) => Some(err),
            Self::PaddleApi(_) => None,
            Self::QueryString(err) => Some(err),
            Self::PaddleSignature(_) => None,
            Self::ParseIntError(err) => Some(err),
            Self::MacError(err) => Some(err),
            Self::JsonError(err) => Some(err),
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

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<hmac::digest::MacError> for Error {
    fn from(err: hmac::digest::MacError) -> Self {
        Self::MacError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
