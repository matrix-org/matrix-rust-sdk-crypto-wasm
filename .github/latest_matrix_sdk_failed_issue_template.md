---
title: Building matrix-rust-sdk-crypto-wasm against the latest matrix-sdk Rust is failing
---
Something changed in [matrix-rust-sdk](https://github.com/matrix-org/matrix-rust-sdk)'s crypto crate that will break the build of this repo (matrix-rust-sdk-crypto-wasm) when we update to it.

To see the latest changes in matrix-sdk-crypto, use [git cliff](https://git-cliff.org/):

```sh
git cliff from_commit..to_commit
```

(You can see which commits to supply by running `cargo update matrix-sdk-common` in this repo and diffing `Cargo.lock`.)

This should give you a hint about what changed.
