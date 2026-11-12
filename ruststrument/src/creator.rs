use crate::{
    classify::{self, InstrumentFamily},
    pitch,
    profile::{analyze_region, family_from_name, InstrumentBank, SynthConfig, Trigger},
    technique,
    wav::{self, Wav},
};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

const MAX_EXTRACTED_REGIONS: usize = 96;

#[derive(Clone, Debug)]
struct Label {
    root: Option<u8>,
    velocity: u8,
    velocity_explicit: bool,
    rr: u8,
    rr_explicit: bool,
    articulation: String,
    trigger: Trigger,
    looped: bool,
}

#[derive(Debug)]
struct PendingRegion {
    region: crate::profile::Region,
    source_peak: f32,
    velocity_explicit: bool,
    rr_explicit: bool,
    fixed_key: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ExtractedNote {
    pub name: String,
    pub root: u8,
    pub frequency: f32,
    pub peak: f32,
    pub samples: Vec<f32>,
}

pub fn note_to_midi(value: &str) -> Option<u8> {
    let value = value.trim();
    if let Ok(number) = value.parse::<i16>() {
        return (0..=127).contains(&number).then_some(number as u8);
    }

    let bytes = value.as_bytes();
    if bytes.len() < 2 || !bytes.is_ascii() {
        return None;
    }
    let mut pitch_class = match bytes[0].to_ascii_uppercase() {
        b'C' => 0_i16,
        b'D' => 2,
        b'E' => 4,
        b'F' => 5,
        b'G' => 7,
        b'A' => 9,
        b'B' => 11,
        _ => return None,
    };
    let mut position = 1;
    match bytes.get(position) {
        Some(b'#') => {
            pitch_class += 1;
            position += 1;
        }
        Some(b'b') => {
            pitch_class -= 1;
            position += 1;
        }
        _ => {}
    }
    let octave = std::str::from_utf8(&bytes[position..])
        .ok()?
        .parse::<i16>()
        .ok()?;
    let midi = (octave + 1) * 12 + pitch_class;
    (0..=127).contains(&midi).then_some(midi as u8)
}

fn filename_note(stem: &str) -> Option<u8> {
    stem.split(|character: char| !(character.is_ascii_alphanumeric() || character == '#'))
        .find_map(|token| {
            note_to_midi(token).or_else(|| {
                // Several supplied recordings use E4b/D4b to mean take "b",
                // rather than the musically equivalent flat spelling bE4.
                let stripped = token.strip_suffix('b')?;
                stripped
                    .bytes()
                    .any(|byte| byte.is_ascii_digit())
                    .then(|| note_to_midi(stripped))
                    .flatten()
            })
        })
}

fn label(path: &Path) -> Label {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    let lower = stem.to_ascii_lowercase();
    let structured = stem.contains("__");
    let mut label = Label {
        root: None,
        velocity: 96,
        velocity_explicit: false,
        rr: 1,
        rr_explicit: false,
        articulation: "sustain".into(),
        trigger: Trigger::Attack,
        looped: false,
    };

    if structured {
        for (index, token) in stem.split("__").enumerate() {
            if let Some(value) = token
                .strip_prefix('v')
                .and_then(|value| value.parse::<u8>().ok())
            {
                label.velocity = value.clamp(1, 127);
                label.velocity_explicit = true;
            } else if let Some(value) = token
                .strip_prefix("rr")
                .and_then(|value| value.parse::<u8>().ok())
            {
                label.rr = value.max(1);
                label.rr_explicit = true;
            } else if token.eq_ignore_ascii_case("release") {
                label.trigger = Trigger::Release;
            } else if token.eq_ignore_ascii_case("loop") {
                label.looped = true;
            } else if let Some(root) = note_to_midi(token) {
                label.root = Some(root);
            } else if index == 0 && !token.is_empty() && !token.eq_ignore_ascii_case("attack") {
                label.articulation = token.to_ascii_lowercase();
            }
        }
    } else {
        label.root = filename_note(stem);
        if lower.contains("release") {
            label.trigger = Trigger::Release;
        }
        label.looped = lower.contains("loop");
        label.articulation = [
            "staccato",
            "tremolo",
            "pizzicato",
            "muted",
            "mute",
            "slide",
            "pluck",
            "pick",
        ]
        .into_iter()
        .find(|name| lower.contains(name))
        .unwrap_or("sustain")
        .to_string();
    }
    label
}

fn collect(dir: &Path, output: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, output)?;
        } else if path.extension().is_some_and(|extension| {
            [
                "wav", "wave", "flac", "mp3", "ogg", "oga", "m4a", "aac", "aif", "aiff",
            ]
            .iter()
            .any(|accepted| extension.eq_ignore_ascii_case(accepted))
        }) {
            output.push(path);
        }
    }
    Ok(())
}

