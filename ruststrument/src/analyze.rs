// analyze.rs — Full audio spectrum analysis
//
// Does everything the Python scripts did:
// - Sliding window FFT scan with pitch detection
// - Per-window harmonic extraction (H1-H8)
// - Envelope tracking (RMS)
// - Spectral centroid, rolloff, ZCR, HNR, inharmonicity
// - Onset detection and per-note analysis
// - Note change detection
// - Summary statistics across the whole recording
// - Batch mode for analyzing multiple files
//
// No Python dependency. Pure Rust.

use crate::{pitch, wav};
use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::TAU;
use std::path::Path;

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub fn midi_note_name(midi: u8) -> String {
    format!("{}{}", NOTE_NAMES[midi as usize % 12], midi as i16 / 12 - 1)
}

fn note_name(hz: f32) -> String {
    if hz <= 0.0 {
        return "?".into();
    }
    midi_note_name(hz_to_midi(hz))
}

fn midi_to_hz(midi: u8) -> f32 {
    440.0 * 2.0f32.powf((midi as f32 - 69.0) / 12.0)
}

pub fn hz_to_midi(hz: f32) -> u8 {
    ((69.0 + 12.0 * (hz / 440.0).log2())
        .round()
        .clamp(0.0, 127.0)) as u8
}

#[derive(Clone, Copy, Debug)]
pub struct DetectedPitch {
    pub hz: f32,
    pub midi: u8,
    pub cents: f32,
}

pub fn identify_pitch(samples: &[f32], sample_rate: u32) -> Option<DetectedPitch> {
    let peak = samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max);
    if peak < 1e-5 {
        return None;
    }
    let onset = samples
        .iter()
        .position(|sample| sample.abs() >= peak * 0.025)?;
    let pitch_start = onset
        .saturating_add(sample_rate as usize / 100)
        .min(samples.len());
    let estimate = pitch::estimate(
        &samples[pitch_start..],
        sample_rate,
        20.0,
        (sample_rate as f32 * 0.45).min(5_000.0),
        440.0,
    )?;
    Some(DetectedPitch {
        hz: estimate.hz,
        midi: estimate.midi,
        cents: estimate.cents,
    })
}

// ─── Autocorrelation pitch detection with parabolic interpolation ────────
fn detect_pitch(samples: &[f32], sr: u32) -> f32 {
    pitch::estimate(samples, sr, 35.0, 2_200.0, 440.0)
        .map(|estimate| estimate.hz)
        .unwrap_or(0.0)
}

// ─── FFT magnitude spectrum ─────────────────────────────────────────────
fn fft_magnitudes(samples: &[f32], n: usize) -> Vec<f32> {
    let fft_size = n;
    let mut buffer = vec![Complex::new(0.0_f32, 0.0); fft_size];
    for (index, sample) in samples.iter().take(n).enumerate() {
        let window = 0.5 - 0.5 * (TAU * index as f32 / n.max(1) as f32).cos();
        buffer[index].re = *sample * window;
    }
    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(fft_size).process(&mut buffer);
    buffer[..fft_size / 2]
        .iter()
        .map(|value| value.norm() / n.max(1) as f32)
        .collect()
}

// ─── Extract harmonic amplitudes at specific harmonic frequencies ──────
fn extract_harmonics(samples: &[f32], sr: u32, fundamental: f32) -> [f32; 8] {
    let n = samples.len().min(8192);
    if n < 256 || fundamental < 30.0 {
        return [0.0; 8];
    }
    let window: Vec<f32> = samples[..n]
        .iter()
        .enumerate()
        .map(|(i, &s)| s * (0.5 - 0.5 * (TAU * i as f32 / n as f32).cos()))
        .collect();
    let mut hw = [0.0f32; 8];
    let mut max_h = 0.0f32;
    for h in 0..8 {
        let freq = fundamental * (h + 1) as f32;
        if freq > sr as f32 * 0.45 {
            break;
        }
        let mut re = 0.0f32;
        let mut im = 0.0f32;
        for i in 0..n {
            let angle = TAU * freq * i as f32 / sr as f32;
            re += window[i] * angle.cos();
            im -= window[i] * angle.sin();
        }
        hw[h] = (re * re + im * im).sqrt() / n as f32;
        if hw[h] > max_h {
            max_h = hw[h];
        }
    }
    if max_h > 0.0 {
        for h in &mut hw {
            *h /= max_h;
        }
    }
    hw
}

