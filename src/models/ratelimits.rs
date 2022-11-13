use serde::{Deserialize, Serialize};

use crate::conf::{EffisRatelimitData, OprishRatelimits, RatelimitData};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceRatelimits {
    pub oprish: OprishRatelimits,
    pub pandemonium: RatelimitData,
    pub effis: EffisRatelimitData,
}
