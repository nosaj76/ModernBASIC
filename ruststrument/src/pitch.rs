//! Fundamental-frequency estimation for musical audio.
//!
//! The estimator follows the cumulative-mean normalized difference stage of
//! YIN.  Unlike a largest-FFT-bin detector, it remains useful when the
//! fundamental is weaker than its overtones and returns an explicit clarity
//! value so callers can reject noise and inharmonic percussion.

use rustfft::{num_complex::Complex, FftPlanner};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PitchEstimate {
    pub hz: f32,
    pub midi: u8,
    pub cents: f32,
    pub clarity: f32,
}

pub fn midi_frequency(midi: f32, tuning_hz: f32) -> f32 {
    tuning_hz * 2.0_f32.powf((midi - 69.0) / 12.0)
}

pub fn frequency_to_midi(hz: f32, tuning_hz: f32) -> Option<(u8, f32)> {
    if !hz.is_finite() || hz <= 0.0 || !tuning_hz.is_finite() || tuning_hz <= 0.0 {
        return None;
    }
    let exact = 69.0 + 12.0 * (hz / tuning_hz).log2();
    if !(-0.5..=127.5).contains(&exact) {
        return None;
    }
    let rounded = exact.round().clamp(0.0, 127.0);
    Some((rounded as u8, (exact - rounded) * 100.0))
}

