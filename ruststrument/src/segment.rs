//! Extract isolated, labeled note WAVs from a longer reference recording.
//!
//! The same onset and pitch detector is used by the create command, so
//! running segment first is optional. It is useful when a musician wants to
//! audit or hand-edit the extracted samples before building a bank.

use crate::{
    creator::extract_notes,
    transcribe::{self, NoteEvent, TranscriptionConfig},
    wav::{self, Wav},
};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
};

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Clone, Debug)]
pub struct AnalyzedExtractionConfig {
    pub min_midi: u8,
    pub max_midi: u8,
    pub min_note_ms: f32,
    pub min_confidence: f32,
    pub max_cents: f32,
    pub max_regions: usize,
    pub round_robins: usize,
    pub preferred_sources: BTreeSet<usize>,
}

impl Default for AnalyzedExtractionConfig {
    fn default() -> Self {
        Self {
            min_midi: 36,
            max_midi: 96,
            min_note_ms: 80.0,
            min_confidence: 0.90,
            max_cents: 25.0,
            max_regions: 96,
            round_robins: 2,
            preferred_sources: BTreeSet::new(),
        }
    }
}

impl AnalyzedExtractionConfig {
    pub fn validate(&self) -> io::Result<()> {
        if self.min_midi > self.max_midi
            || self.max_midi > 127
            || !self.min_note_ms.is_finite()
            || !(10.0..=2_000.0).contains(&self.min_note_ms)
            || !self.min_confidence.is_finite()
            || !(0.0..=1.0).contains(&self.min_confidence)
            || !self.max_cents.is_finite()
            || !(0.0..=100.0).contains(&self.max_cents)
            || !(1..=512).contains(&self.max_regions)
            || !(1..=8).contains(&self.round_robins)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid analyzer extraction configuration",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct AnalyzedCandidate {
    source: usize,
    event_index: usize,
    event: NoteEvent,
    velocity_tier: u8,
    source_priority: f32,
}

fn analyzed_score(candidate: &AnalyzedCandidate) -> f32 {
    let duration = candidate.event.end_s - candidate.event.start_s;
    candidate.event.confidence * 4.0 + candidate.source_priority
        - candidate.event.cents.abs() / 50.0
        + duration.min(0.8) * 0.55
        - (duration - 1.8).max(0.0) * 0.25
}

fn analyzed_velocity_tier(velocity: u8, low: u8, high: u8) -> u8 {
    if low == high {
        80
    } else if velocity <= low {
        36
    } else if velocity >= high {
        116
    } else {
        80
    }
}

fn select_analyzed_candidates(
    mut candidates: Vec<AnalyzedCandidate>,
    config: &AnalyzedExtractionConfig,
) -> Vec<AnalyzedCandidate> {
    if candidates.is_empty() {
        return candidates;
    }
    let mut velocities: Vec<u8> = candidates
        .iter()
        .map(|candidate| candidate.event.velocity)
        .collect();
    velocities.sort_unstable();
    // Use outer quartiles as soft and hard anchors so the middle layer keeps
    // enough dynamic width even when a performance has a narrow velocity span.
    let low = velocities[(velocities.len() - 1) / 4];
    let high = velocities[(velocities.len() - 1) * 3 / 4];
    for candidate in &mut candidates {
        candidate.velocity_tier = analyzed_velocity_tier(candidate.event.velocity, low, high);
    }

    let mut pitch_support: BTreeMap<u8, (usize, BTreeSet<usize>)> = BTreeMap::new();
    for candidate in &candidates {
        let support = pitch_support
            .entry(candidate.event.midi)
            .or_insert_with(|| (0, BTreeSet::new()));
        support.0 += 1;
        support.1.insert(candidate.source);
    }
    candidates.retain(|candidate| {
        pitch_support
            .get(&candidate.event.midi)
            .is_some_and(|(count, sources)| *count >= 3 || sources.len() >= 2)
    });

    let mut by_pitch: BTreeMap<u8, Vec<usize>> = BTreeMap::new();
    let mut by_group: BTreeMap<(u8, u8), Vec<usize>> = BTreeMap::new();
    for (index, candidate) in candidates.iter().enumerate() {
        by_pitch
            .entry(candidate.event.midi)
            .or_default()
            .push(index);
        by_group
            .entry((candidate.event.midi, candidate.velocity_tier))
            .or_default()
            .push(index);
    }

    let mut chosen = Vec::new();
    let mut chosen_set = BTreeSet::new();
    let mut group_counts: BTreeMap<(u8, u8), usize> = BTreeMap::new();
    for indices in by_pitch.values() {
        if chosen.len() >= config.max_regions {
            break;
        }
        if let Some(&index) = indices.iter().max_by(|&&left, &&right| {
            analyzed_score(&candidates[left])
                .partial_cmp(&analyzed_score(&candidates[right]))
                .unwrap_or(Ordering::Equal)
        }) {
            chosen.push(index);
            chosen_set.insert(index);
            *group_counts
                .entry((
                    candidates[index].event.midi,
                    candidates[index].velocity_tier,
                ))
                .or_default() += 1;
        }
    }

    let mut group_best = Vec::new();
    for (group, indices) in &by_group {
        if group_counts.get(group).copied().unwrap_or(0) > 0 {
            continue;
        }
        if let Some(&index) = indices.iter().max_by(|&&left, &&right| {
            analyzed_score(&candidates[left])
                .partial_cmp(&analyzed_score(&candidates[right]))
                .unwrap_or(Ordering::Equal)
        }) {
            group_best.push(index);
        }
    }
    group_best.sort_by(|&left, &right| {
        analyzed_score(&candidates[right])
            .partial_cmp(&analyzed_score(&candidates[left]))
            .unwrap_or(Ordering::Equal)
    });
    for index in group_best {
        if chosen.len() >= config.max_regions {
            break;
        }
        chosen.push(index);
        chosen_set.insert(index);
        *group_counts
            .entry((
                candidates[index].event.midi,
                candidates[index].velocity_tier,
            ))
            .or_default() += 1;
    }

    let mut remaining: Vec<usize> = (0..candidates.len())
        .filter(|index| !chosen_set.contains(index))
        .collect();
    remaining.sort_by(|&left, &right| {
        analyzed_score(&candidates[right])
            .partial_cmp(&analyzed_score(&candidates[left]))
            .unwrap_or(Ordering::Equal)
    });
    for index in remaining {
        if chosen.len() >= config.max_regions {
            break;
        }
        let group = (
            candidates[index].event.midi,
            candidates[index].velocity_tier,
        );
        let count = group_counts.get(&group).copied().unwrap_or(0);
        if count < config.round_robins {
            chosen.push(index);
            *group_counts.entry(group).or_default() += 1;
        }
    }

    chosen
        .into_iter()
        .map(|index| candidates[index].clone())
        .collect()
}

fn load_analyzed_audio(path: &Path) -> io::Result<crate::AudioData> {
    let path_text = path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "audio path is not valid UTF-8")
    })?;
    crate::load_audio(path_text).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn note_name(midi: u8) -> String {
    format!("{}{}", NOTE_NAMES[midi as usize % 12], midi as i16 / 12 - 1)
}

