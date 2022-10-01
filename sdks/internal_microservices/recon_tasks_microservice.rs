use async_trait::async_trait;
use reqwest::{Error, Response, StatusCode};
use serde::Serialize;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};
use crate::internal::shared_reconciler_rust_libraries::models::view_models::recon_task_response_details::{FileResponseSummary, ReconTaskResponseDetails};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::constants::{APP_ID_HEADER_NAME, CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::interfaces::recon_tasks_microservice::ReconTasksMicroserviceClientInterface;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::{AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest};

pub struct ReconTasksMicroserviceClient {
    //the dapr server ip
    pub host: String,

    //the dapr component name
    pub recon_tasks_service_app_id: String,
}

#[async_trait]
impl ReconTasksMicroserviceClientInterface for ReconTasksMicroserviceClient {
    async fn create_recon_task(&self, request: &CreateReconTaskRequest) -> Result<ReconTaskResponseDetails, AppError> {
        //format body and url
        //http://localhost:3602/v1.0/invoke/checkout/method/checkout/100
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.host.clone();
        let url = format!("{host}/recon-tasks");

        let payload = Self::convert_to_json_string(request);

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

        return Self::parse_recon_task_response(http_response).await;
    }

    async fn attach_primary_file_to_task(
        &self,
        request: &AttachPrimaryFileRequest,
    ) -> Result<FileResponseSummary, AppError> {
        //format body and url
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.host.clone();
        let url = format!("{host}/recon-tasks/attach-primary-file");

        let payload = Self::convert_to_json_string(request);

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

        return Self::parse_attach_file_response(http_response).await;
    }

    async fn attach_comparison_file_to_task(
        &self,
        request: &AttachComparisonFileRequest,
    ) -> Result<FileResponseSummary, AppError> {
        //format body and url
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.host.clone();
        let url = format!("{host}/recon-tasks/attach-comparison-file");

        let payload = Self::convert_to_json_string(request);

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

        return Self::parse_attach_file_response(http_response).await;
    }

    async fn get_recon_task(&self, recon_task_id: &str) -> Result<ReconTaskResponseDetails, AppError> {
        //format body and url
        //http://localhost:3602/v1.0/invoke/checkout/method/checkout/100
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.host.clone();
        let url = format!("{host}/recon-tasks/{recon_task_id}");

        //create client
        let client = self.get_http_client().await?;

        //send request
        let http_response = client
            .get(url)
            .header(CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE)
            .header(APP_ID_HEADER_NAME, app_id)
            .send()
            .await;

        return Self::parse_recon_task_response(http_response).await;
    }
}

impl ReconTasksMicroserviceClient {
    async fn get_http_client(&self) -> Result<reqwest::Client, AppError> {
        return Ok(reqwest::Client::new());
    }

    async fn parse_attach_file_response(http_response: Result<Response, Error>) -> Result<FileResponseSummary, AppError> {
        match http_response {
            //success
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let service_response = resp.json::<FileResponseSummary>().await;

                    return match service_response {
                        Ok(file_info) => Ok(file_info),
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

    async fn parse_recon_task_response(http_response: Result<Response, Error>) -> Result<ReconTaskResponseDetails, AppError> {
        match http_response {
            //success
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let create_recon_task_response = resp.json::<ReconTaskResponseDetails>().await;

                    return match create_recon_task_response {
                        Ok(recon_task) => Ok(recon_task),
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

    fn convert_to_json_string<T>(request: &T) -> String where T: ?Sized + Serialize {
        serde_json::to_string(request).unwrap_or("".to_string())
    }
}
