<p align="center">
  <a href="https://sideko.dev">
    <img src="https://storage.googleapis.com/sideko.appspot.com/public_assets/website_assets/logo-symbol.svg" height="96">
    <h3 align="center">Sideko, Inc.</h3>
  </a>
</p>

<p align="center">
  Put the spotlight on your API
</p>

<p align="center">
  <a href="https://sideko.dev"><strong>Website</strong></a>
</p>
<br/>

## Generate API developer experience tools with Sideko

Use _OpenAPI 3.x_ specifications to generate:
- SDKs with types, tests, authentication, and code snippets
- Mock servers that mimic the behavior of your API
- Hosted API Documentation that stays up to date automatically

## Installation

_Install via pip_

```bash
pip install sideko-py
```

OR

_Install via curl (macOS, Linux, Windows w/ WSL)_

```bash
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```

## Usage

### Login

You need a Sideko API Key to generate SDKs. Create one for **free** by either:

1. Signing up / logging in via the CLI, or
2. Signing up / logging in at [app.sideko.dev](http://app.sideko.dev) (`User Icon > API Keys`)

```bash
sideko login
# Complete login via browser pop-up
```

_Run `sideko login --help` to list all options._

The API key is read from (in order of priority):

- `$PWD/.sideko`
- `$HOME/.sideko`
- `SIDEKO_API_KEY` environment variable

## Free features

### Generate a Stateless SDK

| Language   | Supported |
| ---------- | :-------: |
| Python     |    ✅     |
| Ruby       |    ✅     |
| Go         |    ✅     |
| Typescript |    ✅     |
| Rust       |    ✅     |
| C#         |    🚧     |
| Java       |    🚧     |

```bash
sideko sdk try (your Swagger/OpenAPI spec) python
```

- The OpenAPI source can be a path to a local `.json` or `.yaml` file, a URL, or the raw spec as a string.
- Language options: `python`, `typescript`, `ruby`, `rust`, or `go`

_Run `sideko --help` to list all options._

## Enterprise features

The following features are only available if you have an enterprise account with Sidkeo. Reach us at team@sideko.dev to talk about upgrading.

### SDK Management
Use the sdk subcommand to learn how to generate and maintain SDKs
```
sideko sdk
```


### API Specification Management
Use the api subcommand to learn how to manage API specifications and create mock servers
```
sideko api
```

### Documentation Management
Use the doc subcommand to learn how to trigger updates to API documentation 
```
sideko doc
```


## Excited about the project?

Reach us at team@sideko.dev

## Reference

- [License](./LICENSE)
- [Issues](https://github.com/Sideko-Inc/sideko/issues/new)
