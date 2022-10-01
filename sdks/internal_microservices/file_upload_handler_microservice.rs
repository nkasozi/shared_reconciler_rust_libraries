use async_trait::async_trait;
use reqwest::{Error, Response, StatusCode};

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::interfaces::file_upload_handler_microservice::FileChunksUploadHandlerMicroserviceClientInterface;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::responses::UploadFileChunkResponse;

use super::constants::{APP_ID_HEADER_NAME, CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE};

pub struct FileChunksUploadHandlerMicroserviceClient {
    //the dapr server ip
    pub host: String,

    //the dapr component name
    pub file_chunks_service_app_id: String,
}

#[async_trait]
impl FileChunksUploadHandlerMicroserviceClientInterface for FileChunksUploadHandlerMicroserviceClient {
    async fn upload_file_chunk(
        &self,
        request: &UploadFileChunkRequest,
    ) -> Result<UploadFileChunkResponse, AppError> {
        //format body and url
        let app_id = self.file_chunks_service_app_id.clone();
        let host = self.host.clone();
        let url = format!("{host}/upload-file-chunk");
        let payload = serde_json::to_string(&request).unwrap_or("".to_string());

        //create client
        let client = self.get_http_client().await?;

        //send request
        let http_response = client
            .post(url)
            .body(payload)
            .header(CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE)
            .header(APP_ID_HEADER_NAME, app_id)
            .send()
            .await;

        // //handle the bindings response
        return Self::parse_response(http_response).await;
    }
}

impl FileChunksUploadHandlerMicroserviceClient {
    async fn get_http_client(&self) -> Result<reqwest::Client, AppError> {
        return Ok(reqwest::Client::new());
    }

    async fn parse_response(http_response: Result<Response, Error>) -> Result<UploadFileChunkResponse, AppError> {
        match http_response {
            //success
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let upload_response = resp.json::<UploadFileChunkResponse>().await;

                    return match upload_response {
                        Ok(upload_chunk_response) => Ok(upload_chunk_response),
                        Err(e) => {
                            Err(AppError::new(
                                AppErrorKind::ResponseUnmarshalError,
                                e.to_string(),
                            ))
                        }
                    };
                }
                _ => Err(AppError::new(
                    AppErrorKind::ExternalServerError,
                    resp.text().await.unwrap_or("".to_string()),
                )),
            },
            //failure
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}
