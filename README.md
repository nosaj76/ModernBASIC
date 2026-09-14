# ModernBASIC 0.8.2

ModernBASIC is a native, line-numbered BASIC implementation for modern
64-bit computers. It is written in Rust and executes BASIC through its own
tokenizer, program store, expression evaluator, and statement runtime.

It is not an emulator, ROM wrapper, binary translator, or collection of
selectable dialect modes. It provides one public language: **ModernBASIC**.

## What 0.8.2 contains

- Classic numbered programs and immediate statements at a `READY.` prompt.
- `LIST`, `RUN`, `STEP`, `STOP`, `CONT`, `NEW`, `LOAD`, `SAVE`, `MERGE`, `DELETE`,
  `RENUM`, `AUTO`, `EDIT`, and `VERIFY` program editing.
- Ctrl+Up/Ctrl+Down graphical list-edit navigation for changing stored lines
  in place; F2 remains the explicit LIST/edit shortcut.
- Ctrl+U toggles uppercase and mixed-case keyboard input, and completed
  `RUN`/`CONT` execution prints the classic `READY.` return prompt.
- Checked 64-bit integers, finite binary64 reals, UTF-8 strings, arrays,
  lists, complex values, and bounded program storage.
- `GOTO`, `GOSUB`, `RETURN`, `ON ... GOTO/GOSUB`, `FOR/NEXT`,
  `WHILE/WEND`, `DO/LOOP`, `REPEAT/UNTIL`, loop `EXIT`, single-line and block
  `IF/THEN/ELSE`, `SELECT CASE`, `DATA/READ/RESTORE`, and `MAT` matrix
  operations.
- Validated multi-line `SUB` and value-returning numeric/string `FUNCTION`
  procedures with default/explicit `BYVAL`, explicit `BYREF` scalar and array-
  element aliases, exact-type primitive and typed whole-record array aliases,
  recursion, per-call typed scalars and scalar/array records, persistent typed
  `STATIC` scalars and scalar/array records,
  `SHARED`, early `EXIT`, and calls from running programs or the `READY.` prompt.
- Validated user-defined `TYPE` records with primitive numeric, dynamic/fixed
  string, and nested record fields; forward layout references; scalar and array
  instances; multi-level dotted access; typed I/O; exact-type whole-record and
  subrecord assignment; atomic `REDIM [PRESERVE]`; record-aware `ERASE`; and
  scoped procedure storage.
- Sandboxed sequential file channels, including `#` channel forms.
- Built-in vector and pixel graphics, a retained point-perspective XYZ world for
  pixels, objects, and sprites, retained scene rendering, function graphing,
  buffered three-axis sprite velocity/acceleration, collision queries/traps,
  logical mouse input, nonblocking keys, natural-media brush strokes, common still
  imports, retained flipbooks, local/direct-HTTP media playback, still export,
  and bounded program-generated MP4/WebM/GIF animation.
- Logical text-screen controls including `LOCATE`, `CSRLIN`, `VIEW PRINT`,
  `HOME`, `HTAB`, `VTAB`, `TEXTCOLOR`, Unicode-safe `WIDTH` wrapping,
  key-abortable `SLEEP`, and live bare or parenthesized `INKEY$`.
- Explicit bounded `WEB GET`, `WEB POST`, `WEB DOWNLOAD`, and host-browser
  `WEB OPEN` statements, disabled unless the host grants network access.
- Sixteen-voice native synthesis, SoundFont instruments, WAV export, stereo
  placement, optional eSpeak speech, and timed lyric-to-note singing.
- A native Linux/X11 graphical shell plus a terminal `READY.` fallback.
- No Cargo runtime dependencies: the SoundFont engine is vendored in `src/`.
- A catalog of 365 BASIC source files, including examples, games, music, and tests.

For animation and game-loop guidance, including the optimized static-background
`GET`/`PUT` pattern, see [GRAPHICS_PERFORMANCE.md](GRAPHICS_PERFORMANCE.md).

Optional third-party SoundFont banks are installed separately.
`install-soundfont.sh` obtains the MuseScore General bank and its license;
any other bank must be installed with its own license and attribution. See
[programs/SOUNDFONT_REQUIREMENTS.md](programs/SOUNDFONT_REQUIREMENTS.md).
The Python audio-analysis and instrument-profile utilities under `tools/` are
development helpers and are not part of the BASIC runtime.

