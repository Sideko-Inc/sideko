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
  <a href="https://sideko.dev/documentation"><strong>Site</strong></a>
</p>
<br/>

## Generate SDKs for your API

<img width="1209" alt="Screenshot 2023-10-06 at 11 01 06 AM" src="https://github.com/Sideko-Inc/sideko/assets/13861016/f3359ae9-b77e-42d6-abf0-565d6b1ff0d2">

From _OpenAPI 3.x_ specifications -> SDK repos with types, tests, authentication, and code snippets

| Feature    | Supported |
| ---------- | :-------: |
| Types      |    ✅     |
| Tests      |    ✅     |
| Form data  |    ✅     |
| All Auth   |    ✅     |
| Files      |    ✅     |
| Websockets |    🚧     |
| Graphql    |    🚧     |

## Premium Features

#### Edit code configuration in the UI

https://github.com/Sideko-Inc/sideko/assets/13861016/965954fd-e2bb-4bf3-b593-0fc7370bfe2e.mov

#### State-of-the-art documentation

https://github.com/Sideko-Inc/sideko/assets/13861016/f9e93fc6-27eb-4294-8202-b11f381e291c.mov

- Version management

Join the [waitlist](https://forms.gle/KHidhBtoWsBJ3w387) to get premium access

## How to use the SDK Generator

Use CURL or the sideko CLI to generate SDKs

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
     -F "name=mysdk" \
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
     -F "name=mysdk" \
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

## Excited about the project?

Reach us at team@sideko.dev

## Reference

- [License](./LICENSE)
- [Issues](https://github.com/Sideko-Inc/sideko/issues/new)
