<p align="center">
  <a href="https://sideko.dev">
    <img src="https://storage.googleapis.com/sideko.appspot.com/public_assets/website_assets/logo-symbol.svg" height="96">
    <h3 align="center">Sideko, Inc.</h3>
  </a>
</p>

<p align="center">
  Your API Ecosystem, On Autopilot
</p>

<p align="center">
  <a href="https://sideko.dev"><strong>Website</strong></a> |
  <a href="https://docs.sideko.dev"><strong>Docs</strong></a>
</p>
<br/>

# The single platform to generate and maintain an ecosystem of tools for your APIs.

Use _OpenAPI 3.x_ specifications to generate:

- SDKs with types, tests, authentication, and code snippets
- Mock servers that mimic the behavior of your API
- API Documentation that stays up to date automatically
- A command line interface for interacting with your API

## Installation

### Install via Homebrew (macOS)

```bash
brew install sideko-inc/tap/sideko
```

### Install via shell script (linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```

### Install via npm

```bash
npm install -g @sideko/cli
```

### Install via pip

```bash
pip install sideko-py
```

### 📜 Legacy installation (v0.10.2)

```bash
# via curl
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/v0.10.2/install.sh | sh

# via pip
pip install sideko-py==0.10.2
```

## Quick Start

```bash
# Complete login via browser pop-up
sideko login

# Interactively construct SDKs
sideko sdk init
```

## Authentication

Sideko uses the native secure store of the operating system to store your token (keychain on macOS, keyutils on linux, etc.).

Use `sideko login` authenticate the CLI interactively via a browser pop-up, this will authenticate the CLI with your User API Key.

For CI environments, set the environment variable `SIDEKO_API_KEY` with either you User API Key or that of a [Service Account](https://docs.sideko.dev/organizations/service-accounts)

## All features

Head over to the complete [CLI Documentation Page](./docs/CLI.md) for detailed CLI Docs.

## Reference

- [Complete CLI Docs](./docs/CLI.md)
- [License](./LICENSE)
- [Open SDK Generation Issue](https://github.com/Sideko-Inc/sideko/issues/new?template=generation-bug.md)
