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
  <a href="https://sideko.dev/cli"><strong>Documentation</strong></a>
</p>
<br/>

## Generate SDKs

Use this CLI to generate typed SDKs from _OpenAPI 3.x_ specifications

### Install script (macOS, Linux, Windows w/ WSL)

```bash
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```

## Using the CLI

Specify the specification path, language, and output directory.

```
$ sideko generate specs/slack.json python ../sdks-dir

Generating Sideko SDK in PYTHON
Successfully generated SDK. Saving to ../sdks-dir
```

| Language   | Supported |
| ---------- | :-------: |
| Python     |    ✅     |
| Ruby       |    ✅     |
| Go         |    ✅     |
| Typescript |    ✅     |
| Rust       |    ✅     |
| C#         |    🚧     |
| Java       |    🚧     |

## Reference

- [License](./LICENSE)
