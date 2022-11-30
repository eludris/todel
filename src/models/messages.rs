use serde::{Deserialize, Serialize};

/// The message payload
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub author: String,
    pub content: String,
}