## Build

Requirements:

- Rust with Cargo, version 1.74 or newer.
- Linux X11 development files for the graphical shell.
- Optional: `paplay` or `aplay` for live audio and smoothed `SING` playback.
- Optional: eSpeak NG or eSpeak for `SAY` and the source voice used by `SING`.
- Optional: a General MIDI `.sf2` bank for `INSTRUMENT`.
- Optional: FFmpeg for compressed still import/PNG export, local or HTTP(S) `PLAY AUDIO`/`PLAY VIDEO`, and `VIDEO` animation encoding.
- Optional: `curl` for `WEB GET`, `WEB POST`, and `WEB DOWNLOAD`; `xdg-open` plus an installed browser for `WEB OPEN` on Linux.

On Ubuntu:

```sh
sudo apt update
sudo apt install cargo rustc libx11-dev
./build.sh
```

The optimized executable is `target/release/modernbasic`.
Use `./build-native.sh` instead when the binary is intended only for the
computer that builds it; this enables native CPU instructions. See
[docs/MODERN_BASIC_CONTRACT.md](docs/MODERN_BASIC_CONTRACT.md) for the boundary
between compatible BASIC behavior and modern native implementation.

## Start

```sh
# Native graphical READY environment; terminal fallback if X11 is unavailable
./run.sh

# Explicit terminal environment
./run.sh repl

# Run a numbered program
./run.sh run programs/examples/hello.bas

# Evaluate one expression
./run.sh eval 'SQR(81)+2^3'

# Show the complete command-line help
./run.sh help
```

For an appliance that boots from a Linux kernel directly into the fullscreen
ModernBASIC UI, without a desktop or login prompt on the display, see
[docs/APPLIANCE_BOOT.md](docs/APPLIANCE_BOOT.md). Its installer only stages a
separate image root and deliberately refuses to modify the running host root.
For the current VM-first path, `./appliance/build-iso.sh` writes
`target/modernbasic-live.iso`; `./appliance/vm-test.sh` boots an automated ISO
and exercises hardware input plus managed windows, while
`./appliance/iso-smoke-test.sh` boots the final interactive artifact.

## First program

At the `READY.` prompt:

```basic
10 PRINT "MODERNBASIC"
20 FOR I=1 TO 5
30 PRINT I;I^2
40 NEXT I
50 END
LIST
RUN
```

## System hub and concurrent programs

The graphical system hub and the BASIC language are separate environments.
`HELP`, `EDIT SPRITE`, `EDIT FONT`, `TASKS`, `FOCUS`, `STOP`, `CLOSE`,
`RESTART`, and `SHUTDOWN` are hub commands;
they are not BASIC statements. Every program window has its own `READY.`
environment, stored program, variables, input state, graphics screen, interrupt
flag, synth score, speech queue, and PCM playback session.

Opening another program does not suspend an existing one. Several games may
run in separate windows while several other programs play music. A single
program may also queue independent songs or parts on voices 1 through 16 before
`SOUND RESUME`; separate program tasks add separate live audio streams which
the host audio server mixes. `STOP` and `CLOSE` target only the selected task.
Up to 32 program tasks may be open simultaneously; at that boundary the hub
asks you to close a task before opening another.

`./concurrency-test.sh` launches four complete sound-and-graphics programs in
parallel and validates their isolated outputs. `./concurrency-test.sh --live`
uses the real audio device so the four songs intentionally overlap.

`EDIT SPRITE` opens the built-in eight-slot 24x21 image-sprite editor and
`EDIT FONT` opens the 96-glyph 8x8 bitmap-font editor. The exported PPM assets
load directly through `SPRITE IMAGE` and `FONT`, with no host conversion tool.

Modern game framebuffers have named `SCREEN` presets from `HD` (1280x720)
through `FULLHD`, `QHD`, `ULTRAWIDE`, and `UHD`/4K. The point-perspective mode
keeps one bounded in-memory XYZ world for objects, pixels, lights, and sprites.
`SPRITE IMAGE id,path$,(x,y,z),width,height[,key]` maps a load-time-decoded
texture into that world; copies share resident pixels while their transforms,
depth order, lighting, motion, and projected collision bounds remain
independent.

