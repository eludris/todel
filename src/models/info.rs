use serde::{Deserialize, Serialize};

/// The instance info payload
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub instance_name: String,
    // This should be somewhere between 1 character and 2048 characters long
    pub description: Option<String>,
    pub message_limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oprish_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pandemonium_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effis_url: Option<String>,
}
