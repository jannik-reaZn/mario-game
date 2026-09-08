#!/bin/bash

set -e
cargo build --release --target wasm32-unknown-unknown
python3 -m http.server 8000
