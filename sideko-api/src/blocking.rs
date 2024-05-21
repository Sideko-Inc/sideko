/// Generatedby Sideko (sideko.dev)
use crate::auth;
use crate::request_types::*;
use crate::result;
use crate::error_enums;
use crate::schemas::*;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::RequestBuilder as ReqwestRequestBuilder;
#[allow(unused)]
use reqwest::blocking::multipart as reqwest_multipart;
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub struct Client {
    pub base_url: String,
    auth: BTreeMap<String, auth::AuthProvider>,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: "https://api.sideko.dev".to_string(),
            auth: BTreeMap::new(),
        }
    }
}
impl Client {
    /// Override the default base url
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Authentication  builder function to store api-key credentials in the client
    pub fn with_api_key_auth(mut self, val: &str) -> Self {
        self.auth
            .insert(
                "ApiKeyAuth".to_string(),
                auth::AuthProvider::KeyHeader(
                    "x-sideko-key".to_string(),
                    val.to_string(),
                ),
            );
        self
    }
    fn builder_with_auth(
        &self,
        mut req_builder: ReqwestRequestBuilder,
        auth_names: &[&str],
    ) -> ReqwestRequestBuilder {
        for auth_name in auth_names {
            if let Some(provider) = self.auth.get(&auth_name.to_string()) {
                req_builder = provider.add_auth_blocking(req_builder);
            }
        }
        req_builder
    }
    pub fn list_api_projects(
        &self,
    ) -> result::Result<Vec<ApiProject>, error_enums::ListApiProjectsErrors> {
        let endpoint = "/v1/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiProject>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiProjectsErrors::Status401(data),
                })
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn list_api_versions(
        &self,
        request: ListApiVersionsRequest,
    ) -> result::Result<Vec<ApiVersion>, error_enums::ListApiVersionsErrors> {
        let endpoint = format!("/v1/api_project/{}/version", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiVersion>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "GET".to_string(),
                    url: url.to_string(),
                    data: error_enums::ListApiVersionsErrors::Status401(data),
                })
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn exchange_code_for_key(
        &self,
        request: ExchangeCodeForKeyRequest,
    ) -> result::Result<ApiKey, error_enums::ExchangeCodeForKeyErrors> {
        let endpoint = "/v1/auth/exchange_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("code", request.code.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiKey>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn cli_check_updates(
        &self,
        request: CliCheckUpdatesRequest,
    ) -> result::Result<Vec<CliUpdate>, error_enums::CliCheckUpdatesErrors> {
        let endpoint = format!("/v1/cli/updates/{}", request.cli_version);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<CliUpdate>>(&response_text)
                    .unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_api_project(
        &self,
        request: CreateApiProjectRequest,
    ) -> result::Result<ApiProject, error_enums::CreateApiProjectErrors> {
        let endpoint = "/v1/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiProject>(&response_text).unwrap();
                Ok(data)
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn create_api_version(
        &self,
        request: CreateApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::CreateApiVersionErrors> {
        let endpoint = format!("/v1/api_project/{}/version", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
                Ok(data)
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::CreateApiVersionErrors::Status401(data),
                })
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn stateless_generate_sdk(
        &self,
        request: StatelessGenerateSdkRequest,
    ) -> result::Result<BinaryResponse, error_enums::StatelessGenerateSdkErrors> {
        let endpoint = "/v1/stateless/generate_sdk";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            201 => {
                let res_bytes = response.bytes().map_err(result::Error::ResponseBytes)?;
                let data = BinaryResponse {
                    content: res_bytes,
                };
                Ok(data)
            }
            400 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::StatelessGenerateSdkErrors::Status400(data),
                })
            }
            401 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Error>(&response_text).unwrap();
                Err(result::Error::Response {
                    status_code,
                    method: "POST".to_string(),
                    url: url.to_string(),
                    data: error_enums::StatelessGenerateSdkErrors::Status401(data),
                })
            }
            _ => {
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
}
