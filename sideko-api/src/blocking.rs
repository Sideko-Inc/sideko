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
            base_url: "http://server-not-specified".to_string(),
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
    pub fn exchange_code_for_key(
        &self,
        request: ExchangeCodeForKeyRequest,
    ) -> result::Result<ApiKey, error_enums::ExchangeCodeForKeyErrors> {
        let endpoint = "/api/auth/exchange_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("code", format!("{}", & request.code)));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiKey>(&response_text)
                    .map_err(|serde_err| result::Error::UnexpectedResponseBody {
                        status_code,
                        method: "GET".to_string(),
                        url: url.to_string(),
                        response_text,
                        expected_signature: "ApiKey".to_string(),
                        serde_err,
                    })?;
                Ok(data)
            }
            _ => {
                let expected_status_codes: Vec<String> = vec!["200".to_string(),];
                Err(result::Error::BlockingUnexpectedStatus {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                    expected_status_codes,
                })
            }
        }
    }
    pub fn cli_check_updates(
        &self,
        request: CliCheckUpdatesRequest,
    ) -> result::Result<Vec<CliUpdate>, error_enums::CliCheckUpdatesErrors> {
        let endpoint = format!("/api/cli/updates/{}", request.cli_version);
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
                    .map_err(|serde_err| result::Error::UnexpectedResponseBody {
                        status_code,
                        method: "GET".to_string(),
                        url: url.to_string(),
                        response_text,
                        expected_signature: "Vec<CliUpdate>".to_string(),
                        serde_err,
                    })?;
                Ok(data)
            }
            _ => {
                let expected_status_codes: Vec<String> = vec!["200".to_string(),];
                Err(result::Error::BlockingUnexpectedStatus {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                    expected_status_codes,
                })
            }
        }
    }
    pub fn stateless_generate_sdk(
        &self,
        request: StatelessGenerateSdkRequest,
    ) -> result::Result<BinaryResponse, error_enums::StatelessGenerateSdkErrors> {
        let endpoint = "/api/stateless/generate_sdk";
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
            _ => {
                let expected_status_codes: Vec<String> = vec!["201".to_string(),];
                Err(result::Error::BlockingUnexpectedStatus {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                    expected_status_codes,
                })
            }
        }
    }
}