fn quiet_point(samples: &[f32], target: usize) -> usize {
    let low = target.saturating_sub(1_200).max(1);
    let high = (target + 1_200).min(samples.len().saturating_sub(2));
    (low..high)
        .min_by(|&left, &right| {
            let left_score = samples[left].abs() + (samples[left + 1] - samples[left - 1]).abs();
            let right_score =
                samples[right].abs() + (samples[right + 1] - samples[right - 1]).abs();
            left_score
                .partial_cmp(&right_score)
                .unwrap_or(Ordering::Equal)
        })
        .unwrap_or(target)
}

fn percentile(mut values: Vec<f32>, numerator: usize, denominator: usize) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    values[(values.len().saturating_sub(1) * numerator / denominator).min(values.len() - 1)]
}

fn detect_onsets(samples: &[f32], sample_rate: u32) -> Vec<usize> {
    const FRAME_SIZE: usize = 1_024;
    const HOP: usize = 256;
    if samples.len() < FRAME_SIZE {
        return vec![0];
    }
    let mut frames = Vec::new();
    let mut energy: f32 = samples[..FRAME_SIZE]
        .iter()
        .map(|sample| sample * sample)
        .sum();
    let mut previous_rms = 0.0_f32;
    for position in (0..=samples.len() - FRAME_SIZE).step_by(HOP) {
        if position > 0 {
            let removed_start = position - HOP;
            let added_start = position + FRAME_SIZE - HOP;
            for index in 0..HOP {
                let removed = samples[removed_start + index];
                let added = samples[added_start + index];
                energy += added * added - removed * removed;
            }
            energy = energy.max(0.0);
        }
        let frame_rms = (energy / FRAME_SIZE as f32).sqrt();
        let onset_strength = (frame_rms - previous_rms).max(0.0);
        frames.push((position, frame_rms, onset_strength));
        previous_rms = frame_rms;
    }

    let energy_peak = frames.iter().map(|frame| frame.1).fold(0.0_f32, f32::max);
    let energy_floor = percentile(frames.iter().map(|frame| frame.1).collect(), 1, 5);
    let energy_gate = (energy_floor * 3.0).max(energy_peak * 0.02).max(1e-5);
    let onset_values: Vec<f32> = frames
        .iter()
        .filter(|frame| frame.1 >= energy_gate)
        .map(|frame| frame.2)
        .collect();
    let onset_gate = (percentile(onset_values.clone(), 1, 2) * 2.2)
        .max(onset_values.iter().copied().fold(0.0_f32, f32::max) * 0.04);

    let mut onsets = Vec::new();
    if let Some(frame) = frames.iter().find(|frame| frame.1 >= energy_gate) {
        onsets.push(frame.0);
    }
    let minimum_gap = (sample_rate as usize / 12).max(HOP);
    for index in 1..frames.len().saturating_sub(1) {
        let (position, energy, onset_strength) = frames[index];
        if energy >= energy_gate
            && onset_strength >= onset_gate
            // A fresh isolated attack raises the short-term envelope sharply.
            // Slow beating and body-mode energy swells can still form local
            // peaks, but their rise is small relative to the ringing note.
            && onset_strength >= energy * 0.18
            && onset_strength > frames[index - 1].2
            && onset_strength >= frames[index + 1].2
            && onsets
                .last()
                .map_or(true, |last| position >= *last + minimum_gap)
        {
            onsets.push(position);
        }
    }
    onsets.truncate(MAX_EXTRACTED_REGIONS);
    onsets
}

