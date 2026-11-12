#!/bin/sh
set -eu

cargo build --release
printf '%s\n' "Built target/release/ruststrument and the Ruststrument libraries."

