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
            base_url: "https://api.sideko.dev/v1".to_string(),
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
    /// Authentication  builder function to store api-key credentials in the client
    pub fn with_cookie_auth(mut self, val: &str) -> Self {
        self.auth
            .insert(
                "CookieAuth".to_string(),
                auth::AuthProvider::KeyCookie("sideko_jwt".to_string(), val.to_string()),
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
    pub fn delete_api_link(
        &self,
        request: DeleteApiLinkRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiLinkErrors> {
        let endpoint = format!("/api_link/{}", request.link_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_api_link_group(
        &self,
        request: DeleteApiLinkGroupRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiLinkGroupErrors> {
        let endpoint = format!("/api_link_group/{}", request.group_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_api_project(
        &self,
        request: DeleteApiProjectRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_api_project_role(
        &self,
        request: DeleteApiProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteApiProjectRoleErrors> {
        let endpoint = format!(
            "/api_project/{}/role/{}", request.project_id, request.user_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_doc_project(
        &self,
        request: DeleteDocProjectRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_doc_project_role(
        &self,
        request: DeleteDocProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteDocProjectRoleErrors> {
        let endpoint = format!(
            "/doc_project/{}/role/{}", request.project_id, request.user_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_guide(
        &self,
        request: DeleteGuideRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}", request.project_id, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_guide_href(
        &self,
        request: DeleteGuideHrefRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteGuideHrefErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}/href", request.project_id, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("variant", request.variant.to_string()));
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn delete_service_account(
        &self,
        request: DeleteServiceAccountRequest,
    ) -> result::Result<serde_json::Value, error_enums::DeleteServiceAccountErrors> {
        let endpoint = format!("/user/service_account/{}", request.service_account_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default()
            .delete(&url)
            .query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            204 => {
                let response_text = response.text().unwrap_or_default();
                if response_text.is_empty() {
                    return Ok(serde_json::json!({}));
                }
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn list_api_links(
        &self,
        request: ListApiLinksRequest,
    ) -> result::Result<Vec<ApiLink>, error_enums::ListApiLinksErrors> {
        let endpoint = "/api_link";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("doc_version_id", request.doc_version_id.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiLink>>(&response_text).unwrap();
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
    pub fn get_api_link(
        &self,
        request: GetApiLinkRequest,
    ) -> result::Result<ApiLink, error_enums::GetApiLinkErrors> {
        let endpoint = format!("/api_link/{}", request.link_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
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
    pub fn list_api_link_groups(
        &self,
        request: ListApiLinkGroupsRequest,
    ) -> result::Result<Vec<ApiLinkGroup>, error_enums::ListApiLinkGroupsErrors> {
        let endpoint = "/api_link_group";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("doc_version_id", request.doc_version_id.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiLinkGroup>>(&response_text)
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
    pub fn list_api_projects(
        &self,
    ) -> result::Result<Vec<ApiProject>, error_enums::ListApiProjectsErrors> {
        let endpoint = "/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiProject>>(&response_text)
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
    pub fn get_api_project(
        &self,
        request: GetApiProjectRequest,
    ) -> result::Result<ApiProject, error_enums::GetApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
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
    pub fn list_api_project_members(
        &self,
        request: ListApiProjectMembersRequest,
    ) -> result::Result<Vec<ProjectMember>, error_enums::ListApiProjectMembersErrors> {
        let endpoint = format!("/api_project/{}/members", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ProjectMember>>(&response_text)
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
    pub fn list_api_versions(
        &self,
        request: ListApiVersionsRequest,
    ) -> result::Result<Vec<ApiVersion>, error_enums::ListApiVersionsErrors> {
        let endpoint = format!("/api_project/{}/version", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ApiVersion>>(&response_text)
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
    pub fn get_api_version(
        &self,
        request: GetApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::GetApiVersionErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
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
    pub fn get_api_version_openapi(
        &self,
        request: GetApiVersionOpenapiRequest,
    ) -> result::Result<OpenApi, error_enums::GetApiVersionOpenapiErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}/openapi", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<OpenApi>(&response_text).unwrap();
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
    pub fn get_api_version_stats(
        &self,
        request: GetApiVersionStatsRequest,
    ) -> result::Result<Stats, error_enums::GetApiVersionStatsErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}/stats", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Stats>(&response_text).unwrap();
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
    pub fn exchange_code_for_key(
        &self,
        request: ExchangeCodeForKeyRequest,
    ) -> result::Result<UserApiKey, error_enums::ExchangeCodeForKeyErrors> {
        let endpoint = "/auth/exchange_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("code", request.code.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
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
    pub fn login_callback(
        &self,
        request: LoginCallbackRequest,
    ) -> result::Result<serde_json::Value, error_enums::LoginCallbackErrors> {
        let endpoint = "/auth/login_callback";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("code", request.code.to_string()));
        if let Some(state) = request.state {
            query_params.push(("state", state.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            303 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
                    data: error_enums::LoginCallbackErrors::Status401(data),
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
    pub fn login_url(
        &self,
        request: LoginUrlRequest,
    ) -> result::Result<serde_json::Value, error_enums::LoginUrlErrors> {
        let endpoint = "/auth/login_url";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(cli_output) = request.cli_output {
            query_params.push(("cli_output", cli_output.to_string()));
        }
        if let Some(cli_port) = request.cli_port {
            query_params.push(("cli_port", cli_port.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            303 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn cli_check_updates(
        &self,
        request: CliCheckUpdatesRequest,
    ) -> result::Result<Vec<CliUpdate>, error_enums::CliCheckUpdatesErrors> {
        let endpoint = format!("/cli/updates/{}", request.cli_version);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
    pub fn list_doc_projects(
        &self,
    ) -> result::Result<Vec<DocProject>, error_enums::ListDocProjectsErrors> {
        let endpoint = "/doc_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<DocProject>>(&response_text)
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
    pub fn get_doc_project(
        &self,
        request: GetDocProjectRequest,
    ) -> result::Result<DocProject, error_enums::GetDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
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
    /// Retrieves all deployments for a doc project
    pub fn list_deployments(
        &self,
        request: ListDeploymentsRequest,
    ) -> result::Result<Vec<Deployment>, error_enums::ListDeploymentsErrors> {
        let endpoint = format!("/doc_project/{}/deployment", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(limit) = request.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(target) = request.target {
            query_params.push(("target", target.to_string()));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Deployment>>(&response_text)
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
    /// Retrieves single deployment
    pub fn get_deployment(
        &self,
        request: GetDeploymentRequest,
    ) -> result::Result<Deployment, error_enums::GetDeploymentErrors> {
        let endpoint = format!(
            "/doc_project/{}/deployment/{}", request.project_id, request.deployment_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Deployment>(&response_text).unwrap();
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
    pub fn list_doc_project_members(
        &self,
        request: ListDocProjectMembersRequest,
    ) -> result::Result<Vec<ProjectMember>, error_enums::ListDocProjectMembersErrors> {
        let endpoint = format!("/doc_project/{}/members", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ProjectMember>>(&response_text)
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
    pub fn get_doc_project_version(
        &self,
        request: GetDocProjectVersionRequest,
    ) -> result::Result<bool, error_enums::GetDocProjectVersionErrors> {
        let endpoint = format!("/doc_project/{}/preview", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<bool>(&response_text).unwrap();
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
    /// Get the theme attached to a documentation project
    pub fn get_doc_project_theme(
        &self,
        request: GetDocProjectThemeRequest,
    ) -> result::Result<Theme, error_enums::GetDocProjectThemeErrors> {
        let endpoint = format!("/doc_project/{}/theme", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
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
    pub fn list_doc_versions(
        &self,
        request: ListDocVersionsRequest,
    ) -> result::Result<Vec<DocVersion>, error_enums::ListDocVersionsErrors> {
        let endpoint = format!("/doc_project/{}/version", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<DocVersion>>(&response_text)
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
    pub fn get_doc_version(
        &self,
        request: GetDocVersionRequest,
    ) -> result::Result<DocVersion, error_enums::GetDocVersionErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocVersion>(&response_text).unwrap();
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
    pub fn list_guides(
        &self,
        request: ListGuidesRequest,
    ) -> result::Result<Vec<GuideWithChildren>, error_enums::ListGuidesErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<GuideWithChildren>>(&response_text)
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
    pub fn get_guide(
        &self,
        request: GetGuideRequest,
    ) -> result::Result<Guide, error_enums::GetGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}", request.project_id, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
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
    pub fn get_guide_content(
        &self,
        request: GetGuideContentRequest,
    ) -> result::Result<GuideContent, error_enums::GetGuideContentErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}/content", request.project_id, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GuideContent>(&response_text).unwrap();
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
    /// Get user organization
    pub fn get_organization(
        &self,
    ) -> result::Result<Organization, error_enums::GetOrganizationErrors> {
        let endpoint = "/organization";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Organization>(&response_text).unwrap();
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
    /// Get all assets for an organization
    pub fn get_assets(
        &self,
    ) -> result::Result<Vec<Asset>, error_enums::GetAssetsErrors> {
        let endpoint = "/organization/asset";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Asset>>(&response_text).unwrap();
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
    /// Get users in the organization
    pub fn list_organization_members(
        &self,
    ) -> result::Result<
        Vec<OrganizationMember>,
        error_enums::ListOrganizationMembersErrors,
    > {
        let endpoint = "/organization/members";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<
                    Vec<OrganizationMember>,
                >(&response_text)
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
    /// Get documentation project theme configured at the organization level
    pub fn get_organization_theme(
        &self,
    ) -> result::Result<Theme, error_enums::GetOrganizationThemeErrors> {
        let endpoint = "/organization/theme";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
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
    pub fn get_current_user(
        &self,
    ) -> result::Result<User, error_enums::GetCurrentUserErrors> {
        let endpoint = "/user/me";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<User>(&response_text).unwrap();
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
    pub fn get_api_key(
        &self,
    ) -> result::Result<UserApiKey, error_enums::GetApiKeyErrors> {
        let endpoint = "/user/me/api_key";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
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
    /// retrieve current user role for a given project type/id
    pub fn get_user_project_role(
        &self,
        request: GetUserProjectRoleRequest,
    ) -> result::Result<UserProjectRole, error_enums::GetUserProjectRoleErrors> {
        let endpoint = "/user/me/project_role";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        query_params.push(("project_id", request.project_id.to_string()));
        query_params.push(("project_type", request.project_type.to_string()));
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserProjectRole>(&response_text)
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
    pub fn get_service_accounts(
        &self,
    ) -> result::Result<Vec<ServiceAccount>, error_enums::GetServiceAccountsErrors> {
        let endpoint = "/user/service_account";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<ServiceAccount>>(&response_text)
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
    pub fn update_api_link(
        &self,
        request: UpdateApiLinkRequest,
    ) -> result::Result<ApiLink, error_enums::UpdateApiLinkErrors> {
        let endpoint = format!("/api_link/{}", request.link_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
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
    pub fn update_api_link_group(
        &self,
        request: UpdateApiLinkGroupRequest,
    ) -> result::Result<ApiLinkGroup, error_enums::UpdateApiLinkGroupErrors> {
        let endpoint = format!("/api_link_group/{}", request.group_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiLinkGroup>(&response_text).unwrap();
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
    pub fn update_api_project(
        &self,
        request: UpdateApiProjectRequest,
    ) -> result::Result<ApiProject, error_enums::UpdateApiProjectErrors> {
        let endpoint = format!("/api_project/{}", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
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
    pub fn update_api_version(
        &self,
        request: UpdateApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::UpdateApiVersionErrors> {
        let endpoint = format!(
            "/api_project/{}/version/{}", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiVersion>(&response_text).unwrap();
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
    pub fn update_doc_project(
        &self,
        request: UpdateDocProjectRequest,
    ) -> result::Result<DocProject, error_enums::UpdateDocProjectErrors> {
        let endpoint = format!("/doc_project/{}", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
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
    pub fn update_guide(
        &self,
        request: UpdateGuideRequest,
    ) -> result::Result<Guide, error_enums::UpdateGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/{}", request.project_id, request
            .version_id, request.guide_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().patch(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
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
    pub fn create_api_link(
        &self,
        request: CreateApiLinkRequest,
    ) -> result::Result<ApiLink, error_enums::CreateApiLinkErrors> {
        let endpoint = "/api_link";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<ApiLink>(&response_text).unwrap();
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
    pub fn reorder_api_links(
        &self,
        request: ReorderApiLinksRequest,
    ) -> result::Result<ApiReorder, error_enums::ReorderApiLinksErrors> {
        let endpoint = "/api_link/reorder";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<ApiReorder>(&response_text).unwrap();
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
    pub fn create_api_link_group(
        &self,
        request: CreateApiLinkGroupRequest,
    ) -> result::Result<ApiLinkGroup, error_enums::CreateApiLinkGroupErrors> {
        let endpoint = "/api_link_group";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<ApiLinkGroup>(&response_text).unwrap();
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
        let endpoint = "/api_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
    pub fn grant_api_project_role(
        &self,
        request: GrantApiProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::GrantApiProjectRoleErrors> {
        let endpoint = format!("/api_project/{}/role", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn create_api_version(
        &self,
        request: CreateApiVersionRequest,
    ) -> result::Result<ApiVersion, error_enums::CreateApiVersionErrors> {
        let endpoint = format!("/api_project/{}/version", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
    pub fn create_doc_project(
        &self,
        request: CreateDocProjectRequest,
    ) -> result::Result<DocProject, error_enums::CreateDocProjectErrors> {
        let endpoint = "/doc_project";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<DocProject>(&response_text).unwrap();
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
    /// Deploys a new generated version of documentation with linked guides & APIs
    pub fn trigger_deployment(
        &self,
        request: TriggerDeploymentRequest,
    ) -> result::Result<Deployment, error_enums::TriggerDeploymentErrors> {
        let endpoint = format!("/doc_project/{}/deployment", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<Deployment>(&response_text).unwrap();
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
    pub fn grant_doc_project_role(
        &self,
        request: GrantDocProjectRoleRequest,
    ) -> result::Result<serde_json::Value, error_enums::GrantDocProjectRoleErrors> {
        let endpoint = format!("/doc_project/{}/role", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn create_guide(
        &self,
        request: CreateGuideRequest,
    ) -> result::Result<Guide, error_enums::CreateGuideErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide", request.project_id, request.version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<Guide>(&response_text).unwrap();
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
    pub fn reorder_guides(
        &self,
        request: ReorderGuidesRequest,
    ) -> result::Result<Vec<GuideWithChildren>, error_enums::ReorderGuidesErrors> {
        let endpoint = format!(
            "/doc_project/{}/version/{}/guide/reorder", request.project_id, request
            .version_id
        );
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<GuideWithChildren>>(&response_text)
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
    pub fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> result::Result<
        OrganizationWithRedirect,
        error_enums::CreateOrganizationErrors,
    > {
        let endpoint = "/organization";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<
                    OrganizationWithRedirect,
                >(&response_text)
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
    /// Add a assets like logos to an organization
    pub fn upload_assets(
        &self,
        request: UploadAssetsRequest,
    ) -> result::Result<Vec<Asset>, error_enums::UploadAssetsErrors> {
        let endpoint = "/organization/asset";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let mut form_data_body = reqwest_multipart::Form::new();
        form_data_body = form_data_body
            .part(
                "file",
                reqwest_multipart::Part::file(&request.data.file)
                    .map_err(result::Error::File)?,
            );
        let response = authed_builder
            .multipart(form_data_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Vec<Asset>>(&response_text).unwrap();
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
    pub fn stateless_generate_sdk(
        &self,
        request: StatelessGenerateSdkRequest,
    ) -> result::Result<BinaryResponse, error_enums::StatelessGenerateSdkErrors> {
        let endpoint = "/stateless/generate_sdk";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                Err(result::Error::BlockingApiError {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                })
            }
        }
    }
    pub fn invite_user(
        &self,
        request: InviteUserRequest,
    ) -> result::Result<serde_json::Value, error_enums::InviteUserErrors> {
        let endpoint = "/user/invite";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
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
                let data = serde_json::from_str::<serde_json::Value>(&response_text)
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
    pub fn create_service_account(
        &self,
        request: CreateServiceAccountRequest,
    ) -> result::Result<UserApiKey, error_enums::CreateServiceAccountErrors> {
        let endpoint = "/user/service_account";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().post(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<UserApiKey>(&response_text).unwrap();
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
    /// Update a document project theme
    pub fn update_doc_project_theme(
        &self,
        request: UpdateDocProjectThemeRequest,
    ) -> result::Result<Theme, error_enums::UpdateDocProjectThemeErrors> {
        let endpoint = format!("/doc_project/{}/theme", request.project_id);
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().put(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
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
    /// Update  documentation project theme configured at the organization level
    pub fn update_organization_theme(
        &self,
        request: UpdateOrganizationThemeRequest,
    ) -> result::Result<Theme, error_enums::UpdateOrganizationThemeErrors> {
        let endpoint = "/organization/theme";
        let url = format!("{}{}", self.base_url, endpoint);
        let query_params: Vec<(&str, String)> = vec![];
        let unauthed_builder = ReqwestClient::default().put(&url).query(&query_params);
        let authed_builder = self
            .builder_with_auth(unauthed_builder, &["CookieAuth", "ApiKeyAuth"]);
        let request_body: serde_json::Value = serde_json::to_value(request.data)
            .map_err(result::Error::Serialize)?;
        let response = authed_builder
            .json(&request_body)
            .send()
            .map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<Theme>(&response_text).unwrap();
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
}
