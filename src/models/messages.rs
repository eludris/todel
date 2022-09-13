use serde::{Deserialize, Serialize};
use serde_valid::Validate;

/// The message payload.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Message {
    #[validate(
        min_length = 2,
        message = "Author name cannot be less than 2 characters in length"
    )]
    #[validate(
        max_length = 32,
        message = "Author name cannot be more than 32 characters in length"
    )]
    pub author: String,
    #[validate(min_length = 1, message = "Cannot send an empty message")]
    #[validate(
        max_length = 6000,
        message = "Message cannot be greater than 6000 characters in length"
    )]
    pub content: String,
}
