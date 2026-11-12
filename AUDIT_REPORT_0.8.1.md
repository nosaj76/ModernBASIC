# ModernBASIC attachment audit and 0.8.1 repair

Date: 2026-08-19

## Inputs

- Legacy 0.7.5 source archive
- Legacy 0.8.0 source archive
- `BASIC_documents.tar(4).gz`

## Version identity

The file labeled 0.7.5 declares package version 0.7.6. The 0.8 archive declares
0.8.0 and contains three repository revisions, followed by additional
working-tree changes in:

- `src/interpreter.rs`
- `src/sf_engine.rs`
- `src/synth.rs`
- `tests/integration_tests.rs`

It also contains two untracked spectrum-analysis scripts. Those actual bytes,
not an earlier assistant copy, were used as the repair baseline.

## Packaging findings

The 0.8 archive contains about 1.8 MiB of non-repository project files but about
295 MiB after its embedded `.git` database is counted. Relative to its own
recorded revision, 333 tracked documentation, program, test-program, and asset
paths are absent.

Those missing paths were not restored wholesale. Many of the recorded music
programs and documents use `MUSIC`, `SOUNDFONT`, `VOLUME`, voice 0, or a compact
`PLAY` grammar that conflicts with the attached current source. Restoring them
as runnable 0.8 examples would create a larger but internally inconsistent
package.

0.8.1 instead provides a clean source release with current documentation and
small examples. Large external SoundFont banks remain optional.

## Code defects repaired

### SoundFont voice scheduling

The attached code indexed a 1-based BASIC voice directly into a zero-based
position array. Voice 1 therefore used slot 1 rather than slot 0, while voice
16 was clamped into slot 15. A `Vn` token calculated a new channel but discarded
it, so later notes remained on the original MIDI channel. The final cursor was
also written back to the original voice regardless of voice switches.

0.8.1 centralizes SoundFont `PLAY` scheduling in `SfEngine`, validates voice and
tempo inputs, maintains an independent cursor for every voice, and switches
both cursor and channel on `Vn`. A regression test checks two voices that switch
away and back.

### Stereo preservation

SoundFont rendering set MIDI pan but then averaged left and right into mono and
duplicated that sample. `STEREO` and `PAN` therefore could not produce stereo
SoundFont output. 0.8.1 encodes distinct left and right samples for WAV and live
output and includes a channel-preservation test.

### Playback portability and memory

The attached working tree replaced `aplay` with a hard dependency on `paplay`.
0.8.1 prefers `paplay` but falls back to `aplay`, reporting both failures if
neither exists.

Peak-normalized live SoundFont playback previously created a full float song
and a second full PCM byte buffer before writing. It now retains the float
render needed for peak measurement but converts and writes PCM in bounded
2,048-frame blocks.

### Lint and notation corrections

- Removed two unused mastering constants.
- Corrected an unused SoundFont volume binding.
- Replaced a lint-sensitive one-arm `match` with `if`.
- Used `strip_suffix` for dotted denominator notation.
- Added a test for `C4/8.`.

## Reference-document conclusion

The reference archive combines manuals and audits from several generations and
at least two architectures. It is valuable engineering evidence, but it is not
a single normative specification. 0.8.1 follows the attached ModernBASIC source's
one-dialect direction and records conflicts explicitly rather than merging old
claims into the code.

## Remaining release gate

Rust/Cargo were unavailable in the preparation environment. The full 492-test,
Clippy, rustdoc, and optimized-build pipeline must be run on the target machine.
The included `verify.sh` and `smoke-test.sh` make that gate reproducible.

