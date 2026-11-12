//! Musical note, chord, and percussion-onset transcription.
//!
//! This module deliberately uses deterministic DSP rather than a trained
//! classifier.  Harmonic salience with iterative spectral cancellation finds
//! several simultaneous fundamentals; temporal hysteresis turns frame
//! detections into notes instead of printing a new answer every analysis hop.

use crate::pitch;
use rustfft::{num_complex::Complex, FftPlanner};
use std::{cmp::Ordering, fmt::Write as _, fs, io, path::Path};

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Clone, Debug)]
pub struct TranscriptionConfig {
    pub tuning_hz: f32,
    pub min_midi: u8,
    pub max_midi: u8,
    pub max_polyphony: usize,
    pub min_note_ms: f32,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self {
            tuning_hz: 440.0,
            min_midi: 21,
            max_midi: 108,
            max_polyphony: 8,
            min_note_ms: 45.0,
        }
    }
}

impl TranscriptionConfig {
    pub fn validate(&self) -> io::Result<()> {
        if !self.tuning_hz.is_finite()
            || !(400.0..=480.0).contains(&self.tuning_hz)
            || self.min_midi > self.max_midi
            || self.max_midi > 127
            || !(1..=16).contains(&self.max_polyphony)
            || !self.min_note_ms.is_finite()
            || !(10.0..=2_000.0).contains(&self.min_note_ms)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid transcription configuration",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NoteEvent {
    pub start_s: f32,
    pub end_s: f32,
    pub midi: u8,
    pub note: String,
    pub frequency_hz: f32,
    pub cents: f32,
    pub velocity: u8,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChordEvent {
    pub start_s: f32,
    pub end_s: f32,
    pub notes: Vec<u8>,
    pub name: String,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PercussionEvent {
    pub time_s: f32,
    pub velocity: u8,
    pub brightness_hz: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transcription {
    pub sample_rate: u32,
    pub duration_s: f32,
    pub tuning_hz: f32,
    pub frame_size: usize,
    pub hop_size: usize,
    pub notes: Vec<NoteEvent>,
    pub chords: Vec<ChordEvent>,
    pub percussion: Vec<PercussionEvent>,
}

#[derive(Clone, Copy, Debug)]
struct FramePitch {
    midi: u8,
    cents: f32,
    confidence: f32,
    velocity: u8,
}

#[derive(Clone, Debug, Default)]
struct Tracker {
    pending_start: usize,
    pending_frames: usize,
    active_start: Option<usize>,
    absent_frames: usize,
    confidence_sum: f32,
    cents_sum: f32,
    velocity_sum: usize,
    observations: usize,
}

#[derive(Clone, Debug)]
struct FrameResult {
    pitches: Vec<FramePitch>,
    rms: f32,
    flux: f32,
    centroid: f32,
}

pub fn note_name(midi: u8) -> String {
    format!("{}{}", NOTE_NAMES[midi as usize % 12], midi as i16 / 12 - 1)
}

pub fn transcribe(
    samples: &[f32],
    sample_rate: u32,
    config: &TranscriptionConfig,
) -> io::Result<Transcription> {
    config.validate()?;
    if !(8_000..=384_000).contains(&sample_rate) || samples.len() < 256 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "audio is too short or has an unsupported sample rate",
        ));
    }
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "audio contains non-finite samples",
        ));
    }

    let frame_size = ((sample_rate as usize * 93 / 1_000).max(1_024))
        .next_power_of_two()
        .clamp(2_048, 8_192);
    let hop_size = frame_size / 8;
    let global_peak = samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max);
    if global_peak < 1e-6 {
        return Ok(Transcription {
            sample_rate,
            duration_s: samples.len() as f32 / sample_rate as f32,
            tuning_hz: config.tuning_hz,
            frame_size,
            hop_size,
            notes: Vec::new(),
            chords: Vec::new(),
            percussion: Vec::new(),
        });
    }

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(frame_size);
    let window: Vec<f32> = (0..frame_size)
        .map(|index| {
            0.5 - 0.5 * (std::f32::consts::TAU * index as f32 / (frame_size - 1) as f32).cos()
        })
        .collect();
    let mut previous_magnitudes = vec![0.0_f32; frame_size / 2];
    let mut frames = Vec::new();
    let padded_frames = samples.len().div_ceil(hop_size).max(1);
    for frame_index in 0..padded_frames {
        let start = frame_index * hop_size;
        let mut buffer = vec![Complex::new(0.0_f32, 0.0); frame_size];
        let mut energy = 0.0_f32;
        for index in 0..frame_size {
            let sample = samples.get(start + index).copied().unwrap_or(0.0);
            energy += sample * sample;
            buffer[index].re = sample * window[index];
        }
        let rms = (energy / frame_size as f32).sqrt();
        fft.process(&mut buffer);
        let magnitudes: Vec<f32> = buffer[..frame_size / 2]
            .iter()
            .map(|value| value.norm())
            .collect();
        let flux_numerator: f32 = magnitudes
            .iter()
            .zip(previous_magnitudes.iter())
            .map(|(current, previous)| (current - previous).max(0.0))
            .sum();
        let flux_denominator = magnitudes.iter().sum::<f32>().max(1e-9);
        let flux = flux_numerator / flux_denominator;
        let centroid = spectral_centroid(&magnitudes, sample_rate, frame_size);
        let pitches = if rms > global_peak * 0.0015 {
            frame_pitches(
                &magnitudes,
                &samples[start.min(samples.len())..(start + frame_size).min(samples.len())],
                sample_rate,
                frame_size,
                rms,
                global_peak,
                config,
            )
        } else {
            Vec::new()
        };
        frames.push(FrameResult {
            pitches,
            rms,
            flux,
            centroid,
        });
        previous_magnitudes = magnitudes;
    }

    let notes = track_notes(&frames, sample_rate, hop_size, config);
    let chords = track_chords(&frames, sample_rate, hop_size);
    let percussion = find_percussion(&frames, sample_rate, hop_size, global_peak);
    Ok(Transcription {
        sample_rate,
        duration_s: samples.len() as f32 / sample_rate as f32,
        tuning_hz: config.tuning_hz,
        frame_size,
        hop_size,
        notes,
        chords,
        percussion,
    })
}