// ─── Spectral centroid ──────────────────────────────────────────────────
fn spectral_centroid(samples: &[f32], sr: u32) -> f32 {
    let n = samples.len().min(4096);
    if n < 256 {
        return 0.0;
    }
    let mags = fft_magnitudes(samples, n);
    let mut weighted = 0.0f32;
    let mut total = 0.0f32;
    for k in 1..n / 2 {
        let freq = k as f32 * sr as f32 / n as f32;
        weighted += freq * mags[k];
        total += mags[k];
    }
    if total > 0.0 {
        weighted / total
    } else {
        0.0
    }
}

// ─── Spectral rolloff (85% energy) ──────────────────────────────────────
fn spectral_rolloff(samples: &[f32], sr: u32, percentile: f32) -> f32 {
    let n = samples.len().min(4096);
    if n < 256 {
        return 0.0;
    }
    let mags = fft_magnitudes(samples, n);
    let total: f32 = mags.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let threshold = total * percentile;
    let mut running = 0.0f32;
    for k in 0..n / 2 {
        running += mags[k];
        if running >= threshold {
            return k as f32 * sr as f32 / n as f32;
        }
    }
    0.0
}

// ─── Zero crossing rate ─────────────────────────────────────────────────
fn zero_crossing_rate(samples: &[f32]) -> f32 {
    if samples.len() < 2 {
        return 0.0;
    }
    let mut crossings = 0usize;
    for i in 1..samples.len() {
        if (samples[i] >= 0.0) != (samples[i - 1] >= 0.0) {
            crossings += 1;
        }
    }
    crossings as f32 / samples.len() as f32
}

// ─── Harmonic-to-noise ratio ─────────────────────────────────────────────
fn harmonic_noise_ratio(samples: &[f32], sr: u32) -> f32 {
    let n = samples.len().min(8192);
    if n < 256 {
        return 0.0;
    }
    let total_energy: f32 = samples[..n].iter().map(|v| v * v).sum();
    if total_energy < 1e-9 {
        return 0.0;
    }
    let lo = (sr as f32 / 2000.0) as usize;
    let hi = (sr as f32 / 45.0) as usize;
    let mut best_corr = 0.0f32;
    for lag in lo.max(2)..hi.min(n / 2) {
        let mut corr = 0.0f32;
        let limit = (n - lag).min(4096);
        for i in 0..limit {
            corr += samples[i] * samples[i + lag];
        }
        if corr > best_corr {
            best_corr = corr;
        }
    }
    best_corr / total_energy
}

// ─── Inharmonicity ──────────────────────────────────────────────────────
fn inharmonicity(samples: &[f32], sr: u32, fundamental: f32) -> f32 {
    if fundamental < 30.0 {
        return 0.0;
    }
    let n = samples.len().min(8192);
    if n < 256 {
        return 0.0;
    }
    let mags = fft_magnitudes(samples, n);
    let mut deviations = Vec::new();
    for h in 1..=8 {
        let expected_freq = fundamental * h as f32;
        if expected_freq > sr as f32 * 0.45 {
            break;
        }
        let expected_bin = (expected_freq * n as f32 / sr as f32) as usize;
        if expected_bin == 0 || expected_bin >= n / 2 {
            continue;
        }
        let search = 3;
        let mut best_mag = 0.0f32;
        let mut best_bin = expected_bin;
        for k in expected_bin.saturating_sub(search)..=(expected_bin + search).min(n / 2 - 1) {
            if mags[k] > best_mag {
                best_mag = mags[k];
                best_bin = k;
            }
        }
        let actual_freq = best_bin as f32 * sr as f32 / n as f32;
        if expected_freq > 0.0 {
            deviations.push((actual_freq - expected_freq).abs() / expected_freq);
        }
    }
    if deviations.is_empty() {
        return 0.0;
    }
    deviations.iter().sum::<f32>() / deviations.len() as f32
}

