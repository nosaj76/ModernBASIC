#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$project_dir/tools/test-binary.sh"

binary="$(modernbasic_test_binary "$project_dir")"

[ "$("$binary" version)" = 'ModernBASIC 0.8.2' ]
[ "$("$binary" eval '3000000000+10')" = '3000000010' ]

actual="$("$binary" run "$project_dir/programs/examples/hello.bas")"
printf '%s\n' "$actual" | grep -F 'HELLO FROM MODERNBASIC 0.8.2!' >/dev/null
printf '%s\n' "$actual" | grep -F '3000000010' >/dev/null

conformance="$("$binary" run "$project_dir/programs/examples/basic_conformance.bas")"
[ "$conformance" = 'BASIC CONFORMANCE OK' ]

case_output="$(printf 'DIR\nCD EXAMPLES\nPWD\nDIR *.BAS\nCD /\nTREE EXAMPLES\nQUIT\n' \
    | "$binary" repl --file-root "$project_dir/programs")"
printf '%s\n' "$case_output" | grep -F 'hello.bas' >/dev/null
printf '%s\n' "$case_output" | grep -F 'CURRENT DIRECTORY programs/examples' >/dev/null
printf '%s\n' "$case_output" | grep -F 'DIRECTORY programs/examples' >/dev/null
printf '%s\n' "$case_output" | grep -F 'TREE programs/examples' >/dev/null
! printf '%s\n' "$case_output" | grep -F "$project_dir/programs" >/dev/null
printf '%s\n' "$case_output" | grep -F 'FILE(S)' >/dev/null
printf '%s\n' "$case_output" | grep -F 'DIR(S)' >/dev/null

resume_output="$("$binary" run - <<'BASIC'
10 ON ERROR GOTO 100
20 PRINT 10/D
30 END
100 D=2
110 RESUME
BASIC
)"
[ "$resume_output" = ' 5 ' ]

process_output="$(AMBIENT_SECRET=must-not-leak "$binary" run \
    --env Approved=yes - -- alpha 'two words' <<'BASIC'
10 PRINT COMMAND$
20 PRINT ENVIRON$("approved")
30 PRINT "[";ENVIRON$("AMBIENT_SECRET");"]"
40 END
BASIC
)"
[ "$process_output" = $'alpha two words\nyes\n[]' ]

sing_output="$(MODERNBASIC_NO_AUDIO=1 "$binary" run - <<'BASIC'
10 TEMPO 112
20 SING "Hap-py birth-day to you","G4/8 G4/8 A4/4 G4/4 C5/4 B4/2"
30 PRINT "SING PLANNER OK"
40 END
BASIC
)"
[ "$sing_output" = 'SING PLANNER OK' ]

tmp_dir="$(mktemp -d)"
trap 'rm -rf -- "$tmp_dir"' EXIT HUP INT TERM

"$binary" graph 'SIN(X)' "$tmp_dir/sine.svg" -6.3 6.3 -1.2 1.2
grep -F '<svg' "$tmp_dir/sine.svg" >/dev/null

file_output="$("$binary" run --file-root "$tmp_dir" - <<'BASIC'
10 OPEN 1,"records.csv","WRITE"
20 WRITE# 1,42,"HELLO"
30 CLOSE 1
40 OPEN 2,"records.csv","READ"
50 INPUT# 2,A,B$
60 CLOSE 2
70 PRINT A;B$
80 END
BASIC
)"
[ "$file_output" = ' 42 HELLO' ]
[ "$(cat "$tmp_dir/records.csv")" = '42,"HELLO"' ]

MODERNBASIC_NO_AUDIO=1 "$binary" run --file-root "$tmp_dir" - <<'BASIC'
10 TEMPO 120
20 VOICE 1
30 WAVE "SINE"
40 VOL 1,.35
50 PLAY "C4/8 E4/8 G4/4"
60 WAVEFILE "audio-smoke.wav"
70 END
BASIC
[ -s "$tmp_dir/audio-smoke.wav" ]
[ "$(dd if="$tmp_dir/audio-smoke.wav" bs=1 count=4 2>/dev/null)" = 'RIFF' ]
[ "$(dd if="$tmp_dir/audio-smoke.wav" bs=1 skip=8 count=4 2>/dev/null)" = 'WAVE' ]

if command -v ffmpeg >/dev/null 2>&1 && command -v ffprobe >/dev/null 2>&1; then
    "$binary" run --file-root "$tmp_dir" - <<'BASIC'
10 SCREEN 16,16,"BLACK"
20 VIDEO START "video-smoke.mp4",24,"audio-smoke.wav"
30 FOR F=0 TO 23
40 CLS "BLACK"
50 SPRITE DOT,F MOD 12,F MOD 12,4,4,2
60 VIDEO FRAME
70 NEXT F
80 VIDEO STOP
90 END
BASIC
    [ -s "$tmp_dir/video-smoke.mp4" ]
    video_probe="$(ffprobe -v error -count_frames -select_streams v:0 \
        -show_entries stream=codec_name,width,height,r_frame_rate,nb_read_frames \
        -of csv=p=0 "$tmp_dir/video-smoke.mp4")"
    [ "$video_probe" = 'h264,16,16,24/1,24' ]
    audio_probe="$(ffprobe -v error -select_streams a:0 \
        -show_entries stream=codec_name,sample_rate,channels \
        -of csv=p=0 "$tmp_dir/video-smoke.mp4")"
    [ "$audio_probe" = 'aac,44100,2' ]
else
    MODERNBASIC_NO_VIDEO=1 "$binary" run --file-root "$tmp_dir" - <<'BASIC'
10 SCREEN 16,16,"BLACK"
20 VIDEO START "video-smoke.mp4",24,"audio-smoke.wav"
30 VIDEO FRAME
40 VIDEO STOP
50 END
BASIC
fi

while IFS= read -r -d '' basic_file; do
    if ! awk '
        NF == 0 { next }
        $1 !~ /^[0-9]+$/ { exit 1 }
        seen[$1]++ { exit 1 }
        have && $1 <= previous { exit 1 }
        { previous = $1; have = 1 }
        END { if (!have) exit 1 }
    ' "$basic_file"
    then
        printf 'invalid numbered BASIC source: %s\n' "$basic_file" >&2
        exit 1
    fi
done < <(find "$project_dir/programs" -type f -iname '*.bas' -print0)

if find "$project_dir/programs" -type f -iname '*.bas' -exec grep -Eil \
    '^[[:space:]]*[0-9]+[[:space:]]+(MUSIC|VOLUME|STRUM|HIT|SID|SYNTH|CHARDEF|TCP[A-Z]*)($|[[:space:]])' \
    {} + | grep -q .; then
    printf '%s\n' 'unsupported legacy statement remains in the runnable program catalog' >&2
    exit 1
fi

printf '%s\n' 'ModernBASIC 0.8.2 smoke tests passed.'