fn representative_notes(mut notes: Vec<ExtractedNote>, limit: usize) -> Vec<ExtractedNote> {
    if notes.len() <= limit {
        return notes;
    }
    notes.sort_by(|left, right| {
        left.peak
            .partial_cmp(&right.peak)
            .unwrap_or(Ordering::Equal)
    });
    let mut selected = Vec::with_capacity(limit);
    for index in 0..limit {
        let position = index * (notes.len() - 1) / (limit - 1).max(1);
        selected.push(notes[position].clone());
    }
    selected
}

pub(crate) fn detect_pitch_fft(samples: &[f32], sample_rate: u32) -> Option<f32> {
    pitch::estimate(samples, sample_rate, 35.0, 2_200.0, 440.0).map(|estimate| estimate.hz)
}

/// Reject performance fragments that contain a melody rather than one note.
///
/// Estimating pitch from every remaining sample can hide a later note change:
/// the estimator deliberately chooses the highest-energy window. Checking
/// short windows at several points in time makes extracted SoundFont regions
/// monophonic while still allowing natural decay, vibrato, and quiet tails.
fn has_stable_local_pitch(samples: &[f32], sample_rate: u32, reference_hz: f32) -> bool {
    if samples.len() < 256 || !reference_hz.is_finite() || reference_hz <= 0.0 {
        return false;
    }
    let window = (sample_rate as usize * 9 / 50)
        .clamp(2_048, samples.len())
        .min(samples.len());
    let latest_start = samples.len().saturating_sub(window);
    let starts = [
        0,
        latest_start / 4,
        latest_start / 2,
        latest_start * 3 / 4,
        latest_start,
    ];
    let mut pitched_windows = 0;
    for start in starts {
        let frame = &samples[start..(start + window).min(samples.len())];
        let Some(local_hz) = detect_pitch_fft(frame, sample_rate) else {
            continue;
        };
        pitched_windows += 1;
        let interval = (12.0 * (local_hz / reference_hz).log2()).abs();
        if !interval.is_finite() || interval > 1.5 {
            return false;
        }
    }
    pitched_windows > 0
}

fn region_replays_at_offset(region: &crate::profile::Region, semitones: i8) -> bool {
    if region.samples.len() < 4 {
        return false;
    }
    let target = i16::from(region.root_key) + i16::from(semitones);
    if !(0..=127).contains(&target) {
        return true;
    }
    // Mirror the player's sample-position increment. The stored tune offset is
    // corrected during replay, so the rendered result must land on the MIDI
    // key itself rather than merely looking correct in the source recording.
    let cents = semitones as f64 * 100.0 - region.tune_cents as f64;
    let step = 2f64.powf(cents / 1_200.0);
    let frames = (region.samples.len() as f64 / step).ceil() as usize;
    let mut rendered = Vec::with_capacity(frames);
    let mut position = 0.0_f64;
    while position < region.samples.len().saturating_sub(1) as f64 {
        let index = position.floor() as usize;
        let fraction = (position - index as f64) as f32;
        let left = region.samples[index];
        let right = region.samples[(index + 1).min(region.samples.len() - 1)];
        rendered.push(left + (right - left) * fraction);
        position += step;
    }
    // The player renders into a longer output buffer after a short one-shot
    // sample ends. Include that real playback context because some ambiguous
    // fragments appear correct in isolation but resolve to the wrong octave
    // once analyzed as a played note.
    rendered.resize(rendered.len().max(region.sample_rate as usize / 2), 0.0);
    let Some(observed) = crate::analyze::identify_pitch(&rendered, region.sample_rate) else {
        return false;
    };
    let observed_cents = observed.midi as f32 * 100.0 + observed.cents;
    let expected_cents = target as f32 * 100.0;
    (observed_cents - expected_cents).abs() <= 50.0
}

