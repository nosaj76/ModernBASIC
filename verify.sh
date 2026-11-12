#!/usr/bin/env bash
set -euo pipefail

verify_ruststrument=1
if [ "${1:-}" = --core-only ]; then
    verify_ruststrument=0
    shift
fi
if [ "$#" -ne 0 ]; then
    printf '%s\n' 'usage: ./verify.sh [--core-only]' >&2
    exit 2
fi

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
revision_file="$project_dir/.source-revision"
revision_stamp="${CARGO_TARGET_DIR:-$project_dir/target}/release/.modernbasic-source-revision"

if ! command -v cargo >/dev/null 2>&1; then
    printf '%s\n' 'error: Cargo with Rust 1.74 or newer is required.' >&2
    exit 1
fi

cd "$project_dir"
# Match CI: several host acceptance tests exercise process-wide interrupt state.
export RUST_TEST_THREADS="${RUST_TEST_THREADS:-1}"
"$project_dir/source-check.sh"
python3 -m unittest discover -s "$project_dir/tests" -p '*_tests.py'
# Keep upstream vendored sources byte-identical; format the application package.
cargo fmt --package modernbasic -- --check
cargo check --all-targets --locked
rust_report="${CARGO_TARGET_DIR:-$project_dir/target}/acceptance/rust-$(date -u +%Y%m%dT%H%M%S)-$$.json"
python3 "$project_dir/tools/check-rust-batches.py" --report "$rust_report"
cargo clippy --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items --locked
cargo build --release --locked
if [ -f "$revision_file" ]; then
    cp -- "$revision_file" "$revision_stamp"
fi
export MODERNBASIC_TEST_BINARY="$(realpath -- "${CARGO_TARGET_DIR:-$project_dir/target}/release/modernbasic")"
python3 "$project_dir/tools/qualify-math.py" \
    --binary "$MODERNBASIC_TEST_BINARY" \
    --report "${CARGO_TARGET_DIR:-$project_dir/target}/acceptance/math-release.json"
python3 "$project_dir/tools/check-cvode.py" --binary "$MODERNBASIC_TEST_BINARY" \
    --report "${CARGO_TARGET_DIR:-$project_dir/target}/acceptance/cvode.json"
python3 "$project_dir/tools/check-ida.py" --binary "$MODERNBASIC_TEST_BINARY" \
    --report "${CARGO_TARGET_DIR:-$project_dir/target}/acceptance/ida.json"
"$project_dir/appliance/contract-test.sh"
cargo run --example runtime_soak --locked -- --cycles 200
cargo run --example math_soak --locked -- --cycles 200
python3 "$project_dir/tools/check-regressions.py" --binary "$MODERNBASIC_TEST_BINARY"
python3 "$project_dir/tools/check-draft-recovery.py" --binary "$MODERNBASIC_TEST_BINARY" \
    --report "${CARGO_TARGET_DIR:-$project_dir/target}/acceptance/draft-recovery.json"
python3 "$project_dir/tools/check-audio-stream.py" --binary "$MODERNBASIC_TEST_BINARY" \
    --report "$project_dir/target/acceptance/audio-stream.json"
if [ "$verify_ruststrument" -eq 1 ]; then
    (cd "$project_dir/ruststrument" && ./verify.sh)
fi
"$project_dir/smoke-test.sh"
"$project_dir/concurrency-test.sh"
"$project_dir/program-test.sh"
"$project_dir/download-integrity-test.sh"

printf '%s\n' 'ModernBASIC 0.8.2 verification passed.'
