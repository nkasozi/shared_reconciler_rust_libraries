use serde::{Deserialize, Serialize};

use super::{
    file_chunk_queue::FileChunkQueue,
    recon_tasks_models::{ComparisonPair, ReconciliationConfigs},
};

//represents a group of lines inside a file
#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct FileUploadChunk {
    pub id: String,
    pub upload_request_id: String,
    pub chunk_sequence_number: i64,
    pub chunk_source: FileUploadChunkSource,
    pub chunk_rows: Vec<FileUploadChunkRow>,
    pub date_created: i64,
    pub date_modified: i64,
    pub comparison_pairs: Vec<ComparisonPair>,
    pub column_headers: Vec<String>,
    pub recon_config: ReconciliationConfigs,
    pub primary_file_queue: FileChunkQueue,
    pub comparison_file_queue: FileChunkQueue,
    pub results_queue: FileChunkQueue,
}

impl FileUploadChunk {
    #[allow(dead_code)]
    pub fn get_row_identifier_comparison_pairs(&self) -> Vec<ComparisonPair> {
        return self
            .comparison_pairs
            .clone()
            .into_iter()
            .filter(|pair| pair.is_row_identifier)
            .collect();
    }

    #[allow(dead_code)]
    pub fn get_comparison_pairs_that_are_not_row_identifiers(&self) -> Vec<ComparisonPair> {
        return self
            .comparison_pairs
            .clone()
            .into_iter()
            .filter(|pair| !pair.is_row_identifier)
            .collect();
    }
}

//represents a line in a file
#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct FileUploadChunkRow {
    pub row_number: u64,
    pub raw_data: String,
    pub parsed_columns_from_row: Vec<String>,
    pub recon_result: ReconStatus,
    pub recon_result_reasons: Vec<String>,
}

//represents the source of a file chunk
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ReconStatus {
    Failed,
    Successful,
    Pending,
}

//represents the source of a file chunk
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FileUploadChunkSource {
    ComparisonFileChunk,
    PrimaryFileChunk,
}