fn region_replays_at_its_root(region: &crate::profile::Region) -> bool {
    [-2, 0, 2]
        .into_iter()
        .all(|semitones| region_replays_at_offset(region, semitones))
}

fn decode_audio(path: &Path) -> io::Result<Wav> {
    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"))
    {
        return wav::read(path);
    }
    let audio = crate::load_audio(&path.to_string_lossy())
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    Ok(Wav {
        sample_rate: audio.sample_rate,
        samples: audio.samples,
    })
}

fn percussion_note(name: &str) -> u8 {
    let lower = name.to_ascii_lowercase();
    let mappings = [
        (&["kick", "bass drum"] as &[&str], 36),
        (&["snare", "side stick"], 38),
        (&["clap"], 39),
        (&["closed hat", "closed hi", "hihat closed"], 42),
        (&["open hat", "open hi", "hihat open"], 46),
        (&["tom"], 45),
        (&["crash"], 49),
        (&["ride"], 51),
        (&["tambourine"], 54),
        (&["cowbell"], 56),
        (&["bongo"], 60),
        (&["conga"], 64),
        (&["maraca", "shaker"], 70),
        (&["claves"], 75),
        (&["woodblock", "wood block"], 76),
        (&["triangle"], 81),
        (&["bell", "gong", "chime"], 53),
    ];
    mappings
        .iter()
        .find(|(terms, _)| terms.iter().any(|term| lower.contains(term)))
        .map(|(_, note)| *note)
        .unwrap_or(60)
}

fn is_percussion(family: InstrumentFamily) -> bool {
    matches!(
        family,
        InstrumentFamily::PercussionMembrane
            | InstrumentFamily::PercussionMetal
            | InstrumentFamily::PercussionWood
    )
}

fn sample_uses_fixed_key(
    named_family: &str,
    classified_family: InstrumentFamily,
    has_clear_pitch: bool,
) -> bool {
    if has_clear_pitch {
        return false;
    }
    if named_family != "unknown" {
        return named_family.starts_with("percussion_");
    }
    is_percussion(classified_family)
}

fn trim_note(samples: &[f32], sample_rate: u32) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }
    let peak = samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max);
    let threshold = (peak * 0.006).max(1e-5);
    let tail = (sample_rate as usize / 20).max(32);
    let last_loud = samples
        .iter()
        .rposition(|sample| sample.abs() >= threshold)
        .unwrap_or(samples.len() - 1);
    samples[..(last_loud + tail).min(samples.len())].to_vec()
}

pub(crate) fn extract_notes(wav: &Wav, source_name: &str) -> Vec<ExtractedNote> {
    let onsets = detect_onsets(&wav.samples, wav.sample_rate);
    let minimum_length = (wav.sample_rate as usize / 16).max(256);
    let maximum_length = wav.sample_rate as usize * 3;
    let global_peak = wav
        .samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max);
    let mut extracted = Vec::new();

    for (index, &onset) in onsets.iter().enumerate() {
        let next = onsets.get(index + 1).copied().unwrap_or(wav.samples.len());
        let start = onset.saturating_sub((wav.sample_rate as usize / 200).max(1));
        let end = next
            .min(start.saturating_add(maximum_length))
            .min(wav.samples.len());
        if end.saturating_sub(start) < minimum_length {
            continue;
        }
        let note_samples = trim_note(&wav.samples[start..end], wav.sample_rate);
        if note_samples.len() < minimum_length {
            continue;
        }
        let peak = note_samples
            .iter()
            .map(|sample| sample.abs())
            .fold(0.0_f32, f32::max);
        if peak < (global_peak * 0.015).max(1e-5) {
            continue;
        }
        let pitch_start = (wav.sample_rate as usize / 100).min(note_samples.len() / 3);
        let Some(frequency) = detect_pitch_fft(&note_samples[pitch_start..], wav.sample_rate)
        else {
            continue;
        };
        if !(20.0..=5_000.0).contains(&frequency) {
            continue;
        }
        if !has_stable_local_pitch(&note_samples[pitch_start..], wav.sample_rate, frequency) {
            continue;
        }
        let midi = (69.0 + 12.0 * (frequency / 440.0).log2()).round() as i16;
        if !(0..=127).contains(&midi) {
            continue;
        }
        extracted.push(ExtractedNote {
            name: format!("{source_name}_note_{:03}", index + 1),
            root: midi as u8,
            frequency,
            peak,
            samples: note_samples,
        });
    }
    extracted.truncate(MAX_EXTRACTED_REGIONS);
    extracted
}

