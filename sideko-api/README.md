# sideko_api rust 

 Sideko API 

 # Authentication 
  
 ```rust
use sideko_api::Client;

let client = Client::default().with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
```

# list_api_projects

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.list_api_projects();
```
# list_api_versions

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .list_api_versions(ListApiVersionsRequest {
        project_id: "string".to_string(),
    });
```
# exchange_code_for_key

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .exchange_code_for_key(ExchangeCodeForKeyRequest {
        code: "string".to_string(),
    });
```
# cli_check_updates

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .cli_check_updates(CliCheckUpdatesRequest {
        cli_version: serde_json::json!({ "value" : "0.1.0" }),
    });
```
# create_api_project

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_api_project(CreateApiProjectRequest {
        data: NewApiProject {
            title: "A New API Project".to_string(),
        },
    });
```
# create_api_version

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .create_api_version(CreateApiVersionRequest {
        project_id: "string".to_string(),
        data: NewApiVersion {
            openapi: "string".to_string(),
            semver: "string".to_string(),
        },
    });
```
# stateless_generate_sdk

```rust
use sideko_api::Client;
use sideko_api::request_types::*;
use sideko_api::schemas::*;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client
    .stateless_generate_sdk(StatelessGenerateSdkRequest {
        data: StatelessGenerateSdk {
            base_url: Some(
                serde_json::json!({ "value" : "http://127.0.0.1:8080/api" }),
            ),
            language: GenerationLanguageEnum::Python,
            openapi: serde_json::json!(
                { "description" : "OpenAPI Sample in YAML format", "value" :
                "openapi: 3.0.0 info: {title: Sample API, description: 'Optional multiline or single-line description in [CommonMark](http://commonmark.org/help/) or HTML.', version: 0.1.9} servers: [{url: 'http://api.example.com/v1', description: 'Optional server description, e.g. Main (production) server'}, {url: 'http://staging-api.example.com', description: 'Optional server description, e.g. Internal staging server for testing'}] paths: {/users: {get: {summary: 'Returns a list of users.', description: 'Optional extended description in CommonMark or HTML.', responses: {'200': {description: 'A JSON array of user names', content: {application/json: {schema: {type: array, items: {type: string}}}}}}}}}"
                }
            ),
            package_name: Some(serde_json::json!({ "value" : "my_sdk" })),
        },
    });
```
