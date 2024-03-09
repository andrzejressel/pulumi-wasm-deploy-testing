#!/usr/bin/env bash

set -e

pulumi plugin rm language wasm -y

go build

pulumi plugin install language wasm 1.0 --file ./pulumi-language-wasm
