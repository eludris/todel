use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileData {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u128,
    pub name: String,
    pub bucket: String,
    #[serde(skip_serializing_if = "is_false")]
    pub spoiler: bool,
    pub metadata: FileMetadata,
}

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FileMetadata {
    Text,
    Image { width: u32, height: u32 },
    Video { width: u32, height: u32 },
    Other,
}
