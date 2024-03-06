cargo component build -p pulumi_wasm || exit /b 1
cargo component build -p pulumi_wasm_random || exit /b 1
cargo component build -p pulumi_rust_wasm || exit /b 1
cargo component build -p pulumi_wasm_main || exit /b 1
cargo build || exit /b 1
@REM cargo build -p pulumi_rust_wasm || exit /b
@REM @REM wasm-tools component wit target/wasm32-wasi/debug/pulumi_wasm_main.wasm
@REM
echo F|xcopy /b /v /y ".\target\wasm32-wasi\debug\pulumi_wasm.wasm" ".\target\wasm32-wasi\debug\pulumi-wasm.wasm" || exit /b 1
echo F|xcopy /b /v /y ".\target\wasm32-wasi\debug\pulumi_wasm_main.wasm" ".\target\wasm32-wasi\debug\pulumi-wasm-main.wasm" || exit /b 1
echo F|xcopy /b /v /y ".\target\wasm32-wasi\debug\pulumi_wasm_random.wasm" ".\target\wasm32-wasi\debug\pulumi-wasm-random.wasm" || exit /b 1
echo F|xcopy /b /v /y ".\target\wasm32-wasi\debug\pulumi_rust_wasm.wasm" ".\target\wasm32-wasi\debug\pulumi-rust-wasm.wasm" || exit /b 1
@REM
@REM @REM wasm-tools component wit target/wasm32-wasi/debug/pulumi-wasm-main.wasm
@REM @REM wasm-tools component wit target/wasm32-wasi/debug/pulumi-wasm-random.wasm
@REM
wasm-tools compose -o target/wasm32-wasi/debug/composed1.wasm target/wasm32-wasi/debug/pulumi-wasm-main.wasm -d target/wasm32-wasi/debug/pulumi-wasm-random.wasm || exit /b 1
wasm-tools compose -o target/wasm32-wasi/debug/composed2.wasm target/wasm32-wasi/debug/composed1.wasm -d target/wasm32-wasi/debug/pulumi-wasm.wasm || exit /b 1
@REM
@REM cargo build -p pulumi_entrypoint || exit /b
@REM
@REM @REM wasm-tools component wit target/wasm32-wasi/debug/pulumi-wasm-random.wasm
@REM @REM wasm-tools component wit target/wasm32-wasi/debug/pulumi-wasm-main.wasm
@REM
@REM wasm-tools component wit target/wasm32-wasi/debug/composed1.wasm
@REM wasm-tools component wit target/wasm32-wasi/debug/composed2.wasm
@REM
wasm-tools component wit target/wasm32-wasi/debug/composed2.wasm || exit /b 1
wasm-tools print target/wasm32-wasi/debug/composed1.wasm > composed1.wat || exit /b 1
wasm-tools print target/wasm32-wasi/debug/composed2.wasm > composed2.wat || exit /b 1

@REM cargo build --target wasm32-wasi