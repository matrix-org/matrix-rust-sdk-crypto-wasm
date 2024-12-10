// Copyright 2024 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// @ts-check

/**
 * This is the entry point for ESM environments which support the ES Module Integration Proposal for WebAssembly [1].
 *
 * [1]: https://github.com/webassembly/esm-integration
 */

import * as bindings from "./pkg/matrix_sdk_crypto_wasm_bg.js";

/** @type {typeof import("./pkg/matrix_sdk_crypto_wasm_bg.wasm.d.ts")} */
// @ts-expect-error TSC can't find the definitions file, for some reason.
const wasm = await import("./pkg/matrix_sdk_crypto_wasm_bg.wasm");
bindings.__wbg_set_wasm(wasm);
wasm.__wbindgen_start();

/**
 * A no-op, for compatibility with other entry points.
 *
 * @returns {Promise<void>}
 */
export async function initAsync() {}

// Re-export everything from the generated javascript wrappers
export * from "./pkg/matrix_sdk_crypto_wasm_bg.js";
