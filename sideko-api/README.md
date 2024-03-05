# sideko_api rust 

 Sideko API 

 # Authentication 
  
 ```rust
use sideko_api::Client;

let client = Client::default().with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
```

# login_url

```rust
use sideko_api::Client;
let client = Client::default()
    .with_api_key_auth(&std::env::var("API_KEY").expect("API key not defined"));
let response = client.login_url();
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
            openapi: serde_json::json!(
                { "description" : "OpenAPI Sample in YAML format", "value" :
                "openapi: 3.0.0 info: {title: Sample API, description: 'Optional multiline or single-line description in [CommonMark](http://commonmark.org/help/) or HTML.', version: 0.1.9} servers: [{url: 'http://api.example.com/v1', description: 'Optional server description, e.g. Main (production) server'}, {url: 'http://staging-api.example.com', description: 'Optional server description, e.g. Internal staging server for testing'}] paths: {/users: {get: {summary: 'Returns a list of users.', description: 'Optional extended description in CommonMark or HTML.', responses: {'200': {description: 'A JSON array of user names', content: {application/json: {schema: {type: array, items: {type: string}}}}}}}}}"
                }
            ),
            language: GenerationLanguageEnum::Python,
            ..Default::default()
        },
    });
```
