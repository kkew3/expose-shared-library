#!/bin/bash
set -e

cargo clean
cargo build -r
cd python
python3 main.py