fn frame_pitches(
    magnitudes: &[f32],
    time_samples: &[f32],
    sample_rate: u32,
    fft_size: usize,
    rms: f32,
    global_peak: f32,
    config: &TranscriptionConfig,
) -> Vec<FramePitch> {
    let mut residual = locally_whiten(magnitudes);
    let noise_floor = median(
        residual
            .iter()
            .skip(2)
            .copied()
            .filter(|value| value.is_finite())
            .collect(),
    )
    .max(1e-9);
    let velocity = amplitude_to_velocity(rms / global_peak.max(1e-9));
    let mut selected: Vec<FramePitch> = Vec::new();
    let mut first_score = 0.0_f32;
    for pass in 0..config.max_polyphony {
        let best = (config.min_midi..=config.max_midi)
            .filter(|midi| !selected.iter().any(|pitch| pitch.midi == *midi))
            .map(|midi| {
                (
                    midi,
                    harmonic_salience(
                        &residual,
                        pitch::midi_frequency(midi as f32, config.tuning_hz),
                        sample_rate,
                        fft_size,
                        noise_floor,
                    ),
                )
            })
            .max_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(Ordering::Equal));
        let Some((midi, score)) = best else {
            break;
        };
        if pass == 0 {
            first_score = score;
        }
        let relative_gate = if pass == 0 { 0.0 } else { first_score * 0.22 };
        if !score.is_finite() || score <= 1e-9 || score < relative_gate.max(noise_floor * 3.5) {
            break;
        }
        let hz = refine_frequency(
            magnitudes,
            pitch::midi_frequency(midi as f32, config.tuning_hz),
            sample_rate,
            fft_size,
        );
        let (_, cents) = pitch::frequency_to_midi(hz, config.tuning_hz).unwrap_or((midi, 0.0));
        let confidence = (score / (first_score + noise_floor)).clamp(0.0, 1.0);
        selected.push(FramePitch {
            midi,
            cents: cents.clamp(-50.0, 50.0),
            confidence,
            velocity,
        });
        cancel_harmonics(
            &mut residual,
            pitch::midi_frequency(midi as f32, config.tuning_hz),
            sample_rate,
            fft_size,
        );
    }

    // A high-clarity periodic estimate corrects octave mistakes in monophonic
    // frames and recovers a weak/missing fundamental that harmonic salience
    // alone may rank below an overtone.
    let periodic_pitch = pitch::estimate(
        time_samples,
        sample_rate,
        pitch::midi_frequency(config.min_midi as f32 - 0.5, config.tuning_hz),
        pitch::midi_frequency(config.max_midi as f32 + 0.5, config.tuning_hz),
        config.tuning_hz,
    );
    if let Some(yin) = periodic_pitch {
        if yin.clarity >= 0.72 {
            if let Some(existing) = selected.iter_mut().find(|item| item.midi == yin.midi) {
                existing.cents = yin.cents;
                existing.confidence = existing.confidence.max(yin.clarity);
            } else if selected.is_empty() {
                selected.push(FramePitch {
                    midi: yin.midi,
                    cents: yin.cents,
                    confidence: yin.clarity,
                    velocity,
                });
            } else if config.max_polyphony == 1 && yin.clarity > selected[0].confidence {
                selected[0] = FramePitch {
                    midi: yin.midi,
                    cents: yin.cents,
                    confidence: yin.clarity,
                    velocity,
                };
            }
        }
    }
    selected.sort_by_key(|pitch| pitch.midi);
    selected
}

