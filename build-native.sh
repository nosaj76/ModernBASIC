#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
revision_file="$project_dir/.source-revision"
revision_stamp="${CARGO_TARGET_DIR:-$project_dir/target}/release/.modernbasic-source-revision"

if ! command -v cargo >/dev/null 2>&1; then
    printf '%s\n' 'error: Cargo is required (Rust 1.74 or newer).' >&2
    exit 1
fi

if [ "$(uname -s)" = "Linux" ] && command -v pkg-config >/dev/null 2>&1; then
    if ! pkg-config --exists x11; then
        printf '%s\n' 'error: X11 development files are required (Ubuntu: libx11-dev).' >&2
        exit 1
    fi
fi

cd "$project_dir"
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=native"
cargo build --release --locked
if [ -f "$revision_file" ]; then
    cp -- "$revision_file" "$revision_stamp"
fi

printf '%s\n' 'Built target/release/modernbasic for this CPU.'
printf '%s\n' 'Use build.sh instead when the binary must run on older computers.'
