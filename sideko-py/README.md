# Sideko-Python

This crate uses [PyO3](https://pyo3.rs/) & [Maturin](https://www.maturin.rs/) to create and distribute python bindings to generate SDKs with Sideko.

## Local development

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
