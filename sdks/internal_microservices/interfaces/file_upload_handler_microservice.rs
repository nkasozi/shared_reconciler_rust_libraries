use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppError;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::responses::UploadFileChunkResponse;

#[automock]
#[async_trait]
pub trait FileChunksUploadHandlerMicroserviceClientInterface: Send + Sync {
    async fn upload_file_chunk(&self, request: &UploadFileChunkRequest) -> Result<UploadFileChunkResponse, AppError>;
}