/// Reduce the broad spectral envelope before harmonic summation so a dark,
/// loud source does not automatically hide a quieter bright source. The
/// envelope is a frequency-proportional local RMS; blending the original
/// magnitude with its envelope-normalized value preserves useful amplitude
/// differences while reducing formant and microphone-colour bias.
fn locally_whiten(magnitudes: &[f32]) -> Vec<f32> {
    if magnitudes.is_empty() {
        return Vec::new();
    }
    let mut prefix_energy = vec![0.0_f64; magnitudes.len() + 1];
    for (index, magnitude) in magnitudes.iter().enumerate() {
        let finite = if magnitude.is_finite() {
            magnitude.max(0.0) as f64
        } else {
            0.0
        };
        prefix_energy[index + 1] = prefix_energy[index] + finite * finite;
    }
    magnitudes
        .iter()
        .enumerate()
        .map(|(bin, magnitude)| {
            let magnitude = if magnitude.is_finite() {
                magnitude.max(0.0)
            } else {
                0.0
            };
            let radius = (bin / 10).clamp(3, 96);
            let start = bin.saturating_sub(radius);
            let end = (bin + radius + 1).min(magnitudes.len());
            let count = (end - start).max(1) as f64;
            let local_rms = ((prefix_energy[end] - prefix_energy[start]) / count)
                .max(0.0)
                .sqrt() as f32;
            if magnitude <= 0.0 || local_rms <= 1e-12 {
                0.0
            } else {
                (magnitude * (magnitude / local_rms).clamp(0.0, 64.0)).sqrt()
            }
        })
        .collect()
}

fn harmonic_salience(
    magnitudes: &[f32],
    fundamental: f32,
    sample_rate: u32,
    fft_size: usize,
    noise_floor: f32,
) -> f32 {
    let mut score = 0.0_f32;
    let mut weight_sum = 0.0_f32;
    let mut peaks = [0.0_f32; 12];
    let mut peak_count = 0;
    for harmonic in 1..=12 {
        let frequency = fundamental * harmonic as f32;
        if frequency >= sample_rate as f32 * 0.48 {
            break;
        }
        let center = frequency * fft_size as f32 / sample_rate as f32;
        let peak = peak_near(magnitudes, center, 1);
        peaks[harmonic - 1] = peak;
        peak_count = harmonic;
        let weight = if harmonic == 1 {
            1.8
        } else {
            1.0 / (harmonic as f32).sqrt()
        };
        score += (peak - noise_floor).max(0.0) * weight;
        weight_sum += weight;
    }
    if peak_count == 0 {
        return 0.0;
    }
    // A subharmonic can explain one real note as its fourth or fifth partial.
    // Require support in the candidate's first three partials while still
    // allowing a missing fundamental when the second and third are present.
    let strongest = peaks[..peak_count]
        .iter()
        .copied()
        .fold(0.0_f32, f32::max)
        .max(noise_floor);
    let low_support = peaks[0]
        .max(peaks.get(1).copied().unwrap_or(0.0) * 0.85)
        .max(peaks.get(2).copied().unwrap_or(0.0) * 0.7);
    let support = ((low_support - noise_floor).max(0.0) / strongest).clamp(0.0, 1.0);
    score / weight_sum.max(1e-6) * support.sqrt()
}

