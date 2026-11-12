//! Deterministic acoustic-instrument family classification.
//!
//! The result is intentionally a family, not a claim that timbre alone can
//! always distinguish (for example) an oboe from a soprano saxophone. The
//! feature vector is also used to select an appropriate synthesis mechanism.

use crate::{pitch, wav::Wav};
use rustfft::{num_complex::Complex, FftPlanner};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum InstrumentFamily {
    PluckedString,
    BowedString,
    StruckString,
    WindReed,
    WindFlute,
    WindBrass,
    PercussionMembrane,
    PercussionMetal,
    PercussionWood,
    Synth,
    Unknown,
}

impl InstrumentFamily {
    pub fn label(self) -> &'static str {
        match self {
            Self::PluckedString => "plucked_string",
            Self::BowedString => "bowed_string",
            Self::StruckString => "struck_string",
            Self::WindReed => "reed",
            Self::WindFlute => "flute",
            Self::WindBrass => "brass",
            Self::PercussionMembrane => "membrane",
            Self::PercussionMetal => "metal",
            Self::PercussionWood => "wood",
            Self::Synth => "synth",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label {
            "plucked_string" => Self::PluckedString,
            "bowed_string" => Self::BowedString,
            "struck_string" => Self::StruckString,
            "reed" => Self::WindReed,
            "flute" => Self::WindFlute,
            "brass" => Self::WindBrass,
            "membrane" => Self::PercussionMembrane,
            "metal" => Self::PercussionMetal,
            "wood" => Self::PercussionWood,
            "synth" => Self::Synth,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AudioFeatures {
    pub spectral_centroid: f32,
    pub spectral_rolloff: f32,
    pub spectral_flatness: f32,
    pub zero_crossing_rate: f32,
    pub harmonic_noise_ratio: f32,
    pub pitch_clarity: f32,
    pub attack_time_ms: f32,
    pub decay_time_s: f32,
    pub sustain_ratio: f32,
    pub inharmonicity: f32,
    pub low_band_ratio: f32,
    pub high_band_ratio: f32,
    pub transient_crest: f32,
    /// Relative magnitude of the fundamental, harmonics 2–4, and harmonics
    /// 5–16. These timbral groups sum to approximately one for pitched audio.
    pub harmonic_tristimulus: [f32; 3],
    /// Fraction of measured harmonic magnitude carried by odd harmonics.
    pub odd_harmonic_ratio: f32,
    pub has_clear_pitch: bool,
}

#[derive(Clone, Debug)]
pub struct Classification {
    pub family: InstrumentFamily,
    pub confidence: f32,
    pub ranked: Vec<(InstrumentFamily, f32)>,
}

pub fn analyze_recording(wav: &Wav) -> AudioFeatures {
    let sample_rate = wav.sample_rate;
    let samples = &wav.samples;
    if samples.is_empty() || !(8_000..=384_000).contains(&sample_rate) {
        return empty_features();
    }
    let peak = samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max);
    if peak < 1e-6 {
        return empty_features();
    }
    let onset = representative_onset(samples, sample_rate);
    let analysis_end = (onset + sample_rate as usize * 3).min(samples.len());
    let analysis = &samples[onset..analysis_end];
    let spectral = spectral_features(analysis, sample_rate);
    let envelope = envelope_features(analysis, sample_rate);
    let pitch = pitch::estimate(analysis, sample_rate, 25.0, 3_500.0, 440.0);
    let pitch_clarity = pitch.map(|estimate| estimate.clarity).unwrap_or(0.0);
    let inharmonicity = pitch
        .map(|estimate| {
            spectral_inharmonicity(
                &spectral.magnitudes,
                spectral.fft_size,
                sample_rate,
                estimate.hz,
            )
        })
        .unwrap_or_else(|| peak_irregularity(&spectral.magnitudes));
    let (harmonic_tristimulus, odd_harmonic_ratio) = pitch
        .map(|estimate| {
            harmonic_colour(
                &spectral.magnitudes,
                spectral.fft_size,
                sample_rate,
                estimate.hz,
            )
        })
        .unwrap_or(([0.0; 3], 0.0));
    let zero_crossing_rate = analysis
        .windows(2)
        .filter(|pair| (pair[0] >= 0.0) != (pair[1] >= 0.0))
        .count() as f32
        / analysis.len().max(1) as f32;

    AudioFeatures {
        spectral_centroid: spectral.centroid,
        spectral_rolloff: spectral.rolloff,
        spectral_flatness: spectral.flatness,
        zero_crossing_rate,
        harmonic_noise_ratio: pitch_clarity,
        pitch_clarity,
        attack_time_ms: envelope.attack_ms,
        decay_time_s: envelope.decay_s,
        sustain_ratio: envelope.sustain,
        inharmonicity,
        low_band_ratio: spectral.low_ratio,
        high_band_ratio: spectral.high_ratio,
        transient_crest: envelope.crest,
        harmonic_tristimulus,
        odd_harmonic_ratio,
        has_clear_pitch: pitch_clarity >= 0.58,
    }
}

fn representative_onset(samples: &[f32], sample_rate: u32) -> usize {
    let frame_size = (sample_rate as usize / 200).max(16);
    let envelope: Vec<f32> = samples
        .chunks(frame_size)
        .map(|frame| {
            (frame.iter().map(|sample| sample * sample).sum::<f32>() / frame.len() as f32).sqrt()
        })
        .collect();
    let peak = envelope.iter().copied().fold(0.0_f32, f32::max).max(1e-9);
    let best = envelope
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[1] >= peak * 0.03)
        .max_by(|(_, left), (_, right)| {
            let left_score = (left[1] - left[0]).max(0.0) * (left[1] / peak).sqrt();
            let right_score = (right[1] - right[0]).max(0.0) * (right[1] / peak).sqrt();
            left_score
                .partial_cmp(&right_score)
                .unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| (index + 1) * frame_size)
        .unwrap_or(0);
    best.saturating_sub(frame_size)
}

pub fn classify(features: &AudioFeatures) -> InstrumentFamily {
    classify_ranked(features).family
}

pub fn classify_ranked(features: &AudioFeatures) -> Classification {
    use InstrumentFamily::*;
    let fast_attack = inverse_ramp(features.attack_time_ms, 8.0, 80.0);
    let slow_attack = ramp(features.attack_time_ms, 25.0, 180.0);
    let short_decay = inverse_ramp(features.decay_time_s, 0.12, 1.2);
    let long_decay = ramp(features.decay_time_s, 0.35, 2.5);
    let bright = ramp(features.spectral_centroid, 1_200.0, 5_000.0);
    let dark = 1.0 - bright;
    let tonal = features.pitch_clarity.clamp(0.0, 1.0);
    let noisy = features.spectral_flatness.clamp(0.0, 1.0);
    let sustained = features.sustain_ratio.clamp(0.0, 1.0);
    let inharmonic = ramp(features.inharmonicity, 0.008, 0.12);
    let transient = ramp(features.transient_crest, 1.5, 8.0);
    let fundamental_colour = features.harmonic_tristimulus[0];
    let middle_colour = features.harmonic_tristimulus[1];
    let upper_colour = features.harmonic_tristimulus[2];

    let mut scores = vec![
        (
            PluckedString,
            2.0 * tonal
                + 1.8 * fast_attack
                + 1.4 * short_decay
                + transient
                + 0.4 * bright
                + 0.3 * upper_colour,
        ),
        (
            StruckString,
            1.8 * tonal + 1.8 * fast_attack + 1.2 * inharmonic + 0.8 * long_decay + transient,
        ),
        (
            BowedString,
            1.7 * tonal
                + 1.6 * slow_attack
                + 1.4 * sustained
                + 0.8 * bright
                + 0.4 * noisy
                + 0.2 * middle_colour,
        ),
        (
            WindFlute,
            1.5 * tonal
                + 1.3 * slow_attack
                + 1.2 * sustained
                + dark
                + 0.7 * noisy
                + 0.4 * fundamental_colour,
        ),
        (
            WindReed,
            1.7 * tonal
                + slow_attack
                + 1.2 * sustained
                + 0.8 * noisy
                + 0.6 * bright
                + 0.3 * middle_colour,
        ),
        (
            WindBrass,
            1.8 * tonal + slow_attack + 1.2 * sustained + 1.2 * bright - 0.3 * noisy
                + 0.35 * upper_colour,
        ),
        (
            PercussionMembrane,
            1.8 * (1.0 - tonal)
                + 1.5 * fast_attack
                + 1.2 * short_decay
                + 1.5 * features.low_band_ratio
                + transient,
        ),
        (
            PercussionMetal,
            1.2 * (1.0 - tonal)
                + 1.5 * fast_attack
                + 1.5 * inharmonic
                + 1.4 * features.high_band_ratio
                + 0.8 * long_decay
                + noisy,
        ),
        (
            PercussionWood,
            1.5 * (1.0 - tonal)
                + 1.8 * fast_attack
                + 1.5 * short_decay
                + 0.6 * (1.0 - features.high_band_ratio)
                + transient,
        ),
        (
            Synth,
            0.8 * tonal + 0.8 * sustained + 0.5 * (1.0 - transient) + 0.4 * (1.0 - inharmonic),
        ),
    ];
    for (_, score) in &mut scores {
        *score = score.max(0.0);
    }
    scores.sort_by(|left, right| right.1.partial_cmp(&left.1).unwrap_or(Ordering::Equal));
    let best = scores.first().copied().unwrap_or((Unknown, 0.0));
    let second = scores.get(1).map(|entry| entry.1).unwrap_or(0.0);
    let evidence = best.1 / 6.5;
    let separation = (best.1 - second) / best.1.max(1e-6);
    let confidence = (evidence * 0.7 + separation * 0.3).clamp(0.0, 1.0);
    let family = if best.1 < 1.0 { Unknown } else { best.0 };
    Classification {
        family,
        confidence,
        ranked: scores,
    }
}

fn ramp(value: f32, low: f32, high: f32) -> f32 {
    ((value - low) / (high - low).max(1e-9)).clamp(0.0, 1.0)
}

fn inverse_ramp(value: f32, low: f32, high: f32) -> f32 {
    1.0 - ramp(value, low, high)
}

struct SpectralFeatures {
    magnitudes: Vec<f32>,
    fft_size: usize,
    centroid: f32,
    rolloff: f32,
    flatness: f32,
    low_ratio: f32,
    high_ratio: f32,
}

fn spectral_features(samples: &[f32], sample_rate: u32) -> SpectralFeatures {
    let fft_size = samples.len().clamp(256, 16_384).next_power_of_two();
    let mut buffer = vec![Complex::new(0.0_f32, 0.0); fft_size];
    let start = samples.len().saturating_sub(fft_size) / 2;
    for index in 0..fft_size {
        let sample = samples.get(start + index).copied().unwrap_or(0.0);
        let window =
            0.5 - 0.5 * (std::f32::consts::TAU * index as f32 / (fft_size - 1).max(1) as f32).cos();
        buffer[index].re = sample * window;
    }
    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(fft_size).process(&mut buffer);
    let magnitudes: Vec<f32> = buffer[..fft_size / 2]
        .iter()
        .map(|value| value.norm())
        .collect();
    let total = magnitudes.iter().sum::<f32>().max(1e-12);
    let centroid = magnitudes
        .iter()
        .enumerate()
        .map(|(bin, magnitude)| *magnitude * bin as f32 * sample_rate as f32 / fft_size as f32)
        .sum::<f32>()
        / total;
    let mut cumulative = 0.0_f32;
    let mut rolloff = 0.0_f32;
    for (bin, magnitude) in magnitudes.iter().enumerate() {
        cumulative += *magnitude;
        if cumulative >= total * 0.85 {
            rolloff = bin as f32 * sample_rate as f32 / fft_size as f32;
            break;
        }
    }
    let arithmetic = total / magnitudes.len().max(1) as f32;
    let geometric = (magnitudes
        .iter()
        .map(|value| value.max(1e-12).ln())
        .sum::<f32>()
        / magnitudes.len().max(1) as f32)
        .exp();
    let flatness = (geometric / arithmetic.max(1e-12)).clamp(0.0, 1.0);
    let low = band_energy(&magnitudes, sample_rate, fft_size, 20.0, 400.0);
    let high = band_energy(
        &magnitudes,
        sample_rate,
        fft_size,
        4_000.0,
        sample_rate as f32 * 0.49,
    );
    SpectralFeatures {
        magnitudes,
        fft_size,
        centroid,
        rolloff,
        flatness,
        low_ratio: low / total,
        high_ratio: high / total,
    }
}

fn band_energy(magnitudes: &[f32], sample_rate: u32, fft_size: usize, low: f32, high: f32) -> f32 {
    magnitudes
        .iter()
        .enumerate()
        .filter(|(bin, _)| {
            let frequency = *bin as f32 * sample_rate as f32 / fft_size as f32;
            frequency >= low && frequency < high
        })
        .map(|(_, magnitude)| *magnitude)
        .sum()
}

struct EnvelopeFeatures {
    attack_ms: f32,
    decay_s: f32,
    sustain: f32,
    crest: f32,
}

fn envelope_features(samples: &[f32], sample_rate: u32) -> EnvelopeFeatures {
    let frame_size = (sample_rate as usize / 200).max(16);
    let envelope: Vec<f32> = samples
        .chunks(frame_size)
        .map(|frame| {
            (frame.iter().map(|sample| sample * sample).sum::<f32>() / frame.len() as f32).sqrt()
        })
        .collect();
    let peak = envelope.iter().copied().fold(0.0_f32, f32::max).max(1e-9);
    let mean = envelope.iter().sum::<f32>() / envelope.len().max(1) as f32;
    let onset = envelope
        .iter()
        .position(|value| *value >= peak * 0.05)
        .unwrap_or(0);
    let attack_end = envelope
        .iter()
        .skip(onset)
        .position(|value| *value >= peak * 0.9)
        .map(|index| index + onset)
        .unwrap_or(onset);
    let decay_end = envelope
        .iter()
        .skip(attack_end)
        .position(|value| *value <= peak * 0.37)
        .map(|index| index + attack_end)
        .unwrap_or(envelope.len());
    let sustain_start = envelope.len() * 2 / 3;
    let sustain = if sustain_start < envelope.len() {
        envelope[sustain_start..].iter().sum::<f32>()
            / (envelope.len() - sustain_start) as f32
            / peak
    } else {
        0.0
    };
    EnvelopeFeatures {
        attack_ms: attack_end.saturating_sub(onset) as f32 * frame_size as f32 * 1_000.0
            / sample_rate as f32,
        decay_s: decay_end.saturating_sub(attack_end) as f32 * frame_size as f32
            / sample_rate as f32,
        sustain: sustain.clamp(0.0, 1.0),
        crest: (peak / mean.max(1e-9)).clamp(0.0, 100.0),
    }
}

fn spectral_inharmonicity(
    magnitudes: &[f32],
    fft_size: usize,
    sample_rate: u32,
    fundamental: f32,
) -> f32 {
    let mut weighted_deviation = 0.0_f32;
    let mut weight_sum = 0.0_f32;
    for harmonic in 1..=12 {
        let expected = fundamental * harmonic as f32;
        if expected >= sample_rate as f32 * 0.48 {
            break;
        }
        let bin = (expected * fft_size as f32 / sample_rate as f32).round() as usize;
        if bin < 2 || bin + 2 >= magnitudes.len() {
            continue;
        }
        let best = (bin - 2..=bin + 2)
            .max_by(|left, right| {
                magnitudes[*left]
                    .partial_cmp(&magnitudes[*right])
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap_or(bin);
        let actual = best as f32 * sample_rate as f32 / fft_size as f32;
        let weight = magnitudes[best];
        weighted_deviation += ((actual - expected).abs() / expected.max(1e-9)) * weight;
        weight_sum += weight;
    }
    weighted_deviation / weight_sum.max(1e-9)
}

fn harmonic_colour(
    magnitudes: &[f32],
    fft_size: usize,
    sample_rate: u32,
    fundamental: f32,
) -> ([f32; 3], f32) {
    if !fundamental.is_finite() || fundamental <= 0.0 || magnitudes.len() < 3 {
        return ([0.0; 3], 0.0);
    }
    let mut harmonics = [0.0_f32; 16];
    for (index, magnitude) in harmonics.iter_mut().enumerate() {
        let frequency = fundamental * (index + 1) as f32;
        if frequency >= sample_rate as f32 * 0.48 {
            break;
        }
        let center = (frequency * fft_size as f32 / sample_rate as f32).round() as usize;
        if center >= magnitudes.len() {
            break;
        }
        let start = center.saturating_sub(2);
        let end = (center + 2).min(magnitudes.len() - 1);
        *magnitude = magnitudes[start..=end]
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .fold(0.0_f32, f32::max);
    }
    let total = harmonics.iter().sum::<f32>();
    if total <= 1e-12 {
        return ([0.0; 3], 0.0);
    }
    let first = harmonics[0] / total;
    let middle = harmonics[1..4].iter().sum::<f32>() / total;
    let upper = harmonics[4..].iter().sum::<f32>() / total;
    let odd = harmonics.iter().step_by(2).copied().sum::<f32>() / total;
    ([first, middle, upper], odd.clamp(0.0, 1.0))
}

fn peak_irregularity(magnitudes: &[f32]) -> f32 {
    let mut peaks = Vec::new();
    for index in 2..magnitudes.len().saturating_sub(2) {
        if magnitudes[index] > magnitudes[index - 1] && magnitudes[index] >= magnitudes[index + 1] {
            peaks.push(magnitudes[index]);
        }
    }
    if peaks.len() < 3 {
        return 0.0;
    }
    peaks.sort_by(|left, right| right.partial_cmp(left).unwrap_or(Ordering::Equal));
    let strongest = peaks[0].max(1e-9);
    (peaks
        .iter()
        .take(12)
        .filter(|peak| **peak > strongest * 0.12)
        .count() as f32
        / 12.0)
        .clamp(0.0, 1.0)
}

fn empty_features() -> AudioFeatures {
    AudioFeatures {
        spectral_centroid: 0.0,
        spectral_rolloff: 0.0,
        spectral_flatness: 0.0,
        zero_crossing_rate: 0.0,
        harmonic_noise_ratio: 0.0,
        pitch_clarity: 0.0,
        attack_time_ms: 0.0,
        decay_time_s: 0.0,
        sustain_ratio: 0.0,
        inharmonicity: 0.0,
        low_band_ratio: 0.0,
        high_band_ratio: 0.0,
        transient_crest: 0.0,
        harmonic_tristimulus: [0.0; 3],
        odd_harmonic_ratio: 0.0,
        has_clear_pitch: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_a_decaying_plucked_tone() {
        let sample_rate = 44_100;
        let samples: Vec<f32> = (0..sample_rate as usize)
            .map(|index| {
                let time = index as f32 / sample_rate as f32;
                (std::f32::consts::TAU * 220.0 * time).sin() * (-5.0 * time).exp()
            })
            .collect();
        let features = analyze_recording(&Wav {
            sample_rate,
            samples,
        });
        assert!(features.has_clear_pitch);
        assert_eq!(classify(&features), InstrumentFamily::PluckedString);
    }

    #[test]
    fn labels_round_trip() {
        for family in [
            InstrumentFamily::PluckedString,
            InstrumentFamily::BowedString,
            InstrumentFamily::StruckString,
            InstrumentFamily::WindReed,
            InstrumentFamily::WindFlute,
            InstrumentFamily::WindBrass,
            InstrumentFamily::PercussionMembrane,
            InstrumentFamily::PercussionMetal,
            InstrumentFamily::PercussionWood,
            InstrumentFamily::Synth,
        ] {
            assert_eq!(InstrumentFamily::from_label(family.label()), family);
        }
    }

    #[test]
    fn harmonic_colour_groups_the_spectrum() {
        let sample_rate = 44_100;
        let fft_size = 16_384;
        let mut magnitudes = vec![0.0; fft_size / 2];
        for (harmonic, amplitude) in [1.0_f32, 0.5, 0.3, 0.2, 0.1].iter().enumerate() {
            let frequency = 220.0 * (harmonic + 1) as f32;
            let bin = (frequency * fft_size as f32 / sample_rate as f32).round() as usize;
            magnitudes[bin] = *amplitude;
        }
        let (groups, odd) = harmonic_colour(&magnitudes, fft_size, sample_rate, 220.0);
        assert!((groups.iter().sum::<f32>() - 1.0).abs() < 1e-6);
        assert!((groups[0] - groups[1]).abs() < 0.01);
        assert!(groups[1] > groups[2]);
        assert!((0.0..=1.0).contains(&odd));
    }
}
