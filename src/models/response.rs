use serde::{Deserialize, Serialize};

/// Base type for error responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ErrorData>,
}

/// Preset error types
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorData {
    RatelimitedError(RatelimitError),
    FileSizeRatelimitedError(FileSizeRatelimitedError),
    ValidationError(ValidationError),
    NotFoundError(NotFoundError),
}

/// The error when a client is ratelimited
#[derive(Debug, Serialize, Deserialize)]
pub struct RatelimitError {
    pub retry_after: u64,
}

/// The error caused when a client surpasses the maximum amount of bytes in an Effis ratelimit
/// bucket
#[derive(Debug, Serialize, Deserialize)]
pub struct FileSizeRatelimitedError {
    pub retry_after: u64,
    pub bytes_left: u128,
}

/// The error when the supplied request body is invalid
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field_name: String,
    pub error: String,
}

/// The error when the requested resource is not found.
#[derive(Debug, Serialize, Deserialize)]
pub struct NotFoundError;

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
            data: Some(ErrorData::RatelimitedError(self)),
        }
    }
}

#[cfg(feature = "logic")]
impl ErrorResponseData for FileSizeRatelimitedError {
    fn to_error_response(self) -> ErrorResponse {
        ErrorResponse {
            status: 429,
            message: "You have surpassed your file size limit".to_string(),
            data: Some(ErrorData::FileSizeRatelimitedError(self)),
        }
    }
}

#[cfg(feature = "logic")]
impl ErrorResponseData for ValidationError {
    fn to_error_response(self) -> ErrorResponse {
        ErrorResponse {
            status: 422,
            message: "Invalid request body".to_string(),
            data: Some(ErrorData::ValidationError(self)),
        }
    }
}

#[cfg(feature = "logic")]
impl ErrorResponseData for NotFoundError {
    fn to_error_response(self) -> ErrorResponse {
        ErrorResponse {
            status: 404,
            message: "The requestes resource cannot be found".to_string(),
            data: None,
        }
    }
}
