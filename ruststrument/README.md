# Ruststrument 2.0.0

Ruststrument is an independent Rust library and command-line Forge for creating,
loading, analyzing, and rendering realistic musical instruments. Its native
`.rsti` bank format combines bounded samples with physical and resonator models.

The library is intentionally separate from any host language. ModernBASIC can
link it as a path dependency, while other applications can use the Rust API or
the stable C ABI in `src/lib.rs`.

## Build

Rust 1.75 or newer is required. The crate deliberately uses Rust 2021 rather
than the Rust 2024 edition so it builds with the toolchain shipped on the
target ModernBASIC system.

```sh
cargo build --release
cargo test
```

The release executable is `target/release/ruststrument`. The same build also
produces static and dynamic libraries.

## Quick start

```sh
# Create a compact physical-model instrument without samples.
target/release/ruststrument generate "Five-string Banjo" banjo.rsti

# Inspect and audition it.
target/release/ruststrument inspect banjo.rsti
target/release/ruststrument play banjo.rsti 0.5 C4,D4,E4,G4

# Learn from a directory of labeled recordings.
target/release/ruststrument create samples/banjo "Five-string Banjo" learned-banjo.rsti

# Compare a reference recording with a rendered result.
target/release/ruststrument compare reference.wav rendered.wav
```

Run `ruststrument` without arguments for the complete command reference.

## Embedding

The preferred Rust API is:

```rust
use ruststrument::{profile::InstrumentBank, synth::Engine};

let bank = InstrumentBank::load("banjo.rsti")?;
let mut engine = Engine::new(bank, 44_100);
engine.note_on_for(60, 100, Some(22_050));
let mut mono = vec![0.0_f32; 44_100];
engine.render_into(&mut mono);
# Ok::<(), std::io::Error>(())
```

For a sandboxed host, read a bounded asset yourself and call
`InstrumentBank::from_bytes`. This prevents the library from bypassing the
host's path rules.

## Architecture

- `profile`: versioned `.rsti` serialization, regions, generator catalog
- `synth`: real-time note engine and physical/hybrid rendering
- `physical`, `stringed`, `technique`: instrument construction and articulation
- `analysis`, `transcribe`, `classify`: recording analysis and transcription
- `sf2`, `sfz`: interchange export
- `lib.rs`: Rust API, audio decoder, and stable C ABI
- `main.rs`: Ruststrument Forge command-line interface

The files in `tools/` are compatibility launchers. They delegate DSP to the
Rust implementation instead of maintaining divergent Python algorithms.
