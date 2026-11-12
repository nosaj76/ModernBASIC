//! Strict comparison between a bank's audible output and its captured regions.
//!
//! A learned bank must not be called reference-matched merely because its
//! output lands on the requested MIDI note. This module compares every baked
//! attack region with the same captured region played through the sample path.
//! It also reports mapped keys and techniques for which no exact recording
//! exists. Generated banks contain silent zoning samples, so they are
//! deliberately unverifiable rather than being allowed to pass.

use crate::{
    analyze,
    profile::{InstrumentBank, SynthModel, Trigger},
    synth, technique,
};
use rustfft::{num_complex::Complex, FftPlanner};
use std::fmt::Write;

const MAX_PITCH_CENTS: f32 = 5.0;
const MIN_HARMONIC_SIMILARITY: f32 = 0.95;
const MIN_SPECTRAL_SIMILARITY: f32 = 0.95;
const MIN_ENVELOPE_SIMILARITY: f32 = 0.95;
const MIN_WAVEFORM_SIMILARITY: f32 = 0.90;
const MAX_ATTACK_DELTA_MS: f32 = 5.0;
const MAX_DECAY_DELTA_PERCENT: f32 = 5.0;
const MAX_DURATION_DELTA_PERCENT: f32 = 5.0;
const MAX_LEVEL_DELTA_DB: f32 = 0.5;
// These deliberately reject marginal, pitched contaminants. Listening review
// found that 85% clarity / 80% purity still admitted a tonal noise event as a
// playable note, even though exact source-to-output comparison passed.
const MIN_REFERENCE_CLARITY: f32 = 0.87;
const MIN_REFERENCE_TONAL_PURITY: f32 = 0.90;
const MAX_REFERENCE_FLATNESS: f32 = 0.04;

#[derive(Clone, Debug)]
pub struct ReferenceMatchReport {
    pub text: String,
    pub passed_regions: usize,
    pub failed_regions: usize,
    pub contaminated_regions: usize,
    pub unverifiable_regions: usize,
    pub unmatched_mappings: usize,
    pub missing_techniques: Vec<String>,
}

impl ReferenceMatchReport {
    pub fn passed(&self) -> bool {
        self.failed_regions == 0
            && self.unverifiable_regions == 0
            && self.contaminated_regions == 0
            && self.unmatched_mappings == 0
            && self.missing_techniques.is_empty()
    }
}

#[derive(Clone, Copy, Debug)]
struct Envelope {
    attack_s: f32,
    decay_s: f32,
    duration_s: f32,
}

#[derive(Clone, Copy, Debug)]
struct Metrics {
    pitch_cents: f32,
    harmonic_similarity: f32,
    spectral_similarity: f32,
    envelope_similarity: f32,
    waveform_similarity: f32,
    attack_delta_ms: f32,
    decay_delta_percent: f32,
    duration_delta_percent: f32,
    level_delta_db: f32,
    reference_clarity: f32,
    reference_tonal_purity: f32,
    reference_flatness: f32,
}

#[derive(Clone, Copy, Debug)]
struct ReleaseMetrics {
    spectral_similarity: f32,
    envelope_similarity: f32,
    waveform_similarity: f32,
    attack_delta_ms: f32,
    decay_delta_percent: f32,
    duration_delta_percent: f32,
    level_delta_db: f32,
}

impl ReleaseMetrics {
    fn passed(self) -> bool {
        self.spectral_similarity >= MIN_SPECTRAL_SIMILARITY
            && self.envelope_similarity >= MIN_ENVELOPE_SIMILARITY
            && self.waveform_similarity >= MIN_WAVEFORM_SIMILARITY
            && self.attack_delta_ms <= MAX_ATTACK_DELTA_MS
            && self.decay_delta_percent <= MAX_DECAY_DELTA_PERCENT
            && self.duration_delta_percent <= MAX_DURATION_DELTA_PERCENT
            && self.level_delta_db <= MAX_LEVEL_DELTA_DB
    }
}

