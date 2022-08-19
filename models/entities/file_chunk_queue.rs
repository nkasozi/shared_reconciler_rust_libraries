use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct FileChunkQueue {
    pub queue_id: String,
    pub last_acknowledged_id: Option<String>,
}