fn map_ranges(regions: &mut [crate::profile::Region]) {
    let centers: Vec<(u8, u8)> = regions
        .iter()
        .map(|region| (region.root_key, region.vel_lo))
        .collect();
    let mut groups: BTreeMap<(String, u8), Vec<usize>> = BTreeMap::new();
    for (index, region) in regions.iter().enumerate() {
        groups
            .entry((
                region.articulation.clone(),
                u8::from(region.trigger == Trigger::Release),
            ))
            .or_default()
            .push(index);
    }

    for indices in groups.values() {
        let mut velocities: Vec<u8> = indices.iter().map(|&index| centers[index].1).collect();
        velocities.sort_unstable();
        velocities.dedup();
        for (velocity_index, &velocity) in velocities.iter().enumerate() {
            let low_velocity = if velocity_index == 0 {
                1
            } else {
                ((velocities[velocity_index - 1] as u16 + velocity as u16) / 2 + 1) as u8
            };
            let high_velocity = if velocity_index + 1 == velocities.len() {
                127
            } else {
                ((velocity as u16 + velocities[velocity_index + 1] as u16) / 2) as u8
            };
            let mut roots: Vec<u8> = indices
                .iter()
                .filter(|&&index| centers[index].1 == velocity)
                .map(|&index| centers[index].0)
                .collect();
            roots.sort_unstable();
            roots.dedup();

            for (root_index, &root) in roots.iter().enumerate() {
                let low_key = if root_index == 0 {
                    0
                } else {
                    ((roots[root_index - 1] as u16 + root as u16) / 2 + 1) as u8
                };
                let high_key = if root_index + 1 == roots.len() {
                    127
                } else {
                    ((root as u16 + roots[root_index + 1] as u16) / 2) as u8
                };
                for &index in indices {
                    if centers[index] == (root, velocity) {
                        regions[index].key_lo = low_key;
                        regions[index].key_hi = high_key;
                        regions[index].vel_lo = low_velocity;
                        regions[index].vel_hi = high_velocity;
                    }
                }
            }
        }
    }
}

fn assign_inferred_velocities(pending: &mut [PendingRegion]) {
    let peaks: Vec<f32> = pending
        .iter()
        .filter(|item| !item.velocity_explicit)
        .map(|item| item.source_peak)
        .collect();
    if peaks.is_empty() {
        return;
    }
    let low = percentile(peaks.clone(), 1, 3);
    let high = percentile(peaks, 2, 3);
    for item in pending.iter_mut().filter(|item| !item.velocity_explicit) {
        item.region.vel_lo = if low == high {
            96
        } else if item.source_peak <= low {
            36
        } else if item.source_peak <= high {
            80
        } else {
            116
        };
    }
}

fn assign_round_robins(pending: &mut [PendingRegion]) {
    let mut counters: BTreeMap<(String, u8, u8, u8), u8> = BTreeMap::new();
    for item in pending {
        let key = (
            item.region.articulation.clone(),
            u8::from(item.region.trigger == Trigger::Release),
            item.region.root_key,
            item.region.vel_lo,
        );
        let counter = counters.entry(key).or_insert(0);
        if item.rr_explicit {
            *counter = (*counter).max(item.region.round_robin);
        } else {
            *counter = counter.saturating_add(1).max(1);
            item.region.round_robin = *counter;
        }
    }
}

