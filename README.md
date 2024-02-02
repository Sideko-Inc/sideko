<p align="center">
  <a href="https://sideko.dev">
    <img src="https://storage.googleapis.com/sideko.appspot.com/public_assets/website_assets/logo-symbol.svg" height="96">
    <h3 align="center">Sideko, Inc.</h3>
  </a>
</p>

<p align="center">
  Accelerate API Adoption
</p>

<p align="center">
  <a href="https://sideko.dev"><strong>Site</strong></a>
</p>
<br/>

## Generate SDKs for your API

<img width="1209" alt="generate-cmd" src="assets/generate_cmd.png">

From _OpenAPI 3.x_ specifications to SDK repos with types, tests, authentication, and code snippets in seconds!

| Language   | Supported |
| ---------- | :-------: |
| Python     |    ✅     |
| Ruby       |    ✅     |
| Go         |    ✅     |
| Typescript |    ✅     |
| Rust       |    ✅     |
| C#         |    🚧     |
| Java       |    🚧     |

| Feature      | Supported |
| ------------ | :-------: |
| All Auth     |    ✅     |
| Native Types |    ✅     |
| Tests        |    ✅     |
| Form data    |    ✅     |
| Files        |    ✅     |
| Websockets   |    🚧     |
| Graphql      |    🚧     |

## Installation

_Intstall via curl for macOS, Linux, Windows w/ WSL_

```bash
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```

## Usage

### Login

You need a Sideko API Key to generate SDKs. Create one for **free** by signing up at [app.sideko.dev](http://app.sideko.dev) or login via the CLI:

```bash
sideko login
# Complete login via browser pop-up
```

Run `sideko login --help` to list all options.

### Generate

```
sideko generate specs/myopenapi.yaml python
```

Run `sideko generate --help` to list all options.

## Excited about the project?

Reach us at team@sideko.dev

## Reference

- [License](./LICENSE)
- [Issues](https://github.com/Sideko-Inc/sideko/issues/new)
