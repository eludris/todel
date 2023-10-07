use serde::{Deserialize, Serialize};

/// Shared fields between all error response variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedErrorData {
    /// The HTTP status of the error.
    pub status: u16,
    /// A brief explanation of the error.
    pub message: String,
}

/// All the possible error responses that are returned from Eludris HTTP microservices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorResponse {
    /// The error when the client is missing authorization. This error often occurs when the user
    /// doesn't pass in the required authentication or passes in invalid credentials.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "UNAUTHORIZED",
    ///   "status": 401,
    ///   "message": "The user is missing authentication or the passed credentials are invalid"
    /// }
    /// ```
    Unauthorized {
        #[serde(flatten)]
        shared: SharedErrorData,
    },
    /// The error when a client *has* been succesfully authorized but does not have the required
    /// permissions to execute an action.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "FORBIDDEN",
    ///   "status": 403,
    ///   "message": "The user is missing the requried permissions to execute this action",
    /// }
    /// ```
    Forbidden {
        #[serde(flatten)]
        shared: SharedErrorData,
    },
    /// The error when a client requests a resource that does not exist.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "NOT_FOUND",
    ///   "status": 404,
    ///   "message": "The requested resource could not be found"
    /// }
    /// ```
    NotFound {
        #[serde(flatten)]
        shared: SharedErrorData,
    },
    /// The error when a client's request causes a conflict, usually when they're trying to create
    /// something that already exists.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "CONFLICT",
    ///   "status": 409,
    ///   "message": "The request couldn't be completed due to conflicting with other data on the server",
    ///   "item": "username",
    /// }
    /// ```
    Conflict {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// The conflicting item.
        item: String,
    },
    /// The error when a server isn't able to reduce a response even though the client's request
    /// isn't explicitly wrong. This usually happens when an instance isn't configured to provide a
    /// response.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "MISDIRECTED",
    ///   "status": 421,
    ///   "message": "Misdirected request",
    ///   "info": "The instance isn't configured to deal with unbased individuals"
    /// }
    /// ```
    Misdirected {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// Extra information about what went wrong.
        info: String,
    },
    /// The error when a request a client sends is incorrect and fails validation.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "VALIDATION",
    ///   "status": 422,
    ///   "message": "Invalid request",
    ///   "value_name": "author",
    ///   "info": "author name is a bit too cringe"
    /// }
    /// ```
    Validation {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// The name of the value that failed validation.
        value_name: String,
        /// Extra information about what went wrong.
        info: String,
    },
    /// The error when a client is rate limited.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "RATE_LIMITED",
    ///   "status": 429,
    ///   "message": "You have been rate limited",
    ///   "retry_after": 1234
    /// }
    /// ```
    RateLimited {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// The amount of milliseconds you're still rate limited for.
        retry_after: u64,
    },
    /// The error when the server fails to process a request.
    ///
    /// Getting this error means that it's the server's fault and not the client that the request
    /// failed.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "SERVER",
    ///   "status": 500,
    ///   "message": "Server encountered an unexpected error",
    ///   "info": "Server got stabbed 28 times"
    /// }
    /// ```
    Server {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// Extra information about what went wrong.
        info: String,
    },
}
