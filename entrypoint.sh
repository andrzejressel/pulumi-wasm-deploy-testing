#!/usr/bin/env bash

set -e

./run.sh

cargo run -- run --wasm target/wasm32-wasi/debug/composed2.wasm
