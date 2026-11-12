#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$project_dir/tools/test-binary.sh"
catalog_only=0

if [ "${1:-}" = "--catalog-only" ]; then
    catalog_only=1
    shift
fi
if [ "$#" -ne 0 ]; then
    printf '%s\n' 'usage: ./program-test.sh [--catalog-only]' >&2
    exit 2
fi

binary="$(modernbasic_test_binary "$project_dir")"

timeout_seconds="${MODERNBASIC_PROGRAM_TIMEOUT_SECONDS:-3}"
step_limit="${MODERNBASIC_PROGRAM_STEP_LIMIT:-200000}"
case "$timeout_seconds" in
    ''|*[!0-9]*) printf '%s\n' 'error: timeout must be a positive integer.' >&2; exit 2 ;;
esac
case "$step_limit" in
    ''|*[!0-9]*) printf '%s\n' 'error: step limit must be a positive integer.' >&2; exit 2 ;;
esac
if [ "$timeout_seconds" -eq 0 ] || [ "$step_limit" -eq 0 ]; then
    printf '%s\n' 'error: timeout and step limit must be positive.' >&2
    exit 2
fi
if [ "$catalog_only" -eq 0 ] && ! command -v timeout >/dev/null 2>&1; then
    printf '%s\n' 'error: the coreutils timeout command is required for runtime probes.' >&2
    exit 2
fi

scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT HUP INT TERM
report="$project_dir/target/program-test-report.tsv"
expected_error_file="$project_dir/programs/tests/expected-errors.tsv"
mkdir -p "$project_dir/target"
printf 'result\tprogram\tdetail\n' >"$report"

declare -A expected_errors=()
declare -A expected_error_seen=()
declare -A expected_external=()
external_file="$project_dir/programs/tests/expected-external.tsv"
if [ -f "$external_file" ]; then
    while IFS=$'\t' read -r program pattern extra; do
        case "$program" in \#*) continue ;; esac
        if [ -z "$program" ] || [ -z "$pattern" ] || [ -n "${extra:-}" ] || \
           [ -n "${expected_external[$program]+set}" ] || [ ! -f "$project_dir/$program" ]; then
            printf 'error: invalid external-dependency entry: %s\n' "$program" >&2
            exit 2
        fi
        expected_external["$program"]="$pattern"
    done <"$external_file"
fi
if [ -f "$expected_error_file" ]; then
    while IFS=$'\t' read -r expected_program expected_detail extra; do
        if [ -n "${extra:-}" ] || [ -z "${expected_program:-}" ] || \
           [ -z "${expected_detail:-}" ]; then
            printf 'error: malformed expected-error entry in %s\n' \
                "$expected_error_file" >&2
            exit 2
        fi
        case "$expected_program" in
            \#*) continue ;;
        esac
        if [ -n "${expected_errors[$expected_program]+set}" ]; then
            printf 'error: duplicate expected-error program: %s\n' \
                "$expected_program" >&2
            exit 2
        fi
        expected_errors["$expected_program"]="$expected_detail"
    done <"$expected_error_file"
fi

# Samples are read-only assets outside each disposable BASIC file sandbox.
soundfont_options=()
if [ -d "$project_dir/soundfonts" ]; then
    soundfont_options=(--soundfont-root "$project_dir/soundfonts")
fi

catalog_total=0
catalog_failed=0
runtime_total=0
completed=0
bounded=0
interactive=0
external=0
timed=0
expected=0
runtime_failed=0

