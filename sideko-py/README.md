# Sideko-Python

SDK client generator for HTTP APIs using OpenApi.

Supports OpenApi 3 and can generate clients in Python, Ruby, Rust, Typescript, and Go.

This package will also install the `sideko` CLI, find the CLI's documentation on our [Github](https://github.com/Sideko-Inc/sideko).

## Installation

```
pip install sideko-py
```

## Usage

### Configure API key

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

### Generate

```python
import sideko_py

# The openAPI source can be a URL, Path, or json/yml raw spec (as a string)
openapi_src = "https://raw.githubusercontent.com/Sideko-Inc/sideko/main/specs/brewdog.yml"
sideko_py.generate_sdk(sideko_py.Language.Python, openapi_src, "./sdk-out")
```

## Local development

This rust crate uses [PyO3](https://pyo3.rs/) & [Maturin](https://www.maturin.rs/) to create and distribute python bindings to generate SDKs with Sideko.

_Assumes rust / cargo / python >= 3.8 is installed_

1. Create & activate virtual python environment

```
python3 -m venv .venv
source .venv/bin/activate
```

2. Install maturin

```
pip install maturin
# macOS users can also install with `brew install maturin`
```

3. Build & install python module

```
maturin develop
```

4. Try it out in a python shell

```
python

>>> import sideko_py
```
