pulumi plugin rm language wasm -y

go build

pulumi plugin install language wasm 1.0 --file .\language-host.exe

copy /y %userprofile%\.pulumi\plugins\language-wasm-v1.0.0\pulumi-language-wasm %userprofile%\.pulumi\plugins\language-wasm-v1.0.0\pulumi-language-wasm.exe