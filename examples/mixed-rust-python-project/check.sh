#!/bin/bash
set -e

cargo clean
cargo build -r
cd python
python3 -c "
from my_package import sum_as_string_rs
print(repr(sum_as_string_rs.sum_as_string(1, 2)))"
