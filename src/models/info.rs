use serde::{Deserialize, Serialize};

/// The instance info payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub instance_name: String,
    // This should be somewhere between 1 character and 2048 characters long
    pub description: Option<String>,
}
