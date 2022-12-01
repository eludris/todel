use serde::{Deserialize, Serialize};

/// The instance info payload
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub instance_name: String,
    pub description: Option<String>,
    pub message_limit: usize,
    pub oprish_url: String,
    pub pandemonium_url: String,
    pub effis_url: String,
    pub file_size: u64,
    pub attachment_file_size: u64,
}
