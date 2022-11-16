use serde::{Deserialize, Serialize};

use super::Message;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "op", content = "d")]
pub enum Payload {
    Ping,
    Message(Message),
}