// ─── Attack time in ms ─────────────────────────────────────────────────
fn attack_time_ms(samples: &[f32], sr: u32) -> f32 {
    let n = samples.len().min(sr as usize / 2);
    if n < 64 {
        return 0.0;
    }
    let peak = samples[..n].iter().fold(0.0f32, |a, &v| a.max(v.abs()));
    if peak < 0.001 {
        return 0.0;
    }
    let threshold = peak * 0.9;
    for i in 0..n {
        if samples[i].abs() >= threshold {
            return i as f32 / sr as f32 * 1000.0;
        }
    }
    500.0
}

// ─── Decay time in seconds ─────────────────────────────────────────────
fn decay_time_s(samples: &[f32], sr: u32) -> f32 {
    let n = samples.len().min(sr as usize * 2);
    if n < 256 {
        return 0.0;
    }
    let peak_pos = samples[..n]
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs()).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);
    if peak_pos >= n - 1 {
        return 0.0;
    }
    let peak_val = samples[peak_pos].abs();
    if peak_val < 0.001 {
        return 0.0;
    }
    let threshold = peak_val * 0.37;
    for i in peak_pos..n {
        if samples[i].abs() < threshold {
            return (i - peak_pos) as f32 / sr as f32;
        }
    }
    2.0
}

// ─── Main analysis: scan a recording and report everything ─────────────
pub struct AnalysisReport {
    pub duration_s: f32,
    pub sample_rate: u32,
    pub total_samples: usize,
    pub overall_centroid: f32,
    pub overall_rolloff: f32,
    pub overall_zcr: f32,
    pub overall_hnr: f32,
    pub overall_inharmonicity: f32,
    pub overall_attack_ms: f32,
    pub overall_decay_s: f32,
    pub notes: Vec<NoteAnalysis>,
}

#[derive(Clone, Debug)]
pub struct NoteAnalysis {
    pub time_s: f32,
    pub midi: u8,
    pub hz: f32,
    pub note_name: String,
    pub harmonics: [f32; 8],
    pub centroid: f32,
    pub rolloff: f32,
    pub zcr: f32,
    pub hnr: f32,
    pub inharmonicity: f32,
    pub peak: f32,
}

pub fn analyze_wav(path: impl AsRef<Path>) -> std::io::Result<AnalysisReport> {
    let path = path.as_ref();
    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"))
    {
        let w = wav::read(path)?;
        analyze_samples(&w.samples, w.sample_rate)
    } else {
        let audio = crate::load_audio(&path.to_string_lossy())
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        analyze_samples(&audio.samples, audio.sample_rate)
    }
}

