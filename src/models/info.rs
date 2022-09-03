use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The instance info payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub instance_name: String,
    pub features: HashMap<String, u32>,
}
