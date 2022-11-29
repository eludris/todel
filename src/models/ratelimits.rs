use serde::{Deserialize, Serialize};

use crate::conf::{EffisRatelimitData, OprishRatelimits, RatelimitConf};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceRatelimits {
    pub oprish: OprishRatelimits,
    pub pandemonium: RatelimitConf,
    pub effis: EffisRatelimitData,
}
