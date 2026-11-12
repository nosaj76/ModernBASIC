// db.rs — Instrument database: synth emulator + layered sample engine
//
// The synth emulator generates sound from physics (waveguide + additive).
// It is NOT layered — one adjustable voice.
//
// The sample engine stores real note recordings, stacked in layers.
// Each layer has its own volume control.
//
// At playback: synth generates the base tone, samples stack on top
// to add realism the synth can't capture. The synth_blend controls
// how much of each you hear. Each sample layer has its own volume.

use crate::wav::{self, Wav};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

fn note_name(midi: u8) -> String {
    format!("{}{}", NOTE_NAMES[midi as usize % 12], midi as i16 / 12 - 1)
}

fn velocity_band(v: u8) -> &'static str {
    if v <= 42 {
        "soft"
    } else if v <= 84 {
        "medium"
    } else {
        "hard"
    }
}

// ─── Synth emulator parameters (single voice, adjustable, not layered) ──

#[derive(Clone, Debug)]
pub struct SynthParams {
    pub harmonic_weights: [f32; 8], // H1-H8 relative amplitude
    pub attack_ms: f32,
    pub decay_s: f32,
    pub sustain: f32,
    pub release_s: f32,
    pub noise_level: f32, // pick/finger noise
    pub brightness: f32,  // waveguide brightness
    pub damping: f32,     // waveguide damping
    pub volume: f32,      // 0-1, overall synth volume
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            harmonic_weights: [1.0, 0.7, 0.52, 0.35, 0.22, 0.12, 0.06, 0.03],
            attack_ms: 5.0,
            decay_s: 0.3,
            sustain: 0.5,
            release_s: 0.15,
            noise_level: 0.04,
            brightness: 0.9,
            damping: 0.17,
            volume: 0.5,
        }
    }
}

// ─── Sample layer (one recording of one note, with volume) ──────────────

#[derive(Clone, Debug)]
pub struct SampleLayer {
    pub velocity: u8,
    pub articulation: String,
    pub round_robin: u8,
    pub source_file: String,
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub volume: f32, // 0-1, per-layer volume control
}

// ─── Note slot (all layers for one note) ───────────────────────────────

#[derive(Clone, Debug)]
pub struct NoteSlot {
    pub midi: u8,
    pub layers: Vec<SampleLayer>,
}

// ─── Characteristic layer (body resonance, pick transient, etc.) ───────
// These are sample-based overlays that stack on top of everything.

#[derive(Clone, Debug)]
pub struct Characteristic {
    pub name: String,
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub volume: f32, // 0-1, per-characteristic volume
}

// ─── The database ───────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct InstrumentDB {
    pub name: String,
    pub family: String,
    pub root_dir: PathBuf,
    pub source_recordings: Vec<String>,
    pub synth: SynthParams, // single synth voice, adjustable
    pub synth_volume: f32,  // 0-1, master synth volume
    pub characteristics: Vec<Characteristic>,
    pub notes: BTreeMap<u8, NoteSlot>,
    pub sample_volume: f32, // 0-1, master sample volume
    pub synth_blend: f32,   // 0=pure sample, 1=pure synth
}

impl Default for InstrumentDB {
    fn default() -> Self {
        Self {
            name: String::new(),
            family: String::new(),
            root_dir: PathBuf::new(),
            source_recordings: Vec::new(),
            synth: SynthParams::default(),
            synth_volume: 0.5,
            characteristics: Vec::new(),
            notes: BTreeMap::new(),
            sample_volume: 0.7,
            synth_blend: 0.3,
        }
    }
}

impl InstrumentDB {
    pub fn create(root: impl AsRef<Path>, name: &str, family: &str) -> io::Result<Self> {
        let root_dir = root.as_ref().join(name);
        fs::create_dir_all(&root_dir)?;
        fs::create_dir_all(root_dir.join("notes"))?;
        fs::create_dir_all(root_dir.join("characteristics"))?;
        let db = Self {
            name: name.to_string(),
            family: family.to_string(),
            root_dir,
            ..Default::default()
        };
        db.save_metadata()?;
        db.save_synth_params()?;
        Ok(db)
    }