fn peak_near(magnitudes: &[f32], center: f32, radius: usize) -> f32 {
    if magnitudes.is_empty() {
        return 0.0;
    }
    let bin = center.round().max(0.0) as usize;
    let start = bin.saturating_sub(radius);
    let end = (bin + radius).min(magnitudes.len() - 1);
    magnitudes[start..=end]
        .iter()
        .copied()
        .fold(0.0_f32, f32::max)
}

fn cancel_harmonics(magnitudes: &mut [f32], fundamental: f32, sample_rate: u32, fft_size: usize) {
    for harmonic in 1..=16 {
        let frequency = fundamental * harmonic as f32;
        if frequency >= sample_rate as f32 * 0.49 {
            break;
        }
        let bin = (frequency * fft_size as f32 / sample_rate as f32).round() as usize;
        let radius = 1 + harmonic / 8;
        let start = bin.saturating_sub(radius);
        let end = (bin + radius).min(magnitudes.len().saturating_sub(1));
        for value in magnitudes.get_mut(start..=end).into_iter().flatten() {
            *value *= 0.08;
        }
    }
}

fn refine_frequency(
    magnitudes: &[f32],
    expected_hz: f32,
    sample_rate: u32,
    fft_size: usize,
) -> f32 {
    let expected = expected_hz * fft_size as f32 / sample_rate as f32;
    let center = expected
        .round()
        .clamp(1.0, magnitudes.len().saturating_sub(2) as f32) as usize;
    let left = magnitudes[center - 1].max(1e-12).ln();
    let middle = magnitudes[center].max(1e-12).ln();
    let right = magnitudes[center + 1].max(1e-12).ln();
    let denominator = left - 2.0 * middle + right;
    let offset = if denominator.abs() > 1e-9 {
        (0.5 * (left - right) / denominator).clamp(-0.5, 0.5)
    } else {
        0.0
    };
    (center as f32 + offset) * sample_rate as f32 / fft_size as f32
}

fn track_notes(
    frames: &[FrameResult],
    sample_rate: u32,
    hop_size: usize,
    config: &TranscriptionConfig,
) -> Vec<NoteEvent> {
    let mut trackers = vec![Tracker::default(); 128];
    let mut notes = Vec::new();
    for (frame_index, frame) in frames.iter().enumerate() {
        for midi in 0..=127_u8 {
            let observation = frame.pitches.iter().find(|pitch| pitch.midi == midi);
            let tracker = &mut trackers[midi as usize];
            match (tracker.active_start, observation) {
                (None, Some(pitch)) => {
                    if tracker.pending_frames == 0 {
                        tracker.pending_start = frame_index;
                    }
                    tracker.pending_frames += 1;
                    accumulate(tracker, pitch);
                    if tracker.pending_frames >= 2 {
                        tracker.active_start = Some(tracker.pending_start);
                        tracker.absent_frames = 0;
                    }
                }
                (None, None) => *tracker = Tracker::default(),
                (Some(_), Some(pitch)) => {
                    tracker.absent_frames = 0;
                    accumulate(tracker, pitch);
                }
                (Some(start), None) => {
                    tracker.absent_frames += 1;
                    if tracker.absent_frames >= 3 {
                        let end = frame_index.saturating_sub(tracker.absent_frames - 1);
                        finish_note(
                            &mut notes,
                            midi,
                            start,
                            end,
                            tracker,
                            hop_size as f32 / sample_rate as f32,
                            config,
                        );
                        *tracker = Tracker::default();
                    }
                }
            }
        }
    }
    for (midi, tracker) in trackers.iter().enumerate() {
        if let Some(start) = tracker.active_start {
            finish_note(
                &mut notes,
                midi as u8,
                start,
                frames.len(),
                tracker,
                hop_size as f32 / sample_rate as f32,
                config,
            );
        }
    }
    notes.sort_by(|left, right| {
        left.start_s
            .partial_cmp(&right.start_s)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.midi.cmp(&right.midi))
    });
    notes
}

