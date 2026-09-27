#!/bin/bash

set -e
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/mario-game.wasm web/mario-game.wasm
python3 -m http.server 8000 --directory web
