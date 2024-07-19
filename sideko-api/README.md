# sideko_api rust 

 The Sideko API unlocks features including generating SDKs, setting up API projects with mock servers, creating documentation projects with generated API references and custom pages, managing roles and permissions, and more. 

 # Authentication 
  
 ```rust
use sideko_api::Client;

let client = Client::default().with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined")).with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
```

# delete_api_link

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_api_link(DeleteApiLinkRequest {
        link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# delete_api_link_group

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_api_link_group(DeleteApiLinkGroupRequest {
        group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# delete_api_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_api_project(DeleteApiProjectRequest {
        project_id: "string".to_string(),
    });
```
# delete_api_project_role

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_api_project_role(DeleteApiProjectRoleRequest {
        project_id: "string".to_string(),
        user_id: "string".to_string(),
    });
```
# delete_doc_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_doc_project(DeleteDocProjectRequest {
        project_id: "string".to_string(),
    });
```
# delete_doc_project_role

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_doc_project_role(DeleteDocProjectRoleRequest {
        project_id: "string".to_string(),
        user_id: "string".to_string(),
    });
```
# delete_guide

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_guide(DeleteGuideRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
    });
```
# delete_guide_href

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_guide_href(DeleteGuideHrefRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
        variant: GuideHrefVariantEnum::Prev,
    });
```
# delete_service_account

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .delete_service_account(DeleteServiceAccountRequest {
        service_account_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# list_api_links

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_api_links(ListApiLinksRequest {
        doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# get_api_link

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_api_link(GetApiLinkRequest {
        link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# list_api_link_groups

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_api_link_groups(ListApiLinkGroupsRequest {
        doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# list_api_projects

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.list_api_projects();
```
# get_api_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_api_project(GetApiProjectRequest {
        project_id: "string".to_string(),
    });
```
# list_api_project_members

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_api_project_members(ListApiProjectMembersRequest {
        project_id: "string".to_string(),
    });
```
# list_api_versions

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_api_versions(ListApiVersionsRequest {
        project_id: "string".to_string(),
    });
```
# get_api_version

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_api_version(GetApiVersionRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
    });
```
# get_api_version_openapi

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_api_version_openapi(GetApiVersionOpenapiRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
    });
```
# get_api_version_stats

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_api_version_stats(GetApiVersionStatsRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
    });
```
# exchange_code_for_key

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .exchange_code_for_key(ExchangeCodeForKeyRequest {
        code: "string".to_string(),
    });
```
# login_callback

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .login_callback(LoginCallbackRequest {
        code: "string".to_string(),
        state: Some("string".to_string()),
    });
```
# login_url

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .login_url(LoginUrlRequest {
        cli_output: Some("string".to_string()),
        cli_port: Some(123),
    });
```
# cli_check_updates

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .cli_check_updates(CliCheckUpdatesRequest {
        cli_version: "0.1.0".to_string(),
    });
```
# list_doc_projects

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.list_doc_projects();
```
# get_doc_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_doc_project(GetDocProjectRequest {
        project_id: "string".to_string(),
    });
```
# list_deployments
Retrieves all deployments for a doc project
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_deployments(ListDeploymentsRequest {
        project_id: "string".to_string(),
        limit: Some(123),
        target: Some(DeploymentTargetEnum::Production),
    });
```
# get_deployment
Retrieves single deployment
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_deployment(GetDeploymentRequest {
        project_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        deployment_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
    });
```
# list_doc_project_members

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_doc_project_members(ListDocProjectMembersRequest {
        project_id: "string".to_string(),
    });
```
# get_doc_project_version

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_doc_project_version(GetDocProjectVersionRequest {
        project_id: "string".to_string(),
    });
```
# get_doc_project_theme
Get the theme attached to a documentation project
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_doc_project_theme(GetDocProjectThemeRequest {
        project_id: "string".to_string(),
    });
```
# list_doc_versions

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_doc_versions(ListDocVersionsRequest {
        project_id: "string".to_string(),
    });
```
# get_doc_version

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_doc_version(GetDocVersionRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
    });
```
# list_guides

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_guides(ListGuidesRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
    });
```
# get_guide

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_guide(GetGuideRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
    });
```
# get_guide_content

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_guide_content(GetGuideContentRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
    });
```
# get_organization
Get user organization
```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.get_organization();
```
# get_assets
Get all assets for an organization
```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.get_assets();
```
# list_organization_members
Get users in the organization
```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.list_organization_members();
```
# get_organization_theme
Get documentation project theme configured at the organization level
```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.get_organization_theme();
```
# get_current_user

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.get_current_user();
```
# get_api_key

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.get_api_key();
```
# get_user_project_role
retrieve current user role for a given project type/id
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .get_user_project_role(GetUserProjectRoleRequest {
        project_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        project_type: ProjectTypeEnum::Api,
    });
```
# get_service_accounts

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.get_service_accounts();
```
# update_api_link

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_api_link(UpdateApiLinkRequest {
        link_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: UpdateApiLink {
            api_version_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            nav_label: Some("string".to_string()),
            policy: Some(ApiLinkPolicyEnum::Latest),
            slug: Some("string".to_string()),
        },
    });
```
# update_api_link_group

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_api_link_group(UpdateApiLinkGroupRequest {
        group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        data: UpdateApiLinkGroup {
            nav_label: Some("string".to_string()),
            slug: Some("string".to_string()),
        },
    });
```
# update_api_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_api_project(UpdateApiProjectRequest {
        project_id: "string".to_string(),
        data: UpdateApiProject {
            title: Some("An updated API Project".to_string()),
        },
    });
```
# update_api_version

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_api_version(UpdateApiVersionRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        data: UpdateApiVersion {
            mock_server_enabled: Some(true),
            notes: Some(
                "<p>This version includes a number of excellent improvements</p>"
                    .to_string(),
            ),
            openapi: Some("string".to_string()),
            semver: Some("string".to_string()),
        },
    });
```
# update_doc_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_doc_project(UpdateDocProjectRequest {
        project_id: "string".to_string(),
        data: UpdateDocProject {
            logos: Some(UpdateDocProjectLogos {
                dark: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
                favicon: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
                light: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            }),
            settings: Some(UpdateDocProjectSettings {
                action_button: Some(UpdateDocProjectSettingsActionButton {
                    enabled: Some(true),
                    label: Some("string".to_string()),
                    url: Some("http://www.example.com".to_string()),
                }),
                metadata: Some(UpdateDocProjectSettingsMetadata {
                    description: Some("string".to_string()),
                    title: Some("string".to_string()),
                }),
            }),
            title: Some("A New Documentation Project".to_string()),
        },
    });
```
# update_guide

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_guide(UpdateGuideRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        guide_id: "string".to_string(),
        data: UpdateGuide {
            content: Some("string".to_string()),
            nav_label: Some("string".to_string()),
            next_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            prev_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            slug: Some("string".to_string()),
        },
    });