fn accumulate(tracker: &mut Tracker, pitch: &FramePitch) {
    tracker.confidence_sum += pitch.confidence;
    tracker.cents_sum += pitch.cents;
    tracker.velocity_sum += pitch.velocity as usize;
    tracker.observations += 1;
}

fn finish_note(
    notes: &mut Vec<NoteEvent>,
    midi: u8,
    start: usize,
    end: usize,
    tracker: &Tracker,
    frame_seconds: f32,
    config: &TranscriptionConfig,
) {
    let duration_ms = end.saturating_sub(start) as f32 * frame_seconds * 1_000.0;
    if duration_ms < config.min_note_ms || tracker.observations == 0 {
        return;
    }
    let cents = tracker.cents_sum / tracker.observations as f32;
    notes.push(NoteEvent {
        start_s: start as f32 * frame_seconds,
        end_s: end as f32 * frame_seconds,
        midi,
        note: note_name(midi),
        frequency_hz: pitch::midi_frequency(midi as f32 + cents / 100.0, config.tuning_hz),
        cents,
        velocity: (tracker.velocity_sum / tracker.observations).clamp(1, 127) as u8,
        confidence: (tracker.confidence_sum / tracker.observations as f32).clamp(0.0, 1.0),
    });
}

fn track_chords(frames: &[FrameResult], sample_rate: u32, hop_size: usize) -> Vec<ChordEvent> {
    let mut output = Vec::new();
    let mut current: Vec<u8> = Vec::new();
    let mut start = 0;
    let mut confidence = 0.0_f32;
    let mut observations = 0;
    for (index, frame) in frames.iter().enumerate() {
        let mut notes: Vec<u8> = frame.pitches.iter().map(|pitch| pitch.midi).collect();
        notes.sort_unstable();
        notes.dedup();
        if notes == current {
            if !notes.is_empty() {
                confidence += frame
                    .pitches
                    .iter()
                    .map(|pitch| pitch.confidence)
                    .sum::<f32>()
                    / frame.pitches.len().max(1) as f32;
                observations += 1;
            }
            continue;
        }
        if current.len() >= 2 && index.saturating_sub(start) >= 2 {
            output.push(chord_event(
                &current,
                start,
                index,
                confidence / observations.max(1) as f32,
                sample_rate,
                hop_size,
            ));
        }
        current = notes;
        start = index;
        confidence = frame
            .pitches
            .iter()
            .map(|pitch| pitch.confidence)
            .sum::<f32>()
            / frame.pitches.len().max(1) as f32;
        observations = usize::from(!current.is_empty());
    }
    if current.len() >= 2 && frames.len().saturating_sub(start) >= 2 {
        output.push(chord_event(
            &current,
            start,
            frames.len(),
            confidence / observations.max(1) as f32,
            sample_rate,
            hop_size,
        ));
    }
    output
}

fn chord_event(
    notes: &[u8],
    start: usize,
    end: usize,
    confidence: f32,
    sample_rate: u32,
    hop_size: usize,
) -> ChordEvent {
    ChordEvent {
        start_s: start as f32 * hop_size as f32 / sample_rate as f32,
        end_s: end as f32 * hop_size as f32 / sample_rate as f32,
        notes: notes.to_vec(),
        name: chord_name(notes),
        confidence: confidence.clamp(0.0, 1.0),
    }
}

fn chord_name(notes: &[u8]) -> String {
    let mut pitch_classes: Vec<u8> = notes.iter().map(|note| note % 12).collect();
    pitch_classes.sort_unstable();
    pitch_classes.dedup();
    let patterns: &[(&[u8], &str)] = &[
        (&[0, 4, 7, 11], "maj7"),
        (&[0, 4, 7, 10], "7"),
        (&[0, 3, 7, 10], "m7"),
        (&[0, 3, 6, 9], "dim7"),
        (&[0, 4, 7], ""),
        (&[0, 3, 7], "m"),
        (&[0, 3, 6], "dim"),
        (&[0, 4, 8], "aug"),
        (&[0, 2, 7], "sus2"),
        (&[0, 5, 7], "sus4"),
    ];
    for root in &pitch_classes {
        let mut intervals: Vec<u8> = pitch_classes
            .iter()
            .map(|pitch_class| (12 + pitch_class - root) % 12)
            .collect();
        intervals.sort_unstable();
        for (pattern, suffix) in patterns {
            if intervals == *pattern {
                return format!("{}{}", NOTE_NAMES[*root as usize], suffix);
            }
        }
    }
    notes
        .iter()
        .map(|note| note_name(*note))
        .collect::<Vec<_>>()
        .join("+")
}

