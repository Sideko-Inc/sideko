# Sideko-Python

SDK client generator for HTTP APIs using OpenApi.

Supports OpenApi 3.x and can generate clients in Python, Ruby, Rust, Typescript, and Go.

This package will also install the `sideko` CLI, find the CLI's documentation on our [Github](https://github.com/Sideko-Inc/sideko).

## Installation

```
pip install sideko-py
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