```
# create_api_link

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_api_link(CreateApiLinkRequest {
        data: NewApiLink {
            api_version_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            group_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            nav_label: "string".to_string(),
            policy: Union::LatestApiLinkPolicy(LatestApiLinkPolicy {
                api_project_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
                type_field: LatestApiLinkPolicyTypeEnum::Latest,
            }),
            slug: "string".to_string(),
        },
    });
```
# reorder_api_links

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .reorder_api_links(ReorderApiLinksRequest {
        data: ApiReorder {
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            groups: vec![
                ApiLinkGroupReorder { id : "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                .to_string(), order : 123 }
            ],
            links: vec![
                ApiLinkReorder { group_id : "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                .to_string(), id : "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                .to_string(), order : 123 }
            ],
        },
    });
```
# create_api_link_group

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_api_link_group(CreateApiLinkGroupRequest {
        data: NewApiLinkGroup {
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            nav_label: "string".to_string(),
            slug: "string".to_string(),
        },
    });
```
# create_api_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_api_project(CreateApiProjectRequest {
        data: NewApiProject {
            title: "A New API Project".to_string(),
        },
    });
```
# grant_api_project_role

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .grant_api_project_role(GrantApiProjectRoleRequest {
        project_id: "string".to_string(),
        data: NewProjectRole {
            role: ProjectRoleEnum::Viewer,
            user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        },
    });
```
# create_api_version

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_api_version(CreateApiVersionRequest {
        project_id: "string".to_string(),
        data: NewApiVersion {
            mock_server_enabled: Some(true),
            notes: Some(
                "<p>This version includes a number of excellent improvements</p>"
                    .to_string(),
            ),
            openapi: "string".to_string(),
            semver: "string".to_string(),
        },
    });
