mod effis;
mod oprish;
mod pandemonium;

use serde::{Deserialize, Serialize};

pub use effis::*;
pub use oprish::*;
pub use pandemonium::*;

/// Represents a single rate limit.
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "reset_after": 60,
///   "limit": 30
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitConf {
    /// The amount of seconds after which the rate limit resets.
    pub reset_after: u32,
    /// The amount of requests that can be made within the `reset_after` interval.
    pub limit: u32,
}