impl Metrics {
    fn signal_passed(self) -> bool {
        self.pitch_cents <= MAX_PITCH_CENTS
            && self.harmonic_similarity >= MIN_HARMONIC_SIMILARITY
            && self.spectral_similarity >= MIN_SPECTRAL_SIMILARITY
            && self.envelope_similarity >= MIN_ENVELOPE_SIMILARITY
            && self.waveform_similarity >= MIN_WAVEFORM_SIMILARITY
            && self.attack_delta_ms <= MAX_ATTACK_DELTA_MS
            && self.decay_delta_percent <= MAX_DECAY_DELTA_PERCENT
            && self.duration_delta_percent <= MAX_DURATION_DELTA_PERCENT
            && self.level_delta_db <= MAX_LEVEL_DELTA_DB
    }

    fn source_clean(self) -> bool {
        self.reference_clarity >= MIN_REFERENCE_CLARITY
            && self.reference_tonal_purity >= MIN_REFERENCE_TONAL_PURITY
            && self.reference_flatness <= MAX_REFERENCE_FLATNESS
    }

    fn passed(self) -> bool {
        self.signal_passed() && self.source_clean()
    }
}

fn peak(samples: &[f32]) -> f32 {
    samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max)
}

fn onset(samples: &[f32]) -> usize {
    let threshold = peak(samples) * 0.025;
    samples
        .iter()
        .position(|sample| sample.abs() >= threshold)
        .unwrap_or(0)
}

fn cosine(left: &[f32], right: &[f32]) -> f32 {
    let length = left.len().max(right.len());
    let mut dot = 0.0_f64;
    let mut left_energy = 0.0_f64;
    let mut right_energy = 0.0_f64;
    for index in 0..length {
        let left = f64::from(left.get(index).copied().unwrap_or(0.0));
        let right = f64::from(right.get(index).copied().unwrap_or(0.0));
        dot += left * right;
        left_energy += left * left;
        right_energy += right * right;
    }
    if left_energy <= 1e-18 || right_energy <= 1e-18 {
        0.0
    } else {
        (dot / (left_energy * right_energy).sqrt()).clamp(-1.0, 1.0) as f32
    }
}

fn rms(samples: &[f32], length: usize) -> f32 {
    let length = length.min(samples.len()).max(1);
    (samples[..length]
        .iter()
        .map(|sample| sample * sample)
        .sum::<f32>()
        / length as f32)
        .sqrt()
}

fn frame_envelope(samples: &[f32], sample_rate: u32) -> Vec<f32> {
    let frame = (sample_rate as usize / 200).max(1);
    samples
        .chunks(frame)
        .map(|chunk| {
            (chunk.iter().map(|sample| sample * sample).sum::<f32>() / chunk.len() as f32).sqrt()
        })
        .collect()
}

fn envelope(samples: &[f32], sample_rate: u32) -> (Envelope, Vec<f32>) {
    let mut values = frame_envelope(samples, sample_rate);
    let maximum = values.iter().copied().fold(0.0_f32, f32::max);
    if maximum <= 1e-9 {
        return (
            Envelope {
                attack_s: 0.0,
                decay_s: 0.0,
                duration_s: 0.0,
            },
            values,
        );
    }
    for value in &mut values {
        *value /= maximum;
    }
    let peak_index = values.iter().position(|value| *value >= 0.999).unwrap_or(0);
    let decay_index = values[peak_index..]
        .iter()
        .position(|value| *value <= 0.37)
        .map(|index| peak_index + index)
        .unwrap_or(values.len().saturating_sub(1));
    let duration_index = values.iter().rposition(|value| *value >= 0.01).unwrap_or(0);
    let seconds_per_frame = 0.005;
    (
        Envelope {
            attack_s: peak_index as f32 * seconds_per_frame,
            decay_s: decay_index.saturating_sub(peak_index) as f32 * seconds_per_frame,
            duration_s: (duration_index + 1) as f32 * seconds_per_frame,
        },
        values,
    )
}