while IFS= read -r -d '' basic_file; do
    catalog_total=$((catalog_total + 1))
    relative="${basic_file#"$project_dir/"}"
    listing="$scratch/listing-$catalog_total.txt"
    diagnostic="$scratch/listing-$catalog_total.err"
    if "$binary" list <"$basic_file" >"$listing" 2>"$diagnostic"; then
        printf 'catalog-ok\t%s\tloaded and canonically listed\n' "$relative" >>"$report"
    else
        catalog_failed=$((catalog_failed + 1))
        detail="$(tr '\n\t' '  ' <"$diagnostic" | cut -c1-240)"
        printf 'catalog-failed\t%s\t%s\n' "$relative" "$detail" >>"$report"
        continue
    fi

    if [ "$catalog_only" -eq 1 ]; then
        continue
    fi

    runtime_total=$((runtime_total + 1))
    expected_detail="${expected_errors[$relative]-}"
    if [ -n "$expected_detail" ]; then
        expected_error_seen["$relative"]=1
    fi
    case_dir="$scratch/run-$runtime_total"
    mkdir -p "$case_dir/files"
    # OPEN/PRINT#/SAVE/KILL use the file root, not the media output root.
    # Give each probe its own source/fixture copy so tests cannot edit the
    # maintained catalog or leave data that changes a later test's behavior.
    cp -a -- "$(dirname -- "$basic_file")/." "$case_dir/files/"
    runtime_program="$case_dir/files/$(basename -- "$basic_file")"
    stdout="$case_dir/stdout.txt"
    stderr="$case_dir/stderr.txt"
    set +e
    timeout --signal=TERM --kill-after=2s "${timeout_seconds}s" \
        env MODERNBASIC_NO_AUDIO=1 MODERNBASIC_NO_VIDEO=1 \
        "$binary" run \
        --max-steps "$step_limit" \
        --file-root "$case_dir/files" \
        --output-root "$case_dir" \
        "${soundfont_options[@]}" \
        "$runtime_program" </dev/null >"$stdout" 2>"$stderr"
    status=$?
    set -e

    if [ "$status" -ge 128 ] || { [ "$status" -ne 0 ] && [ "$status" -ne 1 ] && [ "$status" -ne 124 ]; }; then
        runtime_failed=$((runtime_failed + 1))
        result='runtime-failed'
        detail="abnormal runtime exit: $status"
    elif [ -n "$expected_detail" ] && [ "$status" -eq 0 ]; then
        runtime_failed=$((runtime_failed + 1))
        result='runtime-failed'
        detail="expected error was not raised: $expected_detail"
    elif [ -n "$expected_detail" ] && grep -Fqi -- "$expected_detail" "$stderr"; then
        expected=$((expected + 1))
        result='expected-error'
        detail="required diagnostic observed: $expected_detail"
    elif [ -n "$expected_detail" ]; then
        runtime_failed=$((runtime_failed + 1))
        result='runtime-failed'
        actual="$(tr '\n\t' '  ' <"$stderr" | cut -c1-180)"
        detail="expected '$expected_detail' but received: $actual"
    elif [ "$status" -eq 0 ]; then
        completed=$((completed + 1))
        result='completed'
        detail='bounded headless run completed'
    elif [ "$status" -eq 124 ]; then
        timed=$((timed + 1))
        result='timed'
        detail="wall-clock guard reached after ${timeout_seconds}s"
    elif grep -Fqi 'execution step limit' "$stderr"; then
        bounded=$((bounded + 1))
        result='bounded'
        detail="statement guard reached at $step_limit steps"
    elif grep -Fqi 'end of input' "$stderr"; then
        interactive=$((interactive + 1))
        result='interactive'
        detail='requires runtime keyboard/input data'
    elif [ -n "${expected_external[$relative]-}" ] && \
         grep -Eqi -- "${expected_external[$relative]}" "$stderr"; then
        external=$((external + 1))
        result='external'
        detail="$(tr '\n\t' '  ' <"$stderr" | cut -c1-240)"
    else
        runtime_failed=$((runtime_failed + 1))
        result='runtime-failed'
        detail="$(tr '\n\t' '  ' <"$stderr" | cut -c1-240)"
    fi
    printf '%s\t%s\t%s\n' "$result" "$relative" "$detail" >>"$report"
done < <(find "$project_dir/programs" -type f -iname '*.bas' -print0 | sort -z)

if [ "$catalog_only" -eq 0 ]; then
    for expected_program in "${!expected_errors[@]}"; do
        if [ -z "${expected_error_seen[$expected_program]+set}" ]; then
            runtime_failed=$((runtime_failed + 1))
            printf 'runtime-failed\t%s\texpected-error manifest entry does not name a catalog program\n' \
                "$expected_program" >>"$report"
        fi
    done
fi

printf 'Program catalog: %s loaded, %s failed.\n' \
    "$((catalog_total - catalog_failed))" "$catalog_failed"
if [ "$catalog_only" -eq 0 ]; then
    printf 'Runtime probes: %s completed, %s step-bounded, %s interactive, %s external, %s timed, %s expected errors, %s failed.\n' \
        "$completed" "$bounded" "$interactive" "$external" "$timed" "$expected" "$runtime_failed"
fi
printf 'Detailed report: %s\n' "$report"

if [ "$catalog_failed" -ne 0 ] || [ "$runtime_failed" -ne 0 ]; then
    exit 1
fi