pub fn analyze_samples(samples: &[f32], sr: u32) -> std::io::Result<AnalysisReport> {
    let total = samples.len();
    let duration = total as f32 / sr as f32;

    // Overall stats from first 2 seconds of audio with signal
    let onset = samples.iter().position(|v| v.abs() > 0.05).unwrap_or(0);
    let chunk_end = (onset + sr as usize * 2).min(total);
    let chunk = &samples[onset..chunk_end];

    let overall_centroid = spectral_centroid(chunk, sr);
    let overall_rolloff = spectral_rolloff(chunk, sr, 0.85);
    let overall_zcr = zero_crossing_rate(chunk);
    let overall_hnr = harmonic_noise_ratio(chunk, sr);

    // Fundamental for inharmonicity
    let fund = detect_pitch(chunk, sr);
    let overall_inharmonicity = inharmonicity(chunk, sr, fund);
    let overall_attack = attack_time_ms(chunk, sr);
    let overall_decay = decay_time_s(chunk, sr);

    // Sliding window scan — detect note changes and report each
    let win = (sr as f32 * 0.15) as usize; // 150ms windows
    let hop = (sr as f32 * 0.1) as usize; // 100ms hop
    let mut notes = Vec::new();
    let mut prev_note = String::new();

    for i in (0..total.saturating_sub(win)).step_by(hop) {
        let window_samples = &samples[i..i + win];
        let peak = window_samples.iter().fold(0.0f32, |a, &v| a.max(v.abs()));
        if peak < 0.02 {
            continue;
        }

        let hz = detect_pitch(window_samples, sr);
        if hz < 30.0 {
            continue;
        }
        let name = note_name(hz);
        let t = i as f32 / sr as f32;

        if name != prev_note {
            let midi = hz_to_midi(hz);
            let hw = extract_harmonics(window_samples, sr, hz);
            let centroid = spectral_centroid(window_samples, sr);
            let rolloff = spectral_rolloff(window_samples, sr, 0.85);
            let zcr = zero_crossing_rate(window_samples);
            let hnr = harmonic_noise_ratio(window_samples, sr);
            let inharm = inharmonicity(window_samples, sr, hz);

            notes.push(NoteAnalysis {
                time_s: t,
                midi,
                hz,
                note_name: name.clone(),
                harmonics: hw,
                centroid,
                rolloff,
                zcr,
                hnr,
                inharmonicity: inharm,
                peak,
            });
            prev_note = name;
        }
    }

    Ok(AnalysisReport {
        duration_s: duration,
        sample_rate: sr,
        total_samples: total,
        overall_centroid,
        overall_rolloff,
        overall_zcr,
        overall_hnr,
        overall_inharmonicity,
        overall_attack_ms: overall_attack,
        overall_decay_s: overall_decay,
        notes,
    })
}

// ─── Print a full report ────────────────────────────────────────────────
pub fn print_report(r: &AnalysisReport) {
    println!(
        "Duration: {:.1}s  Sample rate: {}  Samples: {}",
        r.duration_s, r.sample_rate, r.total_samples
    );
    println!(
        "Overall: centroid={:.0}Hz rolloff={:.0}Hz zcr={:.3} hnr={:.3} inharmonicity={:.4} attack={:.1}ms decay={:.2}s",
        r.overall_centroid,
        r.overall_rolloff,
        r.overall_zcr,
        r.overall_hnr,
        r.overall_inharmonicity,
        r.overall_attack_ms,
        r.overall_decay_s
    );
    println!();
    println!("Notes detected: {}", r.notes.len());
    println!(
        "{:>6}  {:>8}  {:>5}  {:>7} {:>7} {:>7} {:>6} {:>6} {:>6} {:>6} {:>7} {:>6}",
        "time", "hz", "note", "H1", "H2", "H3", "H4", "H5", "H6", "H7", "centroid", "peak"
    );

    for n in &r.notes {
        println!(
            "{:5.1}s  {:7.1}Hz  {:>5}  {:.2}   {:.2}   {:.2}   {:.2}   {:.2}   {:.2}   {:.2}   {:5.0}Hz  {:.2}",
            n.time_s,
            n.hz,
            n.note_name,
            n.harmonics[0],
            n.harmonics[1],
            n.harmonics[2],
            n.harmonics[3],
            n.harmonics[4],
            n.harmonics[5],
            n.harmonics[6],
            n.centroid,
            n.peak
        );
    }

    // Summary statistics
    if !r.notes.is_empty() {
        let h2s: Vec<f32> = r
            .notes
            .iter()
            .filter(|n| n.harmonics[0] > 0.0)
            .map(|n| n.harmonics[1] / n.harmonics[0])
            .collect();
        let h3s: Vec<f32> = r
            .notes
            .iter()
            .filter(|n| n.harmonics[0] > 0.0)
            .map(|n| n.harmonics[2] / n.harmonics[0])
            .collect();
        if !h2s.is_empty() {
            let h2_avg = h2s.iter().sum::<f32>() / h2s.len() as f32;
            let h3_avg = h3s.iter().sum::<f32>() / h3s.len() as f32;
            println!();
            println!("Summary: {} notes analyzed", r.notes.len());
            println!(
                "  H2/H1 ratio: min={:.2} avg={:.2} max={:.2}",
                h2s.iter().fold(99.0f32, |a, &b| a.min(b)),
                h2_avg,
                h2s.iter().fold(0.0f32, |a, &b| a.max(b))
            );
            println!(
                "  H3/H1 ratio: min={:.2} avg={:.2} max={:.2}",
                h3s.iter().fold(99.0f32, |a, &b| a.min(b)),
                h3_avg,
                h3s.iter().fold(0.0f32, |a, &b| a.max(b))
            );
        }

        // Note range
        let min_midi = r.notes.iter().map(|n| n.midi).min().unwrap_or(0);
        let max_midi = r.notes.iter().map(|n| n.midi).max().unwrap_or(0);
        println!(
            "  Note range: {} (midi {}) to {} (midi {})",
            note_name(midi_to_hz(min_midi)),
            min_midi,
            note_name(midi_to_hz(max_midi)),
            max_midi
        );

        // Unique notes
        let unique: std::collections::HashSet<u8> = r.notes.iter().map(|n| n.midi).collect();
        println!("  Unique notes: {}", unique.len());
    }
}

