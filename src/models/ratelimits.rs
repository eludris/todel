use serde::{Deserialize, Serialize};

use crate::conf::{EffisRatelimits, OprishRatelimits, RatelimitConf};

/// The type which represents all of an instance's ratelimit configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceRatelimits {
    pub oprish: OprishRatelimits,
    pub pandemonium: RatelimitConf,
    pub effis: EffisRatelimits,
}