/// Cross-check a frame-derived transcription event against the dominant
/// periodicity of the complete event. Performance audio can contain a strong
/// upper partial that looks stable for several frames; accepting that partial
/// as the root creates octave-wrong high-key samples. The whole-event estimate
/// is deliberately independent of the transcription frame votes.
fn analyzed_event_matches_fundamental(
    samples: &[f32],
    sample_rate: u32,
    event: &NoteEvent,
) -> bool {
    let start = (event.start_s * sample_rate as f32).round() as usize;
    let end = (event.end_s * sample_rate as f32).round() as usize;
    let start = start.min(samples.len());
    let end = end.min(samples.len());
    if end.saturating_sub(start) < 256 {
        return false;
    }
    let Some(observed) = crate::analyze::identify_pitch(&samples[start..end], sample_rate) else {
        return false;
    };
    let observed_cents = observed.midi as f32 * 100.0 + observed.cents;
    let claimed_cents = event.midi as f32 * 100.0 + event.cents;
    (observed_cents - claimed_cents).abs() <= 50.0
}

fn velocity_tier(peak: f32, maximum_peak: f32) -> u8 {
    let relative = peak / maximum_peak.max(1e-9);
    if relative < 0.3 {
        36
    } else if relative < 0.6 {
        80
    } else {
        116
    }
}