`programs/games/neon_depth_runner.bas` is the playable demonstration. It
generates three portable sprite textures, retains and shares their decoded
pixels, and runs an XYZ obstacle/collection game across selectable HD, Full HD,
QHD, ultrawide, and UHD screens. Run it with:

```sh
./target/release/modernbasic shell programs/games/neon_depth_runner.bas
```

At the graphical `READY.` prompt, enter `RUN`, then press Space or Enter to
start. A/D or Left/Right changes lanes, W/S or Up/Down changes height, Q/E
changes depth, P pauses, and Escape quits.

## Command-line surface

```text
shell [options] [program.bas]       graphical READY environment
repl [run options]                 terminal READY environment
eval <expression>                  evaluate one expression
tokenize <line>                    canonicalize one BASIC line
list                               list source read from standard input
run [options] [program.bas|-] [-- arguments...]  execute a numbered program
graph                              render one function to SVG
graph-multi                        render several functions to SVG
graph-parametric                   render X=f(T), Y=g(T)
graph-polar                        render R=f(T)
graph-scatter                      render paired list data
graph-histogram                    render a list histogram
graph-data                         export sampled functions to CSV
audio-profile                      report host audio and synthesis profile
version                            print the build version
help                               print full help
```

Use `./run.sh help` for arguments and limits.

`SCREEN width,height` creates a logical BASIC graphics device. `SCREEN TEXT`,
`SCREEN GRAPHICS2D`, `SCREEN GRAPHICS3D`, and `SCREEN SPLIT` select its
operating view. Its pixel
coordinates and aspect ratio do not change when the native window is resized
or when the program is moved to another monitor. The shell fits and centers
that device with black letterbox or pillarbox bars by default. Choose a useful
initial host-window size with `--display-profile compact`, `desktop`, `hd`, or
`fullhd`; explicit `--width` and `--height` override the profile, and the result
is clamped to the physical display. `--graphics-scale stretch` is available
when deliberate non-uniform scaling is wanted; `fit` is the stable default.

```sh
modernbasic shell --display-profile hd --graphics-scale fit program.bas
```

For an explicitly bounded process context, repeat `--env NAME=VALUE` and place
the BASIC command tail after `--`. `ENVIRON$` never inherits ambient host
variables:

```sh
./run.sh run --env MODE=production program.bas -- input.dat -v
```

## READY commands

The interactive environment supports numbered-line insertion and deletion plus:

```text
LIST      DELETE    RENUM     AUTO      EDIT      RUN       STEP      CONT
LOAD      SAVE      MERGE     VERIFY    OLD       NEW       SCRATCH
STATUS    LENGTH    DIR       FILES     CAT       CD        PWD
MKDIR     RMDIR     KILL      NAME      RENAME    COPY      EVAL
HELP      MANUAL    QUIT      EXIT      BYE
```

Statements may also be executed directly without a line number. Direct-mode
variables and arrays remain available until the program state is cleared.

## User guide and programmer manual

The [Language Reference](docs/LANGUAGE_REFERENCE.md) is the combined user and
programmer manual, covering the program model, expressions, statements,
graphics, audio, and resource limits. It is also embedded in the executable.
For focused guides, see:

- [Program editor and user files](docs/EDITOR_AND_FILES.md): navigation, editing,
  loading, and saving programs.
- [Audio reference](docs/AUDIO_REFERENCE.md): sound and music syntax.
- [Painting guide](BASIC_PAINTING_GUIDE.md): procedural graphics and image creation.
- [Graphics performance](GRAPHICS_PERFORMANCE.md): animation and game loops.

Press **F1**, or enter `HELP`/`MANUAL` in a graphical window, to open the full
user manual embedded in the executable. Navigate with Up/Down, Page Up/Page
Down, Home/End, or the mouse wheel. Press `/` to search, **F3** for the next
match, and **Escape** or **F1** to close it. Program-window F1 opens help only at
`READY.`, leaving F1 available to running BASIC programs. In `repl`, `HELP`
prints quick help and `MANUAL` or `HELP MANUAL` prints the complete manual.

