# manual-build-pyo3

The crate intends to simplify manual build of pyo3 without [`maturin`][maturin].
This can be useful in various circumstances.

## Functions

Only one function is provided: `expose_shared_library`.
This function exposes the rust shared library to python side according to [pyo3 doc][pyo3-doc].

## Installation

```bash
cargo add --git https://github.com/kkew3/manual-build-pyo3.git
```

## Known issue

The crate hard-codes the target directory as "target". It currently does not respect [cargo config][build-target-dir].


[maturin]: https://www.maturin.rs
[pyo3-doc]: https://pyo3.rs/v0.22.2/building-and-distribution#manual-builds
[build-target-dir]: https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir
