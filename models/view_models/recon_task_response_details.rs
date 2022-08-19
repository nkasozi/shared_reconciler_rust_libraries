use serde::{Deserialize, Serialize};

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    file_chunk_queue::FileChunkQueue,
    recon_tasks_models::{ReconFileMetaData, ReconTaskDetails},
};

#[derive(PartialEq, Serialize, Clone, Deserialize, Debug)]
pub struct ReconTaskResponseDetails {
    pub task_id: String,
    pub task_details: ReconTaskDetails,
    pub source_file_metadata: ReconFileMetaData,
    pub comparison_file_metadata: ReconFileMetaData,
    pub results_queue_info: FileChunkQueue,
}
