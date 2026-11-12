#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$project_dir/tools/test-binary.sh"
live=0

if [ "${1:-}" = "--live" ]; then
    live=1
    shift
fi
if [ "$#" -ne 0 ]; then
    printf '%s\n' 'usage: ./concurrency-test.sh [--live]' >&2
    exit 2
fi

binary="$(modernbasic_test_binary "$project_dir")"

scratch="$(mktemp -d)"
pids=''
cleanup() {
    for pid in $pids; do
        kill "$pid" 2>/dev/null || true
    done
    rm -rf -- "$scratch"
}
trap cleanup EXIT HUP INT TERM

run_task() {
    task="$1"
    frequency="$2"
    task_root="$scratch/task-$task"
    mkdir -p "$task_root"
    if [ "$live" -eq 0 ]; then
        export MODERNBASIC_NO_AUDIO=1
    else
        unset MODERNBASIC_NO_AUDIO || true
    fi
    "$binary" run --file-root "$task_root" --output-root "$task_root" - \
        >"$task_root/stdout.txt" 2>"$task_root/stderr.txt" <<BASIC
10 SYNTHPROFILE "STANDARD"
20 SCREEN 160,100,"BLACK"
30 WAVE 1,"SINE"
40 VOL 1,.18
50 PLAY 1,"C4/8 E4/8 G4/8 C5/8 G4/8 E4/8 C4/4"
60 SOUND $frequency,2,2000,.12
70 SOUND RESUME
80 FOR F=0 TO 239
90 CLS "BLACK"
100 LINE 0,F MOD 100,159,(F*3) MOD 100,"CYAN"
110 NEXT F
120 WAVEFILE "task-$task.wav"
130 PRINT "TASK $task COMPLETE"
140 END
BASIC
}

# Four complete interpreter processes run together. Each owns its source,
# variables, raster, synth score, output root, and playback session.
for specification in '1 220' '2 277.18' '3 329.63' '4 440'; do
    run_task $specification &
    pids="$pids $!"
done

failed=0
for pid in $pids; do
    if ! wait "$pid"; then
        failed=1
    fi
done
pids=''
if [ "$failed" -ne 0 ]; then
    for diagnostic in "$scratch"/task-*/stderr.txt; do
        [ ! -s "$diagnostic" ] || { printf '%s:\n' "$diagnostic" >&2; sed -n '1,40p' "$diagnostic" >&2; }
    done
    exit 1
fi

for task in 1 2 3 4; do
    task_root="$scratch/task-$task"
    grep -Fx "TASK $task COMPLETE" "$task_root/stdout.txt" >/dev/null
    wav="$task_root/task-$task.wav"
    [ -s "$wav" ]
    [ "$(dd if="$wav" bs=1 count=4 2>/dev/null)" = 'RIFF' ]
    [ "$(dd if="$wav" bs=1 skip=8 count=4 2>/dev/null)" = 'WAVE' ]
done

if [ "$live" -eq 1 ]; then
    printf '%s\n' 'Four concurrent sound-and-graphics programs completed; their songs should have overlapped.'
else
    printf '%s\n' 'Four concurrent sound-and-graphics programs completed with isolated WAV outputs.'
fi