/// Estimate the strongest periodic fundamental in a frame.
///
/// `min_hz` and `max_hz` constrain the musical range.  The function rejects
/// silence, invalid ranges, weak periodicity, and estimates outside MIDI
/// 0..127.  The input may be longer than needed; the highest-energy analysis
/// window is selected automatically.
pub fn estimate(
    samples: &[f32],
    sample_rate: u32,
    min_hz: f32,
    max_hz: f32,
    tuning_hz: f32,
) -> Option<PitchEstimate> {
    if samples.len() < 64
        || !(8_000..=384_000).contains(&sample_rate)
        || !min_hz.is_finite()
        || !max_hz.is_finite()
        || min_hz < 8.0
        || max_hz <= min_hz
        || max_hz >= sample_rate as f32 * 0.49
    {
        return None;
    }

    let minimum_lag = (sample_rate as f32 / max_hz).floor().max(2.0) as usize;
    let maximum_lag = (sample_rate as f32 / min_hz).ceil() as usize;
    let wanted = (maximum_lag * 3).clamp(1_024, 16_384);
    let frame_length = wanted.min(samples.len());
    if frame_length <= maximum_lag + 8 {
        return None;
    }
    let frame = highest_energy_frame(samples, frame_length);
    let mean = frame.iter().sum::<f32>() / frame.len() as f32;
    let mut centered = Vec::with_capacity(frame.len());
    let mut energy = 0.0_f32;
    for (index, sample) in frame.iter().enumerate() {
        let window = 0.5
            - 0.5 * (std::f32::consts::TAU * index as f32 / (frame.len() - 1).max(1) as f32).cos();
        let value = (*sample - mean) * window;
        centered.push(value);
        energy += value * value;
    }
    let rms = (energy / centered.len() as f32).sqrt();
    if !rms.is_finite() || rms < 1e-5 {
        return None;
    }

    let maximum_lag = maximum_lag.min(centered.len() / 2);
    if maximum_lag <= minimum_lag + 2 {
        return None;
    }
    // Wiener-Khinchin reduces YIN's difference stage from O(frame * lag)
    // to O(frame log frame). Prefix energies retain the exact finite-window
    // difference rather than assuming circular correlation.
    let fft_size = (centered.len() * 2).next_power_of_two();
    let mut spectrum = vec![Complex::new(0.0_f32, 0.0); fft_size];
    for (target, sample) in spectrum.iter_mut().zip(centered.iter()) {
        target.re = *sample;
    }
    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(fft_size).process(&mut spectrum);
    for value in &mut spectrum {
        *value = Complex::new(value.norm_sqr(), 0.0);
    }
    planner.plan_fft_inverse(fft_size).process(&mut spectrum);
    let inverse_scale = 1.0 / fft_size as f32;
    let mut prefix_energy = Vec::with_capacity(centered.len() + 1);
    prefix_energy.push(0.0_f32);
    for sample in &centered {
        prefix_energy.push(prefix_energy.last().copied().unwrap_or(0.0) + sample * sample);
    }
    let total_energy = *prefix_energy.last().unwrap_or(&0.0);
    let mut difference = vec![0.0_f32; maximum_lag + 1];
    for lag in 1..=maximum_lag {
        let left_energy = prefix_energy[centered.len() - lag];
        let right_energy = total_energy - prefix_energy[lag];
        let correlation = spectrum[lag].re * inverse_scale;
        difference[lag] = (left_energy + right_energy - 2.0 * correlation).max(0.0);
    }

    let mut normalized = vec![1.0_f32; maximum_lag + 1];
    let mut running = 0.0_f32;
    for lag in 1..=maximum_lag {
        running += difference[lag];
        normalized[lag] = if running > 1e-20 {
            difference[lag] * lag as f32 / running
        } else {
            1.0
        };
    }

    const ABSOLUTE_THRESHOLD: f32 = 0.18;
    let mut selected = None;
    let mut lag = minimum_lag;
    while lag < maximum_lag {
        if normalized[lag] < ABSOLUTE_THRESHOLD {
            while lag < maximum_lag && normalized[lag + 1] < normalized[lag] {
                lag += 1;
            }
            selected = Some(lag);
            break;
        }
        lag += 1;
    }
    let selected = selected.unwrap_or_else(|| {
        (minimum_lag..=maximum_lag)
            .min_by(|left, right| {
                normalized[*left]
                    .partial_cmp(&normalized[*right])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(minimum_lag)
    });
    let clarity = (1.0 - normalized[selected]).clamp(0.0, 1.0);
    if clarity < 0.52 {
        return None;
    }

    let left = normalized[selected.saturating_sub(1)];
    let center = normalized[selected];
    let right = normalized[(selected + 1).min(maximum_lag)];
    let denominator = left - 2.0 * center + right;
    let offset = if denominator.abs() > 1e-12 {
        (0.5 * (left - right) / denominator).clamp(-0.5, 0.5)
    } else {
        0.0
    };
    let mut hz = sample_rate as f32 / (selected as f32 + offset);
    // A short, bright pluck can make YIN choose a subharmonic when all of the
    // observed energy belongs to even multiples of that candidate. Preserve a
    // genuine missing fundamental when its third harmonic is present, but
    // promote the octave when the candidate itself and its odd harmonic are
    // both effectively absent.
    if hz * 2.0 <= max_hz * 1.02 {
        let strength = |frequency: f32| {
            let mut real = 0.0_f32;
            let mut imaginary = 0.0_f32;
            for (index, sample) in centered.iter().enumerate() {
                let angle = std::f32::consts::TAU * frequency * index as f32 / sample_rate as f32;
                real += sample * angle.cos();
                imaginary -= sample * angle.sin();
            }
            (real * real + imaginary * imaginary).sqrt()
        };
        let fundamental = strength(hz);
        let second = strength(hz * 2.0);
        let third = strength(hz * 3.0);
        if second > fundamental * 4.0 && third < second * 0.22 {
            hz *= 2.0;
        }
    }
    if hz < min_hz * 0.98 || hz > max_hz * 1.02 {
        return None;
    }
    let (midi, cents) = frequency_to_midi(hz, tuning_hz)?;
    Some(PitchEstimate {
        hz,
        midi,
        cents,
        clarity,
    })
}

fn highest_energy_frame(samples: &[f32], length: usize) -> &[f32] {
    if samples.len() <= length {
        return samples;
    }
    let hop = (length / 4).max(1);
    let mut best_start = 0;
    let mut best_energy = -1.0_f64;
    for start in (0..=samples.len() - length).step_by(hop) {
        let energy = samples[start..start + length]
            .iter()
            .map(|sample| f64::from(*sample) * f64::from(*sample))
            .sum::<f64>();
        if energy > best_energy {
            best_energy = energy;
            best_start = start;
        }
    }
    &samples[best_start..best_start + length]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(frequencies: &[(f32, f32)], seconds: f32) -> Vec<f32> {
        let sample_rate = 44_100.0;
        (0..(sample_rate * seconds) as usize)
            .map(|index| {
                frequencies
                    .iter()
                    .map(|(frequency, amplitude)| {
                        amplitude
                            * (std::f32::consts::TAU * frequency * index as f32 / sample_rate).sin()
                    })
                    .sum()
            })
            .collect()
    }

    #[test]
    fn finds_concert_a_with_cent_accuracy() {
        let estimate =
            estimate(&tone(&[(440.0, 0.8)], 0.4), 44_100, 30.0, 2_000.0, 440.0).expect("pitch");
        assert_eq!(estimate.midi, 69);
        assert!(estimate.cents.abs() < 1.0, "{}", estimate.cents);
        assert!(estimate.clarity > 0.9, "{}", estimate.clarity);
    }

    #[test]
    fn handles_a_missing_fundamental() {
        let estimate = estimate(
            &tone(&[(440.0, 0.0), (880.0, 0.7), (1_320.0, 0.45)], 0.5),
            44_100,
            100.0,
            2_000.0,
            440.0,
        )
        .expect("pitch");
        assert_eq!(estimate.midi, 69);
    }

    #[test]
    fn rejects_silence_and_invalid_ranges() {
        assert!(estimate(&[0.0; 8_192], 44_100, 30.0, 2_000.0, 440.0).is_none());
        assert!(estimate(&[1.0; 8_192], 44_100, 2_000.0, 30.0, 440.0).is_none());
    }
}
