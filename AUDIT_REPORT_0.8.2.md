# ModernBASIC 0.8.2 audit report

Date: 2026-08-19

## Requested outcomes

This maintenance release addresses four user-visible requirements:

1. Navigate a graphical `LIST` and edit an existing numbered line without
   retyping it.
2. Use the packaged `programs` directory as the default BASIC user root.
3. Show only BASIC-visible `programs/...` paths and provide complete browsing.
4. Port the attached program catalog to current statements and repair native
   Linux audio playback without adding the removed statement families.

The release pass also audited the executable statement dispatcher for
placeholder behavior and incorrect native-hardware adaptation.

## Editor result

The X11 shell now has a list-edit state. F2 emits `LIST` and captures a
nonempty canonical program. Ctrl+Up or Ctrl+Down can also initiate that capture
automatically, so a loaded program navigates without requiring F2 first. The
requested stored line is then placed in the existing character editor.
Ordinary Up and Down browse command and numbered-statement history. Left,
Right, Home, End, Backspace, and Delete edit a selected line; Enter submits the
normal numbered-line replacement; Escape leaves list-edit mode. Ctrl+Left and
Ctrl+Right remain safe cursor movement instead of aliasing control-letter
shortcuts. Every cached program index is checked before use. The implementation
therefore reuses the authoritative program insertion path instead of
maintaining a second editor-side program.

Non-numbered command transitions invalidate the editor cache so `NEW`, `LOAD`,
`RUN`, and related actions cannot expose stale program lines. Function-key
commands are accepted only at `READY.` and are retained in history. Ctrl+Escape
is the sole BREAK shortcut and immediately returns a graphics/demo view to the
text screen so the `BREAK` result is visible; plain Escape remains local
cancellation/view return without interrupting BASIC. Ctrl+U toggles uppercase
input and is now identified in the status bar. READY state is delivered
explicitly by the REPL instead of inferred from displayed text, preventing
program output from enabling command-only keyboard actions. `RUN` and `CONT`
also print the classic `READY.` return line after completion, STOP/BREAK, or an
error, without attaching it to an unterminated PRINT line.

The CLI form `shell program.bas`, previously parsed but ignored by the X11
shell, now loads the requested program.

The launcher also records a source-revision stamp beside the optimized binary.
`run.sh` rebuilds when that stamp is absent or differs from the packaged source,
so replacing a same-version source tree cannot silently reuse an older 0.8.2
binary. `build.sh` now anchors its build in the project directory regardless of
the caller's current working directory.

## Filesystem result

Without an explicit `--file-root`, the launcher and executable discover the
packaged lowercase `programs` directory and use it as the sandbox root. A
program opened beneath that tree retains the common `programs` root instead of
being confined to the program's immediate subdirectory. Explicit file roots
remain authoritative.

Existing path components are matched case-insensitively inside the canonical
root, so uppercase BASIC input can enter lowercase Linux directories and open
lowercase files. Exact host spelling wins when present; an ambiguous folded
match is rejected; canonicalization after every component preserves the root
and symlink-escape boundary. New outputs retain the requested spelling, while
an existing folded match is reused instead of creating a case-only duplicate.

