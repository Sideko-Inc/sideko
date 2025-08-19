<p align="center">
 <a href="https://sideko.dev">
<img src="https://storage.googleapis.com/sideko.appspot.com/public_assets/website_assets/logo-symbol.svg" height="96">
<h3 align="center">Sideko</h3>
</a>
</p>
<p align="center">
 Automate your API Work and Accelerate Integrations
</p>
<p align="center">
<a href="https://sideko.dev"><strong>Website</strong></a> ·
<a href="https://docs.sideko.dev"><strong>Documentation</strong></a>
</p>

## OpenAPI -> Dozens of Tools
- 🚀 **SDKs** with types, tests, authentication, and code snippets
- 🔄 **Mock Servers** that mimic API behavior
- 📚 **API Documentation** that stays up to date automatically
- 💻 **CLI Tools** for simple API interactions

## Installation


```bash
# 🍏 macOS
brew install sideko-inc/tap/sideko

# 🐍 python - pypi
pip install sideko-py

# 📦 js - npm
npm install -g @sideko/cli

# ⚡ curl
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```


## Quickstart - Premium SDKs in Seconds
```bash
# authenticate (uses native keychain to store credentials on host)
sideko login

# interactively create sdks
sideko sdk init
```

## Local Authentication
This CLI utilizes your computer's native secure store:
- macOS: Keychain
- Linux: keyutils
- Windows: Credential Manager

To authenticate locally run: ```sideko login```


## Authenticating in CI/CD
Set `SIDEKO_API_KEY` environment variable in CI/CD.
1. With your own API key run: `sideko account get-my-api-key`
2. With a service account [see documentation](https://docs.sideko.dev/sideko-platform/service-accounts)

# Resources
- [📘 Complete CLI Reference Docs](./docs/CLI.md)
- [📝 Report an Issue](https://github.com/Sideko-Inc/sideko/issues/new?template=generation-bug.md)
- [⚖️ License](./LICENSE)
- Shell autocomplete: `sideko config autocomplete --shell (your shell)` (must run this command with each Sideko CLI update)

---

</details>

<details>
<summary>Legacy CLI Installations</summary>

```bash
# via curl
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/v0.10.2/install.sh | sh

# via pip
pip install sideko-py==0.10.2
```
</details>