fn relative_delta(left: f32, right: f32) -> f32 {
    (left - right).abs() / right.abs().max(0.005) * 100.0
}

fn harmonic_profile(samples: &[f32], sample_rate: u32, fundamental: f32) -> [f32; 16] {
    let mut output = [0.0_f32; 16];
    if samples.len() < 256 || !fundamental.is_finite() || fundamental <= 0.0 {
        return output;
    }
    let start = onset(samples);
    let available = samples.len().saturating_sub(start);
    let length = available.min(8_192);
    if length < 256 {
        return output;
    }
    for (index, amplitude) in output.iter_mut().enumerate() {
        let frequency = fundamental * (index + 1) as f32;
        if frequency >= sample_rate as f32 * 0.48 {
            break;
        }
        let mut real = 0.0_f64;
        let mut imaginary = 0.0_f64;
        for offset in 0..length {
            let window =
                0.5 - 0.5 * (std::f32::consts::TAU * offset as f32 / length.max(1) as f32).cos();
            let angle =
                std::f64::consts::TAU * f64::from(frequency) * offset as f64 / sample_rate as f64;
            let sample = f64::from(samples[start + offset] * window);
            real += sample * angle.cos();
            imaginary -= sample * angle.sin();
        }
        *amplitude = (real * real + imaginary * imaginary).sqrt() as f32;
    }
    let maximum = output.iter().copied().fold(0.0_f32, f32::max);
    if maximum > 1e-9 {
        for amplitude in &mut output {
            *amplitude /= maximum;
        }
    }
    output
}

fn spectral_profile(samples: &[f32], sample_rate: u32) -> [f32; 8] {
    const EDGES: [f32; 9] = [
        20.0, 60.0, 250.0, 500.0, 2_000.0, 4_000.0, 6_000.0, 12_000.0, 20_000.0,
    ];
    let mut output = [0.0_f32; 8];
    let start = onset(samples);
    let available = samples.len().saturating_sub(start);
    if available < 256 {
        return output;
    }
    let used = available.min(8_192);
    let fft_size = used.next_power_of_two().min(8_192);
    let mut buffer = vec![Complex::new(0.0_f32, 0.0); fft_size];
    for index in 0..used.min(fft_size) {
        let window = 0.5 - 0.5 * (std::f32::consts::TAU * index as f32 / used.max(1) as f32).cos();
        buffer[index].re = samples[start + index] * window;
    }
    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(fft_size).process(&mut buffer);
    for (bin, value) in buffer.iter().take(fft_size / 2).enumerate() {
        let frequency = bin as f32 * sample_rate as f32 / fft_size as f32;
        if let Some(band) = (0..output.len())
            .find(|index| frequency >= EDGES[*index] && frequency < EDGES[*index + 1])
        {
            output[band] += value.norm_sqr();
        }
    }
    let total = output.iter().sum::<f32>();
    if total > 1e-12 {
        for value in &mut output {
            *value = (*value / total).sqrt();
        }
    }
    output
}

