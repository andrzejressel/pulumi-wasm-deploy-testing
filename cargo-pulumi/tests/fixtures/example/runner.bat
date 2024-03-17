cargo component build -p main -p aimpl -p bimpl || exit 1

wasm-tools compose -o target/wasm32-wasi/debug/composed1.wasm target/wasm32-wasi/debug/main.wasm -d target/wasm32-wasi/debug/aimpl.wasm || exit 1
wasm-tools compose -o target/wasm32-wasi/debug/composed2.wasm target/wasm32-wasi/debug/composed1.wasm -d target/wasm32-wasi/debug/bimpl.wasm || exit 1

wasm-tools component wit target/wasm32-wasi/debug/composed2.wasm || exit 1

echo "DONE"