#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
verifier="$project_dir/tools/verify-sha256.sh"
test_dir="$(mktemp -d "${TMPDIR:-/tmp}/modernbasic-integrity.XXXXXX")"
trap 'rm -rf -- "$test_dir"' EXIT HUP INT TERM

sample="$test_dir/asset.bin"
printf '%s' 'trusted ModernBASIC test asset' >"$sample"
expected="$(sha256sum -- "$sample")"
expected=${expected%% *}
"$verifier" "$expected" "$sample" test-asset

printf '%s' 'tampered ModernBASIC test asset' >"$sample"
if "$verifier" "$expected" "$sample" test-asset >/dev/null 2>&1; then
    printf '%s\n' 'error: checksum verifier accepted tampered bytes' >&2
    exit 1
fi
if "$verifier" not-a-digest "$sample" test-asset >/dev/null 2>&1; then
    printf '%s\n' 'error: checksum verifier accepted an invalid digest' >&2
    exit 1
fi

for script in install-soundfont.sh install-rock-soundfonts.sh; do
    grep -Fq 'verify-sha256.sh' "$project_dir/$script"
done

printf '%s\n' 'Download integrity checks passed.'
