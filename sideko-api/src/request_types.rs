use crate::schemas::*;
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiVersionsRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ExchangeCodeForKeyRequest {
    pub code: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliCheckUpdatesRequest {
    pub cli_version: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiProjectRequest {
    pub data: NewApiProject,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiVersionRequest {
    pub project_id: String,
    pub data: NewApiVersion,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StatelessGenerateSdkRequest {
    pub data: StatelessGenerateSdk,
}