fn find_percussion(
    frames: &[FrameResult],
    sample_rate: u32,
    hop_size: usize,
    global_peak: f32,
) -> Vec<PercussionEvent> {
    if frames.len() < 3 {
        return Vec::new();
    }
    let flux_floor = median(frames.iter().map(|frame| frame.flux).collect()).max(0.005);
    let mut events = Vec::new();
    let minimum_gap = (sample_rate as usize / 20).div_ceil(hop_size).max(1);
    for index in 1..frames.len() - 1 {
        let frame = &frames[index];
        let local_peak =
            frame.flux > frames[index - 1].flux && frame.flux >= frames[index + 1].flux;
        let pitch_confidence = frame
            .pitches
            .iter()
            .map(|pitch| pitch.confidence)
            .fold(0.0_f32, f32::max);
        if local_peak
            && frame.flux > flux_floor * 2.5
            && frame.rms > global_peak * 0.003
            && pitch_confidence < 0.35
            && events.last().map_or(true, |event: &PercussionEvent| {
                index as f32 * hop_size as f32 / sample_rate as f32 - event.time_s
                    >= minimum_gap as f32 * hop_size as f32 / sample_rate as f32
            })
        {
            events.push(PercussionEvent {
                time_s: index as f32 * hop_size as f32 / sample_rate as f32,
                velocity: amplitude_to_velocity(frame.rms / global_peak.max(1e-9)),
                brightness_hz: frame.centroid,
                confidence: (frame.flux / (flux_floor * 5.0)).clamp(0.0, 1.0),
            });
        }
    }
    events
}

fn spectral_centroid(magnitudes: &[f32], sample_rate: u32, fft_size: usize) -> f32 {
    let sum = magnitudes.iter().sum::<f32>().max(1e-9);
    magnitudes
        .iter()
        .enumerate()
        .map(|(bin, magnitude)| *magnitude * bin as f32 * sample_rate as f32 / fft_size as f32)
        .sum::<f32>()
        / sum
}

fn amplitude_to_velocity(relative: f32) -> u8 {
    let db = 20.0 * relative.max(1e-6).log10();
    ((db + 60.0) / 60.0 * 126.0 + 1.0).round().clamp(1.0, 127.0) as u8
}

fn median(mut values: Vec<f32>) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    values[values.len() / 2]
}

