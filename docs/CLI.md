# The Sideko Command Line Interface for programmatically generating API tools

This document contains the help content for the `Sideko CLI` command-line program.

## `Sideko CLI`

Login to start generating tools for your APIs

**Usage:** `Sideko CLI [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `login` — Log into Sideko interactively to obtain API key for generations
* `sdk` — Generate and configure SDK clients
* `api` — **Enterprise Only!** Manage API specifications
* `doc` — **Enterprise Only!** Manage documentation projects

###### **Options:**

* `-c`, `--config <CONFIG>` — Path to .sideko file containing api key, default locations: ./.sideko then $HOME/.sideko
* `-q`, `--quiet` — No logging except for errors
* `-v`, `--verbose` — Verbose logging



## `Sideko CLI login`

Log into Sideko interactively to obtain API key for generations

**Usage:** `Sideko CLI login [OPTIONS]`

###### **Options:**

* `-o`, `--output <OUTPUT>` — Path to file to store API key, default: $HOME/.sideko



## `Sideko CLI sdk`

Generate and configure SDK clients

**Usage:** `Sideko CLI sdk <COMMAND>`

###### **Subcommands:**

* `try` — Generate a point-in-time SDK (unmanaged/stateless). This command is available to free-tier users
* `create` — **Enterprise Only!** Create a managed SDK that Sideko can track and maintain maintain. This command returns an SDK repo with git tracking
* `update` — **Enterprise Only!** Update a Sideko managed SDK. This command returns the git patch file to update your SDK to match an updated API



## `Sideko CLI sdk try`

Generate a point-in-time SDK (unmanaged/stateless). This command is available to free-tier users

**Usage:** `Sideko CLI sdk try [OPTIONS] <OPENAPI_SOURCE> <LANGUAGE>`

###### **Arguments:**

* `<OPENAPI_SOURCE>` — Path or URL of OpenAPI spec
* `<LANGUAGE>` — Programming language to generate

  Possible values: `go`, `ruby`, `rust`, `typescript`, `python`


###### **Options:**

* `-o`, `--output <OUTPUT>` — Output path of generated source files, default: ./
* `-b`, `--base-url <BASE_URL>` — Base URL of API if not specified in OpenAPI spec
* `-p`, `--package-name <PACKAGE_NAME>` — Name of SDK package to generate
* `-t`, `--tests-mock-server-url <TESTS_MOCK_SERVER_URL>` — URL of Sideko Mock Server for generated testing suite



## `Sideko CLI sdk create`

**Enterprise Only!** Create a managed SDK that Sideko can track and maintain maintain. This command returns an SDK repo with git tracking

**Usage:** `Sideko CLI sdk create [OPTIONS] <API> <LANGUAGE> <REPO_NAME> <SEMVER>`

###### **Arguments:**

* `<API>` — Name of the API Project
* `<LANGUAGE>` — Programming language to generate an SDK for

  Possible values: `go`, `ruby`, `rust`, `typescript`, `python`

* `<REPO_NAME>` — The name of the repository
* `<SEMVER>` — The semantic version to assign to the SDK

###### **Options:**

* `-a`, `--api-semver <API_SEMVER>` — The semantic version of the API to generate from
* `-o`, `--output <OUTPUT>` — Output path of generated source files, default: ./



## `Sideko CLI sdk update`

**Enterprise Only!** Update a Sideko managed SDK. This command returns the git patch file to update your SDK to match an updated API

**Usage:** `Sideko CLI sdk update <REPO_PATH> <API> <LANGUAGE> <SEMVER>`

###### **Arguments:**

* `<REPO_PATH>`
* `<API>` — Name of the API Project
* `<LANGUAGE>` — Programming language of the existing SDK

  Possible values: `go`, `ruby`, `rust`, `typescript`, `python`

* `<SEMVER>` — The semantic version to assign to this updated SDK



## `Sideko CLI api`

**Enterprise Only!** Manage API specifications

**Usage:** `Sideko CLI api <COMMAND>`

###### **Subcommands:**

* `list` — List your API projects
* `create` — Create a new API project
* `update` — Upload a new version of a spec to your existing API project



## `Sideko CLI api list`

List your API projects

**Usage:** `Sideko CLI api list [OPTIONS]`

###### **Options:**

* `-n`, `--name <NAME>` — Pass name to filter by api name to see the versions of a single API e.g. my-rest-api



## `Sideko CLI api create`

Create a new API project

**Usage:** `Sideko CLI api create [OPTIONS] <OPENAPI_SOURCE> <SEMVER>`

###### **Arguments:**

* `<OPENAPI_SOURCE>` — Either a file path to an OpenAPI yml/json OR a public URL hosting the OpenAPI specification yml/json
* `<SEMVER>` — The semantic version to assign to the API

###### **Options:**

* `-n`, `--name <NAME>` — The name of the API in Sideko. e.g. my-rest-api
* `--notes <NOTES>` — Plain text or HTML notes about the new API specification



## `Sideko CLI api update`

Upload a new version of a spec to your existing API project

**Usage:** `Sideko CLI api update [OPTIONS] <NAME> <OPENAPI_SOURCE> <SEMVER>`

###### **Arguments:**

* `<NAME>` — The name of your API in Sideko. e.g. my-rest-api
* `<OPENAPI_SOURCE>` — Either a file path to an OpenAPI yml/json OR a public URL hosting the OpenAPI specification yml/json
* `<SEMVER>` — Either the semantic version OR one of "major" "minor" or "patch" to automatically increment the version from the latest existing version

###### **Options:**

* `-n`, `--notes <NOTES>` — Plain text or HTML notes about the new API specification



## `Sideko CLI doc`

**Enterprise Only!** Manage documentation projects

**Usage:** `Sideko CLI doc <COMMAND>`

###### **Subcommands:**

* `list` — List your documentation projects
* `deploy` — Trigger a documentation deployment to preview or production



## `Sideko CLI doc list`

List your documentation projects

**Usage:** `Sideko CLI doc list`



## `Sideko CLI doc deploy`

Trigger a documentation deployment to preview or production

**Usage:** `Sideko CLI doc deploy [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the Doc Project in Sideko. e.g. my-rest-api-docs

###### **Options:**

* `--prod` — Flag to deploy to production environment. If not set, it will deploy to preview
* `--no-wait` — Flag to not poll until the deployment has completed



