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

Use the CLI to generate typed SDKs from _OpenAPI 3.x_ specifications

### Get the CLI with install script (macOS, Linux, Windows w/ WSL)

```bash
curl -fsSL https://raw.githubusercontent.com/Sideko-Inc/sideko/main/install.sh | sh
```

```
$ sideko generate specs/slack.json python ../sdks-dir

Generating Sideko SDK in PYTHON
Successfully generated SDK. Saving to ../sdks-dir
```

### Using CURL

```bash
curl -X POST "https://api.sideko.dev/v1/sdk/generate/" \
     -F "extension=json" \
     -F "language=python" \
     -F "file=@/path-to/openapi.json" \
     -F "name=openapi.json" \
     -o sdk.tar.gz

tar -xvf sdk.tar.gz
```

### Use the petstore example

```bash
curl -O https://petstore3.swagger.io/api/v3/openapi.json

curl -X POST "https://api.sideko.dev/v1/sdk/generate/" \
     -F "extension=json" \
     -F "language=go" \
     -F "file=@openapi.json" \
     -F "name=openapi.json" \
     -o sdk.tar.gz

tar -xvf sdk.tar.gz
```

## Supported Languages

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
- [Issues](https://github.com/Sideko-Inc/sideko/issues/new)