## Graphical line editing

Press **F2** to list a nonempty stored program and enter list-edit mode, or
press **Ctrl+Up**/**Ctrl+Down** to capture a loaded program automatically and
load its last/first numbered line into the editable input row.
Edit with Left, Right, Home, End, Backspace, and Delete, then press **Enter** to
replace the line. Ordinary **Up** and **Down** always browse command and
numbered-statement history. Press **Escape** to leave list-edit mode; only
**Ctrl+Escape** requests BREAK and returns a running graphics/demo program to
the text screen without closing its window or clearing its BASIC RAM. Clicking
that program window's X terminates the task, closes the window, and releases its
RAM. Ctrl+Left and Ctrl+Right safely move the character cursor.
Page Up and Page Down scroll long listings.

The default BASIC user directory is the packaged `programs` root. `PWD`, `DIR`,
`CD`, and bounded recursive `TREE`/`DIR /S` browsing navigate beneath that root;
`--file-root` selects a different sandbox. `DIR` reports matched files,
subfolders, and byte totals. The host prefix is hidden: the visible root is
`programs`, with children displayed as `programs/examples` or `programs/music`.
`CD /` or `CD \` returns to that root, `CD ..` moves toward it without escaping,
and either slash style works in nested paths. Existing host paths are matched
case-insensitively, so uppercase BASIC input such as `CD EXAMPLES` resolves the
packaged lowercase `examples` directory.
See [docs/EDITOR_AND_FILES.md](docs/EDITOR_AND_FILES.md).

Generated graphics, audio, and video are kept separate from BASIC sources and
data. Packaged runs place `SAVEIMAGE`, `SCREENSHOT`, `RENDER`, `WAVEFILE`,
`VIDEO`, and explicit graph-display results in the root-level `output/` folder;
`--output-root` selects another sandboxed destination.

`LPRINT` appends to the bounded `modernbasic_print.spool` file in that same
output sandbox. It never auto-selects a host printer or writes a shared `/tmp`
file. Embedding hosts may supply an explicit spool path; symbolic-link targets
are rejected and the configured per-file byte limit still applies.

Loading a stored program prepares every declared static asset before the next
`READY.` prompt. SoundFonts are parsed, images are decoded, and local or
HTTP(S) media is retained as immutable bytes. Relative asset paths are anchored
to the loaded `.bas` file's directory and do not move when BASIC later executes
`CD`; the reserved `soundfont/` prefix instead selects the configured backend
bank root. A missing, invalid, or nonliteral declared asset rejects the load
without replacing the previous program; `RUN` uses the retained objects and
does not reopen their source files. Files created during the run, such as `SAVEIMAGE`
and `WAVEFILE` outputs, enter the same in-memory asset set when produced.

For a quick non-audio language check, run:

```sh
./run.sh run programs/examples/basic_conformance.bas
```

It exercises classic control flow, DATA/READ, arrays, virtual memory, virtual
ports, and native WAIT behavior and prints `BASIC CONFORMANCE OK` on success.

## Networking and web access

Networking is disabled by default. Grant it explicitly for a shell, terminal
REPL, or batch run:

```sh
./run.sh shell --network
./run.sh run --network --output-root output program.bas
```

Native compilation is also disabled by default because it invokes Cargo outside
the interpreter's statement and file-size budgets. Trusted sessions may grant
it explicitly:

```sh
./run.sh repl --native-compile
```

The canonical programming interface is small and non-duplicated:

```basic
10 WEB GET "https://example.com/status",STATUS$
20 WEB POST "https://example.com/api","{""ready"":true}",REPLY$
30 WEB DOWNLOAD "https://example.com/image.png","image.png"
40 WEB OPEN "https://www.youtube.com/"
50 PLAY AUDIO "https://example.com/live.mp3"
60 PLAY VIDEO "https://example.com/movie.mp4"
```

Linux and its installed tools retain responsibility for network devices, DNS,
TLS, browser rendering, accounts, and hardware drivers. ModernBASIC validates
HTTP(S) addresses and calls fixed `curl`, `xdg-open`, FFmpeg, or FFprobe
interfaces; it does not implement a second TCP/TLS/browser stack. `WEB GET`
and `WEB POST` return UTF-8 strings, while `WEB DOWNLOAD` writes bounded
binary data beneath the output sandbox. Use
`--max-network-bytes` to change the default 16 MiB response limit.

A YouTube or Netflix address is normally a web page, not a direct video file.
Use `WEB OPEN` so the installed browser handles JavaScript, login state, and
licensed/DRM playback. `PLAY AUDIO` and `PLAY VIDEO` accept direct HTTP(S)
media streams and never bypass site protection.

## Flipbook movie production

ModernBASIC can paint complete still frames, retain their ordered filenames as
one `FLIPBOOK`, project them at a configured frame rate, and encode the exact
same sequence to MP4, WebM, or animated GIF:

```basic
10 FLIPBOOK CREATE MOVIE,"frame####.ppm",1,120,24
20 PLAY FLIPBOOK MOVIE,2
30 VIDEO EXPORT MOVIE,"movie.mp4"
```

Classic `PLAY` note strings remain unchanged. Explicit `PLAY AUDIO source$`,
`PLAY VIDEO source$`, and `PLAY FLIPBOOK ...` select sandboxed local media,
direct HTTP(S) streams, or retained still sequences without introducing another
note or graphics dialect. They do not bypass DRM; full web sites are opened
through `WEB OPEN` and the installed browser.

Three complete BASIC-produced examples are included:

```sh
./run.sh shell programs/demos/stickman_flipbook.bas
./run.sh shell programs/demos/leaves_flipbook.bas
./run.sh shell programs/demos/feather_flipbook.bas
```

The first makes the traditional 72-page paper stickman walk, wave, and raise an
arm. The second paints one detailed leaf entirely in BASIC--there is no `BITMAP`
statement and no imported leaf image--then creates 120 complete 960x540 pages.
Its numbered program builds a depth-scaled, wind-bent serrated path and paints
that mask with two deterministic multi-octave textures, a gradient, clipped
surface sheen, mottling, curved veins, soft brush shadow, and raster sharpening.
The plain neutral canvas intentionally contains no decorative background.
These programs export their stills and animations beneath the configured
output root (normally `output/`). The third constructs a white
feather without an imported image: translucent asymmetric vanes, a curved
rachis, separated diagonal barbs, frayed gaps, downy base wisps, surface sheen,
shadow, and sharpening all follow a strongly wind-bent centerline. One full
end-over-end tumble and two edge-on axial flips reverse the face shading while
position and Z-derived scale animate 120 complete pages. It exports
`one-feather-still.png`, `one-feather-in-wind.gif`, and
`one-feather-in-wind.mp4` beneath the output root.

The renderer also preserves alpha in imported PNG/WebP/TIFF cutouts and opaque
PNG/JPEG/BMP frames, uses premultiplied bilinear resampling, and exports
raster-composited PNG. `BRUSH` supplies hardness, opacity, and flow;
`PAINT TEXTURE` synthesizes repeatable path-clipped raster detail; and `PAINT
FILTER` provides bounded blur, sharpen, and color finishing. These are serious
high-resolution image-production foundations, but they are not a claim that the
current UI duplicates every Photoshop retouching, color-management, typography,
or interactive selection tool. Procedural realism depends on the detail in the
BASIC painting recipe; the flipbook/video path preserves every completed pixel
rather than reducing it to simple vector animation.
## File safety

BASIC file operations resolve beneath an explicit file root. Interactive use
and programs stored under the packaged `programs` tree default to that common
root. A program outside the tree defaults to its containing directory. For a
multi-user installation, give each account a private writable BASIC root and
point every account at the same administrator-owned, read-only instrument
backend:

```sh
mkdir -p "$HOME/ModernBASIC"
modernbasic shell --file-root "$HOME/ModernBASIC" \
  --soundfont-root /usr/share/modernbasic/soundfont
```

Playback queues, selected instruments, envelopes, and effects remain local to
each ModernBASIC process. Only immutable SoundFont files are shared.

`PEEK` and `POKE` operate on a bounded language-owned byte arena. Host memory,
arbitrary machine code, and raw port I/O are not exposed.

## Audio and SoundFonts

Native oscillator music needs no external bank. `modernbasic audio-profile` reports
the available PCM backend and a conservative CPU/memory-based synthesis
arrangement. The sound card receives completed stereo PCM, so oscillator and
voice counts are software-rendering limits rather than sound-card polyphony.
For realistic sampled piano,
strings, brass, woodwinds, bass, and drums, install a SoundFont 2 bank:

```sh
./install-soundfont.sh
cargo run --release -- run programs/examples/soundfont.bas
./install-rock-soundfonts.sh
cargo run --release -- run programs/music/rock/steel_thunder.bas
```

The installers verify pinned SHA-256 digests before parsing or extracting any
downloaded file. An upstream asset change is rejected until its source and
digest are reviewed and deliberately updated.

The general installer obtains the approximately 206 MiB lossless MuseScore General SF2
and its MIT license from MuseScore’s official mirror. The source archive does
not redistribute the bank. The rock installer adds the CC0 FreePats FSBS
distorted Fender rhythm/lead guitar, the compact real-sampled clean Fender lead,
the CC0 Yamaha RBX picked bass, and the five-velocity-layer AVL Red Zeppelin
acoustic drum kit, preserving all upstream licenses. `steel_thunder.bas` is one BASIC
program: it assigns those banks to voice ranges and schedules the complete
performance on one shared timeline. It plays live and creates no audio file.
BASIC can select those banks—or any other legally obtained `.sf2` bank—directly:

```basic
10 SOUNDFONT "soundfont/MuseScore_General.sf2"
20 INSTRUMENT 1,"ACOUSTIC GRAND PIANO"
30 VOL 1,.8
40 STEREO 1,-.15
50 EXPRESSION 1,.9
60 SUSTAIN 1,.7
70 REVERB 1,.25
80 PLAY 1,"C4/4@72 E4/4@86 G4/2@104"
90 END
```

BASIC also exposes per-note velocity plus per-voice ADSR, expression,
modulation, sustain, pitch bend, reverb, chorus, volume, and stereo positioning.
`DSP voice,drive,tone,presence,compression,gate` adds isolated per-voice
amplifier/cabinet processing and articulation control; all five settings are
normalized from 0 through 1.
Native voices use the host-scanned `AUTO` profile by default, and each
program can select 1 through 16 voices with 1 through 8 oscillators per voice
using `SYNTHPROFILE`, `VOICES`, and `OSCILLATORS`. Named profiles include an
explicit deterministic `STANDARD` mode; exact program settings override the
host selection. Each voice
also has `FILTER`, `FILTERADSR`, targeted `LFO`, `GLIDE`, and `DRIVE` controls. `MIXER voice,synth-level,midi-level` selects computer synthesis,
MIDI/SoundFont instruments, or both as an explicitly balanced layer for each
`PLAY` voice. `SOUND` remains the arbitrary-frequency native tone statement.
`SOUNDFONT voice,path$` and `SOUNDFONT first-voice,last-voice,path$` bind
different sampled banks to different voices in the same program. Only an
explicit `WAVEFILE path$` statement writes an audio file.

`programs/music/magnetic_dawn.bas` is a self-contained 65-second late-1960s
voltage-rock instrumental for the extended native engine. Eight voices combine
a three-oscillator boogie bass, pulse-width rhythm chops, rotary organ harmony,
an upfront three-oscillator driven lead with long glides, filter contour, and
vibrato, plus synthesized rock drums
without a SoundFont. It plays live and contains no audio-file export path.

SoundFonts are treated as backend instrument hardware in the dedicated
root-level `soundfont/` directory, independent of a BASIC program's current
directory. `--soundfont-root` selects a separate shared backend for multi-user
installations. Without `SOUNDFONT`, `INSTRUMENT` searches that backend and common
Linux locations for MuseScore General, FluidR3 GM, and TimGM6mb. On
Debian or Ubuntu, `sudo apt install fluid-soundfont-gm` provides a free
MIT-licensed system bank.

Live PCM playback prefers `paplay` and falls back to `aplay`. Each graphical
program task owns independent music and speech playback, so stopping one
window does not cancel another. Set `MODERNBASIC_NO_AUDIO=1` or the compatible
`MODERN64BASIC_NO_AUDIO=1` spelling to suppress live audio and speech during
automated tests. Interactive `LOAD` prepares all static SoundFont, bitmap,
flipbook, audio, video, and soundtrack inputs before returning the prompt, and
live sampled scores stream in bounded chunks instead of rendering the whole
song before its first sound. `WAVEFILE` still produces
deterministic audio output and includes retained `SING` vocals when the program
uses the optional eSpeak backend.

The Full-HD capabilities presentation is a live BASIC program by default. It
demonstrates animated antialiased
geometric type, real sprite movement/collision state, gradients, alpha
compositing, curves, subpixel geometry, program-generated stereo music, and
data-driven graphics:

```sh
./run.sh shell programs/demos/modernbasic_capabilities_presentation.bas
```

Enter `RUN` at the graphical `READY.` prompt. Press Ctrl+Escape to stop the
loop and return to the prompt.

With `RECORDING=0` on source line 20, it reads no WAV or video: the interpreter
calculates every frame from `TIMER` and synthesizes the queued score in memory
from direct `SOUND frequency,voice,duration` tones configured by `WAVE`, `ADSR`,
`VOL`, and `STEREO`; their frequencies are chosen to form the music, while
`PLAY` note notation is not involved. At 15 seconds it resets frame zero,
the wall-clock origin, and the score together. Set `RECORDING=1` to create the
1920x1080, 30 fps H.264/AAC presentation and retain its generated PCM soundtrack
in `output/`. Large capitals, digits, and presentation
punctuation use the built-in geometric display face; tiny and unsupported
printable ASCII retain the compact bitmap fallback. Arbitrary font-family/file
loading remains future typography work and is not claimed as complete.

Run `./audio-test.sh` for a deterministic WAV check and a four-note speaker
test, or `./audio-test.sh --no-play` on a headless machine.

See [docs/AUDIO_REFERENCE.md](docs/AUDIO_REFERENCE.md) for exact syntax.

## Verification

```sh
./verify.sh
./smoke-test.sh
./program-test.sh
```

The release verifier checks source integrity, formatting, every target, all tests,
Clippy with warnings denied, rustdoc, the optimized build, the companion
`ruststrument` project, smoke and concurrency suites, the complete program-catalog
probe, and download integrity. The program test loads and canonically lists all
365 BASIC sources, then gives each successfully listed source a bounded headless
run. Its TSV report distinguishes completed programs, expected errors,
interactive input, external assets, step limits, timeouts, and failures. Set
`MODERNBASIC_PROGRAM_TIMEOUT_SECONDS` or `MODERNBASIC_PROGRAM_STEP_LIMIT` to
raise the guards. The smoke suite also checks WAV generation and rejects
duplicate/out-of-order lines or removed legacy statements in the catalog. See
[docs/VERIFICATION.md](docs/VERIFICATION.md) for the precise status of this
package.

## Important documentation

- [docs/LANGUAGE_REFERENCE.md](docs/LANGUAGE_REFERENCE.md)
- [docs/MODERN_BASIC_CONTRACT.md](docs/MODERN_BASIC_CONTRACT.md)
- [docs/AUDIO_REFERENCE.md](docs/AUDIO_REFERENCE.md)
- [docs/MIGRATION_0.7_TO_0.8.md](docs/MIGRATION_0.7_TO_0.8.md)
- [docs/EDITOR_AND_FILES.md](docs/EDITOR_AND_FILES.md)
- [docs/STATEMENT_CONFORMANCE.md](docs/STATEMENT_CONFORMANCE.md)
- [docs/CLASSIC_BASIC_CAPABILITY_MATRIX.md](docs/CLASSIC_BASIC_CAPABILITY_MATRIX.md)
- [docs/REFERENCE_ALIGNMENT.md](docs/REFERENCE_ALIGNMENT.md)
- [AUDIT_REPORT_0.8.2.md](AUDIT_REPORT_0.8.2.md)
- [AUDIT_REPORT_0.8.1.md](AUDIT_REPORT_0.8.1.md) (baseline provenance)
- [LICENSE_STATUS.md](LICENSE_STATUS.md)
