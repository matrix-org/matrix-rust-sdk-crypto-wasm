#!/bin/sh
#
# Build the JavaScript modules
#

set -eux

cd "$(dirname "$0")"/..

WASM_PACK_ARGS="${WASM_PACK_ARGS:-}"

# Generate the JavaScript bindings
# --no-pack disables generation of a `package.json` file, as we're managing it ourselves.
wasm-pack build --no-pack --target bundler --scope matrix-org --out-dir pkg --weak-refs "${WASM_PACK_ARGS}"

# This will generate the following files in the `pkg` directory for us:
#   - matrix_sdk_crypto_wasm.d.ts: TypeScript declarations of the bindings
#   - matrix_sdk_crypto_wasm.js: logic to load the WASM module
#   - matrix_sdk_crypto_wasm_bg.js: the JS <-> WASM glue
#   - matrix_sdk_crypto_wasm_bg.wasm: the actual WASM module
#   - matrix_sdk_crypto_wasm_bg.wasm.d.ts: types for the exports of the WASM module

# We're not interested in the loading logic, as it doesn't work well on all platforms, so we ship our own loader.
rm pkg/matrix_sdk_crypto_wasm.js

# The JS <-> WASM glue uses ESM syntax, so we want to create a CommonJS version of it
babel pkg/matrix_sdk_crypto_wasm_bg.js --out-dir pkg --out-file-extension .cjs --plugins @babel/plugin-transform-modules-commonjs