```
# create_doc_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_doc_project(CreateDocProjectRequest {
        data: NewDocProject {
            title: "A New Documentation Project".to_string(),
        },
    });
```
# trigger_deployment
Deploys a new generated version of documentation with linked guides & APIs
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .trigger_deployment(TriggerDeploymentRequest {
        project_id: "string".to_string(),
        data: NewDeployment {
            doc_version_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            target: DeploymentTargetEnum::Production,
        },
    });
```
# grant_doc_project_role

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .grant_doc_project_role(GrantDocProjectRoleRequest {
        project_id: "string".to_string(),
        data: NewProjectRole {
            role: ProjectRoleEnum::Viewer,
            user_id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
        },
    });
```
# create_guide

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_guide(CreateGuideRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        data: NewGuide {
            content: "string".to_string(),
            is_parent: true,
            nav_label: "string".to_string(),
            next_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            parent_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            prev_id: Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string()),
            slug: "string".to_string(),
        },
    });
```
# reorder_guides

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .reorder_guides(ReorderGuidesRequest {
        project_id: "string".to_string(),
        version_id: "string".to_string(),
        data: vec![
            ReorderGuide { id : "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a".to_string(),
            order : 123, parent_id : Some("3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
            .to_string()) }
        ],
    });
```
# create_organization

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_organization(CreateOrganizationRequest {
        data: NewOrganization {
            name: "string".to_string(),
            subdomain: "string".to_string(),
        },
    });
```
# upload_assets
Add a assets like logos to an organization
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .upload_assets(UploadAssetsRequest {
        data: AssetUpload {
            file: "path/to/file.pdf".to_string(),
        },
    });
```
# stateless_generate_sdk

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .stateless_generate_sdk(StatelessGenerateSdkRequest {
        data: StatelessGenerateSdk {
            base_url: Some("http://127.0.0.1:8080/api".to_string()),
            language: GenerationLanguageEnum::Python,
            openapi: "openapi: 3.0.0".to_string(),
            package_name: Some("my_sdk".to_string()),
            tests_mock_server_url: Some("http://127.0.0.1:8080/mock".to_string()),
        },
    });
```
# invite_user

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .invite_user(InviteUserRequest {
        data: Invite {
            email: "user@example.com".to_string(),
            role: OrganizationRoleEnum::Admin,
        },
    });
```
# create_service_account

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_service_account(CreateServiceAccountRequest {
        data: CreateServiceAccount {
            name: "Documentation Publisher Service Account".to_string(),
            project_roles: vec![
                UserProjectRole { project_id : "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
                .to_string(), project_type : ProjectTypeEnum::Api, role :
                ProjectRoleEnum::Viewer }
            ],
        },
    });
```
# update_doc_project_theme
Update a document project theme
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_doc_project_theme(UpdateDocProjectThemeRequest {
        project_id: "string".to_string(),
        data: ThemeValues {
            api_reference_group_variant: Some("grouped".to_string()),
            dark_active_button_bg_color: Some("#FFFFFF".to_string()),
            dark_active_button_text_color: Some("#FFFFFF".to_string()),
            dark_bg_color: Some("#FFFFFF".to_string()),
            dark_navbar_color: Some("#FFFFFF".to_string()),
            dark_navbar_text_color: Some("#FFFFFF".to_string()),
            light_active_button_bg_color: Some("#FFFFFF".to_string()),
            light_active_button_text_color: Some("#FFFFFF".to_string()),
            light_bg_color: Some("#FFFFFF".to_string()),
            light_navbar_color: Some("#FFFFFF".to_string()),
            light_navbar_text_color: Some("#FFFFFF".to_string()),
        },
    });
```
# update_organization_theme
Update  documentation project theme configured at the organization level
```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"))
    .with_cookie_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .update_organization_theme(UpdateOrganizationThemeRequest {
        data: ThemeValues {
            api_reference_group_variant: Some("grouped".to_string()),
            dark_active_button_bg_color: Some("#FFFFFF".to_string()),
            dark_active_button_text_color: Some("#FFFFFF".to_string()),
            dark_bg_color: Some("#FFFFFF".to_string()),
            dark_navbar_color: Some("#FFFFFF".to_string()),
            dark_navbar_text_color: Some("#FFFFFF".to_string()),
            light_active_button_bg_color: Some("#FFFFFF".to_string()),
            light_active_button_text_color: Some("#FFFFFF".to_string()),
            light_bg_color: Some("#FFFFFF".to_string()),
            light_navbar_color: Some("#FFFFFF".to_string()),
            light_navbar_text_color: Some("#FFFFFF".to_string()),
        },
    });
```
