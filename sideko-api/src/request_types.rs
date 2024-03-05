use crate::schemas::*;
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliCheckUpdatesRequest {
    pub cli_version: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StatelessGenerateSdkRequest {
    pub data: StatelessGenerateSdk,
}
