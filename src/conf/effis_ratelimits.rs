use serde::{Deserialize, Serialize};

use super::RatelimitConf;

/// Effis ratelimit data config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffisRatelimits {
    #[serde(default = "assets_default")]
    pub assets: EffisRatelimitConf,
    #[serde(default = "attachments_default")]
    pub attachments: EffisRatelimitConf,
    #[serde(default = "fetch_file_default")]
    pub fetch_file: RatelimitConf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffisRatelimitConf {
    pub reset_after: u32,
    pub limit: u32,
    pub file_size_limit: String,
}

fn assets_default() -> EffisRatelimitConf {
    EffisRatelimitConf {
        reset_after: 60,
        limit: 5,
        file_size_limit: "30MB".to_string(),
    }
}

fn attachments_default() -> EffisRatelimitConf {
    EffisRatelimitConf {
        reset_after: 180,
        limit: 20,
        file_size_limit: "500MB".to_string(),
    }
}

fn fetch_file_default() -> RatelimitConf {
    RatelimitConf {
        reset_after: 60,
        limit: 30,
    }
}

impl Default for EffisRatelimits {
    fn default() -> Self {
        Self {
            assets: assets_default(),
            attachments: attachments_default(),
            fetch_file: fetch_file_default(),
        }
    }
}