fn source_quality(samples: &[f32], sample_rate: u32, fundamental: f32) -> (f32, f32) {
    let onset = onset(samples);
    let delayed = onset
        .saturating_add(sample_rate as usize / 50)
        .min(samples.len());
    let start = if samples.len().saturating_sub(delayed) >= 256 {
        delayed
    } else {
        onset
    };
    let used = samples.len().saturating_sub(start).min(8_192);
    if used < 256 {
        return (0.0, 1.0);
    }
    let fft_size = used.next_power_of_two().min(8_192);
    let mut buffer = vec![Complex::new(0.0_f32, 0.0); fft_size];
    for index in 0..used.min(fft_size) {
        let window = 0.5 - 0.5 * (std::f32::consts::TAU * index as f32 / used as f32).cos();
        buffer[index].re = samples[start + index] * window;
    }
    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(fft_size).process(&mut buffer);
    let bin_hz = sample_rate as f32 / fft_size as f32;
    let first_bin = (20.0 / bin_hz).floor().max(1.0) as usize;
    let last_bin =
        ((20_000.0_f32.min(sample_rate as f32 * 0.48) / bin_hz).ceil() as usize).min(fft_size / 2);
    if first_bin >= last_bin {
        return (0.0, 1.0);
    }
    let powers: Vec<f32> = buffer[first_bin..last_bin]
        .iter()
        .map(|value| value.norm_sqr())
        .collect();
    let total = powers.iter().sum::<f32>();
    let maximum = powers.iter().copied().fold(0.0_f32, f32::max);
    if total <= 1e-12 || maximum <= 1e-12 {
        return (0.0, 1.0);
    }
    let floor = maximum * 1e-8;
    let geometric = (powers
        .iter()
        .map(|power| power.max(floor).ln())
        .sum::<f32>()
        / powers.len() as f32)
        .exp();
    let arithmetic = total / powers.len() as f32;
    let flatness = (geometric / arithmetic.max(1e-12)).clamp(0.0, 1.0);

    let mut harmonic_energy = 0.0;
    for (offset, power) in powers.iter().enumerate() {
        let frequency = (first_bin + offset) as f32 * bin_hz;
        let harmonic = (frequency / fundamental).round().clamp(1.0, 16.0);
        let center = fundamental * harmonic;
        let tolerance = (center * 0.02).max(bin_hz * 2.0);
        if (frequency - center).abs() <= tolerance {
            harmonic_energy += *power;
        }
    }
    ((harmonic_energy / total).clamp(0.0, 1.0), flatness)
}

fn compare(reference: &[f32], actual: &[f32], sample_rate: u32) -> Option<Metrics> {
    let reference_estimate = crate::pitch::estimate(
        reference,
        sample_rate,
        20.0,
        (sample_rate as f32 * 0.45).min(5_000.0),
        440.0,
    )?;
    let reference_pitch = analyze::identify_pitch(reference, sample_rate)?;
    let actual_pitch = analyze::identify_pitch(actual, sample_rate)?;
    let pitch_cents = (1_200.0 * (actual_pitch.hz / reference_pitch.hz).log2()).abs();
    let reference_harmonics = harmonic_profile(reference, sample_rate, reference_pitch.hz);
    let actual_harmonics = harmonic_profile(actual, sample_rate, actual_pitch.hz);
    let reference_spectrum = spectral_profile(reference, sample_rate);
    let actual_spectrum = spectral_profile(actual, sample_rate);
    let (reference_tonal_purity, reference_flatness) =
        source_quality(reference, sample_rate, reference_pitch.hz);
    let (reference_envelope, reference_envelope_values) = envelope(reference, sample_rate);
    let (actual_envelope, actual_envelope_values) = envelope(actual, sample_rate);
    let level_frames = reference.len().min(actual.len());
    let reference_rms = rms(reference, level_frames);
    let actual_rms = rms(actual, level_frames);
    let level_delta_db = if reference_rms <= 1e-9 || actual_rms <= 1e-9 {
        f32::INFINITY
    } else {
        (20.0 * (actual_rms / reference_rms).log10()).abs()
    };
    Some(Metrics {
        pitch_cents,
        harmonic_similarity: cosine(&reference_harmonics, &actual_harmonics),
        spectral_similarity: cosine(&reference_spectrum, &actual_spectrum),
        envelope_similarity: cosine(&reference_envelope_values, &actual_envelope_values),
        waveform_similarity: cosine(reference, actual),
        attack_delta_ms: (actual_envelope.attack_s - reference_envelope.attack_s).abs() * 1_000.0,
        decay_delta_percent: relative_delta(actual_envelope.decay_s, reference_envelope.decay_s),
        duration_delta_percent: relative_delta(
            actual_envelope.duration_s,
            reference_envelope.duration_s,
        ),
        level_delta_db,
        reference_clarity: reference_estimate.clarity,
        reference_tonal_purity,
        reference_flatness,
    })
}

