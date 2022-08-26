use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::file_upload_chunk::FileUploadChunk;

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct SplitFile {
    pub file_id: String,
    pub upload_request_id: String,
    pub file_chunk_seq_id_to_chunk_details_map: HashMap<i64, FileUploadChunk>,
}
