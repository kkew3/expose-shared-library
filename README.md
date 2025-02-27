# expose-shared-library

[![ci](https://github.com/kkew3/expose-shared-library/actions/workflows/ci.yml/badge.svg)](https://github.com/kkew3/expose-shared-library/actions/workflows/ci.yml)

The crate originally intended to simplify manual build of pyo3 without [`maturin`][maturin], which can be useful in various circumstances.
Now it serves to expose the shared library to both python and lua.

## Functions

Only one function is provided: `expose_shared_library`.
This function exposes the rust shared library to python/lua side according to [pyo3 doc][pyo3-doc].

For usage, please refer to [`examples/mixed-rust-python-project/build.rs`](./examples/mixed-rust-python-project/build.rs) and [`examples/mixed-rust-lua-project/build.rs`](./examples/mixed-rust-lua-project/build.rs).

## Installation

```bash
cargo add \
    --git https://github.com/kkew3/expose-shared-library.git \
    expose-shared-library \
    --tag 0.1.4 --build --features py
```

or

```bash
cargo add \
    --git https://github.com/kkew3/expose-shared-library.git \
    expose-shared-library \
    --tag 0.1.4 --build --features lua
```

depending on the desired feature.
One of the features "py" or "lua" must be enabled, but not both.


[maturin]: https://www.maturin.rs
[pyo3-doc]: https://pyo3.rs/v0.22.2/building-and-distribution#manual-builds
