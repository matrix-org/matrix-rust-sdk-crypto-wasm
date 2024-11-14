#!/bin/sh
#
# Build the JavaScript modules
#

set -eux

cd "$(dirname "$0")"/..

WASM_PACK_ARGS="${WASM_PACK_ARGS:-}"

rm -rf pkg

# Generate the JavaScript bindings
wasm-pack build --no-pack --target bundler --scope matrix-org --out-dir pkg --weak-refs "${WASM_PACK_ARGS}"

# This will output two important files:
#   - pkg/matrix_sdk_crypto_wasm.js
#   - pkg/matrix_sdk_crypto_wasm_bg.js
#
# We're only interested in the last one, as the first one is the loader,
# and the output from wasm-bindgen doesn't work well on all platforms, so we ship our own loader.
# We still want to convert the last one to  CommonJS, so we ship a proper dual commonjs/es6 module.

babel pkg/matrix_sdk_crypto_wasm_bg.js --out-dir pkg --out-file-extension .cjs --plugins @babel/plugin-transform-modules-commonjs
rm pkg/matrix_sdk_crypto_wasm.js
