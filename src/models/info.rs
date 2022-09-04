use serde::{Deserialize, Serialize};

/// The instance info payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub instance_name: String,
    pub features: Vec<Feature>,
}

/// The instance info feature payload
#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub id: u32,
    pub name: String,
}
