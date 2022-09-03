use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub instance_name: String,
    pub features: HashMap<String, u32>,
}