impl Transcription {
    pub fn write_json(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, self.to_json())
    }

    pub fn to_json(&self) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "{{");
        let _ = writeln!(output, "  \"format\": \"Ruststrument transcription 1\",");
        let _ = writeln!(output, "  \"sample_rate\": {},", self.sample_rate);
        let _ = writeln!(output, "  \"duration_s\": {:.6},", self.duration_s);
        let _ = writeln!(output, "  \"tuning_hz\": {:.3},", self.tuning_hz);
        let _ = writeln!(output, "  \"notes\": [");
        for (index, note) in self.notes.iter().enumerate() {
            let comma = if index + 1 == self.notes.len() {
                ""
            } else {
                ","
            };
            let _ = writeln!(
                output,
                "    {{\"start_s\":{:.6},\"end_s\":{:.6},\"midi\":{},\"note\":\"{}\",\"frequency_hz\":{:.4},\"cents\":{:.3},\"velocity\":{},\"confidence\":{:.4}}}{}",
                note.start_s,
                note.end_s,
                note.midi,
                json_escape(&note.note),
                note.frequency_hz,
                note.cents,
                note.velocity,
                note.confidence,
                comma
            );
        }
        let _ = writeln!(output, "  ],");
        let _ = writeln!(output, "  \"chords\": [");
        for (index, chord) in self.chords.iter().enumerate() {
            let comma = if index + 1 == self.chords.len() {
                ""
            } else {
                ","
            };
            let notes = chord
                .notes
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let _ = writeln!(
                output,
                "    {{\"start_s\":{:.6},\"end_s\":{:.6},\"name\":\"{}\",\"notes\":[{}],\"confidence\":{:.4}}}{}",
                chord.start_s,
                chord.end_s,
                json_escape(&chord.name),
                notes,
                chord.confidence,
                comma
            );
        }
        let _ = writeln!(output, "  ],");
        let _ = writeln!(output, "  \"percussion\": [");
        for (index, event) in self.percussion.iter().enumerate() {
            let comma = if index + 1 == self.percussion.len() {
                ""
            } else {
                ","
            };
            let _ = writeln!(
                output,
                "    {{\"time_s\":{:.6},\"velocity\":{},\"brightness_hz\":{:.2},\"confidence\":{:.4}}}{}",
                event.time_s, event.velocity, event.brightness_hz, event.confidence, comma
            );
        }
        let _ = writeln!(output, "  ]");
        let _ = writeln!(output, "}}");
        output
    }

    pub fn write_csv(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut output =
            String::from("start_s,end_s,midi,note,frequency_hz,cents,velocity,confidence\n");
        for note in &self.notes {
            let _ = writeln!(
                output,
                "{:.6},{:.6},{},{},{:.4},{:.3},{},{:.4}",
                note.start_s,
                note.end_s,
                note.midi,
                note.note,
                note.frequency_hz,
                note.cents,
                note.velocity,
                note.confidence
            );
        }
        fs::write(path, output)
    }

    /// Write a Standard MIDI File (format 0, 480 PPQN, 120 BPM).
    pub fn write_midi(&self, path: impl AsRef<Path>) -> io::Result<()> {
        #[derive(Clone, Copy)]
        struct Event {
            tick: u32,
            status: u8,
            note: u8,
            velocity: u8,
            off: bool,
        }
        let ticks_per_second = 960.0_f32;
        let mut events = Vec::new();
        for note in &self.notes {
            events.push(Event {
                tick: (note.start_s * ticks_per_second).round().max(0.0) as u32,
                status: 0x90,
                note: note.midi,
                velocity: note.velocity,
                off: false,
            });
            events.push(Event {
                tick: (note.end_s * ticks_per_second).round().max(0.0) as u32,
                status: 0x80,
                note: note.midi,
                velocity: 0,
                off: true,
            });
        }
        for event in &self.percussion {
            let tick = (event.time_s * ticks_per_second).round().max(0.0) as u32;
            let note = if event.brightness_hz > 5_000.0 {
                42
            } else {
                38
            };
            events.push(Event {
                tick,
                status: 0x99,
                note,
                velocity: event.velocity,
                off: false,
            });
            events.push(Event {
                tick: tick + 60,
                status: 0x89,
                note,
                velocity: 0,
                off: true,
            });
        }
        events.sort_by_key(|event| (event.tick, !event.off, event.status, event.note));

        let mut track = vec![0x00, 0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20];
        let mut previous_tick = 0;
        for event in events {
            put_variable_length(&mut track, event.tick.saturating_sub(previous_tick));
            track.extend([event.status, event.note, event.velocity]);
            previous_tick = event.tick;
        }
        track.extend([0x00, 0xFF, 0x2F, 0x00]);

        let mut bytes = b"MThd".to_vec();
        bytes.extend(6_u32.to_be_bytes());
        bytes.extend(0_u16.to_be_bytes());
        bytes.extend(1_u16.to_be_bytes());
        bytes.extend(480_u16.to_be_bytes());
        bytes.extend(b"MTrk");
        bytes.extend(
            u32::try_from(track.len())
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "MIDI track too large"))?
                .to_be_bytes(),
        );
        bytes.extend(track);
        fs::write(path, bytes)
    }
}

fn put_variable_length(output: &mut Vec<u8>, mut value: u32) {
    value = value.min(0x0FFF_FFFF);
    let mut buffer = [0_u8; 4];
    let mut index = 3;
    buffer[index] = (value & 0x7F) as u8;
    while {
        value >>= 7;
        value > 0
    } {
        index -= 1;
        buffer[index] = ((value & 0x7F) as u8) | 0x80;
    }
    output.extend_from_slice(&buffer[index..]);
}

