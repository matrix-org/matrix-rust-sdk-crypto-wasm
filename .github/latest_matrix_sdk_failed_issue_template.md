---
title: Building matrix-rust-sdk-crypto-wasm against the latest matrix-sdk Rust is failing
---
Something changed in
[matrix-rust-sdk](https://github.com/matrix-org/matrix-rust-sdk)'s crypto crate
that will break the build of this repo (matrix-rust-sdk-crypto-wasm) when we
update to it.

See the crypto crate's
[CHANGELOG](https://github.com/matrix-org/matrix-rust-sdk/blob/main/crates/matrix-sdk-crypto/CHANGELOG.md)
for possible hints about what changed and how to fix it.
