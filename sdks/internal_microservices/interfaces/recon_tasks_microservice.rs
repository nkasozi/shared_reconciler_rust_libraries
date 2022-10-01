use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppError;
use crate::internal::shared_reconciler_rust_libraries::models::view_models::recon_task_response_details::{FileResponseSummary, ReconTaskResponseDetails};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::{AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest};

#[automock]
#[async_trait]
pub trait ReconTasksMicroserviceClientInterface: Send + Sync {
    async fn create_recon_task(&self, request: &CreateReconTaskRequest) -> Result<ReconTaskResponseDetails, AppError>;

    async fn attach_primary_file_to_task(&self, request: &AttachPrimaryFileRequest)
                                         -> Result<FileResponseSummary, AppError>;

    async fn attach_comparison_file_to_task(
        &self,
        request: &AttachComparisonFileRequest,
    ) -> Result<FileResponseSummary, AppError>;

    async fn get_recon_task(&self, recon_task_id: &str) -> Result<ReconTaskResponseDetails, AppError>;
}
