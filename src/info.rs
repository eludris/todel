use crate::conf::{EffisRateLimits, OprishRateLimits, RateLimitConf};
use serde::{Deserialize, Serialize};

/// Represents information about the connected Eludris instance.
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "instance_name": "eludris",
///   "description": "The *almost* official Eludris instance - ooliver.",
///   "version": "0.3.2",
///   "message_limit": 2000,
///   "bio_limit": 250,
///   "oprish_url": "https://api.eludris.gay",
///   "pandemonium_url": "wss://ws.eludris.gay/",
///   "effis_url": "https://cdn.eludris.gay",
///   "file_size": 20000000,
///   "attachment_file_size": 25000000,
///   "rate_limits": {
///     "oprish": {
///       "info": {
///         "reset_after": 5,
///         "limit": 2
///       },
///       "message_create": {
///         "reset_after": 5,
///         "limit": 10
///       },
///       "rate_limits": {
///         "reset_after": 5,
///         "limit": 2
///       }
///     },
///     "pandemonium": {
///       "reset_after": 10,
///       "limit": 5
///     },
///     "effis": {
///       "assets": {
///         "reset_after": 60,
///         "limit": 5,
///         "file_size_limit": 30000000
///       },
///       "attachments": {
///         "reset_after": 180,
///         "limit": 20,
///         "file_size_limit": 500000000
///       },
///       "fetch_file": {
///         "reset_after": 60,
///         "limit": 30
///       }
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    /// The instance's name.
    pub instance_name: String,
    /// The instance's description.
    ///
    /// This is between 1 and 2048 characters long.
    pub description: Option<String>,
    /// The instance's Eludris version.
    pub version: String,
    /// The maximum length of a message's content.
    pub message_limit: usize,
    /// The maximum length of a user's bio.
    pub bio_limit: usize,
    /// The URL of the instance's Oprish (REST API) endpoint.
    pub oprish_url: String,
    /// The URL of the instance's Pandemonium (WebSocket API) endpoint.
    pub pandemonium_url: String,
    /// The URL of the instance's Effis (CDN) endpoint.
    pub effis_url: String,
    /// The maximum file size (in bytes) of an asset.
    pub file_size: u64,
    /// The maximum file size (in bytes) of an attachment.
    pub attachment_file_size: u64,
    /// The instance's email address if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The rate limits that apply to the connected Eludris instance.
    ///
    /// This is not present if the `rate_limits` query parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<InstanceRateLimits>,
}

/// Represents all rate limits that apply to the connected Eludris instance.
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "oprish": {
///     "info": {
///       "reset_after": 5,
///       "limit": 2
///     },
///     "message_create": {
///       "reset_after": 5,
///       "limit": 10
///     },
///     "rate_limits": {
///       "reset_after": 5,
///       "limit": 2
///     }
///   },
///   "pandemonium": {
///     "reset_after": 10,
///     "limit": 5
///   },
///   "effis": {
///     "assets": {
///       "reset_after": 60,
///       "limit": 5,
///       "file_size_limit": 30000000
///     },
///     "attachments": {
///       "reset_after": 180,
///       "limit": 20,
///       "file_size_limit": 500000000
///     },
///     "fetch_file": {
///       "reset_after": 60,
///       "limit": 30
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceRateLimits {
    /// The instance's Oprish rate limit information (The REST API).
    pub oprish: OprishRateLimits,
    /// The instance's Pandemonium rate limit information (The WebSocket API).
    pub pandemonium: RateLimitConf,
    /// The instance's Effis rate limit information (The CDN).
    pub effis: EffisRateLimits,
}
