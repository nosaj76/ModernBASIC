#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$project_dir/tools/test-binary.sh"

binary="$(modernbasic_test_binary "$project_dir")"

audio_tmp="$(mktemp -d)"
trap 'rm -rf -- "$audio_tmp"' EXIT HUP INT TERM

MODERNBASIC_NO_AUDIO=1 "$binary" run --file-root "$audio_tmp" - <<'BASIC'
10 WAVE 1,"SINE"
20 ADSR 1,5,60,.7,120
30 VOL 1,.35
40 SOUND 261.63,1,250
50 SOUND 329.63,1,250
60 SOUND 392,1,250
70 SOUND 523.25,1,500
80 WAVEFILE "modernbasic-audio-test.wav"
90 END
BASIC

wav="$audio_tmp/modernbasic-audio-test.wav"
if [ ! -s "$wav" ] || \
   [ "$(dd if="$wav" bs=1 count=4 2>/dev/null)" != 'RIFF' ] || \
   [ "$(dd if="$wav" bs=1 skip=8 count=4 2>/dev/null)" != 'WAVE' ]; then
    printf '%s\n' 'error: the synthesizer did not produce a valid WAV file' >&2
    exit 1
fi
printf '%s\n' 'WAV synthesis passed.'

if [ "${1:-}" = '--no-play' ]; then
    exit 0
fi
if ! command -v paplay >/dev/null 2>&1 && ! command -v aplay >/dev/null 2>&1; then
    printf '%s\n' 'error: install paplay (PulseAudio/PipeWire) or aplay (ALSA) for live audio' >&2
    exit 1
fi

printf '%s\n' 'Playing a four-note live audio test...'
"$binary" run --file-root "$audio_tmp" - <<'BASIC'
10 WAVE 1,"SINE"
20 ADSR 1,5,60,.7,120
30 VOL 1,.35
40 SOUND 261.63,1,250
50 SOUND 329.63,1,250
60 SOUND 392,1,250
70 SOUND 523.25,1,500
80 SOUND RESUME
90 WAIT 1400
100 SOUND OFF
110 END
BASIC
printf '%s\n' 'Live playback command completed. You should have heard four notes.'