fn compare_release(reference: &[f32], actual: &[f32], sample_rate: u32) -> ReleaseMetrics {
    let reference_spectrum = spectral_profile(reference, sample_rate);
    let actual_spectrum = spectral_profile(actual, sample_rate);
    let (reference_envelope, reference_envelope_values) = envelope(reference, sample_rate);
    let (actual_envelope, actual_envelope_values) = envelope(actual, sample_rate);
    let level_frames = reference.len().min(actual.len());
    let reference_rms = rms(reference, level_frames);
    let actual_rms = rms(actual, level_frames);
    let level_delta_db = if reference_rms <= 1e-9 || actual_rms <= 1e-9 {
        f32::INFINITY
    } else {
        (20.0 * (actual_rms / reference_rms).log10()).abs()
    };
    ReleaseMetrics {
        spectral_similarity: cosine(&reference_spectrum, &actual_spectrum),
        envelope_similarity: cosine(&reference_envelope_values, &actual_envelope_values),
        waveform_similarity: cosine(reference, actual),
        attack_delta_ms: (actual_envelope.attack_s - reference_envelope.attack_s).abs() * 1_000.0,
        decay_delta_percent: relative_delta(actual_envelope.decay_s, reference_envelope.decay_s),
        duration_delta_percent: relative_delta(
            actual_envelope.duration_s,
            reference_envelope.duration_s,
        ),
        level_delta_db,
    }
}

fn exact_mapping_gaps(bank: &InstrumentBank) -> usize {
    let mut gaps = 0;
    for region in bank
        .regions
        .iter()
        .filter(|region| region.trigger == Trigger::Attack)
    {
        for key in region.key_lo..=region.key_hi {
            if key != region.root_key {
                gaps += 1;
            }
        }
    }
    gaps
}

