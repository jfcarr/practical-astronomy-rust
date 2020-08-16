#!/usr/bin/bash

rm -rf docs/rust
cargo doc --document-private-items --target-dir docs/rust
