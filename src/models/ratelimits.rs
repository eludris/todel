use serde::{Deserialize, Serialize};

use crate::conf::{EffisRatelimits, OprishRatelimits, RatelimitConf};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceRatelimits {
    pub oprish: OprishRatelimits,
    pub pandemonium: RatelimitConf,
    pub effis: EffisRatelimits,
}
