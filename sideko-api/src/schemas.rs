pub struct BinaryResponse {
    pub content: bytes::Bytes,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkApiVersion {
    pub api_project_id: String,
    pub api_project_title: String,
    pub id: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkDocVersion {
    pub doc_project_id: String,
    pub doc_project_title: String,
    pub id: String,
    pub version: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkGroup {
    pub doc_version_id: String,
    pub id: String,
    pub nav_label: String,
    pub order: i64,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiProject {
    pub created_at: String,
    pub id: String,
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ProjectMember {
    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub role: ProjectRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiMockServer {
    pub enabled: bool,
    pub url: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Validation {
    pub message: String,
    pub severity: ValidationSeverityEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Stats {
    pub authenticated_methods: f64,
    pub authentication_schemes: Vec<String>,
    pub endpoints: f64,
    pub methods: f64,
    pub public_methods: f64,
    pub response_codes: Vec<f64>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UserApiKey {
    pub api_key: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Error {
    pub description: String,
    pub error: ErrorCodeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CliUpdate {
    pub message: String,
    pub severity: CliUpdateSeverityEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectDomains {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Asset {
    pub extension: String,
    pub id: String,
    pub name: String,
    pub url: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectActionButton {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocVersion {
    pub created_at: String,
    pub doc_project_id: String,
    pub id: String,
    pub status: DocVersionStatusEnum,
    pub version: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ThemeValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_reference_group_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_active_button_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_active_button_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_navbar_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_navbar_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_active_button_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_active_button_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_navbar_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_navbar_text_color: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideWithChildren {
    pub children: Vec<Box<GuideWithChildren>>,
    pub created_at: String,
    pub id: String,
    pub is_parent: bool,
    pub nav_label: String,
    pub order: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideHref {
    pub id: String,
    pub nav_label: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GuideContent {
    pub content: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationFeatures {
    pub max_api_projects: i64,
    pub max_doc_projects: i64,
    pub max_mock_servers: i64,
    pub max_teamates: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationMember {
    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub role: OrganizationRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct User {
    pub avatar_url: String,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub organization_role: OrganizationRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UserProjectRole {
    pub project_id: String,
    pub project_type: ProjectTypeEnum,
    pub role: ProjectRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ServiceAccount {
    pub api_key: String,
    pub created_at: String,
    pub id: String,
    pub name: String,
    pub project_roles: Vec<UserProjectRole>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ApiLinkPolicyEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiLinkGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateApiVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock_server_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semver: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectLogos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettingsActionButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettingsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateGuide {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct LatestApiLinkPolicy {
    pub api_project_id: String,
    #[serde(rename = "type")]
    pub type_field: LatestApiLinkPolicyTypeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PinnedApiLinkPolicy {
    pub api_version_id: String,
    #[serde(rename = "type")]
    pub type_field: PinnedApiLinkPolicyTypeEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkGroupReorder {
    pub id: String,
    pub order: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLinkReorder {
    pub group_id: String,
    pub id: String,
    pub order: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiLinkGroup {
    pub doc_version_id: String,
    pub nav_label: String,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiProject {
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewProjectRole {
    pub role: ProjectRoleEnum,
    pub user_id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock_server_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub openapi: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewDocProject {
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewDeployment {
    pub doc_version_id: String,
    pub target: DeploymentTargetEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewGuide {
    pub content: String,
    pub is_parent: bool,
    pub nav_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_id: Option<String>,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ReorderGuide {
    pub id: String,
    pub order: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewOrganization {
    pub name: String,
    pub subdomain: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct AssetUpload {
    pub file: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StatelessGenerateSdk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub language: GenerationLanguageEnum,
    pub openapi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests_mock_server_url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Invite {
    pub email: String,
    pub role: OrganizationRoleEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateServiceAccount {
    pub name: String,
    pub project_roles: Vec<UserProjectRole>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiLink {
    pub api_version: ApiLinkApiVersion,
    pub created_at: String,
    pub doc_version: ApiLinkDocVersion,
    pub group_id: String,
    pub id: String,
    pub nav_label: String,
    pub order: i64,
    pub policy: ApiLinkPolicyEnum,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiVersion {
    pub api_project_id: String,
    pub created_at: String,
    pub id: String,
    pub mock_server: ApiMockServer,
    pub notes: String,
    pub semver: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OpenApi {
    pub is_config_valid: bool,
    pub is_valid: bool,
    pub openapi: String,
    pub validations: Vec<Validation>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectLogos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<Asset>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProjectSettings {
    pub action_button: DocProjectActionButton,
    pub metadata: DocProjectMetadata,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Deployment {
    pub created_at: String,
    pub current_preview: bool,
    pub current_prod: bool,
    pub doc_version: DocVersion,
    pub id: String,
    pub status: DeploymentStatusEnum,
    pub target: DeploymentTargetEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Theme {
    pub owner: ThemeOwnerEnum,
    pub values: ThemeValues,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Guide {
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_next_href: Option<GuideHref>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prev_href: Option<GuideHref>,
    pub id: String,
    pub is_parent: bool,
    pub nav_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<GuideHref>,
    pub order: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_href: Option<GuideHref>,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Organization {
    pub features: OrganizationFeatures,
    pub id: String,
    pub name: String,
    pub subdomain: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProjectSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<UpdateDocProjectSettingsActionButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<UpdateDocProjectSettingsMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct NewApiLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_id: Option<String>,
    pub doc_version_id: String,
    pub group_id: String,
    pub nav_label: String,
    pub policy: Union,
    pub slug: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ApiReorder {
    pub doc_version_id: String,
    pub groups: Vec<ApiLinkGroupReorder>,
    pub links: Vec<ApiLinkReorder>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrganizationWithRedirect {
    pub organization: Organization,
    pub redirect_to: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DocProject {
    pub created_at: String,
    pub current_version_id: String,
    pub domains: DocProjectDomains,
    pub id: String,
    pub logos: DocProjectLogos,
    pub settings: DocProjectSettings,
    pub title: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct UpdateDocProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logos: Option<UpdateDocProjectLogos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateDocProjectSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Union {
    LatestApiLinkPolicy(LatestApiLinkPolicy),
    PinnedApiLinkPolicy(PinnedApiLinkPolicy),
}
impl Default for Union {
    fn default() -> Self {
        Union::PinnedApiLinkPolicy(Default::default())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GuideHrefVariantEnum {
    #[default]
    #[serde(rename = "prev")]
    Prev,
    #[serde(rename = "next")]
    Next,
}
impl std::fmt::Display for GuideHrefVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GuideHrefVariantEnum::Prev => "prev",
            GuideHrefVariantEnum::Next => "next",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ApiLinkPolicyEnum {
    #[default]
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pinned")]
    Pinned,
}
impl std::fmt::Display for ApiLinkPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ApiLinkPolicyEnum::Latest => "latest",
            ApiLinkPolicyEnum::Pinned => "pinned",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ProjectRoleEnum {
    #[default]
    #[serde(rename = "viewer")]
    Viewer,
    #[serde(rename = "contributor")]
    Contributor,
    #[serde(rename = "admin")]
    Admin,
}
impl std::fmt::Display for ProjectRoleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ProjectRoleEnum::Viewer => "viewer",
            ProjectRoleEnum::Contributor => "contributor",
            ProjectRoleEnum::Admin => "admin",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ValidationSeverityEnum {
    #[default]
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl std::fmt::Display for ValidationSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ValidationSeverityEnum::Info => "info",
            ValidationSeverityEnum::Warning => "warning",
            ValidationSeverityEnum::Error => "error",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ErrorCodeEnum {
    #[default]
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "internal_server_error")]
    InternalServerError,
    #[serde(rename = "Bad Request")]
    BadRequest,
    #[serde(rename = "unavailable_subdomain")]
    UnavailableSubdomain,
    #[serde(rename = "invalid_openapi")]
    InvalidOpenapi,
    #[serde(rename = "invalid_url")]
    InvalidUrl,
}
impl std::fmt::Display for ErrorCodeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ErrorCodeEnum::Forbidden => "forbidden",
            ErrorCodeEnum::Unauthorized => "unauthorized",
            ErrorCodeEnum::NotFound => "not_found",
            ErrorCodeEnum::InternalServerError => "internal_server_error",
            ErrorCodeEnum::BadRequest => "Bad Request",
            ErrorCodeEnum::UnavailableSubdomain => "unavailable_subdomain",
            ErrorCodeEnum::InvalidOpenapi => "invalid_openapi",
            ErrorCodeEnum::InvalidUrl => "invalid_url",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum CliUpdateSeverityEnum {
    #[default]
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "suggested")]
    Suggested,
    #[serde(rename = "required")]
    Required,
}
impl std::fmt::Display for CliUpdateSeverityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            CliUpdateSeverityEnum::Info => "info",
            CliUpdateSeverityEnum::Suggested => "suggested",
            CliUpdateSeverityEnum::Required => "required",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum DeploymentTargetEnum {
    #[default]
    #[serde(rename = "Production")]
    Production,
    #[serde(rename = "Preview")]
    Preview,
}
impl std::fmt::Display for DeploymentTargetEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DeploymentTargetEnum::Production => "Production",
            DeploymentTargetEnum::Preview => "Preview",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum DocVersionStatusEnum {
    #[default]
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Publishing")]
    Publishing,
    #[serde(rename = "Published")]
    Published,
}
impl std::fmt::Display for DocVersionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DocVersionStatusEnum::Draft => "Draft",
            DocVersionStatusEnum::Publishing => "Publishing",
            DocVersionStatusEnum::Published => "Published",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum DeploymentStatusEnum {
    #[default]
    #[serde(rename = "Building")]
    Building,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Generated")]
    Generated,
}
impl std::fmt::Display for DeploymentStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            DeploymentStatusEnum::Building => "Building",
            DeploymentStatusEnum::Cancelled => "Cancelled",
            DeploymentStatusEnum::Complete => "Complete",
            DeploymentStatusEnum::Created => "Created",
            DeploymentStatusEnum::Error => "Error",
            DeploymentStatusEnum::Generated => "Generated",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ThemeOwnerEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "self")]
    _Self,
    #[serde(rename = "organization")]
    Organization,
}
impl std::fmt::Display for ThemeOwnerEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ThemeOwnerEnum::Default => "default",
            ThemeOwnerEnum::_Self => "self",
            ThemeOwnerEnum::Organization => "organization",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum OrganizationRoleEnum {
    #[default]
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "member")]
    Member,
}
impl std::fmt::Display for OrganizationRoleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            OrganizationRoleEnum::Admin => "admin",
            OrganizationRoleEnum::Manager => "manager",
            OrganizationRoleEnum::Member => "member",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum ProjectTypeEnum {
    #[default]
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "documentation")]
    Documentation,
}
impl std::fmt::Display for ProjectTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            ProjectTypeEnum::Api => "api",
            ProjectTypeEnum::Documentation => "documentation",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum LatestApiLinkPolicyTypeEnum {
    #[default]
    #[serde(rename = "latest")]
    Latest,
}
impl std::fmt::Display for LatestApiLinkPolicyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            LatestApiLinkPolicyTypeEnum::Latest => "latest",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PinnedApiLinkPolicyTypeEnum {
    #[default]
    #[serde(rename = "pinned")]
    Pinned,
}
impl std::fmt::Display for PinnedApiLinkPolicyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PinnedApiLinkPolicyTypeEnum::Pinned => "pinned",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GenerationLanguageEnum {
    #[default]
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "typescript")]
    Typescript,
}
impl std::fmt::Display for GenerationLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GenerationLanguageEnum::Python => "python",
            GenerationLanguageEnum::Go => "go",
            GenerationLanguageEnum::Rust => "rust",
            GenerationLanguageEnum::Ruby => "ruby",
            GenerationLanguageEnum::Typescript => "typescript",
        };
        write!(f, "{}", str_val)
    }
}
