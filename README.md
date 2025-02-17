# expose-shared-library

The crate originally intended to simplify manual build of pyo3 without [`maturin`][maturin], which can be useful in various circumstances.
Now it serves to expose the shared library to both python and lua.

## Functions

Only one function is provided: `expose_shared_library`.
This function exposes the rust shared library to python/lua side according to [pyo3 doc][pyo3-doc].

For usage, please refer to [`examples/mixed-rust-python-project/build.rs`](./examples/mixed-rust-python-project/build.rs).

## Installation

```bash
cargo add \
    --git https://github.com/kkew3/manual-build-pyo3.git \
    --tag 0.1.3 --build \
    expose-shared-library
    --features py
```

or

```bash
cargo add \
    --git https://github.com/kkew3/manual-build-pyo3.git \
    --tag 0.1.3 --build \
    expose-shared-library
    --features lua
```

depending on the desired feature.
One of the features "py" or "lua" must be enabled, but not both.


[maturin]: https://www.maturin.rs
[pyo3-doc]: https://pyo3.rs/v0.22.2/building-and-distribution#manual-builds
