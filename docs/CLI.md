# The Sideko Command Line Interface for generating API Ecosystem tools

This document contains the help content for the `sideko` command-line program.

**Command Overview:**

* [`sideko`↴](#sideko)
* [`sideko login`↴](#sideko-login)
* [`sideko api`↴](#sideko-api)
* [`sideko api version`↴](#sideko-api-version)
* [`sideko api version create`↴](#sideko-api-version-create)
* [`sideko api version list`↴](#sideko-api-version-list)
* [`sideko api version update`↴](#sideko-api-version-update)
* [`sideko api create`↴](#sideko-api-create)
* [`sideko api list`↴](#sideko-api-list)
* [`sideko api stats`↴](#sideko-api-stats)
* [`sideko sdk`↴](#sideko-sdk)
* [`sideko sdk init`↴](#sideko-sdk-init)
* [`sideko sdk config`↴](#sideko-sdk-config)
* [`sideko sdk config init`↴](#sideko-sdk-config-init)
* [`sideko sdk config sync`↴](#sideko-sdk-config-sync)
* [`sideko sdk create`↴](#sideko-sdk-create)
* [`sideko sdk update`↴](#sideko-sdk-update)
* [`sideko doc`↴](#sideko-doc)
* [`sideko doc list`↴](#sideko-doc-list)
* [`sideko doc deploy`↴](#sideko-doc-deploy)
* [`sideko logout`↴](#sideko-logout)
* [`sideko config`↴](#sideko-config)
* [`sideko config autocomplete`↴](#sideko-config-autocomplete)

## `sideko`


```
.*....*......*.....*......*....*........*....*.....

..####...######..#####...######..##..##...####..
.##........##....##..##..##......##.##...##..##.
..####.....##....##..##..####....####....##..##.
.....##....##....##..##..##......##.##...##..##.
..####...######..#####...######..##..##...####..
................................................

- Your API Ecosystem, On Autopilot
*....*......*.....*......*.....*......*.....*.....*            
```                                                                                                                             


**Usage:** `sideko [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `login` — Authenticate CLI interactively via browser
* `api` — Manage API specifications
* `sdk` — Generate, customize, and sync SDKs
* `doc` — Manage API documentation websites
* `logout` — Logout of Sideko
* `config` — Configure the CLI

###### **Options:**

* `-q`, `--quiet` — No logging except for errors
* `-v`, `--verbose` — Verbose logging (-v) or trace logging (-vv)
* `--config <CONFIG>` — Load config from custom path



## `sideko login`

Authenticate CLI interactively via browser

**Usage:** `sideko login [OPTIONS]`

###### **Options:**

* `--key <KEY>` — Manually provide you Sideko API key to the CLI, this will take priority over browser login
* `--output <OUTPUT>` — Path to file to store API key, default: $HOME/.sideko



## `sideko api`

Manage API specifications

**Usage:** `sideko api <COMMAND>`

###### **Subcommands:**

* `version` — Manage API specification versions
* `create` — Create a new API
* `list` — List all APIs
* `stats` — Display stats gathered from the API specification



## `sideko api version`

Manage API specification versions

**Usage:** `sideko api version <COMMAND>`

###### **Subcommands:**

* `create` — Create a new version of an API with an OpenAPI spec
* `list` — List an APIs versions
* `update` — Updates an existing API version,



## `sideko api version create`

Create a new version of an API with an OpenAPI spec

**Usage:** `sideko api version create [OPTIONS] --name <NAME> --version <VERSION> --spec <SPEC>`

###### **Options:**

* `--name <NAME>` — API name or id e.g. my-api
* `--version <VERSION>` — Semantic version (e.g. `2.1.5`) or version bump (`patch`, `minor`, `major`, `rc`)
* `--spec <SPEC>` — Path to OpenAPI spec (YAML or JSON format)
* `--disable-mock` — Disable mock server for new version [default: enabled]
* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api version list`

List an APIs versions

**Usage:** `sideko api version list [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — API name or id e.g. my-api
* `--limit <LIMIT>` — Limit results to most recent N versions
* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api version update`

Updates an existing API version,

**Usage:** `sideko api version update [OPTIONS] --name <NAME> --version <VERSION>`

###### **Options:**

* `--name <NAME>` — API name or id e.g. my-api
* `--version <VERSION>` — Version to update (e.g. `2.1.5` or `latest`)
* `--new-version <NEW_VERSION>` — Version to update with (e.g. `2.1.5`)
* `--spec <SPEC>` — Path to OpenAPI spec (YAML or JSON format) to update with
* `--mock <MOCK>` — Enable or disable the mock server

  Possible values: `true`, `false`

* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api create`

Create a new API

**Usage:** `sideko api create [OPTIONS] --name <NAME> --version <VERSION> --spec <SPEC>`

###### **Options:**

* `--name <NAME>` — Name of API (only alphanumeric characters and dashes, e.g. `my-api`)
* `--version <VERSION>` — Semantic version of initial version (e.g. `2.1.5`)
* `--spec <SPEC>` — Path to OpenAPI spec of initial version (YAML or JSON format)
* `--disable-mock` — Disable mock server for initial version [default: enabled]
* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api list`

List all APIs

**Usage:** `sideko api list [OPTIONS]`

###### **Options:**

* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api stats`

Display stats gathered from the API specification

**Usage:** `sideko api stats [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — API name or id e.g. my-api
* `--version <VERSION>` — API name or id e.g. my-api

  Default value: `latest`
* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko sdk`

Generate, customize, and sync SDKs

**Usage:** `sideko sdk <COMMAND>`

###### **Subcommands:**

* `init` — Interactively configure and create suite of SDKs (recommended command for getting started)
* `config` — Manage SDK configs
* `create` — Create an SDK using an SDK Config
* `update` — Update SDK to implement changes to APIs



## `sideko sdk init`

Interactively configure and create suite of SDKs (recommended command for getting started)

**Usage:** `sideko sdk init`



## `sideko sdk config`

Manage SDK configs

**Usage:** `sideko sdk config <COMMAND>`

###### **Subcommands:**

* `init` — Generate the default SDK configuration for an API
* `sync` — Sync SDK configuration file with an API version



## `sideko sdk config init`

Generate the default SDK configuration for an API

**Usage:** `sideko sdk config init [OPTIONS] --api-name <API_NAME>`

###### **Options:**

* `--api-name <API_NAME>` — API name or id e.g. my-api
* `--api-version <API_VERSION>` — Generate config for specific version (e.g. `2.1.5`)

  Default value: `latest`
* `--x-mods` — Use the `x-sideko-*` x-fields in OpenAPI to define the module structure/function names for the SDK

   Including this flag will cause the module config to be omitted from the generated config file.
* `--output <OUTPUT>` — Custom output path of SDK config (must be .yaml or .yml)

  Default value: `./sdk-config.yaml`



## `sideko sdk config sync`

Sync SDK configuration file with an API version

**Usage:** `sideko sdk config sync [OPTIONS] --name <NAME> --config <CONFIG>`

###### **Options:**

* `--name <NAME>` — API name or id e.g. my-api
* `--version <VERSION>` — Sync config with specific version (e.g. `2.1.5`)

  Default value: `latest`
* `--spec <SPEC>` — Sync config with local OpenAPI specification
* `--config <CONFIG>` — Config to sync
* `--output <OUTPUT>` — Custom output path of SDK config (must be .yaml or .yml) [defaults to same path as --config]
* `--x-mods` — Use the `x-sideko-*` x-fields in OpenAPI to define the module structure/function names for the SDK

   Including this flag will cause the module config to be omitted from the generated config file.



## `sideko sdk create`

Create an SDK using an SDK Config

**Usage:** `sideko sdk create [OPTIONS] --config <CONFIG> --lang <LANG>`

###### **Options:**

* `--config <CONFIG>` — Path to SDK config
* `--lang <LANG>` — Programming language to generate

  Possible values: `python`, `typescript`, `rust`, `go`, `java`

* `--version <VERSION>` — Semantic version of generated SDK

  Default value: `0.1.0`
* `--api-version <API_VERSION>` — Generate SDK for a specific version of the API (e.g. `2.1.5`)

  Default value: `latest`
* `--gh-actions` — Include Github actions for testing and publishing the SDK in the generation
* `--output <OUTPUT>` — Path to save SDK

  Default value: `./`



## `sideko sdk update`

Update SDK to implement changes to APIs

**Usage:** `sideko sdk update [OPTIONS] --config <CONFIG> --repo <REPO> --version <VERSION>`

###### **Options:**

* `--config <CONFIG>` — Path to SDK config
* `--repo <REPO>` — Path to root of SDK repo
* `--version <VERSION>` — Semantic version of generated SDK (e.g. `2.1.5`) or version bump (`patch`, `minor`, `major`, `rc`)
* `--api-version <API_VERSION>` — API version to update SDK with (e.g. `2.1.5`)

  Default value: `latest`



## `sideko doc`

Manage API documentation websites

**Usage:** `sideko doc <COMMAND>`

###### **Subcommands:**

* `list` — List all documentation websites
* `deploy` — Trigger documentation website deployment to preview or production



## `sideko doc list`

List all documentation websites

**Usage:** `sideko doc list [OPTIONS]`

###### **Options:**

* `--display <DISPLAY>` — Display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko doc deploy`

Trigger documentation website deployment to preview or production

**Usage:** `sideko doc deploy [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — Doc project name or id e.g. my-docs
* `--prod` — Deploy to production [default: preview]
* `--no-wait` — Exit command after successful trigger [default: waits until deployment completes]



## `sideko logout`

Logout of Sideko

Removes the Sideko API key from the OS-native key service (e.g. `keychain` on macOS, `keyutils` on Linux, `Windows Credential Manager` on Windows)

**Usage:** `sideko logout`



## `sideko config`

Configure the CLI

**Usage:** `sideko config <COMMAND>`

###### **Subcommands:**

* `autocomplete` — Writes shell completion for the CLI to stdout



## `sideko config autocomplete`

Writes shell completion for the CLI to stdout

Example (`zsh`): `sideko config autocomplete --shell zsh > ~/sideko-complete.sh`

Then add `source ~/sideko-complete.sh` to `~/.zshrc`

**Usage:** `sideko config autocomplete --shell <SHELL>`

###### **Options:**

* `--shell <SHELL>`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