pub fn create(dir: impl AsRef<Path>, name: &str) -> io::Result<InstrumentBank> {
    let mut files = Vec::new();
    collect(dir.as_ref(), &mut files)?;
    files.sort_by(|left, right| {
        label(left)
            .root
            .is_none()
            .cmp(&label(right).root.is_none())
            .then_with(|| left.cmp(right))
    });
    if files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no supported audio samples found",
        ));
    }

    let named_family = family_from_name(name).to_string();
    let mut pending = Vec::new();
    let mut family_votes: BTreeMap<String, usize> = BTreeMap::new();
    for path in files {
        let mut label = label(&path);
        label.articulation = if label.articulation == "sustain" {
            technique::infer_from_filename(name, &named_family, &path.to_string_lossy())
        } else {
            technique::canonicalize(name, &named_family, &label.articulation)
        };
        let wav = decode_audio(&path)?;
        let display = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("sample");
        let features = classify::analyze_recording(&wav);
        let classification = classify::classify_ranked(&features);
        if classification.family != InstrumentFamily::Unknown {
            *family_votes
                .entry(classification.family.label().to_string())
                .or_default() += 1;
        }

        if let Some(root) = label.root {
            let source_peak = wav
                .samples
                .iter()
                .map(|sample| sample.abs())
                .fold(0.0_f32, f32::max);
            let mut region = analyze_region(display, &wav, Some(root));
            region.articulation = label.articulation;
            region.trigger = label.trigger;
            region.round_robin = label.rr;
            region.vel_lo = label.velocity;
            region.vel_hi = label.velocity;
            let fixed_key = sample_uses_fixed_key(
                &named_family,
                classification.family,
                features.has_clear_pitch,
            );
            if region.trigger == Trigger::Release || fixed_key {
                region.tune_cents = 0.0;
            }
            if label.looped && region.samples.len() > 2_048 {
                let start = quiet_point(&region.samples, region.samples.len() * 45 / 100);
                let end = quiet_point(&region.samples, region.samples.len() * 85 / 100);
                if end > start + 256 {
                    region.loop_start = Some(start as u32);
                    region.loop_end = Some(end as u32);
                }
            }
            // A filename is a useful pitch hint, not proof. Analyzer-derived
            // corpora are explicitly labeled, so they used to bypass the same
            // replay validation applied to unlabeled recordings. Reject a
            // tonal attack unless the finished region replays at its claimed
            // root and neighboring keys; release noise and fixed percussion
            // do not have a meaningful pitched-root check.
            if region.trigger != Trigger::Release
                && !fixed_key
                && (region.tune_cents.abs() > 35.0 || !region_replays_at_its_root(&region))
            {
                continue;
            }
            pending.push(PendingRegion {
                region,
                source_peak,
                velocity_explicit: label.velocity_explicit,
                rr_explicit: label.rr_explicit,
                fixed_key,
            });
            continue;
        }

        let remaining = MAX_EXTRACTED_REGIONS.saturating_sub(pending.len());
        if remaining == 0 {
            break;
        }
        let extracted = representative_notes(extract_notes(&wav, display), remaining);
        for note in &extracted {
            let note_wav = Wav {
                sample_rate: wav.sample_rate,
                samples: note.samples.clone(),
            };
            let mut region = analyze_region(&note.name, &note_wav, Some(note.root));
            let root_frequency = 440.0 * 2.0_f32.powf((note.root as f32 - 69.0) / 12.0);
            region.tune_cents =
                (1_200.0 * (note.frequency / root_frequency).log2()).clamp(-100.0, 100.0);
            region.articulation = label.articulation.clone();
            if !region_replays_at_its_root(&region) {
                continue;
            }
            pending.push(PendingRegion {
                region,
                source_peak: note.peak,
                velocity_explicit: false,
                rr_explicit: false,
                fixed_key: false,
            });
        }
        if extracted.is_empty() && is_percussion(classification.family) {
            let root = percussion_note(display);
            let source_peak = wav
                .samples
                .iter()
                .map(|sample| sample.abs())
                .fold(0.0_f32, f32::max);
            let mut region = analyze_region(display, &wav, Some(root));
            region.articulation = label.articulation;
            region.trigger = label.trigger;
            region.round_robin = label.rr;
            region.vel_lo = label.velocity;
            region.vel_hi = label.velocity;
            region.tune_cents = 0.0;
            pending.push(PendingRegion {
                region,
                source_peak,
                velocity_explicit: label.velocity_explicit,
                rr_explicit: label.rr_explicit,
                fixed_key: true,
            });
        }
    }
    if pending.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "no pitched notes or percussion events could be learned from the audio files",
        ));
    }

    assign_inferred_velocities(&mut pending);
    assign_round_robins(&mut pending);
    let fixed_keys: Vec<bool> = pending.iter().map(|item| item.fixed_key).collect();
    let mut regions: Vec<_> = pending.into_iter().map(|item| item.region).collect();
    map_ranges(&mut regions);
    for (region, fixed_key) in regions.iter_mut().zip(fixed_keys) {
        if fixed_key {
            region.key_lo = region.root_key;
            region.key_hi = region.root_key;
        }
    }

    let mut articulations: Vec<String> = regions
        .iter()
        .filter(|region| region.trigger == Trigger::Attack)
        .map(|region| region.articulation.clone())
        .collect();
    articulations.sort();
    articulations.dedup();
    let default_articulation = if articulations.iter().any(|name| name == "sustain") {
        "sustain".to_string()
    } else {
        articulations
            .first()
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no attack samples"))?
    };
    let family = if named_family != "unknown" {
        named_family
    } else {
        family_votes
            .into_iter()
            .max_by_key(|(_, votes)| *votes)
            .map(|(family, _)| family)
            .unwrap_or_else(|| "unknown".to_string())
    };
    let synth = SynthConfig::learn_instrument(name, &family, &regions);
    let string_profile = crate::stringed::StringInstrumentProfile::for_instrument(name, &family);
    let keyboard_profile = crate::physical::KeyboardProfile::for_instrument(name, &family);
    let wind_profile = crate::physical::WindProfile::for_instrument(name, &family);
    let percussion_profile = crate::physical::PercussionProfile::for_instrument(name, &family);
    let bank = InstrumentBank {
        name: name.into(),
        family,
        default_articulation,
        polyphony: 128,
        synth,
        string_profile,
        keyboard_profile,
        wind_profile,
        percussion_profile,
        regions,
    };
    bank.validate()?;
    Ok(bank)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_notes_and_rejects_out_of_range_notes() {
        assert_eq!(note_to_midi("C4"), Some(60));
        assert_eq!(note_to_midi("F#3"), Some(54));
        assert_eq!(note_to_midi("Bb2"), Some(46));
        assert_eq!(note_to_midi("127"), Some(127));
        assert_eq!(note_to_midi("C15"), None);
        assert_eq!(note_to_midi("128"), None);
    }

    #[test]
    fn recognizes_supplied_filename_style() {
        assert_eq!(filename_note("banjo_sample_C#4"), Some(61));
        assert_eq!(filename_note("banjo_sample_E4b"), Some(64));
        assert_eq!(filename_note("banjo_sample_D4b"), Some(62));
        assert_eq!(filename_note("banjo_reference"), None);
    }

    #[test]
    fn fast_pitch_detector_finds_a440() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..8_192)
            .map(|index| (std::f32::consts::TAU * 440.0 * index as f32 / sample_rate as f32).sin())
            .collect();
        let frequency = detect_pitch_fft(&samples, sample_rate).unwrap();
        assert!((frequency - 440.0).abs() < 1.0, "{frequency}");
    }

    #[test]
    fn pitch_detector_handles_search_range_boundaries() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..16_384)
            .map(|index| (std::f32::consts::TAU * 35.0 * index as f32 / sample_rate as f32).sin())
            .collect();
        let frequency = detect_pitch_fft(&samples, sample_rate).unwrap();
        assert!(frequency.is_finite());
        assert!((frequency - 35.0).abs() < 1.0, "{frequency}");
    }

    #[test]
    fn local_pitch_gate_rejects_a_melody_fragment() {
        let sample_rate = 44_100;
        let mut samples = Vec::new();
        for frequency in [220.0_f32, 329.627_56] {
            samples.extend((0..sample_rate / 2).map(|index| {
                (std::f32::consts::TAU * frequency * index as f32 / sample_rate as f32).sin()
            }));
        }
        assert!(!has_stable_local_pitch(&samples, sample_rate, 220.0));

        let stable: Vec<f32> = (0..sample_rate)
            .map(|index| {
                let vibrato = 1.0
                    + 0.003
                        * (std::f32::consts::TAU * 5.0 * index as f32 / sample_rate as f32).sin();
                (std::f32::consts::TAU * 220.0 * vibrato * index as f32 / sample_rate as f32).sin()
            })
            .collect();
        assert!(has_stable_local_pitch(&stable, sample_rate, 220.0));
    }

    #[test]
    fn onset_detector_ignores_slow_ringing_envelope_swells() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..sample_rate * 2)
            .map(|index| {
                let time = index as f32 / sample_rate as f32;
                let decay = (-1.8 * time).exp();
                let beating = 0.78 + 0.22 * (std::f32::consts::TAU * 8.0 * time).sin();
                0.55 * decay * beating * (std::f32::consts::TAU * 220.0 * time).sin()
            })
            .collect();
        assert_eq!(detect_onsets(&samples, sample_rate).len(), 1);
    }

    #[test]
    fn onset_detector_keeps_separated_note_attacks() {
        let sample_rate = 44_100;
        let mut samples = vec![0.0; sample_rate as usize * 2];
        for start in [sample_rate as usize / 10, sample_rate as usize] {
            for offset in 0..sample_rate as usize / 3 {
                let time = offset as f32 / sample_rate as f32;
                samples[start + offset] = 0.5 * (std::f32::consts::TAU * 220.0 * time).sin();
            }
        }
        assert_eq!(detect_onsets(&samples, sample_rate).len(), 2);
    }

    #[test]
    fn playback_gate_rejects_a_wrong_root() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..sample_rate / 2)
            .map(|index| (std::f32::consts::TAU * 220.0 * index as f32 / sample_rate as f32).sin())
            .collect();
        let wav = Wav {
            sample_rate,
            samples,
        };
        let correct = analyze_region("A3", &wav, Some(57));
        let wrong = analyze_region("A4", &wav, Some(69));
        assert!(region_replays_at_its_root(&correct));
        assert!(!region_replays_at_its_root(&wrong));
    }

    #[test]
    fn named_pitched_family_overrides_a_percussive_slice_vote() {
        assert!(!sample_uses_fixed_key(
            "struck_string",
            InstrumentFamily::PercussionMetal,
            false
        ));
        assert!(sample_uses_fixed_key(
            "percussion_membrane",
            InstrumentFamily::PluckedString,
            false
        ));
        assert!(sample_uses_fixed_key(
            "unknown",
            InstrumentFamily::PercussionWood,
            false
        ));
        assert!(!sample_uses_fixed_key(
            "percussion_metal",
            InstrumentFamily::PercussionMetal,
            true
        ));
    }

    #[test]
    fn range_mapper_covers_keyboard_and_velocity() {
        let wav = Wav {
            sample_rate: 44_100,
            samples: (0..2_000).map(|index| (index as f32 * 0.1).sin()).collect(),
        };
        let mut low = analyze_region("low", &wav, Some(48));
        low.vel_lo = 36;
        let mut high = analyze_region("high", &wav, Some(72));
        high.vel_lo = 116;
        let mut regions = vec![low, high];
        map_ranges(&mut regions);
        assert_eq!(regions[0].key_lo, 0);
        assert_eq!(regions[1].key_hi, 127);
        assert_eq!(regions[0].vel_lo, 1);
        assert_eq!(regions[1].vel_hi, 127);
    }
}