    pub fn load(root: impl AsRef<Path>) -> io::Result<Self> {
        let meta_path = root.as_ref().join("metadata.json");
        if !meta_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "no metadata.json"));
        }
        let text = fs::read_to_string(&meta_path)?;

        let mut db = Self {
            name: extract_json_string(&text, "name"),
            family: extract_json_string(&text, "family"),
            root_dir: root.as_ref().to_path_buf(),
            synth_blend: extract_json_f32(&text, "synth_blend").unwrap_or(0.3),
            synth_volume: extract_json_f32(&text, "synth_volume").unwrap_or(0.5),
            sample_volume: extract_json_f32(&text, "sample_volume").unwrap_or(0.7),
            ..Default::default()
        };

        // Load note samples
        let notes_dir = db.root_dir.join("notes");
        if notes_dir.exists() {
            for entry in fs::read_dir(&notes_dir)? {
                let entry = entry?;
                let note_dir = entry.path();
                let note_str = entry.file_name().to_string_lossy().to_string();
                if let Some(midi) = parse_note_dir(&note_str) {
                    let mut slot = NoteSlot {
                        midi,
                        layers: Vec::new(),
                    };
                    load_note_layers(&note_dir, &mut slot)?;
                    if !slot.layers.is_empty() {
                        db.notes.insert(midi, slot);
                    }
                }
            }
        }

        // Load characteristics
        let char_dir = db.root_dir.join("characteristics");
        if char_dir.exists() {
            for entry in fs::read_dir(&char_dir)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                let name = name.trim_end_matches(".wav").to_string();
                let w = wav::read(entry.path())?;
                let vol = extract_json_f32(&text, &format!("char_{}_volume", name)).unwrap_or(0.5);
                db.characteristics.push(Characteristic {
                    name,
                    samples: w.samples,
                    sample_rate: w.sample_rate,
                    volume: vol,
                });
            }
        }

        // Load synth params
        let synth_path = db.root_dir.join("synth_params.json");
        if synth_path.exists() {
            let st = fs::read_to_string(&synth_path)?;
            let mut hw = [1.0f32; 8];
            if let Some(pos) = st.find("\"harmonic_weights\"") {
                let rest = &st[pos..];
                if let Some(arr_start) = rest.find('[') {
                    let arr_end = rest[arr_start..]
                        .find(']')
                        .map(|i| arr_start + i)
                        .unwrap_or(rest.len());
                    for (i, val) in rest[arr_start + 1..arr_end].split(',').enumerate() {
                        if i >= 8 {
                            break;
                        }
                        hw[i] = val.trim().parse::<f32>().unwrap_or(1.0);
                    }
                }
            }
            db.synth = SynthParams {
                harmonic_weights: hw,
                attack_ms: extract_json_f32(&st, "attack_ms").unwrap_or(5.0),
                decay_s: extract_json_f32(&st, "decay_s").unwrap_or(0.3),
                sustain: extract_json_f32(&st, "sustain").unwrap_or(0.5),
                release_s: extract_json_f32(&st, "release_s").unwrap_or(0.15),
                noise_level: extract_json_f32(&st, "noise_level").unwrap_or(0.04),
                brightness: extract_json_f32(&st, "brightness").unwrap_or(0.9),
                damping: extract_json_f32(&st, "damping").unwrap_or(0.17),
                volume: extract_json_f32(&st, "volume").unwrap_or(0.5),
            };
        }

        Ok(db)
    }

    fn save_metadata(&self) -> io::Result<()> {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"name\": \"{}\",\n", self.name));
        s.push_str(&format!("  \"family\": \"{}\",\n", self.family));
        s.push_str(&format!("  \"synth_blend\": {},\n", self.synth_blend));
        s.push_str(&format!("  \"synth_volume\": {},\n", self.synth_volume));
        s.push_str(&format!("  \"sample_volume\": {},\n", self.sample_volume));
        s.push_str("  \"source_recordings\": [");
        for (i, r) in self.source_recordings.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&format!("\"{}\"", r));
        }
        s.push_str("],\n");
        for c in &self.characteristics {
            s.push_str(&format!("  \"char_{}_volume\": {},\n", c.name, c.volume));
        }
        s.push_str("}\n");
        fs::write(self.root_dir.join("metadata.json"), s)
    }

    fn save_synth_params(&self) -> io::Result<()> {
        let sp = &self.synth;
        let s = format!(
            "{{\n  \"harmonic_weights\": [{}, {}, {}, {}, {}, {}, {}, {}],\n  \"attack_ms\": {},\n  \"decay_s\": {},\n  \"sustain\": {},\n  \"release_s\": {},\n  \"noise_level\": {},\n  \"brightness\": {},\n  \"damping\": {},\n  \"volume\": {}\n}}\n",
            sp.harmonic_weights[0],
            sp.harmonic_weights[1],
            sp.harmonic_weights[2],
            sp.harmonic_weights[3],
            sp.harmonic_weights[4],
            sp.harmonic_weights[5],
            sp.harmonic_weights[6],
            sp.harmonic_weights[7],
            sp.attack_ms,
            sp.decay_s,
            sp.sustain,
            sp.release_s,
            sp.noise_level,
            sp.brightness,
            sp.damping,
            sp.volume
        );
        fs::write(self.root_dir.join("synth_params.json"), s)
    }

    // ─── Add a note sample layer ─────────────────────────────────────────
    #[allow(clippy::too_many_arguments)]
    pub fn add_note(
        &mut self,
        midi: u8,
        velocity: u8,
        articulation: &str,
        round_robin: u8,
        source: &str,
        samples: Vec<f32>,
        sr: u32,
    ) -> io::Result<()> {
        let note_dir = self.root_dir.join("notes").join(note_name(midi));
        let vel_dir = note_dir.join(velocity_band(velocity));
        fs::create_dir_all(&vel_dir)?;
        let filename = format!("{}_rr{}.wav", articulation, round_robin);
        wav::write(
            vel_dir.join(&filename),
            &Wav {
                sample_rate: sr,
                samples: samples.clone(),
            },
        )?;

        let slot = self.notes.entry(midi).or_insert_with(|| NoteSlot {
            midi,
            layers: Vec::new(),
        });
        slot.layers.push(SampleLayer {
            velocity,
            articulation: articulation.to_string(),
            round_robin,
            source_file: source.to_string(),
            samples,
            sample_rate: sr,
            volume: 1.0, // default full volume, adjustable later
        });
        Ok(())
    }

    // ─── Set per-layer volume ────────────────────────────────────────────
    pub fn set_layer_volume(
        &mut self,
        midi: u8,
        velocity: u8,
        articulation: &str,
        rr: u8,
        volume: f32,
    ) {
        if let Some(slot) = self.notes.get_mut(&midi) {
            for layer in &mut slot.layers {
                if layer.velocity == velocity
                    && layer.articulation == articulation
                    && layer.round_robin == rr
                {
                    layer.volume = volume.clamp(0.0, 1.0);
                }
            }
        }
    }

    // ─── Add a characteristic (body resonance, pick transient, etc.) ──────
    pub fn add_characteristic(
        &mut self,
        name: &str,
        samples: Vec<f32>,
        sr: u32,
        volume: f32,
    ) -> io::Result<()> {
        let char_dir = self.root_dir.join("characteristics");
        fs::create_dir_all(&char_dir)?;
        wav::write(
            char_dir.join(format!("{}.wav", name)),
            &Wav {
                sample_rate: sr,
                samples: samples.clone(),
            },
        )?;
        self.characteristics.push(Characteristic {
            name: name.to_string(),
            samples,
            sample_rate: sr,
            volume,
        });
        self.save_metadata()
    }

    // ─── Set characteristic volume ─────────────────────────────────────────
    pub fn set_characteristic_volume(&mut self, name: &str, volume: f32) -> io::Result<()> {
        for c in &mut self.characteristics {
            if c.name == name {
                c.volume = volume.clamp(0.0, 1.0);
            }
        }
        self.save_metadata()
    }

    // ─── Update synth params ──────────────────────────────────────────────
    pub fn set_synth_params(&mut self, params: SynthParams) -> io::Result<()> {
        self.synth = params;
        self.save_synth_params()
    }

    // ─── Set master volumes and blend ────────────────────────────────────
    pub fn set_synth_volume(&mut self, vol: f32) -> io::Result<()> {
        self.synth_volume = vol.clamp(0.0, 1.0);
        self.save_metadata()
    }

    pub fn set_sample_volume(&mut self, vol: f32) -> io::Result<()> {
        self.sample_volume = vol.clamp(0.0, 1.0);
        self.save_metadata()
    }

    pub fn set_blend(&mut self, blend: f32) -> io::Result<()> {
        self.synth_blend = blend.clamp(0.0, 1.0);
        self.save_metadata()
    }

    // ─── List the database ───────────────────────────────────────────────
    pub fn list(&self) {
        println!("Instrument: {} (family: {})", self.name, self.family);
        println!(
            "Blend: {:.0}% synth / {:.0}% sample",
            self.synth_blend * 100.0,
            (1.0 - self.synth_blend) * 100.0
        );
        println!(
            "Master volumes: synth={:.0}% sample={:.0}%",
            self.synth_volume * 100.0,
            self.sample_volume * 100.0
        );
        println!(
            "Synth: hw=[{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}] attack={:.0}ms decay={:.2}s vol={:.0}%",
            self.synth.harmonic_weights[0],
            self.synth.harmonic_weights[1],
            self.synth.harmonic_weights[2],
            self.synth.harmonic_weights[3],
            self.synth.harmonic_weights[4],
            self.synth.harmonic_weights[5],
            self.synth.harmonic_weights[6],
            self.synth.harmonic_weights[7],
            self.synth.attack_ms,
            self.synth.decay_s,
            self.synth.volume * 100.0
        );
        if !self.characteristics.is_empty() {
            println!("Characteristics:");
            for c in &self.characteristics {
                println!("  {} vol={:.0}%", c.name, c.volume * 100.0);
            }
        }
        println!("Notes ({}):", self.notes.len());
        for (&midi, slot) in &self.notes {
            let vel_bands: Vec<&str> = slot
                .layers
                .iter()
                .map(|l| velocity_band(l.velocity))
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .copied()
                .collect();
            let max_rr = slot.layers.iter().map(|l| l.round_robin).max().unwrap_or(0);
            println!(
                "  {} (midi={}) layers={} vel=[{}] rr=1-{}",
                note_name(midi),
                midi,
                slot.layers.len(),
                vel_bands.join(","),
                max_rr
            );
        }
    }

    // ─── Find best sample layer for a note ───────────────────────────────
    pub fn find_layer(
        &self,
        midi: u8,
        velocity: u8,
        articulation: &str,
        rr_state: &mut u8,
    ) -> Option<&SampleLayer> {
        let slot = self.notes.get(&midi)?;
        let max_rr = slot.layers.iter().map(|l| l.round_robin).max().unwrap_or(1);
        let wanted_rr = (*rr_state % max_rr).max(1);
        let mut best: Option<(&SampleLayer, i32)> = None;
        for l in &slot.layers {
            let vel_score = (l.velocity as i32 - velocity as i32).abs();
            let art_score = if l.articulation == articulation {
                0
            } else {
                100
            };
            let rr_score = if l.round_robin == wanted_rr { 0 } else { 50 };
            let score = vel_score + art_score + rr_score;
            if best.is_none() || score < best.unwrap().1 {
                best = Some((l, score));
            }
        }
        *rr_state = wanted_rr + 1;
        best.map(|(l, _)| l)
    }

    // ─── Fill missing note slots by transposing from nearest sample ──────
    // Analyzes existing notes, transposes them to fill empty MIDI slots.
    // The transposed sample carries the real instrument character into
    // pitches that were never recorded.
    pub fn fill_missing_notes(&mut self, range_lo: u8, range_hi: u8) -> io::Result<usize> {
        let mut filled = 0;
        let existing: Vec<u8> = self.notes.keys().copied().collect();

        for midi in range_lo..=range_hi {
            if self.notes.contains_key(&midi) {
                continue;
            }

            // Find nearest existing note
            let nearest = existing
                .iter()
                .min_by_key(|&&m| (m as i32 - midi as i32).abs());

            let Some(&source_midi) = nearest else {
                continue;
            };
            let source_slot = match self.notes.get(&source_midi) {
                Some(s) => s,
                None => continue,
            };

            // Transpose each layer from source_midi to target midi
            let semitones = midi as f32 - source_midi as f32;
            let ratio = 2.0f32.powf(semitones / 12.0);

            let mut new_slot = NoteSlot {
                midi,
                layers: Vec::new(),
            };

            for layer in &source_slot.layers {
                let transposed = transpose_samples(&layer.samples, ratio);
                let sr = layer.sample_rate;

                // Write to disk
                let note_dir = self.root_dir.join("notes").join(note_name(midi));
                let vel_dir = note_dir.join(velocity_band(layer.velocity));
                fs::create_dir_all(&vel_dir)?;
                let filename = format!("{}_rr{}.wav", layer.articulation, layer.round_robin);
                wav::write(
                    vel_dir.join(&filename),
                    &Wav {
                        sample_rate: sr,
                        samples: transposed.clone(),
                    },
                )?;

                new_slot.layers.push(SampleLayer {
                    velocity: layer.velocity,
                    articulation: layer.articulation.clone(),
                    round_robin: layer.round_robin,
                    source_file: format!("transposed_from_{}", note_name(source_midi)),
                    samples: transposed,
                    sample_rate: sr,
                    volume: layer.volume,
                });
            }

            self.notes.insert(midi, new_slot);
            filled += 1;
        }

        Ok(filled)
    }

    // ─── Learn synth params from a single note recording ─────────────────
    // Analyzes one note's harmonic content and envelope, then configures
    // the synth emulator. The synth can then generate any pitch.
    pub fn learn_synth_from_note(&mut self, midi: u8, samples: &[f32], sr: u32) -> io::Result<()> {
        let hz = 440.0 * 2.0f32.powf((midi as f32 - 69.0) / 12.0);

        // Extract harmonic amplitudes via DFT at harmonic frequencies
        let n = samples.len().min(8192);
        let window: Vec<f32> = samples[..n]
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                s * (0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos())
            })
            .collect();

        let mut hw = [0.0f32; 8];
        let mut max_h = 0.0f32;
        for h in 0..8 {
            let freq = hz * (h + 1) as f32;
            if freq > sr as f32 * 0.45 {
                break;
            }
            let mut re = 0.0f32;
            let mut im = 0.0f32;
            for i in 0..n {
                let angle = 2.0 * std::f32::consts::PI * freq * i as f32 / sr as f32;
                re += window[i] * angle.cos();
                im -= window[i] * angle.sin();
            }
            hw[h] = (re * re + im * im).sqrt() / n as f32;
            if hw[h] > max_h {
                max_h = hw[h];
            }
        }
        // Normalize to H1 = 1.0
        if max_h > 0.0 {
            for h in &mut hw {
                *h /= max_h;
            }
        }

        // Measure envelope: attack time and decay
        let peak = samples.iter().fold(0.0f32, |a, &v| a.max(v.abs()));
        if peak < 0.001 {
            return Ok(());
        }

        let threshold = peak * 0.9;
        let mut attack_samples = 0usize;
        for (i, &s) in samples.iter().enumerate() {
            if s.abs() >= threshold {
                attack_samples = i;
                break;
            }
        }
        let attack_ms = attack_samples as f32 / sr as f32 * 1000.0;

        // Find peak position
        let peak_pos = samples
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Measure decay from peak to 37% (-8.7 dB)
        let decay_threshold = peak * 0.37;
        let mut decay_end = samples.len();
        for i in peak_pos..samples.len() {
            if samples[i].abs() < decay_threshold {
                decay_end = i;
                break;
            }
        }
        let decay_s = (decay_end - peak_pos) as f32 / sr as f32;

        // Estimate sustain level (amplitude at 75% through the note)
        let sustain_pos = peak_pos + (samples.len() - peak_pos) * 3 / 4;
        let sustain = samples
            .get(sustain_pos)
            .map(|v| v.abs() / peak)
            .unwrap_or(0.3);

        // Estimate release (how fast it fades at the end)
        let release_s = 0.15; // default, would need note-off detection

        self.synth = SynthParams {
            harmonic_weights: hw,
            attack_ms: attack_ms.clamp(1.0, 50.0),
            decay_s: decay_s.clamp(0.01, 2.0),
            sustain: sustain.clamp(0.05, 0.95),
            release_s,
            noise_level: 0.04,
            brightness: 0.9,
            damping: 0.17,
            volume: 0.5,
        };

        self.save_synth_params()
    }

    // ─── Replace a transposed slot with a real recording ──────────────────
    #[allow(clippy::too_many_arguments)]
    pub fn replace_note(
        &mut self,
        midi: u8,
        velocity: u8,
        articulation: &str,
        round_robin: u8,
        source: &str,
        samples: Vec<f32>,
        sr: u32,
    ) -> io::Result<()> {
        if let Some(slot) = self.notes.get_mut(&midi) {
            slot.layers.retain(|l| {
                !(l.source_file.starts_with("transposed_from_")
                    && l.velocity == velocity
                    && l.articulation == articulation)
            });
        }
        self.add_note(
            midi,
            velocity,
            articulation,
            round_robin,
            source,
            samples,
            sr,
        )
    }

    pub fn find_nearest_note(&self, midi: u8) -> Option<u8> {
        if self.notes.contains_key(&midi) {
            return Some(midi);
        }
        let mut best_midi = None;
        let mut best_dist = 128i32;
        for &avail_midi in self.notes.keys() {
            let dist = (avail_midi as i32 - midi as i32).abs();
            if dist < best_dist {
                best_dist = dist;
                best_midi = Some(avail_midi);
            }
        }
        best_midi
    }
}

