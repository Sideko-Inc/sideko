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
  <a href="https://sideko.dev"><strong>Website</strong></a> |
  <a href="https://docs.sideko.dev"><strong>Docs</strong></a>
</p>
<br/>

## Welcome to Sideko, the platform for creating REST API tools and documentation.

Use _OpenAPI 3.x_ specifications to generate:
- SDKs with types, tests, authentication, and code snippets
- Mock servers that mimic the behavior of your API
- Hosted API Documentation that stays up to date automatically
- A command line interface for your API

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

You need a Sideko API Key to generate SDKs. Start generating **free** SDKs by either:

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

## Free tier

### Generate an unmanaged SDK with "sideko sdk try"

| Language   | Supported |
| ---------- | :-------: |
| Python     |    ✅     |
| Ruby       |    ✅     |
| Go         |    ✅     |
| Typescript |    ✅     |
| Rust       |    ✅     |
| C#         |    🚧     |
| Java       |    ✅     |
| CLI        |    ✅     |


```bash
sideko sdk try (your Swagger/OpenAPI spec) python
```

- The OpenAPI source can be a path to a local `.json` or `.yaml` file, a URL, or the raw spec as a string.
- Language options: `python`, `typescript`, `ruby`, `rust`, or `go`

_Run `sideko --help` to list all options._

## Enterprise features

Head over to the complete [CLI Documentation Page](./docs/CLI.md) for instructions on how to programmatically maintain your complete API toolset. 

## Excited about Sideko?

Reach us at `team@sideko.dev`

## Reference

- [Complete CLI Documentation](./docs/CLI.md)
- [License](./LICENSE)
- [Issues](https://github.com/Sideko-Inc/sideko/issues/new)