pub fn report(bank: &InstrumentBank) -> ReferenceMatchReport {
    let mut output = String::new();
    let mut passed_regions = 0;
    let mut failed_regions = 0;
    let mut contaminated_regions = 0;
    let mut unverifiable_regions = 0;
    let unmatched_mappings = exact_mapping_gaps(bank);
    let captured: std::collections::BTreeSet<String> = bank
        .regions
        .iter()
        .filter(|region| peak(&region.samples) >= 1e-5)
        .map(|region| technique::canonicalize(&bank.name, &bank.family, &region.articulation))
        .collect();
    let missing_techniques: Vec<String> = technique::recommended(&bank.name, &bank.family)
        .into_iter()
        .filter(|name| !captured.contains(*name))
        .map(str::to_string)
        .collect();

    writeln!(output, "# Strict reference-match report: {}\n", bank.name).unwrap();
    writeln!(output, "A pass requires every baked attack and release to match its captured reference and forbids unrecorded key stretching or missing recommended techniques.\n").unwrap();
    writeln!(output, "## Thresholds\n").unwrap();
    writeln!(output, "- Pitch: <= {MAX_PITCH_CENTS:.1} cents").unwrap();
    writeln!(
        output,
        "- Harmonic and spectral similarity: >= {:.0}%",
        MIN_HARMONIC_SIMILARITY * 100.0
    )
    .unwrap();
    writeln!(
        output,
        "- Envelope similarity: >= {:.0}%",
        MIN_ENVELOPE_SIMILARITY * 100.0
    )
    .unwrap();
    writeln!(
        output,
        "- Waveform similarity: >= {:.0}%",
        MIN_WAVEFORM_SIMILARITY * 100.0
    )
    .unwrap();
    writeln!(output, "- Attack: <= {MAX_ATTACK_DELTA_MS:.1} ms; decay and duration: <= {MAX_DECAY_DELTA_PERCENT:.1}%").unwrap();
    writeln!(output, "- Level: <= {MAX_LEVEL_DELTA_DB:.1} dB\n").unwrap();
    writeln!(
        output,
        "- Reference cleanliness: clarity >= {:.0}%, tonal purity >= {:.0}%, spectral flatness <= {:.3}\n",
        MIN_REFERENCE_CLARITY * 100.0,
        MIN_REFERENCE_TONAL_PURITY * 100.0,
        MAX_REFERENCE_FLATNESS
    )
    .unwrap();
    writeln!(output, "## Region results\n").unwrap();
    writeln!(output, "| Region | Root | Velocity | Pitch | Harmonics | Spectrum | Envelope | Waveform | Attack | Decay | Duration | Level | Clarity | Purity | Flatness | Result |").unwrap();
    writeln!(
        output,
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|:---:|"
    )
    .unwrap();

    let mut reference_bank = bank.clone();
    reference_bank.synth.model = SynthModel::SampleOnly;
    reference_bank.synth.sample_level = 1.0;
    reference_bank.synth.synth_level = 0.0;
    for (index, region) in bank.regions.iter().enumerate() {
        if region.trigger != Trigger::Attack {
            continue;
        }
        if peak(&region.samples) < 1e-5 {
            unverifiable_regions += 1;
            writeln!(
                output,
                "| {index} | {} | {}-{} | - | - | - | - | - | - | - | - | - | - | - | - | UNVERIFIABLE |",
                region.root_key, region.vel_lo, region.vel_hi
            )
            .unwrap();
            continue;
        }
        let reference = synth::bake_region(&reference_bank, index).ok();
        let actual = synth::bake_region(bank, index).ok();
        let metrics = reference
            .as_ref()
            .zip(actual.as_ref())
            .and_then(|(reference, actual)| {
                if reference.sample_rate != actual.sample_rate {
                    None
                } else {
                    compare(&reference.samples, &actual.samples, reference.sample_rate)
                }
            });
        let Some(metrics) = metrics else {
            failed_regions += 1;
            writeln!(
                output,
                "| {index} | {} | {}-{} | - | - | - | - | - | - | - | - | - | - | - | - | FAIL |",
                region.root_key, region.vel_lo, region.vel_hi
            )
            .unwrap();
            continue;
        };
        let passed = metrics.passed();
        if !metrics.source_clean() {
            contaminated_regions += 1;
        }
        if passed {
            passed_regions += 1;
        } else {
            failed_regions += 1;
        }
        writeln!(
            output,
            "| {index} | {} | {}-{} | {:.2}c | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}ms | {:.1}% | {:.1}% | {:.2}dB | {:.1}% | {:.1}% | {:.4} | {} |",
            region.root_key,
            region.vel_lo,
            region.vel_hi,
            metrics.pitch_cents,
            metrics.harmonic_similarity * 100.0,
            metrics.spectral_similarity * 100.0,
            metrics.envelope_similarity * 100.0,
            metrics.waveform_similarity * 100.0,
            metrics.attack_delta_ms,
            metrics.decay_delta_percent,
            metrics.duration_delta_percent,
            metrics.level_delta_db,
            metrics.reference_clarity * 100.0,
            metrics.reference_tonal_purity * 100.0,
            metrics.reference_flatness,
            if passed { "PASS" } else { "FAIL" },
        )
        .unwrap();
    }

    let release_regions: Vec<_> = bank
        .regions
        .iter()
        .enumerate()
        .filter(|(_, region)| region.trigger == Trigger::Release)
        .collect();
    if !release_regions.is_empty() {
        writeln!(output, "\n## Release-region results\n").unwrap();
        writeln!(output, "Release and key-off recordings may be intentionally unpitched, so they are compared across broad spectrum, amplitude envelope, waveform, timing, duration, and level without a tonal-source-purity requirement.\n").unwrap();
        writeln!(output, "| Region | Root | Velocity | Spectrum | Envelope | Waveform | Attack | Decay | Duration | Level | Result |").unwrap();
        writeln!(
            output,
            "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|:---:|"
        )
        .unwrap();
        for (index, region) in release_regions {
            if peak(&region.samples) < 1e-5 {
                unverifiable_regions += 1;
                writeln!(
                    output,
                    "| {index} | {} | {}-{} | - | - | - | - | - | - | - | UNVERIFIABLE |",
                    region.root_key, region.vel_lo, region.vel_hi
                )
                .unwrap();
                continue;
            }
            let reference = synth::bake_region(&reference_bank, index).ok();
            let actual = synth::bake_region(bank, index).ok();
            let metrics =
                reference
                    .as_ref()
                    .zip(actual.as_ref())
                    .and_then(|(reference, actual)| {
                        (reference.sample_rate == actual.sample_rate).then(|| {
                            compare_release(
                                &reference.samples,
                                &actual.samples,
                                reference.sample_rate,
                            )
                        })
                    });
            let Some(metrics) = metrics else {
                failed_regions += 1;
                writeln!(
                    output,
                    "| {index} | {} | {}-{} | - | - | - | - | - | - | - | FAIL |",
                    region.root_key, region.vel_lo, region.vel_hi
                )
                .unwrap();
                continue;
            };
            let passed = metrics.passed();
            if passed {
                passed_regions += 1;
            } else {
                failed_regions += 1;
            }
            writeln!(
                output,
                "| {index} | {} | {}-{} | {:.1}% | {:.1}% | {:.1}% | {:.1}ms | {:.1}% | {:.1}% | {:.2}dB | {} |",
                region.root_key,
                region.vel_lo,
                region.vel_hi,
                metrics.spectral_similarity * 100.0,
                metrics.envelope_similarity * 100.0,
                metrics.waveform_similarity * 100.0,
                metrics.attack_delta_ms,
                metrics.decay_delta_percent,
                metrics.duration_delta_percent,
                metrics.level_delta_db,
                if passed { "PASS" } else { "FAIL" },
            )
            .unwrap();
        }
    }

    let passed = failed_regions == 0
        && contaminated_regions == 0
        && unverifiable_regions == 0
        && unmatched_mappings == 0
        && missing_techniques.is_empty();
    writeln!(output, "\n## Coverage and verdict\n").unwrap();
    writeln!(output, "- Passing captured regions: {passed_regions}").unwrap();
    writeln!(output, "- Failing captured regions: {failed_regions}").unwrap();
    writeln!(
        output,
        "- References failing cleanliness: {contaminated_regions}"
    )
    .unwrap();
    writeln!(
        output,
        "- Regions without audible references: {unverifiable_regions}"
    )
    .unwrap();
    writeln!(
        output,
        "- Unrecorded mapped-key positions: {unmatched_mappings}"
    )
    .unwrap();
    writeln!(
        output,
        "- Missing recorded techniques: {}",
        if missing_techniques.is_empty() {
            "none".to_string()
        } else {
            missing_techniques.join(", ")
        }
    )
    .unwrap();
    writeln!(
        output,
        "- **Strict result: {}**",
        if passed { "PASS" } else { "FAIL" }
    )
    .unwrap();

    ReferenceMatchReport {
        text: output,
        passed_regions,
        failed_regions,
        contaminated_regions,
        unverifiable_regions,
        unmatched_mappings,
        missing_techniques,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{profile::InstrumentBank, wav::Wav};

    #[test]
    fn sample_only_exact_root_passes_signal_comparison() {
        let source = Wav {
            sample_rate: 44_100,
            samples: (0..22_050)
                .map(|index| {
                    let time = index as f32 / 44_100.0;
                    (std::f32::consts::TAU * 440.0 * time).sin() * (-time * 5.0).exp()
                })
                .collect(),
        };
        let mut bank = InstrumentBank::single("reference", &source);
        bank.synth.model = SynthModel::SampleOnly;
        bank.synth.sample_level = 1.0;
        bank.synth.synth_level = 0.0;
        bank.regions[0].key_lo = bank.regions[0].root_key;
        bank.regions[0].key_hi = bank.regions[0].root_key;
        let report = report(&bank);
        assert_eq!(report.passed_regions, 1);
        assert_eq!(report.failed_regions, 0);
    }

    #[test]
    fn release_regions_are_matched_and_cover_key_off() {
        let source = Wav {
            sample_rate: 44_100,
            samples: (0..22_050)
                .map(|index| {
                    let time = index as f32 / 44_100.0;
                    (std::f32::consts::TAU * 440.0 * time).sin() * (-time * 5.0).exp()
                })
                .collect(),
        };
        let mut bank = InstrumentBank::single("Concert Grand Piano", &source);
        bank.synth.model = SynthModel::SampleOnly;
        bank.synth.sample_level = 1.0;
        bank.synth.synth_level = 0.0;
        bank.regions[0].key_lo = bank.regions[0].root_key;
        bank.regions[0].key_hi = bank.regions[0].root_key;
        let mut release = bank.regions[0].clone();
        release.name = "key off".into();
        release.articulation = "key_off".into();
        release.trigger = Trigger::Release;
        bank.regions.push(release);

        let matched = report(&bank);
        assert_eq!(matched.passed_regions, 2);
        assert_eq!(matched.failed_regions, 0);
        assert!(!matched
            .missing_techniques
            .iter()
            .any(|name| name == "key_off"));
        assert!(matched.text.contains("## Release-region results"));

        bank.synth.sample_level = 0.5;
        let attenuated = report(&bank);
        assert_eq!(attenuated.passed_regions, 0);
        assert_eq!(attenuated.failed_regions, 2);
    }

    #[test]
    fn generated_bank_is_not_a_reference_match() {
        let bank = InstrumentBank::generator("Concert Flute");
        let report = report(&bank);
        assert!(!report.passed());
        assert!(report.unverifiable_regions > 0);
    }

    #[test]
    fn unrelated_tone_fails_reference_purity() {
        let clean: Vec<f32> = (0..22_050)
            .map(|index| (std::f32::consts::TAU * 440.0 * index as f32 / 44_100.0).sin())
            .collect();
        let contaminated: Vec<f32> = clean
            .iter()
            .enumerate()
            .map(|(index, sample)| {
                *sample + 0.8 * (std::f32::consts::TAU * 523.25 * index as f32 / 44_100.0).sin()
            })
            .collect();
        let (clean_purity, _) = source_quality(&clean, 44_100, 440.0);
        let (contaminated_purity, _) = source_quality(&contaminated, 44_100, 440.0);
        assert!(clean_purity >= MIN_REFERENCE_TONAL_PURITY, "{clean_purity}");
        assert!(
            contaminated_purity < MIN_REFERENCE_TONAL_PURITY,
            "{contaminated_purity}"
        );
    }

    #[test]
    fn marginal_pitched_contamination_fails_reference_purity() {
        let sample_rate = 44_100;
        let fundamental = 207.65;
        let contaminated: Vec<f32> = (0..22_050)
            .map(|index| {
                let time = index as f32 / sample_rate as f32;
                (std::f32::consts::TAU * fundamental * time).sin()
                    + 0.4 * (std::f32::consts::TAU * 329.63 * time).sin()
            })
            .collect();
        let (purity, _) = source_quality(&contaminated, sample_rate, fundamental);
        assert!(
            purity >= 0.80,
            "regression must exercise the old permissive gate: {purity}"
        );
        assert!(purity < MIN_REFERENCE_TONAL_PURITY, "{purity}");
    }
}
