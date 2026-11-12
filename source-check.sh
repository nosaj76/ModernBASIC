#!/usr/bin/env bash
set -euo pipefail

project_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
program_root="$project_dir/programs"

while IFS= read -r -d '' script; do
    bash -n "$script"
done < <(find "$project_dir" -maxdepth 1 -type f -name '*.sh' -print0)

while IFS= read -r -d '' script; do
    bash -n "$script"
done < <(find "$project_dir/tools" "$project_dir/appliance" -type f -name '*.sh' -print0)
python3 "$project_dir/tools/check-contract.py"
python3 "$project_dir/tools/check-analytics-vendor.py" --root "$project_dir"
python3 "$project_dir/services/portal/build_content.py" --check
python3 - "$project_dir" <<'PYTHON'
import pathlib, re, sys
root = pathlib.Path(sys.argv[1])
# Audit owned Rust code separately from unchanged/recorded third-party sources.
# Reading all matches avoids find/grep -q SIGPIPE false negatives under pipefail.
for directory in ('src', 'tests'):
    for path in (root / directory).rglob('*.rs'):
        if path.is_relative_to(root / 'src/vendor'):
            continue
        for number, line in enumerate(path.read_text().splitlines(), 1):
            if re.search(r'(^|[^A-Za-z0-9_])(todo!|unimplemented!)\(', line):
                raise SystemExit(f'error: unfinished Rust placeholder: {path}:{number}')
            if re.search(r'^\s*unsafe\s+extern\s+"[^"]*"\s*[{]', line):
                raise SystemExit(f'error: unsafe extern block requires newer than Rust 1.74: {path}:{number}')
# Apply the same full-input check to the BASIC catalog; a short-circuiting
# grep pipeline can miss forbidden statements when its upstream receives SIGPIPE.
removed_statement = re.compile(
    r'^\s*[0-9]+\s+(MUSIC|VOLUME|STRUM|HIT|SID|SYNTH|CHARDEF|TCP[A-Z]*)($|\s)',
    re.IGNORECASE,
)
for path in (root / 'programs').rglob('*'):
    if path.is_file() and path.suffix.lower() == '.bas':
        for number, line in enumerate(path.read_text().splitlines(), 1):
            if removed_statement.search(line):
                raise SystemExit(f'error: removed experimental statement: {path}:{number}')
for directory in ('tools', 'appliance', 'tests', 'services'):
    for path in (root / directory).rglob('*.py'):
        compile(path.read_text(), str(path), 'exec')
# Installed helpers often have no filename extension; inspect their shebang too.
import subprocess
for path in (root / 'appliance').rglob('*'):
    if not path.is_file() or path.suffix or '__pycache__' in path.parts:
        continue
    text = path.read_text()
    first = text.splitlines()[0] if text else ''
    if first.startswith('#!') and 'python' in first:
        compile(text, str(path), 'exec')
    elif first.startswith('#!') and ('/sh' in first or 'bash' in first):
        subprocess.run(['bash', '-n', str(path)], check=True)
PYTHON

grep -F 'name = "modernbasic"' "$project_dir/Cargo.toml" >/dev/null
grep -F 'version = "0.8.2"' "$project_dir/Cargo.toml" >/dev/null
grep -F 'rust-version = "1.74"' "$project_dir/Cargo.toml" >/dev/null

# Only the live waveguide/Santur modules belong to the native instrument
# engine. These names identify superseded prototype or no-op implementations;
# reintroducing one would create a second sound path with different behavior.
for obsolete_audio_module in \
    instrument_synth/additive.rs \
    instrument_synth/banjo_body.rs \
    instrument_synth/instrument_model.rs \
    instrument_synth/modal.rs
do
    if [ -e "$project_dir/src/$obsolete_audio_module" ]; then
        printf 'error: duplicate audio-engine module returned: %s\n' \
            "$obsolete_audio_module" >&2
        exit 1
    fi
done
if grep -Eq \
    'fn render_midi_score_floats|pub fn (render_note|render_strum|render_release|note_on|note_off)' \
    "$project_dir/src/sf_engine.rs"; then
    printf '%s\n' 'error: duplicate sequential SoundFont renderer returned.' >&2
    exit 1
fi
if grep -Eq 'fn articulation_gate' "$project_dir/src/sf_engine.rs"; then
    printf '%s\n' 'error: obsolete sampled-articulation helper returned.' >&2
    exit 1
fi
if grep -Fq '/proc/self/fd/' "$project_dir/src/video.rs"; then
    printf '%s\n' 'error: VIDEO soundtrack transport depends on Linux procfs.' >&2
    exit 1
fi

# True-color constructors must remain expression built-ins. Without this
# dispatch, RGB(...) silently falls through to array lookup and reports
# ?bad subscript in otherwise valid pixel-painting programs.
grep -F '| "RGB"' "$project_dir/src/functions.rs" >/dev/null
grep -F '"RGB" => eval_color_constructor(args, false)' \
    "$project_dir/src/functions.rs" >/dev/null
grep -F 'self.graphics.plot_pixel(x, y, &color)?;' \
    "$project_dir/src/interpreter.rs" >/dev/null
grep -F 'fn rgb_calls_are_functions_not_array_subscripts()' \
    "$project_dir/src/eval.rs" >/dev/null

# A graphics GET must remain an ordinary BASIC array and a reusable dense
# image. These structural markers protect the fast animation path even on a
# machine where a Rust toolchain has not yet been installed.
grep -F 'graphics_image: Option<GraphicsImageCache>' \
    "$project_dir/src/arrays.rs" >/dev/null
grep -F 'self.graphics_image = None;' "$project_dir/src/arrays.rs" >/dev/null
grep -F 'blit_opaque(pixels, width, height, left, top)' \
    "$project_dir/src/graphics.rs" >/dev/null
grep -F '"raster_rle" => parse_inline_raster_rle' \
    "$project_dir/src/artist_renderer.rs" >/dev/null
grep -F 'fn rasterize_exact_device_bitmap(' \
    "$project_dir/src/artist_renderer.rs" >/dev/null

basic_files=0
while IFS= read -r -d '' basic_file; do
    basic_files=$((basic_files + 1))
    if ! awk '
        NF == 0 { next }
        $1 !~ /^[0-9]+$/ { exit 1 }
        $1 > 65529 { exit 1 }
        seen[$1]++ { exit 1 }
        have && $1 <= previous { exit 1 }
        { previous = $1; have = 1 }
        END { if (!have) exit 1 }
    ' "$basic_file"
    then
        printf 'error: invalid numbered BASIC source: %s\n' "$basic_file" >&2
        exit 1
    fi
done < <(find "$program_root" -type f -iname '*.bas' -print0)


printf 'ModernBASIC source checks passed: %s numbered BASIC programs.\n' "$basic_files"
