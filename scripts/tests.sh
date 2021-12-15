#!/usr/bin/env bash

set -euxo pipefail

cargo install --git https://github.com/c410-f3r/rust-tools --force

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags -Aunstable_features,-Aunused_crate_dependencies)"

$rt rustfmt
$rt clippy

rust-tools test-generic cleu-orm
cargo test cleu-orm-derive