// ─── Helpers ───────────────────────────────────────────────────────────

fn parse_note_dir(s: &str) -> Option<u8> {
    if let Ok(midi) = s.parse::<u8>() {
        if midi < 128 {
            return Some(midi);
        }
    }
    let bytes = s.as_bytes();
    if bytes.len() < 2 {
        return None;
    }
    let pc = match bytes[0].to_ascii_uppercase() {
        b'C' => 0,
        b'D' => 2,
        b'E' => 4,
        b'F' => 5,
        b'G' => 7,
        b'A' => 9,
        b'B' => 11,
        _ => return None,
    };
    let (pc, idx) = if bytes.get(1) == Some(&b'#') {
        (pc + 1, 2)
    } else if bytes.get(1) == Some(&b'b') {
        (pc - 1, 2)
    } else {
        (pc, 1)
    };
    let octave: i32 = std::str::from_utf8(&bytes[idx..]).ok()?.parse().ok()?;
    let midi = (octave + 1) * 12 + pc;
    if (0..128).contains(&midi) {
        Some(midi as u8)
    } else {
        None
    }
}

fn load_note_layers(note_dir: &Path, slot: &mut NoteSlot) -> io::Result<()> {
    for entry in fs::read_dir(note_dir)? {
        let entry = entry?;
        let vel_dir = entry.path();
        if !vel_dir.is_dir() {
            continue;
        }
        for file_entry in fs::read_dir(&vel_dir)? {
            let file_entry = file_entry?;
            let path = file_entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("wav") {
                continue;
            }
            let filename = file_entry.file_name().to_string_lossy().to_string();
            let parts: Vec<&str> = filename.trim_end_matches(".wav").split("__").collect();
            let articulation = parts.first().unwrap_or(&"sustain").to_string();
            let rr: u8 = parts
                .iter()
                .find(|p| p.starts_with("rr"))
                .and_then(|p| p[2..].parse().ok())
                .unwrap_or(1);
            let vel_band_name = entry.file_name().to_string_lossy().to_string();
            let velocity: u8 = match vel_band_name.as_str() {
                "soft" => 32,
                "medium" => 64,
                "hard" => 120,
                _ => 80,
            };
            let w = wav::read(&path)?;
            slot.layers.push(SampleLayer {
                velocity,
                articulation,
                round_robin: rr,
                source_file: filename,
                samples: w.samples,
                sample_rate: w.sample_rate,
                volume: 1.0,
            });
        }
    }
    Ok(())
}

