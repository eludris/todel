use serde::{Deserialize, Deserializer, Serialize};
use ubyte::ByteUnit;

use super::RateLimitConf;

/// Effis configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffisConf {
    pub url: String,
    #[serde(deserialize_with = "deserialize_file_size")]
    pub file_size: u64,
    #[serde(deserialize_with = "deserialize_file_size")]
    pub attachment_file_size: u64,
    pub rate_limits: EffisRateLimits,
}

/// Rate limits that apply to Effis (The CDN).
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "assets": {
///     "reset_after": 60,
///     "limit": 5,
///     "file_size_limit": 30000000
///   },
///   "attachments": {
///     "reset_after": 180,
///     "limit": 20,
///     "file_size_limit": 500000000
///   },
///   "fetch_file": {
///     "reset_after": 60,
///     "limit": 30
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffisRateLimits {
    /// Rate limits for the asset buckets.
    pub assets: EffisRateLimitConf,
    /// Rate limits for the attachment bucket.
    pub attachments: EffisRateLimitConf,
    /// Rate limits for the file fetching endpoints.
    pub fetch_file: RateLimitConf,
}

/// Represents a single rate limit for Effis.
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "reset_after": 60,
///   "limit": 5,
///   "file_size_limit": 30000000
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffisRateLimitConf {
    /// The amount of seconds after which the rate limit resets.
    pub reset_after: u32,
    /// The amount of requests that can be made within the `reset_after` interval.
    pub limit: u32,
    /// The maximum amount of bytes that can be sent within the `reset_after` interval.
    #[serde(deserialize_with = "deserialize_file_size")]
    pub file_size_limit: u64,
}

pub(crate) fn deserialize_file_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ByteUnit::deserialize(deserializer)?.as_u64())
}
