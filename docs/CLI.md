# A CLI for Constructing an API Ecosystem

This document contains the help content for the `sideko` command-line program.

**Command Overview:**

* [`sideko`↴](#sideko)
* [`sideko login`↴](#sideko-login)
* [`sideko api`↴](#sideko-api)
* [`sideko api version`↴](#sideko-api-version)
* [`sideko api version create`↴](#sideko-api-version-create)
* [`sideko api version list`↴](#sideko-api-version-list)
* [`sideko api version update`↴](#sideko-api-version-update)
* [`sideko api version download`↴](#sideko-api-version-download)
* [`sideko api create`↴](#sideko-api-create)
* [`sideko api list`↴](#sideko-api-list)
* [`sideko api stats`↴](#sideko-api-stats)
* [`sideko api lint`↴](#sideko-api-lint)
* [`sideko account`↴](#sideko-account)
* [`sideko account get-my-api-key`↴](#sideko-account-get-my-api-key)
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


```       _      _        _          
     ___ (_)  __| |  ___ | | __  ___  
    / __|| | / _` | / _ \| |/ / / _ \ 
    \__ \| || (_| ||  __/|   < | (_) |
    |___/|_| \__,_| \___||_|\_\ \___/ 

    your api ecosystem on autopilot
```                                                                                                                            


**Usage:** `sideko [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `login` — authenticate cli interactively via browser
* `api` — manage api specifications
* `account` — manage your sideko account
* `sdk` — generate, customize, and sync sdks
* `doc` — manage api documentation websites
* `logout` — logout of sideko
* `config` — configure the cli

###### **Options:**

* `-q`, `--quiet` — no logging except for errors
* `-v`, `--verbose` — verbose logging (-v) or trace logging (-vv)
* `--config <CONFIG>` — load config from custom path



## `sideko login`

authenticate cli interactively via browser

**Usage:** `sideko login [OPTIONS]`

###### **Options:**

* `--key <KEY>` — manually provide your api key to the cli, this will take priority over browser login
* `--output <OUTPUT>` — path to file to store api key, default: $HOME/.sideko



## `sideko api`

manage api specifications

**Usage:** `sideko api <COMMAND>`

###### **Subcommands:**

* `version` — manage api specification versions
* `create` — create a new api
* `list` — list all apis
* `stats` — display stats gathered from the specification
* `lint` — linting errors gathered from the specification



## `sideko api version`

manage api specification versions

**Usage:** `sideko api version <COMMAND>`

###### **Subcommands:**

* `create` — create a new version of an api with an openapi spec
* `list` — list api versions
* `update` — updates an existing api version
* `download` — downloads the openapi specification of an api version



## `sideko api version create`

create a new version of an api with an openapi spec

**Usage:** `sideko api version create [OPTIONS] --name <NAME> --version <VERSION> --spec <SPEC>`

###### **Options:**

* `--name <NAME>` — api name or id e.g. my-api
* `--version <VERSION>` — semantic version (e.g. `2.1.5`) or version bump (`patch`, `minor`, `major`, `rc`)
* `--spec <SPEC>` — path to openapi specification (YAML or JSON format)
* `--allow-lint-errors` — Allow linting errors to be present in the provided spec [default: false]

   By default creating a new version with an OpenAPI that contains linting errors is disallowed. If you wish to allow linting errors you may experience issues later with SDK generation or mock servers.
* `--disable-mock` — disable mock server for new version [default: enabled]
* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api version list`

list api versions

**Usage:** `sideko api version list [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — api name or id e.g. my-api
* `--limit <LIMIT>` — limit results to most recent N versions
* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api version update`

updates an existing api version

**Usage:** `sideko api version update [OPTIONS] --name <NAME> --version <VERSION>`

###### **Options:**

* `--name <NAME>` — api name or id e.g. my-api
* `--version <VERSION>` — version to update (e.g. `2.1.5` or `latest`)
* `--new-version <NEW_VERSION>` — version to update with (e.g. `2.1.5`)
* `--spec <SPEC>` — path to openapi spec (yaml or json format) to update with
* `--allow-lint-errors` — Allow linting errors to be present in the provided spec [default: false]

   By default using an OpenAPI that contains linting errors is disallowed. If you wish to allow linting errors you may experience issues later with SDK generation or mock servers.
* `--mock <MOCK>` — enable or disable the mock server

  Possible values: `true`, `false`

* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api version download`

downloads the openapi specification of an api version

**Usage:** `sideko api version download [OPTIONS] --name <NAME> --version <VERSION>`

###### **Options:**

* `--name <NAME>` — api name or id e.g. my-api
* `--version <VERSION>` — version to update (e.g. `2.1.5` or `latest`)
* `--output <OUTPUT>` — custom output path of sdk config (must be .yaml or .yml or .json) the command may alter the extension according to the format of the downloaded OpenAPI file



## `sideko api create`

create a new api

**Usage:** `sideko api create [OPTIONS] --name <NAME> --version <VERSION> --spec <SPEC>`

###### **Options:**

* `--name <NAME>` — name of api (only alphanumeric characters and dashes, e.g. `my-api`)
* `--version <VERSION>` — semantic version of initial version (e.g. `0.1.0`)
* `--spec <SPEC>` — path to openapi spec of initial version (yaml or json format)
* `--allow-lint-errors` — Allow linting errors to be present in the provided spec [default: false]

   By default using an OpenAPI that contains linting errors is disallowed. If you wish to allow linting errors you may experience issues later with SDK generation or mock servers.
* `--disable-mock` — disable mock server for initial version [default: enabled]
* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api list`

list all apis

**Usage:** `sideko api list [OPTIONS]`

###### **Options:**

* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api stats`

display stats gathered from the specification

**Usage:** `sideko api stats [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — api name or id e.g. my-api
* `--version <VERSION>` — api name or id e.g. my-api

  Default value: `latest`
* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko api lint`

linting errors gathered from the specification

**Usage:** `sideko api lint [OPTIONS]`

###### **Options:**

* `--spec <SPEC>` — Path to local OpenAPI file to lint
* `--name <NAME>` — API name or id e.g. my-api
* `--version <VERSION>` — API version e.g. v1, latest

  Default value: `latest`
* `--errors` — Show errors only
* `--display <DISPLAY>` — display result as a raw json or prettified

  Default value: `pretty`

  Possible values: `raw`, `pretty`

* `--save` — save result as a CSV



## `sideko account`

manage your sideko account

**Usage:** `sideko account <COMMAND>`

###### **Subcommands:**

* `get-my-api-key` — retrieves value of your personal api key and pastes it to your clipboard



## `sideko account get-my-api-key`

retrieves value of your personal api key and pastes it to your clipboard

**Usage:** `sideko account get-my-api-key`



## `sideko sdk`

generate, customize, and sync sdks

**Usage:** `sideko sdk <COMMAND>`

###### **Subcommands:**

* `init` — interactively configure and create suite of sdks (recommended command for getting started)
* `config` — manage sdk configs
* `create` — create an sdk using an existing sdk config
* `update` — update sdk to implement changes to apis



## `sideko sdk init`

interactively configure and create suite of sdks (recommended command for getting started)

**Usage:** `sideko sdk init`



## `sideko sdk config`

manage sdk configs

**Usage:** `sideko sdk config <COMMAND>`

###### **Subcommands:**

* `init` — generate the default sdk configuration for an api
* `sync` — sync sdk configuration file with an api version



## `sideko sdk config init`

generate the default sdk configuration for an api

**Usage:** `sideko sdk config init [OPTIONS] --api-name <API_NAME>`

###### **Options:**

* `--api-name <API_NAME>` — api name or id e.g. my-api
* `--api-version <API_VERSION>` — generate config for specific version (e.g. `2.1.5`)

  Default value: `latest`
* `--module-structure <MODULE_STRUCTURE>` — default module structure that should be generated for the SDK config

  Possible values: `path`, `flat`, `tag`

* `--output <OUTPUT>` — custom output path of sdk config (must be .yaml or .yml)

  Default value: `./sdk-config.yaml`



## `sideko sdk config sync`

sync sdk configuration file with an api version

**Usage:** `sideko sdk config sync [OPTIONS] --config <CONFIG>`

###### **Options:**

* `--api-version <API_VERSION>` — sync config with specific api version (e.g. `2.1.5`)

  Default value: `latest`
* `--spec <SPEC>` — sync config with local openapi specification
* `--config <CONFIG>` — config to sync
* `--output <OUTPUT>` — custom output path of sdk config (must be .yaml or .yml) [defaults to same path as --config]



## `sideko sdk create`

create an sdk using an existing sdk config

**Usage:** `sideko sdk create [OPTIONS] --config <CONFIG> --lang <LANG>`

###### **Options:**

* `--config <CONFIG>` — path to sdk config
* `--lang <LANG>` — programming language to generate

  Possible values: `python`, `typescript`, `rust`, `go`, `csharp`, `java`

* `--version <VERSION>` — semantic version of generated sdk

  Default value: `0.1.0`
* `--api-version <API_VERSION>` — generate sdk for a specific version of the api (e.g. `2.1.5`)

  Default value: `latest`
* `--gh-actions` — include github actions for testing and publishing the sdk in the generation
* `--output <OUTPUT>` — path to save sdk

  Default value: `./`



## `sideko sdk update`

update sdk to implement changes to apis

**Usage:** `sideko sdk update [OPTIONS] --config <CONFIG> --repo <REPO> --version <VERSION>`

###### **Options:**

* `--config <CONFIG>` — path to sdk config
* `--repo <REPO>` — path to root of sdk repo
* `--version <VERSION>` — semantic version of generated sdk (e.g. `2.1.5`) or version bump (`patch`, `minor`, `major`, `rc`)
* `--api-version <API_VERSION>` — api version to update sdk with (e.g. `2.1.5`)

  Default value: `latest`



## `sideko doc`

manage api documentation websites

**Usage:** `sideko doc <COMMAND>`

###### **Subcommands:**

* `list` — list all documentation websites
* `deploy` — trigger documentation website deployment to preview or production



## `sideko doc list`

list all documentation websites

**Usage:** `sideko doc list [OPTIONS]`

###### **Options:**

* `--display <DISPLAY>` — display result format

  Default value: `pretty`

  Possible values: `raw`, `pretty`




## `sideko doc deploy`

trigger documentation website deployment to preview or production

**Usage:** `sideko doc deploy [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — doc project name or id e.g. my-docs
* `--prod` — deploy to production [default: preview]
* `--no-wait` — exit command after successful trigger [default: waits until deployment completes]



## `sideko logout`

logout of sideko

removes the api key from the os-native key service (e.g. `keychain` on macos, `keyutils` on linux,  or `windows credential manager`)

**Usage:** `sideko logout`



## `sideko config`

configure the cli

**Usage:** `sideko config <COMMAND>`

###### **Subcommands:**

* `autocomplete` — writes shell completion for the cli to stdout



## `sideko config autocomplete`

writes shell completion for the cli to stdout

**Usage:** `sideko config autocomplete --shell <SHELL>`

###### **Options:**

* `--shell <SHELL>` — Ggnerate completions for the specified shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




