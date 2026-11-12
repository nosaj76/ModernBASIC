# Host integration contract

Ruststrument remains an independent crate. A host such as ModernBASIC should
link it instead of copying its synthesizer implementation.

## Rust dependency

Place the complete crate at `vendor/ruststrument`, then add:

```toml
[dependencies]
ruststrument = { path = "vendor/ruststrument" }
```

## Safe bank loading

The host owns path resolution and byte limits:

1. Resolve the `.rsti` path through the host's asset sandbox.
2. Read no more than the host's configured asset limit.
3. Call `InstrumentBank::from_bytes(&bytes)`.
4. Retain the validated bank and construct one `Engine` per logical voice.

This prevents a library load from escaping the BASIC program's asset root.

## BASIC surface

No duplicate music command family is needed. The intended overload is:

```basic
10 INSTRUMENT 1,"instrument/banjo.rsti"
20 TEMPO 120
30 PLAY 1,"C4/8 E4/8 G4/4"
```

Strings ending in `.rsti` select a Ruststrument bank. Existing General MIDI
names and numeric programs retain their current SoundFont behavior.

## Rendering

- Convert scheduled BASIC notes to MIDI 0 through 127 and velocity 1 through 127.
- Use `Engine::note_on_for` for bounded-duration score events.
- Render with `Engine::render_into` at the host sample rate.
- Mix Ruststrument PCM with native and SoundFont PCM before playback or WAV export.
- Keep one engine per BASIC voice so articulation, sympathetic strings, and
  release state do not leak between instruments or program tasks.
- Do not use the process-global C ABI from a multi-task interpreter; that ABI is
  for simple single-engine C hosts. ModernBASIC should use the Rust API.

## Lifecycle

- `LOAD`: parse literal `.rsti` assets before committing the program.
- `RUN`: select already prepared banks; never reopen stored-program assets.
- `CLEAR` and `SOUND OFF`: release notes, queued scores, and per-voice engines.
- Direct mode may prepare an evaluated path on demand under the same sandbox.