fn extract_json_string(text: &str, key: &str) -> String {
    let pattern = format!("\"{}\":", key);
    if let Some(pos) = text.find(&pattern) {
        let rest = &text[pos + pattern.len()..];
        if let Some(start) = rest.find('"') {
            let rest = &rest[start + 1..];
            if let Some(end) = rest.find('"') {
                return rest[..end].to_string();
            }
        }
    }
    String::new()
}

fn extract_json_f32(text: &str, key: &str) -> Option<f32> {
    let pattern = format!("\"{}\":", key);
    if let Some(pos) = text.find(&pattern) {
        let rest = &text[pos + pattern.len()..];
        let trimmed = rest.trim_start();
        let end = trimmed
            .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
            .unwrap_or(trimmed.len());
        return trimmed[..end].parse::<f32>().ok();
    }
    None
}
// ─── Pitch shift via linear interpolation ──────────────────────────────
fn transpose_samples(samples: &[f32], ratio: f32) -> Vec<f32> {
    if ratio <= 0.0 || samples.is_empty() {
        return samples.to_vec();
    }
    let out_len = (samples.len() as f32 / ratio) as usize;
    let mut out = Vec::with_capacity(out_len);
    let mut pos = 0.0f32;
    for _ in 0..out_len {
        let i = pos as usize;
        let frac = pos - i as f32;
        if i + 1 < samples.len() {
            out.push(samples[i] * (1.0 - frac) + samples[i + 1] * frac);
        } else if i < samples.len() {
            out.push(samples[i]);
        } else {
            break;
        }
        pos += ratio;
    }
    out
}
