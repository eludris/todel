use std::{convert::Infallible, fmt::Display, net::IpAddr, str::FromStr};

use rocket::{
    async_trait,
    http::{Header, Status},
    request::{FromRequest, Outcome, Request},
    response::{self, Responder},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};

/// The type for all responses in Oprish
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Success(T),
    Failure(ErrorResponse),
}

/// The type for all Error responses in Oprish
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u16,
    pub data: ErrorResponseData,
}

impl ErrorResponse {
    /// Crete a new ErrorResponse based on the type of the passed data
    pub fn new(data: ErrorResponseData) -> ErrorResponse {
        let (message, status) = match &data {
            ErrorResponseData::Ratelimited { .. } => ("You have been ratelimited".to_string(), 429),
            ErrorResponseData::ValidationError { invalid_key, .. } => {
                (format!("Validation error at {}", invalid_key), 400)
            }
        };
        ErrorResponse {
            message,
            status,
            data,
        }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let status = self.status;
        response::Response::build_from(Json(self).respond_to(req)?)
            .status(Status::from_code(status).unwrap())
            .ok()
    }
}

impl<'r, T: Responder<'r, 'static>> Responder<'r, 'static> for Response<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::Success(v) => response::Response::build_from(v.respond_to(req)?),
            Self::Failure(v) => response::Response::build_from(v.respond_to(req)?),
        }
        .ok()
    }
}

/// The data of ErrorResponses
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponseData {
    Ratelimited { retry_after: u64 },
    ValidationError { invalid_key: String, info: String },
}

/// A type alias for the return type of routes which have ratelimits
pub type RatelimitedRoutResponse<T> = Result<RatelimitHeaderWrapper<T>, Json<ErrorResponse>>;

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
        // Hey there future reader or probably oliver, in case you're wondering why these two lines
        // got removed it's because apparently rocket already checks the `X-Real-IP` header when
        // the `client_ip` method is called
        //
        // Docs: https://api.rocket.rs/v0.5-rc/rocket/request/struct.Request.html#method.client_ip
        //
        // if let Some(ip) = req.headers().get_one("X-Real-IP") {
        // Outcome::Success(ClientIP(IpAddr::from_str(ip).unwrap()))
        // } else
        if let Some(ip) = req.headers().get_one("CF-Connecting-IP") {
            Outcome::Success(ClientIP(IpAddr::from_str(ip).unwrap()))
        } else {
            Outcome::Success(ClientIP(
                req.client_ip()
                    .unwrap_or_else(|| IpAddr::from_str("127.0.0.1").unwrap()),
            ))
        }
    }
}
