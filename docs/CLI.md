# The Sideko Command Line Interface for programmatically generating API tools

This document contains the help content for the `sideko` command-line program.

## `sideko`

Login to start generating tools for your APIs

**Usage:** `sideko [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `login` — Log into Sideko interactively to obtain API key for generations
* `sdk` — Generate and configure SDK clients
* `api` — **Enterprise Only!** Manage API specifications
* `doc` — **Enterprise Only!** Manage documentation projects

###### **Options:**

* `-c`, `--config <CONFIG>` — Path to .sideko file containing api key, default locations: ./.sideko then $HOME/.sideko
* `-q`, `--quiet` — No logging except for errors
* `-v`, `--verbose` — Verbose logging



## `sideko login`

Log into Sideko interactively to obtain API key for generations

**Usage:** `sideko login [OPTIONS]`

###### **Options:**

* `-o`, `--output <OUTPUT>` — Path to file to store API key, default: $HOME/.sideko



## `sideko sdk`

Generate and configure SDK clients

**Usage:** `sideko sdk <COMMAND>`

###### **Subcommands:**

* `try` — Generate a point-in-time SDK (unmanaged/stateless). This command is available to free-tier users
* `create` — **Enterprise Only!** Create a managed SDK that Sideko can track and maintain maintain. This command returns an SDK repo with git tracking
* `update` — **Enterprise Only!** Update a Sideko managed SDK. This command returns the git patch file to update your SDK to match an updated API
* `list` — **Enterprise Only!** List all Sideko managed SDKs for an API Specification Collection
* `config` — **Enterprise Only!** Manage SDK Configurations specifications



## `sideko sdk try`

Generate a point-in-time SDK (unmanaged/stateless). This command is available to free-tier users

**Usage:** `sideko sdk try [OPTIONS] <OPENAPI_SOURCE> <LANGUAGE>`

###### **Arguments:**

* `<OPENAPI_SOURCE>` — Path or URL of OpenAPI spec
* `<LANGUAGE>` — Programming language to generate

  Possible values: `go`, `ruby`, `rust`, `typescript`, `python`, `java`


###### **Options:**

* `-o`, `--output <OUTPUT>` — Output path of generated source files, default: ./
* `-b`, `--base-url <BASE_URL>` — Base URL of API if not specified in OpenAPI spec
* `-p`, `--package-name <PACKAGE_NAME>` — Name of SDK package to generate



## `sideko sdk create`

**Enterprise Only!** Create a managed SDK that Sideko can track and maintain maintain. This command returns an SDK repo with git tracking

**Usage:** `sideko sdk create [OPTIONS] <CONFIG_PATH> <LANGUAGE>`

###### **Arguments:**

* `<CONFIG_PATH>` — Name of the API Specification Collection
* `<LANGUAGE>` — Programming language to generate an SDK for

  Possible values: `go`, `ruby`, `rust`, `typescript`, `python`, `java`


###### **Options:**

* `--api-version <API_VERSION>` — Optionally generate from a specific API version
* `--sdk-version <SDK_VERSION>` — Optionally set an initial SDK semantic version
* `--output <OUTPUT>` — Output path of generated source files, default: ./



## `sideko sdk update`

**Enterprise Only!** Update a Sideko managed SDK. This command returns the git patch file to update your SDK to match an updated API

**Usage:** `sideko sdk update [OPTIONS] <REPO_PATH> <CONFIG_PATH> <RELEASE_TYPE_OR_SEMVER>`

###### **Arguments:**

* `<REPO_PATH>` — Path to the existing SDK
* `<CONFIG_PATH>` — Name of the API Specification Collection
* `<RELEASE_TYPE_OR_SEMVER>` — The release type or semantic version to assign to the updated SDK

###### **Options:**

* `--api-version <API_VERSION>` — Optional specific API version to generate from (default is latest non-rc semantic version)



## `sideko sdk list`

**Enterprise Only!** List all Sideko managed SDKs for an API Specification Collection

**Usage:** `sideko sdk list [OPTIONS]`

###### **Options:**

* `--api-name <API_NAME>` — The name of the API in Sideko. e.g. my-rest-api
* `--successful-only <SUCCESSFUL_ONLY>` — Only show successful SDK generations

  Possible values: `true`, `false`




## `sideko sdk config`

**Enterprise Only!** Manage SDK Configurations specifications

**Usage:** `sideko sdk config <COMMAND>`

###### **Subcommands:**

* `init` — Initialize an SDK Configuration
* `sync` — Sync an SDK Configuration file with the latest state of the API



## `sideko sdk config init`

Initialize an SDK Configuration

**Usage:** `sideko sdk config init [OPTIONS] <API_NAME>`

###### **Arguments:**

* `<API_NAME>` — Name of the API in Sideko. e.g. my-rest-api

###### **Options:**

* `--api-version <API_VERSION>` — Optionally specify a specific API version to intitialize the config with



## `sideko sdk config sync`

Sync an SDK Configuration file with the latest state of the API

**Usage:** `sideko sdk config sync <CONFIG> [API_VERSION]`

###### **Arguments:**

* `<CONFIG>` — Path to the Sideko SDK Configuration File
* `<API_VERSION>` — Optionally specify a specific API version to sync the config with



## `sideko api`

**Enterprise Only!** Manage API specifications

**Usage:** `sideko api <COMMAND>`

###### **Subcommands:**

* `list` — List your API Specification Collections
* `create` — Create a new API Specification Collection
* `update` — Upload a new version of a spec to your existing API Specification Collection



## `sideko api list`

List your API Specification Collections

**Usage:** `sideko api list [OPTIONS]`

###### **Options:**

* `-n`, `--name <NAME>` — Pass name to filter by api name to see the versions of a single API e.g. my-rest-api



## `sideko api create`

Create a new API Specification Collection

**Usage:** `sideko api create [OPTIONS] <OPENAPI_SOURCE> <SEMVER> <NAME>`

###### **Arguments:**

* `<OPENAPI_SOURCE>` — Either a file path to an OpenAPI yml/json OR a public URL hosting the OpenAPI specification yml/json
* `<SEMVER>` — The semantic version to assign to the API
* `<NAME>` — The name of the API in Sideko. e.g. my-rest-api

###### **Options:**

* `--notes <NOTES>` — Plain text or HTML notes about the new API specification



## `sideko api update`

Upload a new version of a spec to your existing API Specification Collection

**Usage:** `sideko api update [OPTIONS] <NAME> <OPENAPI_SOURCE> <SEMVER>`

###### **Arguments:**

* `<NAME>` — The name of your API in Sideko. e.g. my-rest-api
* `<OPENAPI_SOURCE>` — Either a file path to an OpenAPI yml/json OR a public URL hosting the OpenAPI specification yml/json
* `<SEMVER>` — Either the semantic version OR one of "major" "minor" or "patch" to automatically increment the version from the latest existing version

###### **Options:**

* `-n`, `--notes <NOTES>` — Plain text or HTML notes about the new API specification



## `sideko doc`

**Enterprise Only!** Manage documentation projects

**Usage:** `sideko doc <COMMAND>`

###### **Subcommands:**

* `list` — List your documentation projects
* `deploy` — Trigger a documentation deployment to preview or production



## `sideko doc list`

List your documentation projects

**Usage:** `sideko doc list`



## `sideko doc deploy`

Trigger a documentation deployment to preview or production

**Usage:** `sideko doc deploy [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the Doc Project in Sideko. e.g. my-rest-api-docs

###### **Options:**

* `--prod` — Flag to deploy to production environment. If not set, it will deploy to preview
* `--no-wait` — Flag to not poll until the deployment has completed



