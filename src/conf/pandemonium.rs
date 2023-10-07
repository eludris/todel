use serde::{Deserialize, Serialize};

use super::RateLimitConf;

/// Pandemonium configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PandemoniumConf {
    pub url: String,
    pub rate_limit: RateLimitConf,
}
