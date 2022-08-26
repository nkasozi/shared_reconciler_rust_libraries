use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::ReconFileType;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct File {
    pub id: String,
    pub upload_request_id: String,
    pub file_path: Option<String>,
    pub file_type: ReconFileType,
}