pub fn segment(input: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> io::Result<usize> {
    let wav = wav::read(&input)?;
    let source_name = input
        .as_ref()
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("recording");
    let notes = extract_notes(&wav, source_name);
    if notes.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "no stable pitched notes found in the recording",
        ));
    }
    fs::create_dir_all(&out_dir)?;

    let maximum_peak = notes.iter().map(|note| note.peak).fold(0.0_f32, f32::max);
    let mut round_robins: BTreeMap<(u8, u8), u8> = BTreeMap::new();
    for note in &notes {
        let velocity = velocity_tier(note.peak, maximum_peak);
        let round_robin = round_robins
            .entry((note.root, velocity))
            .and_modify(|value| *value = value.saturating_add(1))
            .or_insert(1);
        let filename = format!(
            "sustain__{}__v{velocity:03}__rr{round_robin}.wav",
            note_name(note.root)
        );
        wav::write(
            out_dir.as_ref().join(filename),
            &Wav {
                sample_rate: wav.sample_rate,
                samples: note.samples.clone(),
            },
        )?;
    }
    Ok(notes.len())
}

/// Extract a balanced labeled sample set from analyzer-derived note events.
///
/// Unlike the conservative onset segmenter, this path follows stable pitches
/// through a complete performance, cross-checks tuning and confidence, then
/// balances pitch anchors, velocity layers, and round robins across sources.
pub fn extract_analyzed(
    inputs: &[PathBuf],
    out_dir: impl AsRef<Path>,
    config: &AnalyzedExtractionConfig,
) -> io::Result<usize> {
    config.validate()?;
    if inputs.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "at least one recording is required",
        ));
    }
    if config
        .preferred_sources
        .iter()
        .any(|source| *source >= inputs.len())
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "preferred source index is outside the recording list",
        ));
    }

    let transcription_config = TranscriptionConfig {
        min_midi: config.min_midi,
        max_midi: config.max_midi,
        max_polyphony: 1,
        min_note_ms: config.min_note_ms,
        ..TranscriptionConfig::default()
    };
    let mut candidates = Vec::new();
    for (source, input) in inputs.iter().enumerate() {
        let audio = load_analyzed_audio(input)?;
        let transcription =
            transcribe::transcribe(&audio.samples, audio.sample_rate, &transcription_config)?;
        for (event_index, event) in transcription.notes.into_iter().enumerate() {
            let duration = event.end_s - event.start_s;
            if event.confidence >= config.min_confidence
                && event.cents.abs() <= config.max_cents
                && duration >= config.min_note_ms / 1_000.0
                && duration <= 3.0
                && analyzed_event_matches_fundamental(&audio.samples, audio.sample_rate, &event)
            {
                candidates.push(AnalyzedCandidate {
                    source,
                    event_index,
                    event,
                    velocity_tier: 80,
                    source_priority: if config.preferred_sources.contains(&source) {
                        0.75
                    } else {
                        0.0
                    },
                });
            }
        }
    }

    let mut selected = select_analyzed_candidates(candidates, config);
    if selected.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the analyzer found no note events that passed extraction gates",
        ));
    }
    selected.sort_by(|left, right| {
        (
            left.event.midi,
            left.velocity_tier,
            left.source,
            left.event_index,
        )
            .cmp(&(
                right.event.midi,
                right.velocity_tier,
                right.source,
                right.event_index,
            ))
    });
    let mut round_robins: BTreeMap<(u8, u8), u8> = BTreeMap::new();
    let mut labels = BTreeMap::new();
    for candidate in &selected {
        let round_robin = round_robins
            .entry((candidate.event.midi, candidate.velocity_tier))
            .and_modify(|value| *value = value.saturating_add(1))
            .or_insert(1);
        labels.insert(
            (candidate.source, candidate.event_index),
            (candidate.velocity_tier, *round_robin),
        );
    }

    fs::create_dir_all(&out_dir)?;
    let mut by_source: BTreeMap<usize, Vec<&AnalyzedCandidate>> = BTreeMap::new();
    for candidate in &selected {
        by_source
            .entry(candidate.source)
            .or_default()
            .push(candidate);
    }
    let mut written = 0;
    for (source, source_candidates) in by_source {
        let audio = load_analyzed_audio(&inputs[source])?;
        let sample_rate = audio.sample_rate;
        let pre_roll = (sample_rate as usize * 30 / 1_000).max(1);
        let tail = (sample_rate as usize * 80 / 1_000).max(1);
        let maximum_length = sample_rate as usize * 3;
        for candidate in source_candidates {
            let event_start = (candidate.event.start_s * sample_rate as f32).round() as usize;
            let event_end = (candidate.event.end_s * sample_rate as f32).round() as usize;
            let start = event_start.saturating_sub(pre_roll);
            let end = event_end
                .saturating_add(tail)
                .min(start.saturating_add(maximum_length))
                .min(audio.samples.len());
            if end <= start + 4 {
                continue;
            }
            let mut samples = audio.samples[start..end].to_vec();
            let fade = (sample_rate as usize / 100).min(samples.len() / 3).max(1);
            for offset in 0..fade {
                let gain = 1.0 - offset as f32 / fade as f32;
                let index = samples.len() - 1 - offset;
                samples[index] *= gain;
            }
            let peak = samples
                .iter()
                .map(|sample| sample.abs())
                .fold(0.0_f32, f32::max);
            if peak < 1e-5 {
                continue;
            }
            let (velocity, round_robin) = labels
                .get(&(candidate.source, candidate.event_index))
                .copied()
                .expect("selected candidate has a label");
            let filename = format!(
                "sustain__{}__v{velocity:03}__rr{round_robin}__src{source:02}_evt{:04}.wav",
                note_name(candidate.event.midi),
                candidate.event_index
            );
            wav::write(
                out_dir.as_ref().join(filename),
                &Wav {
                    sample_rate,
                    samples,
                },
            )?;
            written += 1;
        }
    }
    if written == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "analyzer-selected notes contained no writable audio",
        ));
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_notes_and_bands_velocity() {
        assert_eq!(note_name(60), "C4");
        assert_eq!(note_name(61), "C#4");
        assert_eq!(velocity_tier(0.2, 1.0), 36);
        assert_eq!(velocity_tier(0.5, 1.0), 80);
        assert_eq!(velocity_tier(0.9, 1.0), 116);
    }

    fn analyzed_candidate(
        source: usize,
        event_index: usize,
        midi: u8,
        velocity: u8,
    ) -> AnalyzedCandidate {
        AnalyzedCandidate {
            source,
            event_index,
            event: NoteEvent {
                start_s: event_index as f32,
                end_s: event_index as f32 + 0.4,
                midi,
                note: note_name(midi),
                frequency_hz: 440.0,
                cents: 1.0,
                velocity,
                confidence: 0.98,
            },
            velocity_tier: 80,
            source_priority: 0.0,
        }
    }

    #[test]
    fn analyzed_selector_balances_pitch_and_velocity_and_rejects_one_offs() {
        let candidates = vec![
            analyzed_candidate(0, 0, 60, 20),
            analyzed_candidate(0, 1, 60, 70),
            analyzed_candidate(1, 2, 60, 115),
            analyzed_candidate(0, 3, 72, 24),
            analyzed_candidate(1, 4, 72, 72),
            analyzed_candidate(1, 5, 72, 120),
            analyzed_candidate(0, 6, 84, 100),
        ];
        let config = AnalyzedExtractionConfig {
            max_regions: 6,
            round_robins: 1,
            ..AnalyzedExtractionConfig::default()
        };
        let selected = select_analyzed_candidates(candidates, &config);
        assert_eq!(selected.len(), 6);
        assert!(selected.iter().all(|candidate| candidate.event.midi != 84));
        for midi in [60, 72] {
            let tiers: BTreeSet<u8> = selected
                .iter()
                .filter(|candidate| candidate.event.midi == midi)
                .map(|candidate| candidate.velocity_tier)
                .collect();
            assert_eq!(tiers, BTreeSet::from([36, 80, 116]));
        }
    }

    #[test]
    fn analyzer_extraction_configuration_rejects_invalid_bounds() {
        let config = AnalyzedExtractionConfig {
            min_midi: 90,
            max_midi: 60,
            ..AnalyzedExtractionConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn analyzer_event_gate_rejects_an_upper_partial_label() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..sample_rate / 2)
            .map(|index| {
                let phase = std::f32::consts::TAU * index as f32 / sample_rate as f32;
                0.75 * (220.0 * phase).sin() + 0.25 * (440.0 * phase).sin()
            })
            .collect();
        let mut event = NoteEvent {
            start_s: 0.0,
            end_s: 0.5,
            midi: 57,
            note: "A3".into(),
            frequency_hz: 220.0,
            cents: 0.0,
            velocity: 96,
            confidence: 0.99,
        };
        assert!(analyzed_event_matches_fundamental(
            &samples,
            sample_rate,
            &event
        ));
        event.midi = 69;
        event.note = "A4".into();
        event.frequency_hz = 440.0;
        assert!(!analyzed_event_matches_fundamental(
            &samples,
            sample_rate,
            &event
        ));
    }
}
