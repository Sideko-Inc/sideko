<p align="center">
 <a href="https://sideko.dev">
<img src="https://storage.googleapis.com/sideko.appspot.com/public_assets/website_assets/logo-symbol.svg" height="96">
<h3 align="center">Sideko</h3>
</a>
</p>
<p align="center">
 Your API Ecosystem, On Autopilot
</p>
<p align="center">
<a href="https://sideko.dev"><strong>Website</strong></a> ·
<a href="https://docs.sideko.dev"><strong>Documentation</strong></a>
</p>

## The single platform to generate an ecosystem for your APIs.
Using OpenAPI 3.x specifications you can create:
- 🚀 **SDKs** with types, tests, authentication, and code snippets
- 🔄 **Mock Servers** that mimic your API behavior
- 📚 **API Documentation** that updates automatically
- 💻 **CLI Tools** for simple API interactions

## Installation


```bash
# 🍏 macOS
brew install sideko-inc/tap/sideko

# 🐍 python - pypi
pip install sideko-py

# 📦 js - npm
npm install -g @sideko/cli

# ⚡ curl (no windows support)
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```


## Quickstart - Premium SDKs in less than 5 minutes
```bash
# Authenticate via browser pop-up
sideko login
# (optional) shell autocomplete
sideko config autocomplete --shell [bash, elvish, fish, powershell, zsh]
# Interactively Create SDKs
sideko sdk init
```

## Authentication
Sideko securely stores your authentication token using your system's native secure store:
- macOS: Keychain
- Linux: keyutils
- Windows: Credential Manager

To authenticate:
1. Run `sideko login` for interactive browser authentication
2. In CI/CD: Set `SIDEKO_API_KEY` environment variable with your [User API Key or Service Account](https://docs.sideko.dev/organizations/service-accounts)

# Resources
- [📘 Complete CLI Documentation](./docs/CLI.md)
- [📝 Report SDK Generation Issue](https://github.com/Sideko-Inc/sideko/issues/new?template=generation-bug.md)
- [⚖️ License](./LICENSE)

----

<details>
<summary>Legacy Installations</summary>

```bash
# via curl
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/v0.10.2/install.sh | sh

# via pip
pip install sideko-py==0.10.2
```
</details>