The READY file browser reports only BASIC-visible paths, sorts subfolders before
files, and prints matched file/directory counts and byte totals. Its root is
shown as `programs`, a child as `programs/music`, and no `/home/...` host prefix
is exposed. Displayed paths are valid input to every file command. `CD /`,
`CD \`, `CD ..`, compact `CD..`, forward/backslash nested
paths, quoted paths, wildcard `DIR`, and bounded recursive `TREE` browsing are
covered; `DIR /S` is the recursive listing alias. Parent navigation clamps at
the BASIC root. Recursive browsing has
fixed depth and entry ceilings, reports links without following them, and
retains the same canonical root boundary.

## Program catalog result

The attached catalog was imported without its generated WAV recordings. Its
210 runnable `.bas` files now have unique, strictly increasing line numbers and
use the current 0.8 statement set. The obsolete `MUSIC`, `VOLUME`,
`STRUM`, `HIT`, `SID`, `SYNTH`, `CHARDEF`, and TCP commands were removed from
runnable sources rather than added to the interpreter. `SOUNDFONT` now has one
bounded meaning: select a sandboxed `.sf2` bank before `INSTRUMENT` and `PLAY`.
General MIDI expression, modulation, sustain, pitch bend, reverb, chorus, and
note velocity are available as bounded BASIC controls.
The menu music, chat, glyph editor, calculator, Game of Life, drone, bowl, and
Canon programs were repaired or ported as required.

The 130 programs under `programs/incompatible` remain preserved source
material. They depend on foreign syntax or host facilities and are not included
in the runnable claim. The smoke test enforces both the numbered-line invariant
and absence of the removed statement families outside that directory.

## Audio result

Native oscillator playback previously invoked only `aplay`, unlike the
SoundFont path. Both now use the same Linux strategy: prefer `paplay` for
PulseAudio/PipeWire and fall back to ALSA `aplay`. Headless verification creates
and checks a stereo RIFF/WAVE through BASIC `WAVEFILE`; `audio-test.sh` adds an
optional live four-note acceptance check.

## Statement findings and repairs

### WAIT

`WAIT` was the sole explicit `NotImplemented` statement branch. It now has two
native, bounded forms:

- `WAIT milliseconds` performs an interruptible delay of at most one hour.
- `WAIT port,mask[,xor]` implements the classic masked wait against the
  language-owned 65,536-byte virtual port bank.

Neither form executes privileged x86 port instructions.

### WHILE / WEND

The earlier implementation did not jump over a false initial loop body and
created another loop frame each time a true loop repeated. 0.8.2 validates
loop pairing, skips through the matching `WEND`, supports nesting, reuses the
active frame, removes it when the condition becomes false, and diagnoses
unmatched constructs.

### Classic expressions

Microsoft/BBC integer division (`\` and `DIV`), `XOR`/BBC `EOR`, rounded real
operands for integer operations, bare `RND`, `BIN$`, and the `FACTORIAL` alias
now execute through checked native arithmetic and have regression coverage.

### Inline and block IF

Inline `ELSE` now selects exactly one colon-aware branch and accepts numeric
alternate targets. Multi-line `IF...THEN`, `ELSE`, and
`END IF`/`ENDIF`/`IFEND` blocks nest and are structurally validated before
execution.

### Structured loops and selection

`DO`/`LOOP` accepts top- or bottom-tested `WHILE` and `UNTIL`; BBC
`REPEAT`/`UNTIL` shares the bounded loop stack. Structured `EXIT` unwinds the
nearest requested `DO`, `FOR`, `WHILE`, or `REPEAT` frame and jumps past its
matching terminator. Standard `SELECT CASE` supports lists, ranges, relational
clauses, defaults, strings, and nesting; BBC `CASE...OF`, `WHEN`, `OTHERWISE`,
and `ENDCASE` are accepted. The supplied `test_select.bas` now passes all six
checks.

### Dynamic arrays and constants

`REDIM` now resets and resizes numeric or string arrays, while `REDIM PRESERVE`
retains the complete overlapping coordinate region of same-rank shapes.
Multi-array statements, logical-capacity accounting, and sparse stored payloads
commit atomically. `ERASE` now releases retained-value charges and the full
reserved capacity, including arrays reshaped by `MAT`.

`CONST` creates immutable calculated numeric or string scalars. Comma-separated
declarations are transactional, later initializers can reference earlier
constants, `CLEAR` preserves them, and scalar assignment paths reject mutation
before consuming runtime input where applicable.

### WRITE#

`WRITE#` previously delegated to `PRINT#`, producing print-zone spaces rather
than reusable BASIC data. It now uses the `WRITE` serializer: comma-delimited
fields, quoted strings, doubled embedded quotes, preserved empty fields, and
depth-aware expressions containing commas.

### Structured procedures

Stored programs now support validated `SUB`/`END SUB` declarations and
`CALL name(arguments)`. Calls use bounded frames, value parameters, recursive
local scalar scopes, explicit scalar `SHARED`, persistent scalar `STATIC`, and
`EXIT SUB`. Classic `GOSUB` remains usable from a procedure. Illegal entry into
or `GOTO` escape from procedure bodies is rejected, control blocks cannot cross
a procedure boundary, failed frame binding is atomic and reaches `ON ERROR`,
and local storage charges are released on return.

### ON ERROR / RESUME

The earlier trap disabled itself permanently on the first error, plain
`RESUME` did not retry the failed statement, and `ON ERROR GOTO 0` attempted to
branch to nonexistent line zero. The repaired state machine distinguishes an
active handler from its configured target, restores loop depth, implements
plain and `NEXT` resume positions, rearms the handler for later faults, blocks
recursive trapping inside the handler, and disables cleanly at target zero.

### Token/dispatch consistency

Inverse text mappings were completed for `NAME`, `LINE INPUT`, and the internal
default-type token. A regression enumerates the statement-token surface so a
future token cannot silently lose its canonical text mapping.

## Native x86-64 boundary

Classic hardware-looking operations remain executable through checked runtime
devices:

- `PEEK`/`POKE`: 1 MiB virtual memory arena.
- `INP`/`OUT`/classic `WAIT`: 65,536-byte virtual port bank.
- `SYS`/`CALL`/`USR`: explicitly registered host service table only.
- `CMD`: open sandboxed BASIC channel only.
- File operations: configured root with traversal and symlink escape rejection.

This is a native x86-64 program, not a 6502, 8080, 8086, or Commodore emulator.
The virtual devices preserve useful BASIC semantics without granting raw access
to host memory, machine code, or privileged CPU instructions.

## Verification status

On 2026-08-22 this workspace passed the complete 0.8.2 `./verify.sh` sequence:
repository-wide formatting, all-target checking, all 559 unit and integration
tests, Clippy with warnings denied, warning-strict private-item rustdoc, and the
locked optimized release build. The packaged `./smoke-test.sh` catalog/runtime
checks and headless WAV check also passed. Automated X11 keyboard/navigation,
continuous-graphics, Ctrl+Escape, and window-shutdown acceptance passed, and the
live four-note playback command completed through the available host audio
stack.
