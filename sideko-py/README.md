# Sideko-Python

SDK client generator for HTTP APIs using OpenApi.

Supports OpenApi 3 and can generate clients in Python, Ruby, Rust, Typescript, and Go.

This package offers a python binding for the `sideko generate` command, to use the CLI follow instructions on [Github](https://github.com/Sideko-Inc/sideko).

## Installation

```
pip install sideko-py
```

## Usage

### Configure API key

1. Create an account with [Sideko](https://app.sideko.dev) and create an API key (User Icon > API keys).
2. Set environment variable `SIDEKO_API_KEY` or create file `$HOME/.sideko` containing `SIDEKO_API_KEY=your-key`

### Generate

```python
import sideko_py

# The openAPI source can be a URL, Path, or json/yml raw spec (as a string)
openapi_src = "https://raw.githubusercontent.com/Sideko-Inc/sideko/main/specs/brewdog.yml"
sideko_py.generate_sdk(sideko_py.Language.Python, openapi_src, "./sdk-out")
```

## Local development

This crate uses [PyO3](https://pyo3.rs/) & [Maturin](https://www.maturin.rs/) to create and distribute python bindings to generate SDKs with Sideko.

_Assumes rust / cargo is installed_

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
