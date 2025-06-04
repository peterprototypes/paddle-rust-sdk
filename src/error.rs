//! Error handling module for Paddle API client

use std::error;
use std::fmt;

use chrono::Duration;
use serde::Deserialize;

use crate::ErrorResponse;

/// Type of error encountered.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    /// Typically means there's a problem with the request that you made.
    RequestError,
    /// Typically means there's a problem with the Paddle API.
    ApiError,
}

/// Error generated when validating webhook signatures
#[derive(Debug)]
pub enum SignatureError {
    /// No signature provided
    Empty,
    /// Invalid signature format
    InvalidFormat,
    /// A part of the signature is invalid
    InvalidPartFormat,
    /// Unable to extract timestamp or signature
    ParseError,
    /// Generated when the signature was calculated earlier in time then allowed
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

/// Error struct for a single invalid field.
#[derive(Debug, Deserialize)]
pub struct ValidationError {
    /// Field where validation error occurred.
    pub field: String,
    /// Information about how the field failed validation.
    pub message: String,
}

/// Error type returned from the Paddle API
#[derive(Debug, Deserialize)]
pub struct PaddleApiError {
    /// Type of error encountered.
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    /// Short snake case string that describes this error. Use to search the error reference.
    pub code: String,
    /// Some information about what went wrong as a human-readable string.
    pub detail: String,
    /// Link to a page in the error reference for this specific error.
    pub documentation_url: String,
    /// List of validation errors.
    pub errors: Option<Vec<ValidationError>>,
}

/// Paddle SDK Error type
///
/// If an error is generated anywhere in this crate, it will return this enum.
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
