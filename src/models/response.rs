use serde::{Deserialize, Serialize};

/// Base type for error responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub data: ErrorData,
}

/// Preset error types
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorData {
    RatelimitedError(RatelimitError),
    ValidationError(ValidationError),
}

/// The error when a client is ratelimited
#[derive(Debug, Serialize, Deserialize)]
pub struct RatelimitError {
    pub retry_after: u64,
}

/// The error when the supplied request body is invalid
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field_name: String,
    pub error: String,
}

#[cfg(feature = "logic")]
/// The trait for valid error response data types
pub trait ErrorResponseData {
    fn to_error_response(self) -> ErrorResponse;
}

#[cfg(feature = "logic")]
impl ErrorResponseData for RatelimitError {
    fn to_error_response(self) -> ErrorResponse {
        ErrorResponse {
            status: 429,
            message: "You have been ratelimited".to_string(),
            data: ErrorData::RatelimitedError(self),
        }
    }
}

#[cfg(feature = "logic")]
impl ErrorResponseData for ValidationError {
    fn to_error_response(self) -> ErrorResponse {
        ErrorResponse {
            status: 422,
            message: "Invalid request body".to_string(),
            data: ErrorData::ValidationError(self),
        }
    }
}
