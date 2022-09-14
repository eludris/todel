use std::{convert::Infallible, fmt::Display, net::IpAddr, str::FromStr};

use rocket::{
    async_trait,
    http::{Header, Status},
    request::{FromRequest, Outcome, Request},
    serde::json::Json,
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_valid::validation::Errors;

use crate::models::Message;

/// The type for all Error responses in Oprish
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u32,
    pub data: ErrorResponseData,
}

impl ErrorResponse {
    /// Crete a new ErrorResponse based on the type of the passed data
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

    /// Generate a respond coupled with a Status code
    pub fn to_response(self) -> (Status, Json<ErrorResponse>) {
        (Status::from_code(self.status as u16).unwrap(), Json(self))
    }
}

/// The data of ErrorResponses
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponseData {
    RateLimited { retry_after: u64 },
}

/// A type alias for the return type of routes which have ratelimits
pub type RatelimitedRoutResponse<T> =
    Result<RatelimitHeaderWrapper<T>, (Status, Json<ErrorResponse>)>;

/// A type that wraps a Response as to add ratelimit-relavent headers to it
#[derive(Debug, Responder)]
#[response(content_type = "json")]
pub struct RatelimitHeaderWrapper<T> {
    pub inner: T,
    pub ratelimit_reset: Header<'static>,
    pub ratelimit_max: Header<'static>,
    pub ratelimit_last_reset: Header<'static>,
    pub ratelimit_request_count: Header<'static>,
}

// TODO: Refactor all returns into one type
/// The response of the message route
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MessageCreateResponse {
    Sucess(Message),
    ValidationError(Errors),
}

/// The *real* IP of a client.
#[derive(Debug)]
pub struct ClientIP(IpAddr);

impl Display for ClientIP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for ClientIP {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(ip) = req.headers().get_one("X-Real-IP") {
            Outcome::Success(ClientIP(IpAddr::from_str(ip).unwrap()))
        } else if let Some(ip) = req.headers().get_one("CF-Connecting-IP") {
            Outcome::Success(ClientIP(IpAddr::from_str(ip).unwrap()))
        } else {
            Outcome::Success(ClientIP(
                req.client_ip()
                    .unwrap_or_else(|| IpAddr::from_str("127.0.0.1").unwrap()),
            ))
        }
    }
}
