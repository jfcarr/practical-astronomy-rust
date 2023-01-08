#!/usr/bin/bash

rm -rf docs/rust
cd practical-astronomy-rust
cargo doc --document-private-items --target-dir ../docs/rust
