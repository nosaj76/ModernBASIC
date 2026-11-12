#!/bin/sh
set -eu

printf '%s\n' "Using $(rustc --version)"
printf '%s\n' "Using $(cargo --version)"
cargo metadata --no-deps --format-version 1 --locked >/dev/null
cargo test --all-targets --locked
cargo fmt --all -- --check
printf '%s\n' "Ruststrument verification passed."
