use serde::{Deserialize, Serialize};

use super::RatelimitConf;

/// Oprish ratelimit config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OprishRatelimits {
    #[serde(default = "info_default")]
    pub info: RatelimitConf,
    #[serde(default = "message_create_default")]
    pub message_create: RatelimitConf,
    #[serde(default = "ratelimits_default")]
    pub ratelimits: RatelimitConf,
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

fn info_default() -> RatelimitConf {
    RatelimitConf {
        reset_after: 5,
        limit: 2,
    }
}

fn message_create_default() -> RatelimitConf {
    RatelimitConf {
        reset_after: 5,
        limit: 10,
    }
}

fn ratelimits_default() -> RatelimitConf {
    RatelimitConf {
        reset_after: 5,
        limit: 2,
    }
}
