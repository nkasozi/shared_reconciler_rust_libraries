use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct FileChunkQueue {
    pub topic_id: String,
    pub last_acknowledged_id: Option<String>,
}
