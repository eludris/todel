use rocket::{
    http::{Header, Status},
    serde::json::Json,
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_valid::validation::Errors;

use crate::models::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u32,
    pub data: ErrorResponseData,
}

impl ErrorResponse {
    pub fn new(data: ErrorResponseData) -> ErrorResponse {
        let (message, status) = match data {
            ErrorResponseData::RateLimited { .. } => ("You have been ratelimited".to_string(), 429),
        };
        ErrorResponse {
            message,
            status,
            data,
        }
    }

    pub fn to_response(self) -> (Status, Json<ErrorResponse>) {
        (Status::from_code(self.status as u16).unwrap(), Json(self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponseData {
    RateLimited { retry_after: u64 },
}

pub type RatelimitedRoutResponse<T> =
    Result<RatelimitHeaderWrapper<T>, (Status, Json<ErrorResponse>)>;

#[derive(Debug, Responder)]
#[response(content_type = "json")]
pub struct RatelimitHeaderWrapper<T> {
    pub inner: T,
    pub ratelimit_reset: Header<'static>,
    pub ratelimit_max: Header<'static>,
    pub ratelimit_last_reset: Header<'static>,
    pub ratelimit_request_count: Header<'static>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MessageCreateResponse {
    Sucess(Message),
    ValidationError(Errors),
}
