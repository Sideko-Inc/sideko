use crate::schemas::*;
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteApiLinkRequest {
    pub link_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteApiLinkGroupRequest {
    pub group_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteApiProjectRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteApiProjectRoleRequest {
    pub project_id: String,
    pub user_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteDocProjectRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteDocProjectRoleRequest {
    pub project_id: String,
    pub user_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteGuideRequest {
    pub project_id: String,
    pub version_id: String,
    pub guide_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteGuideHrefRequest {
    pub project_id: String,
    pub version_id: String,
    pub guide_id: String,
    pub variant: GuideHrefVariantEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteServiceAccountRequest {
    pub service_account_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiLinksRequest {
    pub doc_version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiLinkRequest {
    pub link_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiLinkGroupsRequest {
    pub doc_version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiProjectRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiProjectMembersRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListApiVersionsRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionRequest {
    pub project_id: String,
    pub version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionOpenapiRequest {
    pub project_id: String,
    pub version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionStatsRequest {
    pub project_id: String,
    pub version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ExchangeCodeForKeyRequest {
    pub code: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct LoginCallbackRequest {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct LoginUrlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_port: Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliCheckUpdatesRequest {
    pub cli_version: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocProjectRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListDeploymentsRequest {
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeploymentTargetEnum>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDeploymentRequest {
    pub project_id: String,
    pub deployment_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListDocProjectMembersRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocProjectVersionRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocProjectThemeRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListDocVersionsRequest {
    pub project_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDocVersionRequest {
    pub project_id: String,
    pub version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListGuidesRequest {
    pub project_id: String,
    pub version_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetGuideRequest {
    pub project_id: String,
    pub version_id: String,
    pub guide_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetGuideContentRequest {
    pub project_id: String,
    pub version_id: String,
    pub guide_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetUserProjectRoleRequest {
    pub project_id: String,
    pub project_type: ProjectTypeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLinkRequest {
    pub link_id: String,
    pub data: UpdateApiLink,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLinkGroupRequest {
    pub group_id: String,
    pub data: UpdateApiLinkGroup,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiProjectRequest {
    pub project_id: String,
    pub data: UpdateApiProject,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiVersionRequest {
    pub project_id: String,
    pub version_id: String,
    pub data: UpdateApiVersion,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectRequest {
    pub project_id: String,
    pub data: UpdateDocProject,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateGuideRequest {
    pub project_id: String,
    pub version_id: String,
    pub guide_id: String,
    pub data: UpdateGuide,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiLinkRequest {
    pub data: NewApiLink,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderApiLinksRequest {
    pub data: ApiReorder,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiLinkGroupRequest {
    pub data: NewApiLinkGroup,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiProjectRequest {
    pub data: NewApiProject,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GrantApiProjectRoleRequest {
    pub project_id: String,
    pub data: NewProjectRole,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateApiVersionRequest {
    pub project_id: String,
    pub data: NewApiVersion,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateDocProjectRequest {
    pub data: NewDocProject,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct TriggerDeploymentRequest {
    pub project_id: String,
    pub data: NewDeployment,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GrantDocProjectRoleRequest {
    pub project_id: String,
    pub data: NewProjectRole,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateGuideRequest {
    pub project_id: String,
    pub version_id: String,
    pub data: NewGuide,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderGuidesRequest {
    pub project_id: String,
    pub version_id: String,
    pub data: Vec<ReorderGuide>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateOrganizationRequest {
    pub data: NewOrganization,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UploadAssetsRequest {
    pub data: AssetUpload,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StatelessGenerateSdkRequest {
    pub data: StatelessGenerateSdk,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct InviteUserRequest {
    pub data: Invite,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateServiceAccountRequest {
    pub data: CreateServiceAccount,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectThemeRequest {
    pub project_id: String,
    pub data: ThemeValues,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateOrganizationThemeRequest {
    pub data: ThemeValues,
}
