#!/usr/bin/env bash

set -e

cargo component build -p wasm_common
cargo component build -p pulumi_wasm
cargo component build -p pulumi_wasm_random
cargo component build -p pulumi_rust_wasm
cargo component build -p pulumi_wasm_main
cargo build


cp "target/wasm32-wasi/debug/pulumi_wasm.wasm" "target/wasm32-wasi/debug/pulumi-wasm.wasm"
cp "target/wasm32-wasi/debug/pulumi_wasm_main.wasm" "target/wasm32-wasi/debug/pulumi-wasm-main.wasm"
cp "target/wasm32-wasi/debug/pulumi_wasm_random.wasm" "target/wasm32-wasi/debug/pulumi-wasm-random.wasm"
cp "target/wasm32-wasi/debug/pulumi_rust_wasm.wasm" "target/wasm32-wasi/debug/pulumi-rust-wasm.wasm"


wasm-tools compose -o target/wasm32-wasi/debug/composed1.wasm target/wasm32-wasi/debug/pulumi-wasm-main.wasm -d target/wasm32-wasi/debug/pulumi-wasm-random.wasm
wasm-tools compose -o target/wasm32-wasi/debug/composed2.wasm target/wasm32-wasi/debug/composed1.wasm -d target/wasm32-wasi/debug/pulumi-wasm.wasm
cargo run -- compile --wasm target/wasm32-wasi/debug/composed2.wasm --output target/wasm32-wasi/debug/composed2.cwasm
#wasmtime compile -D debug-info=y,coredump=y,address-map=y -o target/wasm32-wasi/debug/composed2.cwasm target/wasm32-wasi/debug/composed2.wasm || exit /b 1

wasm-tools component wit target/wasm32-wasi/debug/composed2.wasm
wasm-tools print target/wasm32-wasi/debug/composed1.wasm > composed1.wat
wasm-tools print target/wasm32-wasi/debug/composed2.wasm > composed2.wat