fn json_escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '\"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect(),
            '\n' => "\\n".chars().collect(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            character if character.is_control() => "?".chars().collect(),
            character => vec![character],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_common_chords() {
        assert_eq!(chord_name(&[60, 64, 67]), "C");
        assert_eq!(chord_name(&[57, 60, 64]), "Am");
        assert_eq!(chord_name(&[62, 67, 69]), "Dsus4");
    }

    #[test]
    fn transcribes_a_synthetic_a_major_chord() {
        let sample_rate = 44_100;
        let frequencies = [220.0_f32, 277.182_62, 329.627_56];
        let samples: Vec<f32> = (0..sample_rate as usize)
            .map(|index| {
                frequencies
                    .iter()
                    .map(|frequency| {
                        0.2 * (std::f32::consts::TAU * frequency * index as f32
                            / sample_rate as f32)
                            .sin()
                    })
                    .sum()
            })
            .collect();
        let result = transcribe(&samples, sample_rate, &TranscriptionConfig::default()).unwrap();
        for expected in [57, 61, 64] {
            assert!(
                result.notes.iter().any(|note| note.midi == expected),
                "missing {expected}: {:?}",
                result.notes
            );
        }
    }

    #[test]
    fn whitening_preserves_a_quieter_voice_above_a_strong_bass_envelope() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..sample_rate as usize)
            .map(|index| {
                let time = index as f32 / sample_rate as f32;
                let bass = (1..=8)
                    .map(|harmonic| {
                        0.32 / harmonic as f32
                            * (std::f32::consts::TAU * 110.0 * harmonic as f32 * time).sin()
                    })
                    .sum::<f32>();
                let upper = 0.11 * (std::f32::consts::TAU * 523.251 * time).sin()
                    + 0.035 * (std::f32::consts::TAU * 1_046.502 * time).sin();
                bass + upper
            })
            .collect();
        let result = transcribe(&samples, sample_rate, &TranscriptionConfig::default()).unwrap();
        assert!(result.notes.iter().any(|note| note.midi == 45));
        assert!(
            result.notes.iter().any(|note| note.midi == 72),
            "quieter C5 was lost: {:?}",
            result.notes
        );
    }

    #[test]
    fn local_whitening_is_finite_for_silence_and_peaks() {
        let mut magnitudes = vec![0.0; 512];
        magnitudes[20] = 10.0;
        magnitudes[200] = 0.5;
        let whitened = locally_whiten(&magnitudes);
        assert_eq!(whitened.len(), magnitudes.len());
        assert!(whitened.iter().all(|value| value.is_finite()));
        assert!(whitened[20] > 0.0 && whitened[200] > 0.0);
    }

    #[test]
    fn midi_writer_emits_standard_header() {
        let transcription = Transcription {
            sample_rate: 44_100,
            duration_s: 1.0,
            tuning_hz: 440.0,
            frame_size: 4_096,
            hop_size: 512,
            notes: vec![NoteEvent {
                start_s: 0.0,
                end_s: 0.5,
                midi: 69,
                note: "A4".into(),
                frequency_hz: 440.0,
                cents: 0.0,
                velocity: 100,
                confidence: 1.0,
            }],
            chords: Vec::new(),
            percussion: Vec::new(),
        };
        let path = std::env::temp_dir().join("ruststrument-transcription-test.mid");
        transcription.write_midi(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        let _ = fs::remove_file(path);
        assert_eq!(&bytes[..4], b"MThd");
        assert!(bytes.windows(4).any(|window| window == b"MTrk"));
    }

    #[test]
    fn detects_unpitched_impulse_events_without_calling_tonal_attacks_drums() {
        let sample_rate = 44_100;
        let mut samples = vec![0.0_f32; sample_rate as usize];
        for start in [sample_rate as usize / 5, sample_rate as usize * 3 / 5] {
            for index in 0..1_024 {
                let noise = if (index * 17 + 11) % 29 < 14 {
                    1.0
                } else {
                    -1.0
                };
                samples[start + index] = noise * (-(index as f32) / 180.0).exp() * 0.8;
            }
        }
        let result = transcribe(&samples, sample_rate, &TranscriptionConfig::default()).unwrap();
        assert!(result.percussion.len() >= 2, "{:?}", result.percussion);
    }
}
