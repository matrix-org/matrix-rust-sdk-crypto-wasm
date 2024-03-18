#!/bin/bash
#
# Build the JavaScript modules
#
# This script is really a workaround for https://github.com/rustwasm/wasm-pack/issues/1074.
#
# Currently, the only reliable way to load WebAssembly in all the JS
# environments we want to target (web-via-webpack, web-via-browserify, jest)
# seems to be to pack the WASM into base64, and then unpack it and instantiate
# it at runtime.
#
# Hopefully one day, https://github.com/rustwasm/wasm-pack/issues/1074 will be
# fixed and this will be unnecessary.

set -e

cd $(dirname "$0")/..

wasm-pack build --target nodejs --scope matrix-org --out-dir pkg --weak-refs "${WASM_PACK_ARGS[@]}"

# Convert the Wasm into a JS file that exports the base64'ed Wasm.
{
  printf 'module.exports = `'
  base64 < pkg/matrix_sdk_crypto_wasm_bg.wasm
  printf '`;'
} > pkg/matrix_sdk_crypto_wasm_bg.wasm.js

# In the JavaScript:
#  1. Strip out the lines that load the WASM, and our new epilogue.
#  2. Remove the imports of `TextDecoder` and `TextEncoder`. We rely on the global defaults.
#
# We create a new file, rather than overwriting the old one, otherwise any
# webpack-dev-server instance which happens to be watching will get upset over
# the `require("path")`. We call the output "index.js" because we may as well.
{
  sed -e '/Text..coder.*= require(.util.)/d' \
      -e '/^const path = /,$d' pkg/matrix_sdk_crypto_wasm.js
  cat scripts/epilogue.js
} > pkg/index.js

# also extend the typescript, and give it a name to match the JS.
cat pkg/matrix_sdk_crypto_wasm.d.ts scripts/epilogue.d.ts > pkg/index.d.ts
