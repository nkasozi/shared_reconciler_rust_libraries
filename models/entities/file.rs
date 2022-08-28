use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::ReconFileType;
use serde::{Deserialize, Serialize};

use super::file_row::FileRow;

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct File {
    pub id: String,
    pub upload_request_id: String,
    pub file_storage_location: FileStorageLocation,
    pub file_extension: SupportedFileExtension,
    pub file_path: Option<String>,
    pub file_type: ReconFileType,
}

pub struct FileThatHasBeenRead {
    pub id: String,
    pub upload_request_id: String,
    pub file_type: ReconFileType,
    pub column_headers: Vec<String>,
    pub file_rows: Vec<FileRow>,
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub enum SupportedFileExtension {
    Csv,
    Excel,
    Pdf,
}

impl Default for SupportedFileExtension {
    fn default() -> Self {
        SupportedFileExtension::Csv
    }
}

#[derive(Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub enum FileStorageLocation {
    LocalFileSystem,
    S3FileSystem,
}

impl Default for FileStorageLocation {
    fn default() -> Self {
        FileStorageLocation::LocalFileSystem
    }
}