// ─── Batch analysis: scan multiple files and summarize ──────────────────
pub fn analyze_batch(paths: &[String]) -> std::io::Result<()> {
    let mut all_h2 = Vec::new();
    let mut all_h3 = Vec::new();
    let mut total_notes = 0;

    for path in paths {
        let p = Path::new(path);
        if !p.exists() {
            eprintln!("{}: file not found", path);
            continue;
        }
        println!("\n{}", "=".repeat(60));
        println!("{}", path);
        println!("{}", "=".repeat(60));

        let report = analyze_wav(p)?;
        print_report(&report);

        for n in &report.notes {
            if n.harmonics[0] > 0.0 {
                all_h2.push(n.harmonics[1] / n.harmonics[0]);
                all_h3.push(n.harmonics[2] / n.harmonics[0]);
            }
        }
        total_notes += report.notes.len();
    }

    if !all_h2.is_empty() {
        println!("\n{}", "=".repeat(60));
        println!(
            "BATCH SUMMARY: {} notes across {} files",
            total_notes,
            paths.len()
        );
        println!("{}", "=".repeat(60));
        let h2_avg = all_h2.iter().sum::<f32>() / all_h2.len() as f32;
        let h3_avg = all_h3.iter().sum::<f32>() / all_h3.len() as f32;
        println!(
            "  H2/H1: min={:.2} avg={:.2} max={:.2}",
            all_h2.iter().fold(99.0f32, |a, &b| a.min(b)),
            h2_avg,
            all_h2.iter().fold(0.0f32, |a, &b| a.max(b))
        );
        println!(
            "  H3/H1: min={:.2} avg={:.2} max={:.2}",
            all_h3.iter().fold(99.0f32, |a, &b| a.min(b)),
            h3_avg,
            all_h3.iter().fold(0.0f32, |a, &b| a.max(b))
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_standard_note_names() {
        assert_eq!(note_name(440.0), "A4");
        assert_eq!(note_name(261.625_55), "C4");
        assert_eq!(note_name(49.0), "G1");
        assert_eq!(note_name(midi_to_hz(0)), "C-1");
    }

    #[test]
    fn identifies_generated_pitch_without_a_file() {
        let samples: Vec<f32> = (0..22_050)
            .map(|index| (TAU * 440.0 * index as f32 / 44_100.0).sin() * 0.5)
            .collect();
        let pitch = identify_pitch(&samples, 44_100).expect("pitch");
        assert_eq!(pitch.midi, 69);
        assert!(pitch.cents.abs() < 1.0, "{} cents", pitch.cents);
        assert!(identify_pitch(&vec![0.0; 8_192], 44_100).is_none());
    }
}
