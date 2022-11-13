use serde::{Deserialize, Serialize};

use super::RatelimitData;

/// Oprish ratelimit config.
#[derive(Debug, Serialize, Deserialize)]
pub struct OprishRatelimits {
    #[serde(default = "info_default")]
    pub info: RatelimitData,
    #[serde(default = "message_create_default")]
    pub message_create: RatelimitData,
    #[serde(default = "ratelimits_default")]
    pub ratelimits: RatelimitData,
}

impl Default for OprishRatelimits {
    fn default() -> Self {
        Self {
            info: info_default(),
            message_create: message_create_default(),
            ratelimits: ratelimits_default(),
        }
    }
}

fn info_default() -> RatelimitData {
    RatelimitData {
        reset_after: 5,
        limit: 2,
    }
}

fn message_create_default() -> RatelimitData {
    RatelimitData {
        reset_after: 5,
        limit: 10,
    }
}

fn ratelimits_default() -> RatelimitData {
    RatelimitData {
        reset_after: 5,
        limit: 2,
    }
}
