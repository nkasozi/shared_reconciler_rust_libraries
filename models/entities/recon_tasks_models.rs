use serde::{Deserialize, Serialize};

use super::file_chunk_queue::FileChunkQueue;

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ReconTaskDetails {
    pub id: String,
    pub primary_file_id: Option<String>,
    pub comparison_file_id: Option<String>,
    pub is_done: bool,
    pub has_begun: bool,
    pub comparison_pairs: Vec<ComparisonPair>,
    pub recon_config: ReconciliationConfigs,
    pub recon_results_queue_info: FileChunkQueue,
    pub primary_file_chunks_queue_info: FileChunkQueue,
    pub comparison_file_chunks_queue_info: FileChunkQueue,
}

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ReconFileMetaData {
    pub id: String,
    pub file_name: String,
    pub row_count: u64,
    pub column_delimiters: Vec<char>,
    pub recon_file_type: ReconFileType,
    pub column_headers: Vec<String>,
    pub file_hash: String,
}

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ComparisonPair {
    pub primary_file_column_index: usize,
    pub comparison_file_column_index: usize,
    pub is_row_identifier: bool,
}

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct ReconciliationConfigs {
    pub should_check_for_duplicate_records_in_comparison_file: bool,
    pub should_reconciliation_be_case_sensitive: bool,
    pub should_ignore_white_space: bool,
    pub should_do_reverse_reconciliation: bool,
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub enum ReconFileType {
    PrimaryFile,
    ComparisonFile,
}

impl Default for ReconFileType {
    fn default() -> Self {
        ReconFileType::PrimaryFile
    }
}
