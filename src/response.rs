//! This module defines the response structures for the Paddle API.

use serde::Deserialize;

use crate::entities::Pagination;
use crate::PaddleApiError;

/// Meta information about the API request.
/// This includes the request ID, which can be used for debugging or tracking purposes.
#[derive(Debug, Deserialize)]
pub struct Meta {
    pub request_id: String,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Response<T> {
    Success(SuccessResponse<T>),
    Error(ErrorResponse),
}

/// Success response structure for the Paddle API.
#[derive(Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub meta: Meta,
}

/// Error response structure for the Paddle API.
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: PaddleApiError,
    pub meta: Meta,
}
