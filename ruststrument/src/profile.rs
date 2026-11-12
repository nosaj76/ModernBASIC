use crate::{
    physical::{
        BoreShape, KeyboardExciter, KeyboardKind, KeyboardProfile, PercussionExciter,
        PercussionProfile, PercussionResonator, WindExciter, WindProfile,
    },
    stringed::{
        ResonatorBody, StringInstrumentKind, StringInstrumentProfile, StringMaterial,
        MAX_PROFILE_STRINGS,
    },
    technique,
    wav::Wav,
};
use rustfft::{num_complex::Complex, FftPlanner};
use std::{cmp::Ordering, f32::consts::TAU, fs, io, path::Path};

pub const PARTIALS: usize = 48;
pub const SYNTH_PARTIALS: usize = 16;
const MAX_REGIONS: usize = 65_535;
const MAX_SAMPLES_PER_REGION: usize = 50_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Trigger {
    Attack,
    Release,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SynthModel {
    SampleOnly = 0,
    Additive = 1,
    PluckedString = 2,
    Noise = 3,
    BowedString = 4,
    StruckString = 5,
    Wind = 6,
    Reed = 7,
    Brass = 8,
    Membrane = 9,
    Modal = 10,
    Banjo = 11,
    Guitar = 12,
    Piano = 13,
    FreeReed = 14,
    Organ = 15,
}

impl SynthModel {
    fn from_byte(value: u8) -> io::Result<Self> {
        match value {
            0 => Ok(Self::SampleOnly),
            1 => Ok(Self::Additive),
            2 => Ok(Self::PluckedString),
            3 => Ok(Self::Noise),
            4 => Ok(Self::BowedString),
            5 => Ok(Self::StruckString),
            6 => Ok(Self::Wind),
            7 => Ok(Self::Reed),
            8 => Ok(Self::Brass),
            9 => Ok(Self::Membrane),
            10 => Ok(Self::Modal),
            11 => Ok(Self::Banjo),
            12 => Ok(Self::Guitar),
            13 => Ok(Self::Piano),
            14 => Ok(Self::FreeReed),
            15 => Ok(Self::Organ),
            _ => Err(invalid_data("unknown synth model")),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SampleOnly => "sample_only",
            Self::Additive => "additive",
            Self::PluckedString => "plucked_string",
            Self::Noise => "noise",
            Self::BowedString => "bowed_string",
            Self::StruckString => "struck_string",
            Self::Wind => "flute_air_jet",
            Self::Reed => "reed",
            Self::Brass => "brass",
            Self::Membrane => "membrane",
            Self::Modal => "modal",
            Self::Banjo => "banjo_physical",
            Self::Guitar => "guitar_physical",
            Self::Piano => "piano_physical",
            Self::FreeReed => "free_reed",
            Self::Organ => "organ_pipe",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SynthConfig {
    pub model: SynthModel,
    pub sample_level: f32,
    pub synth_level: f32,
    pub master_gain: f32,
    pub attack_s: f32,
    pub decay_s: f32,
    pub sustain: f32,
    pub release_s: f32,
    pub noise: f32,
    pub brightness: f32,
    pub damping: f32,
    pub pluck_position: f32,
    pub harmonics: [f32; SYNTH_PARTIALS],
    pub mode_ratios: [f32; SYNTH_PARTIALS],
    pub mode_amplitudes: [f32; SYNTH_PARTIALS],
    pub mode_decays: [f32; SYNTH_PARTIALS],
    pub vibrato_rate_hz: f32,
    pub vibrato_depth_cents: f32,
    pub transient_mix: f32,
}

impl Default for SynthConfig {
    fn default() -> Self {
        let mut harmonics = [0.0; SYNTH_PARTIALS];
        harmonics[0] = 1.0;
        harmonics[1] = 0.45;
        harmonics[2] = 0.24;
        harmonics[3] = 0.12;
        let mut mode_ratios = [0.0; SYNTH_PARTIALS];
        let mut mode_amplitudes = [0.0; SYNTH_PARTIALS];
        let mut mode_decays = [0.0; SYNTH_PARTIALS];
        for index in 0..SYNTH_PARTIALS {
            mode_ratios[index] = (index + 1) as f32;
            mode_amplitudes[index] = harmonics[index];
            mode_decays[index] = 0.8 / ((index + 1) as f32).sqrt();
        }
        Self {
            model: SynthModel::Additive,
            sample_level: 0.85,
            synth_level: 0.25,
            master_gain: 0.85,
            attack_s: 0.005,
            decay_s: 0.45,
            sustain: 0.35,
            release_s: 0.25,
            noise: 0.02,
            brightness: 0.55,
            damping: 0.996,
            pluck_position: 0.2,
            harmonics,
            mode_ratios,
            mode_amplitudes,
            mode_decays,
            vibrato_rate_hz: 5.2,
            vibrato_depth_cents: 0.0,
            transient_mix: 0.2,
        }
    }
}

impl SynthConfig {
    pub fn learn(family: &str, regions: &[Region]) -> Self {
        let mut config = Self {
            model: model_for_family(family),
            ..Self::default()
        };
        match config.model {
            SynthModel::PluckedString => {
                config.synth_level = 0.38;
                config.sustain = 0.08;
                config.damping = 0.9965;
                config.pluck_position = 0.18;
            }
            SynthModel::Noise => {
                config.sample_level = 0.9;
                config.synth_level = 0.2;
                config.sustain = 0.05;
                config.damping = 0.985;
            }
            SynthModel::BowedString => {
                config.synth_level = 0.3;
                config.sustain = 0.78;
                config.noise = 0.06;
                config.damping = 0.9992;
                config.vibrato_depth_cents = 7.0;
            }
            SynthModel::StruckString => {
                config.synth_level = 0.32;
                config.sustain = 0.12;
                config.damping = 0.9984;
                config.transient_mix = 0.32;
            }
            SynthModel::Wind => {
                config.synth_level = 0.28;
                config.sustain = 0.82;
                config.noise = 0.08;
                config.vibrato_depth_cents = 8.0;
            }
            SynthModel::Reed => {
                config.synth_level = 0.28;
                config.sustain = 0.86;
                config.noise = 0.045;
                config.vibrato_depth_cents = 5.0;
            }
            SynthModel::Brass => {
                config.synth_level = 0.3;
                config.sustain = 0.84;
                config.noise = 0.025;
                config.brightness = 0.72;
                config.vibrato_depth_cents = 3.0;
            }
            SynthModel::Membrane => {
                config.sample_level = 0.92;
                config.synth_level = 0.24;
                config.sustain = 0.01;
                config.transient_mix = 0.55;
            }
            SynthModel::Modal => {
                config.sample_level = 0.92;
                config.synth_level = 0.25;
                config.sustain = 0.01;
                config.transient_mix = 0.45;
            }
            SynthModel::Banjo => {
                config.sample_level = 0.0;
                config.synth_level = 1.0;
                config.sustain = 0.01;
                config.damping = 0.985;
                config.transient_mix = 0.9;
            }
            SynthModel::Guitar => {
                config.sample_level = 0.0;
                config.synth_level = 1.0;
                config.sustain = 0.04;
                config.damping = 0.996;
                config.transient_mix = 0.65;
            }
            SynthModel::Piano => {
                config.sample_level = 0.0;
                config.synth_level = 1.0;
                config.sustain = 0.08;
                config.damping = 0.999;
                config.transient_mix = 0.75;
            }
            SynthModel::FreeReed => {
                config.sample_level = 0.0;
                config.synth_level = 1.0;
                config.sustain = 0.88;
                config.noise = 0.025;
                config.vibrato_depth_cents = 3.0;
            }
            SynthModel::Organ => {
                config.sample_level = 0.0;
                config.synth_level = 1.0;
                config.sustain = 0.96;
                config.noise = 0.005;
                config.vibrato_depth_cents = 0.0;
            }
            SynthModel::Additive => config.synth_level = 0.24,
            SynthModel::SampleOnly => {
                config.sample_level = 1.0;
                config.synth_level = 0.0;
            }
        }

        let attacks: Vec<&Region> = regions
            .iter()
            .filter(|region| region.trigger == Trigger::Attack)
            .collect();
        if attacks.is_empty() {
            return config;
        }

        for harmonic in 0..SYNTH_PARTIALS {
            config.harmonics[harmonic] = attacks
                .iter()
                .map(|region| region.partials[harmonic].max(0.0))
                .sum::<f32>()
                / attacks.len() as f32;
        }
        let harmonic_peak = config
            .harmonics
            .iter()
            .copied()
            .fold(0.0_f32, f32::max)
            .max(1e-6);
        for value in &mut config.harmonics {
            *value = (*value / harmonic_peak).clamp(0.0, 1.0);
        }
        let (mode_ratios, mode_amplitudes, mode_decays) = learn_modes(&attacks);
        config.mode_ratios = mode_ratios;
        config.mode_amplitudes = mode_amplitudes;
        config.mode_decays = mode_decays;
        if config.harmonics[0] < 0.05 {
            config.harmonics[0] = 1.0;
        }

        config.attack_s = median(attacks.iter().map(|region| region.attack_s)).clamp(0.001, 2.0);
        config.decay_s = median(attacks.iter().map(|region| region.decay_s)).clamp(0.03, 8.0);
        config.sustain = median(attacks.iter().map(|region| region.sustain)).clamp(0.01, 1.0);
        config.release_s = median(attacks.iter().map(|region| region.release_s)).clamp(0.02, 8.0);
        config.noise = median(attacks.iter().map(|region| region.noise)).clamp(0.0, 0.35);

        let total = config.harmonics.iter().sum::<f32>().max(1e-6);
        let weighted: f32 = config
            .harmonics
            .iter()
            .enumerate()
            .map(|(index, value)| *value * (index + 1) as f32)
            .sum();
        config.brightness = ((weighted / total - 1.0) / 8.0).clamp(0.1, 1.0);
        if matches!(
            config.model,
            SynthModel::PluckedString
                | SynthModel::StruckString
                | SynthModel::Banjo
                | SynthModel::Guitar
                | SynthModel::Piano
        ) {
            config.damping = (0.994 + config.decay_s.min(3.0) * 0.0015).clamp(0.994, 0.9992);
        }
        config
    }

    /// Learn measured spectral/envelope parameters and retain the dedicated
    /// generator for optional extension. When recordings are present, captured
    /// playback is the default so the audible result preserves its reference.
    pub fn learn_instrument(name: &str, family: &str, regions: &[Region]) -> Self {
        let mut config = Self::learn(family, regions);
        let lower = name.to_ascii_lowercase();
        if let Some(profile) = StringInstrumentProfile::for_instrument(name, family) {
            config.model = if profile.kind == StringInstrumentKind::Banjo {
                SynthModel::Banjo
            } else {
                SynthModel::Guitar
            };
            config.sample_level = 0.0;
            config.synth_level = 1.0;
            config.master_gain = profile.master_gain;
            config.attack_s = if profile.kind == StringInstrumentKind::Banjo {
                0.003
            } else {
                0.001
            };
            config.decay_s = if regions.is_empty() {
                profile.decay_s
            } else {
                config
                    .decay_s
                    .clamp(profile.decay_s * 0.55, profile.decay_s * 1.8)
            };
            config.sustain = if profile.kind == StringInstrumentKind::Banjo {
                0.01
            } else {
                0.035
            };
            config.release_s = (profile.decay_s * 0.22).clamp(0.10, 1.2);
            config.noise = if regions.is_empty() {
                profile.pick_noise
            } else {
                config
                    .noise
                    .clamp(profile.pick_noise * 0.35, profile.pick_noise * 1.8)
            };
            config.brightness = if regions.is_empty() {
                profile.brightness
            } else {
                config.brightness.clamp(
                    (profile.brightness - 0.20).max(0.10),
                    (profile.brightness + 0.15).min(1.0),
                )
            };
            config.damping = profile.base_damping;
            config.pluck_position = profile.pluck_position;
            config.transient_mix = profile.transient_mix;
        } else if ["piano", "clavichord", "harpsichord", "cimbalom"]
            .iter()
            .any(|word| contains_term(&lower, word))
        {
            config.model = SynthModel::Piano;
            config.sample_level = 0.0;
            config.synth_level = 1.0;
            config.master_gain = 0.65;
            config.attack_s = 0.001;
            config.decay_s = config.decay_s.clamp(1.2, 5.5);
            config.sustain = 0.08;
            config.release_s = 1.2;
            config.noise = config.noise.max(0.05);
            config.brightness = config.brightness.clamp(0.42, 0.82);
            config.damping = 0.999;
            config.pluck_position = 0.12;
            config.transient_mix = 0.75;
        } else if family != "unknown" {
            config.sample_level = 0.0;
            config.synth_level = 1.0;
            config.master_gain = config.master_gain.min(0.8);
        }
        match config.model {
            SynthModel::Piano if contains_term(&lower, "harpsichord") => {
                config.decay_s = 0.95;
                config.release_s = 0.18;
                config.damping = 0.995;
                config.brightness = 0.92;
                config.transient_mix = 0.95;
            }
            SynthModel::Piano if contains_term(&lower, "clavichord") => {
                config.decay_s = 1.1;
                config.damping = 0.997;
                config.brightness = 0.52;
                config.master_gain = 0.54;
            }
            SynthModel::Piano
                if ["dulcimer", "cimbalom", "celesta"]
                    .iter()
                    .any(|word| contains_term(&lower, word)) =>
            {
                config.decay_s = 1.8;
                config.damping = 0.9975;
                config.brightness = 0.84;
                config.transient_mix = 0.90;
            }
            SynthModel::BowedString => {
                config.attack_s = if contains_term(&lower, "double bass") {
                    0.075
                } else {
                    0.035
                };
                config.brightness = if ["cello", "double bass", "contrabass"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    0.48
                } else if contains_term(&lower, "viola") {
                    0.58
                } else {
                    0.70
                };
            }
            SynthModel::Wind => {
                config.attack_s = 0.025;
                config.noise = if contains_term(&lower, "recorder") {
                    0.045
                } else {
                    0.085
                };
                config.brightness = if contains_term(&lower, "piccolo") {
                    0.82
                } else {
                    0.58
                };
            }
            SynthModel::Reed => {
                config.attack_s = 0.018;
                config.brightness = if ["oboe", "english horn", "cor anglais"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    0.78
                } else if contains_term(&lower, "bassoon") {
                    0.46
                } else {
                    0.66
                };
            }
            SynthModel::Brass => {
                config.attack_s = 0.022;
                config.brightness = if ["french horn", "flugelhorn", "tuba"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    0.50
                } else if contains_term(&lower, "trumpet") {
                    0.82
                } else {
                    0.68
                };
            }
            SynthModel::FreeReed => {
                config.attack_s = 0.035;
                config.decay_s = 0.18;
                config.sustain = 0.90;
                config.release_s = 0.22;
                config.noise = 0.025;
                config.brightness = 0.64;
            }
            SynthModel::Organ => {
                config.attack_s = 0.018;
                config.decay_s = 0.12;
                config.sustain = 0.98;
                config.release_s = 0.18;
                config.noise = 0.004;
                config.brightness = 0.58;
            }
            SynthModel::Membrane => {
                config.attack_s = 0.001;
                config.decay_s = if contains_term(&lower, "snare") {
                    0.22
                } else if contains_term(&lower, "timpani") {
                    1.35
                } else if contains_term(&lower, "bass drum") || contains_term(&lower, "taiko") {
                    0.85
                } else {
                    0.55
                };
                config.sustain = 0.01;
                config.release_s = 0.12;
                config.noise = if contains_term(&lower, "snare") {
                    0.32
                } else {
                    0.06
                };
            }
            SynthModel::Modal => {
                config.attack_s = 0.001;
                config.decay_s = if ["cymbal", "gong"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    5.5
                } else if family == "metal" {
                    3.4
                } else if contains_term(&lower, "marimba") {
                    2.2
                } else {
                    1.15
                };
                config.sustain = 0.01;
                config.release_s = 0.35;
            }
            SynthModel::Noise if family == "noise_percussion" => {
                config.attack_s = 0.001;
                config.decay_s = 0.32;
                config.sustain = 0.01;
                config.release_s = 0.08;
                config.noise = 0.85;
                config.transient_mix = 0.9;
            }
            _ => {}
        }
        if let Some(profile) = KeyboardProfile::for_instrument(name, family) {
            config.model = SynthModel::Piano;
            config.sample_level = 0.0;
            config.synth_level = 1.0;
            config.master_gain = match profile.exciter {
                KeyboardExciter::Plectrum => 0.58,
                KeyboardExciter::Tangent => 0.52,
                _ => 0.66,
            };
            config.attack_s = 0.001;
            config.decay_s = profile.decay_s;
            config.sustain = if profile.pedal_coupling > 0.0 {
                0.06
            } else {
                0.01
            };
            config.release_s = profile.release_s;
            config.noise = profile.hammer_noise;
            config.brightness = profile.brightness;
            config.damping = profile.base_damping;
            config.pluck_position = profile.strike_position;
            config.transient_mix = (0.40 + profile.hammer_hardness * 0.60).clamp(0.0, 1.0);
        }
        if let Some(profile) = WindProfile::for_instrument(name, family) {
            config.sample_level = 0.0;
            config.synth_level = 1.0;
            config.master_gain = match profile.exciter {
                WindExciter::LipReed => 0.52,
                WindExciter::DoubleReed => 0.48,
                _ => 0.58,
            };
            config.attack_s = match profile.exciter {
                WindExciter::Pipe => 0.018,
                WindExciter::FreeReed => 0.035,
                _ => 0.025,
            };
            config.decay_s = 0.16;
            config.sustain = 0.96;
            config.release_s = 0.20;
            config.noise = profile.breath_noise;
            config.brightness = profile.brightness;
            config.damping = (1.0 - profile.bore_loss * 0.012).clamp(0.97, 0.9999);
            config.transient_mix = match profile.exciter {
                WindExciter::AirJet => 0.72,
                WindExciter::LipReed => 0.48,
                _ => 0.56,
            };
        }
        if let Some(profile) = PercussionProfile::for_instrument(name, family) {
            config.sample_level = 0.0;
            config.synth_level = 1.0;
            config.master_gain = if profile.resonator == PercussionResonator::Plate {
                0.42
            } else {
                0.62
            };
            config.attack_s = 0.001;
            config.decay_s = profile.decay_s;
            config.sustain = 0.005;
            config.release_s = (profile.decay_s * 0.15).clamp(0.06, 0.65);
            config.noise = profile.transient_noise.max(profile.shell_noise * 0.5);
            config.brightness = profile.brightness;
            config.transient_mix = (0.40 + profile.hardness * 0.60).clamp(0.0, 1.0);
        }
        if regions.is_empty() {
            config.harmonics = match config.model {
                SynthModel::Banjo => [
                    0.679, 0.973, 1.000, 0.763, 0.740, 0.212, 0.336, 0.227, 0.136, 0.224, 0.059,
                    0.171, 0.083, 0.134, 0.097, 0.055,
                ],
                SynthModel::Wind => [
                    1.0, 0.09, 0.025, 0.012, 0.006, 0.003, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0,
                ],
                SynthModel::Reed => [
                    1.0, 0.05, 0.58, 0.025, 0.34, 0.015, 0.21, 0.01, 0.12, 0.0, 0.07, 0.0, 0.04,
                    0.0, 0.02, 0.0,
                ],
                SynthModel::Brass => [
                    1.0, 0.72, 0.52, 0.36, 0.24, 0.16, 0.11, 0.075, 0.05, 0.035, 0.02, 0.012,
                    0.008, 0.004, 0.002, 0.0,
                ],
                SynthModel::BowedString => [
                    1.0, 0.42, 0.31, 0.24, 0.18, 0.14, 0.11, 0.08, 0.06, 0.045, 0.03, 0.022, 0.016,
                    0.01, 0.006, 0.003,
                ],
                SynthModel::FreeReed => [
                    1.0, 0.34, 0.62, 0.20, 0.40, 0.12, 0.25, 0.075, 0.15, 0.045, 0.09, 0.025, 0.05,
                    0.014, 0.025, 0.008,
                ],
                SynthModel::Organ => [
                    1.0, 0.52, 0.34, 0.24, 0.16, 0.12, 0.085, 0.065, 0.048, 0.036, 0.027, 0.020,
                    0.015, 0.011, 0.008, 0.006,
                ],
                _ => config.harmonics,
            };
            if config.model == SynthModel::Reed {
                config.harmonics = if ["oboe", "english horn"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    [
                        1.0, 0.72, 0.68, 0.52, 0.44, 0.36, 0.29, 0.22, 0.17, 0.13, 0.10, 0.075,
                        0.055, 0.04, 0.028, 0.018,
                    ]
                } else if ["bassoon", "contrabassoon"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    [
                        1.0, 0.62, 0.38, 0.34, 0.22, 0.18, 0.14, 0.09, 0.065, 0.045, 0.032, 0.022,
                        0.015, 0.010, 0.006, 0.003,
                    ]
                } else if contains_term(&lower, "saxophone") {
                    [
                        1.0, 0.55, 0.66, 0.38, 0.45, 0.27, 0.31, 0.18, 0.21, 0.12, 0.14, 0.08,
                        0.09, 0.05, 0.045, 0.025,
                    ]
                } else {
                    config.harmonics
                };
            } else if config.model == SynthModel::FreeReed && contains_term(&lower, "harmonica") {
                config.harmonics = [
                    1.0, 0.18, 0.72, 0.11, 0.46, 0.07, 0.29, 0.045, 0.18, 0.028, 0.11, 0.018,
                    0.065, 0.010, 0.035, 0.006,
                ];
            }
            if config.model == SynthModel::Membrane {
                config.mode_ratios = [
                    1.0, 1.59, 2.14, 2.30, 2.65, 2.92, 3.16, 3.50, 3.60, 3.65, 4.06, 4.15, 4.30,
                    4.60, 4.90, 5.20,
                ];
                config.mode_amplitudes = [
                    1.0, 0.58, 0.41, 0.32, 0.25, 0.20, 0.16, 0.13, 0.11, 0.09, 0.07, 0.055, 0.043,
                    0.032, 0.022, 0.014,
                ];
                config.mode_decays = [
                    1.35, 1.08, 0.88, 0.76, 0.66, 0.58, 0.51, 0.45, 0.40, 0.36, 0.32, 0.28, 0.25,
                    0.22, 0.19, 0.16,
                ];
            } else if config.model == SynthModel::Modal {
                if family == "wood" {
                    config.mode_ratios = [
                        1.0, 3.93, 9.1, 16.2, 25.7, 37.4, 51.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0,
                    ];
                    config.mode_amplitudes = [
                        1.0, 0.48, 0.22, 0.11, 0.06, 0.03, 0.015, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.0,
                    ];
                    config.mode_decays = [
                        2.2, 1.35, 0.82, 0.52, 0.34, 0.23, 0.16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0,
                    ];
                } else if ["cymbal", "gong"]
                    .iter()
                    .any(|word| contains_term(&lower, word))
                {
                    config.mode_ratios = [
                        1.0, 1.13, 1.42, 1.71, 2.03, 2.36, 2.82, 3.31, 3.87, 4.55, 5.28, 6.14,
                        7.25, 8.62, 10.3, 12.4,
                    ];
                    config.mode_amplitudes = [
                        0.58, 0.72, 1.0, 0.83, 0.69, 0.61, 0.53, 0.46, 0.39, 0.33, 0.27, 0.22,
                        0.17, 0.12, 0.08, 0.05,
                    ];
                    config.mode_decays = [
                        5.5, 5.1, 4.8, 4.5, 4.2, 3.9, 3.6, 3.3, 3.0, 2.7, 2.4, 2.1, 1.8, 1.5, 1.2,
                        0.9,
                    ];
                } else {
                    config.mode_ratios = [
                        1.0, 2.01, 2.76, 3.94, 5.40, 6.80, 8.93, 10.5, 12.7, 15.1, 17.8, 20.6,
                        23.7, 27.1, 30.8, 34.8,
                    ];
                    config.mode_amplitudes = [
                        1.0, 0.62, 0.48, 0.36, 0.28, 0.22, 0.17, 0.13, 0.10, 0.078, 0.06, 0.046,
                        0.034, 0.024, 0.016, 0.010,
                    ];
                    config.mode_decays = [
                        3.4, 3.0, 2.7, 2.4, 2.15, 1.92, 1.72, 1.54, 1.38, 1.23, 1.10, 0.98, 0.87,
                        0.76, 0.66, 0.56,
                    ];
                }
            }
        } else {
            // A learned bank must reproduce its recordings by default. The
            // measured model remains stored and can be enabled explicitly for
            // notes or transitions that have no captured reference, but it is
            // not allowed to replace the recorded tone under an HQ claim.
            config.sample_level = 1.0;
            config.synth_level = 0.0;
        }
        config
    }

    fn validate(&self) -> io::Result<()> {
        let values = [
            self.sample_level,
            self.synth_level,
            self.master_gain,
            self.attack_s,
            self.decay_s,
            self.sustain,
            self.release_s,
            self.noise,
            self.brightness,
            self.damping,
            self.pluck_position,
        ];
        if values.iter().any(|value| !value.is_finite())
            || self.harmonics.iter().any(|value| !value.is_finite())
            || self.mode_ratios.iter().any(|value| !value.is_finite())
            || self.mode_amplitudes.iter().any(|value| !value.is_finite())
            || self.mode_decays.iter().any(|value| !value.is_finite())
            || !self.vibrato_rate_hz.is_finite()
            || !self.vibrato_depth_cents.is_finite()
            || !self.transient_mix.is_finite()
        {
            return Err(invalid_data("non-finite synth configuration"));
        }
        if !(0.0..=2.0).contains(&self.sample_level)
            || !(0.0..=2.0).contains(&self.synth_level)
            || !(0.0..=2.0).contains(&self.master_gain)
            || self.attack_s < 0.0
            || self.decay_s < 0.0
            || !(0.0..=1.0).contains(&self.sustain)
            || self.release_s < 0.0
            || !(0.0..=1.0).contains(&self.noise)
            || !(0.0..=1.0).contains(&self.brightness)
            || !(0.9..=1.0).contains(&self.damping)
            || !(0.01..=0.99).contains(&self.pluck_position)
            || !(0.1..=20.0).contains(&self.vibrato_rate_hz)
            || !(0.0..=100.0).contains(&self.vibrato_depth_cents)
            || !(0.0..=1.0).contains(&self.transient_mix)
            || self
                .mode_ratios
                .iter()
                .any(|value| !(0.0..=64.0).contains(value))
            || self
                .mode_amplitudes
                .iter()
                .any(|value| !(0.0..=1.0).contains(value))
            || self
                .mode_decays
                .iter()
                .any(|value| !(0.0..=30.0).contains(value))
        {
            return Err(invalid_data("synth configuration out of range"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub name: String,
    pub articulation: String,
    pub trigger: Trigger,
    pub root_key: u8,
    pub key_lo: u8,
    pub key_hi: u8,
    pub vel_lo: u8,
    pub vel_hi: u8,
    pub round_robin: u8,
    pub sample_rate: u32,
    pub tune_cents: f32,
    pub gain_db: f32,
    pub attack_s: f32,
    pub decay_s: f32,
    pub sustain: f32,
    pub release_s: f32,
    pub noise: f32,
    pub loop_start: Option<u32>,
    pub loop_end: Option<u32>,
    pub partials: [f32; PARTIALS],
    pub samples: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstrumentBank {
    pub name: String,
    pub family: String,
    pub default_articulation: String,
    pub polyphony: u16,
    pub synth: SynthConfig,
    pub string_profile: Option<StringInstrumentProfile>,
    pub keyboard_profile: Option<KeyboardProfile>,
    pub wind_profile: Option<WindProfile>,
    pub percussion_profile: Option<PercussionProfile>,
    pub regions: Vec<Region>,
}

pub(crate) fn detect_pitch(samples: &[f32], sample_rate: u32) -> f32 {
    crate::creator::detect_pitch_fft(samples, sample_rate).unwrap_or(440.0)
}

pub fn analyze_region(name: &str, wav: &Wav, root_hint: Option<u8>) -> Region {
    let peak = wav
        .samples
        .iter()
        .fold(0.0_f32, |value, sample| value.max(sample.abs()))
        .max(1e-9);
    let onset = wav
        .samples
        .iter()
        .position(|sample| sample.abs() > peak * 0.025)
        .unwrap_or(0);
    let detected_hz = detect_pitch(&wav.samples[onset..], wav.sample_rate);
    let detected_midi = (69.0 + 12.0 * (detected_hz / 440.0).log2())
        .round()
        .clamp(0.0, 127.0) as u8;
    let root_key = root_hint.unwrap_or(detected_midi).min(127);
    let root_hz = 440.0 * 2.0_f32.powf((root_key as f32 - 69.0) / 12.0);
    let tune_cents = if detected_hz.is_finite() && detected_hz > 0.0 {
        (1_200.0 * (detected_hz / root_hz).log2()).clamp(-100.0, 100.0)
    } else {
        0.0
    };

    let analysis_length = 8_192.min(wav.samples.len().saturating_sub(onset));
    let mut partials = [0.0; PARTIALS];
    for harmonic in 1..=PARTIALS {
        let frequency = detected_hz * harmonic as f32;
        if frequency > wav.sample_rate as f32 * 0.48 || analysis_length == 0 {
            break;
        }
        let mut real = 0.0;
        let mut imaginary = 0.0;
        for index in 0..analysis_length {
            let window = 0.5 - 0.5 * (TAU * index as f32 / analysis_length as f32).cos();
            let angle = TAU * frequency * index as f32 / wav.sample_rate as f32;
            let sample = wav.samples[onset + index] * window;
            real += sample * angle.cos();
            imaginary -= sample * angle.sin();
        }
        partials[harmonic - 1] =
            (real * real + imaginary * imaginary).sqrt() / analysis_length as f32;
    }
    let partial_peak = partials.iter().copied().fold(0.0_f32, f32::max).max(1e-9);
    for partial in &mut partials {
        *partial /= partial_peak;
    }

    let frame_length = (wav.sample_rate / 200).max(1) as usize;
    let envelope: Vec<f32> = wav
        .samples
        .chunks(frame_length)
        .map(|frame| {
            (frame.iter().map(|sample| sample * sample).sum::<f32>() / frame.len() as f32).sqrt()
        })
        .collect();
    let envelope_peak = envelope.iter().copied().fold(0.0_f32, f32::max).max(1e-9);
    let peak_index = envelope
        .iter()
        .position(|value| (*value - envelope_peak).abs() <= f32::EPSILON)
        .unwrap_or(0);
    let decay_frames = envelope
        .get(peak_index..)
        .and_then(|tail| tail.iter().position(|value| *value < envelope_peak * 0.37))
        .unwrap_or_else(|| envelope.len().saturating_sub(peak_index));
    let sustain = envelope
        .get((envelope.len() * 3 / 4).min(envelope.len().saturating_sub(1)))
        .copied()
        .unwrap_or(0.0)
        / envelope_peak;
    let high_energy: f32 = wav
        .samples
        .windows(2)
        .map(|pair| (pair[1] - pair[0]).abs())
        .sum();
    let total_energy = wav
        .samples
        .iter()
        .map(|sample| sample.abs())
        .sum::<f32>()
        .max(1e-9);

    let mut normalized_samples: Vec<f32> = wav.samples[onset..]
        .iter()
        .map(|sample| (sample / peak).clamp(-1.0, 1.0))
        .collect();
    while normalized_samples
        .last()
        .is_some_and(|sample| sample.abs() < 1e-5)
    {
        normalized_samples.pop();
    }
    normalized_samples.extend(std::iter::repeat(0.0).take(32));

    Region {
        name: name.into(),
        articulation: "sustain".into(),
        trigger: Trigger::Attack,
        root_key,
        key_lo: root_key,
        key_hi: root_key,
        vel_lo: 1,
        vel_hi: 127,
        round_robin: 1,
        sample_rate: wav.sample_rate,
        tune_cents,
        gain_db: 0.0,
        attack_s: peak_index as f32 * 0.005,
        decay_s: decay_frames as f32 * 0.005,
        sustain: sustain.clamp(0.01, 1.0),
        release_s: 0.25,
        noise: (high_energy / total_energy * 0.05).clamp(0.0, 0.3),
        loop_start: None,
        loop_end: None,
        partials,
        samples: normalized_samples,
    }
}

impl InstrumentBank {
    pub fn single(name: &str, wav: &Wav) -> Self {
        let mut region = analyze_region(name, wav, None);
        region.key_lo = 0;
        region.key_hi = 127;
        let named_family = family_from_name(name);
        let family = if named_family != "unknown" {
            named_family.to_string()
        } else {
            crate::classify::classify(&crate::classify::analyze_recording(wav))
                .label()
                .to_string()
        };
        let synth = SynthConfig::learn_instrument(name, &family, std::slice::from_ref(&region));
        let string_profile = StringInstrumentProfile::for_instrument(name, &family);
        let keyboard_profile = KeyboardProfile::for_instrument(name, &family);
        let wind_profile = WindProfile::for_instrument(name, &family);
        let percussion_profile = PercussionProfile::for_instrument(name, &family);
        Self {
            name: name.into(),
            family,
            default_articulation: "sustain".into(),
            polyphony: 64,
            synth,
            string_profile,
            keyboard_profile,
            wind_profile,
            percussion_profile,
            regions: vec![region],
        }
    }

    pub fn articulations(&self) -> Vec<String> {
        let mut articulations: Vec<String> = self
            .regions
            .iter()
            .filter(|region| region.trigger == Trigger::Attack)
            .map(|region| region.articulation.clone())
            .collect();
        articulations.sort();
        articulations.dedup();
        articulations
    }

    /// Build a compact, sample-independent playable instrument. Silent regions
    /// provide MIDI/articulation zoning; every audible sample is generated by
    /// the selected physical or excitation-resonator model.
    pub fn generator(name: &str) -> Self {
        let family = family_from_name(name).to_string();
        let synth = SynthConfig::learn_instrument(name, &family, &[]);
        let string_profile = StringInstrumentProfile::for_instrument(name, &family);
        let keyboard_profile = KeyboardProfile::for_instrument(name, &family);
        let wind_profile = WindProfile::for_instrument(name, &family);
        let percussion_profile = PercussionProfile::for_instrument(name, &family);
        let recommended = technique::recommended(name, &family);
        let articulations: Vec<&str> = if recommended.is_empty() {
            vec!["sustain"]
        } else {
            recommended
        };
        let default_articulation = articulations[0].to_string();
        let (range_low, range_high) = generator_range(name, &family);
        let mut roots: Vec<u8> = (range_low..=range_high).step_by(6).collect();
        if roots.last().copied() != Some(range_high) {
            roots.push(range_high);
        }
        let mut partials = [0.0_f32; PARTIALS];
        partials[..SYNTH_PARTIALS].copy_from_slice(&synth.harmonics);
        let mut regions = Vec::with_capacity(articulations.len() * roots.len());
        for articulation in articulations {
            for (index, root_key) in roots.iter().copied().enumerate() {
                let key_lo = if index == 0 {
                    0
                } else {
                    ((u16::from(roots[index - 1]) + u16::from(root_key)) / 2 + 1) as u8
                };
                let key_hi = if index + 1 == roots.len() {
                    127
                } else {
                    ((u16::from(root_key) + u16::from(roots[index + 1])) / 2) as u8
                };
                regions.push(Region {
                    name: format!("generator_{articulation}_{root_key}"),
                    articulation: articulation.to_string(),
                    trigger: Trigger::Attack,
                    root_key,
                    key_lo,
                    key_hi,
                    vel_lo: 1,
                    vel_hi: 127,
                    round_robin: 1,
                    sample_rate: 44_100,
                    tune_cents: 0.0,
                    gain_db: 0.0,
                    attack_s: synth.attack_s,
                    decay_s: synth.decay_s,
                    sustain: synth.sustain,
                    release_s: synth.release_s,
                    noise: synth.noise,
                    loop_start: None,
                    loop_end: None,
                    partials,
                    samples: vec![0.0; 64],
                });
            }
        }
        Self {
            name: name.to_string(),
            family,
            default_articulation,
            polyphony: 128,
            synth,
            string_profile,
            keyboard_profile,
            wind_profile,
            percussion_profile,
            regions,
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, self.to_bytes()?)
    }

    /// Serialize this bank to the versioned native format without performing
    /// any filesystem access. Embedders can keep their own bounded asset and
    /// permission policy around the library.
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        self.validate()?;
        let mut bytes = Vec::new();
        bytes.extend(b"RSTI\x06");
        put_string(&mut bytes, &self.name)?;
        put_string(&mut bytes, &self.family)?;
        put_string(&mut bytes, &self.default_articulation)?;
        bytes.extend(self.polyphony.to_le_bytes());
        put_synth(&mut bytes, &self.synth);
        put_string_profile(&mut bytes, self.string_profile.as_ref());
        put_keyboard_profile(&mut bytes, self.keyboard_profile.as_ref());
        put_wind_profile(&mut bytes, self.wind_profile.as_ref());
        put_percussion_profile(&mut bytes, self.percussion_profile.as_ref());
        bytes.extend(
            u32::try_from(self.regions.len())
                .map_err(|_| invalid_data("too many regions"))?
                .to_le_bytes(),
        );
        for region in &self.regions {
            put_region(&mut bytes, region)?;
        }
        Ok(bytes)
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let bytes = fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Load a bank from bytes supplied by the host. All versions supported by
    /// the filesystem loader are accepted and validated by the same parsers.
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        match bytes.get(..5) {
            Some(b"RSTI\x01") => load_v1(bytes),
            Some(b"RSTI\x02") => load_v2(bytes),
            Some(b"RSTI\x03") => load_v3(bytes),
            Some(b"RSTI\x04") => load_v4(bytes),
            Some(b"RSTI\x05") => load_v5(bytes),
            Some(b"RSTI\x06") => load_v6(bytes),
            _ => Err(invalid_data("unsupported .rsti version")),
        }
    }

    pub fn validate(&self) -> io::Result<()> {
        if self.name.is_empty() || self.name.len() > 1_000_000 {
            return Err(invalid_data("invalid instrument name"));
        }
        if self.family.len() > 1_000_000 || self.default_articulation.is_empty() {
            return Err(invalid_data("invalid instrument metadata"));
        }
        if self.regions.is_empty() || self.regions.len() > MAX_REGIONS {
            return Err(invalid_data("invalid region count"));
        }
        self.synth.validate()?;
        if self
            .string_profile
            .is_some_and(|profile| !profile.validate())
        {
            return Err(invalid_data("string construction profile out of range"));
        }
        if self
            .keyboard_profile
            .is_some_and(|profile| !profile.validate())
            || self.wind_profile.is_some_and(|profile| !profile.validate())
            || self
                .percussion_profile
                .is_some_and(|profile| !profile.validate())
        {
            return Err(invalid_data("physical construction profile out of range"));
        }
        for region in &self.regions {
            validate_region(region)?;
        }
        if !self.regions.iter().any(|region| {
            region.trigger == Trigger::Attack && region.articulation == self.default_articulation
        }) {
            return Err(invalid_data("default articulation has no attack regions"));
        }
        Ok(())
    }
}

pub const GENERATOR_CATALOG: &[&str] = &[
    // Plucked strings.
    "Five-string Banjo",
    "Acoustic Guitar",
    "Steel-string Guitar",
    "Electric Guitar",
    "Classical Guitar",
    "Twelve-string Guitar",
    "Acoustic Bass Guitar",
    "Electric Bass Guitar",
    "Mandolin",
    "Mandola",
    "Irish Bouzouki",
    "Ukulele",
    "Baritone Ukulele",
    "Lute",
    "Appalachian Dulcimer",
    "Concert Harp",
    "Sitar",
    "Koto",
    "Shamisen",
    "Balalaika",
    // Bowed strings.
    "Violin",
    "Viola",
    "Cello",
    "Double Bass",
    "Erhu",
    // Struck and plucked keyboards.
    "Concert Grand Piano",
    "Upright Piano",
    "Harpsichord",
    "Clavichord",
    "Hammered Dulcimer",
    "Cimbalom",
    "Celesta",
    // Air-jet winds.
    "Concert Flute",
    "Piccolo",
    "Recorder",
    "Ocarina",
    "Pan Flute",
    "Shakuhachi",
    "Tin Whistle",
    "Ney",
    "Quena",
    // Single- and double-reed winds.
    "Clarinet",
    "Bass Clarinet",
    "Soprano Saxophone",
    "Alto Saxophone",
    "Tenor Saxophone",
    "Baritone Saxophone",
    "Oboe",
    "English Horn",
    "Bassoon",
    "Contrabassoon",
    "Bagpipe",
    // Free reeds.
    "Harmonica",
    "Accordion",
    "Concertina",
    "Bandoneon",
    "Melodica",
    "Harmonium",
    // Lip-reed brass.
    "Trumpet",
    "Cornet",
    "Flugelhorn",
    "French Horn",
    "Trombone",
    "Euphonium",
    "Tuba",
    // Aerophone keyboard.
    "Pipe Organ",
    // Membranes.
    "Timpani",
    "Snare Drum",
    "Bass Drum",
    "Tom Drum",
    "Bongos",
    "Conga",
    "Djembe",
    "Tabla",
    "Taiko",
    // Tuned wood and metal; untuned metal.
    "Marimba",
    "Xylophone",
    "Woodblock",
    "Claves",
    "Guiro",
    "Vibraphone",
    "Glockenspiel",
    "Tubular Chimes",
    "Crotales",
    "Steelpan",
    "Cymbal",
    "Gong",
    "Triangle",
    // Noise percussion and electronic fallback.
    "Shaker",
    "Maracas",
    "Tambourine",
    "Cabasa",
    "Rainstick",
    "Additive Synthesizer",
];

fn generator_range(name: &str, family: &str) -> (u8, u8) {
    if let Some(profile) = StringInstrumentProfile::for_instrument(name, family) {
        return (profile.range_low, profile.range_high);
    }
    if let Some(profile) = KeyboardProfile::for_instrument(name, family) {
        return (profile.range_low, profile.range_high);
    }
    if let Some(profile) = WindProfile::for_instrument(name, family) {
        return (profile.range_low, profile.range_high);
    }
    if let Some(profile) = PercussionProfile::for_instrument(name, family) {
        return (profile.range_low, profile.range_high);
    }
    let lower = name.to_ascii_lowercase();
    if contains_term(&lower, "banjo") {
        (50, 86)
    } else if contains_term(&lower, "guitar") {
        (40, 88)
    } else if contains_term(&lower, "mandolin") {
        (55, 91)
    } else if contains_term(&lower, "ukulele") {
        (60, 88)
    } else if contains_term(&lower, "lute") {
        (45, 88)
    } else if contains_term(&lower, "harp") {
        (24, 103)
    } else if ["sitar", "koto", "shamisen", "balalaika"]
        .iter()
        .any(|word| contains_term(&lower, word))
    {
        (48, 91)
    } else if contains_term(&lower, "piano") {
        (21, 108)
    } else if contains_term(&lower, "harpsichord") {
        (29, 89)
    } else if contains_term(&lower, "clavichord") {
        (36, 84)
    } else if ["hammered dulcimer", "cimbalom", "celesta"]
        .iter()
        .any(|word| contains_term(&lower, word))
    {
        (36, 96)
    } else if contains_term(&lower, "violin") {
        (55, 103)
    } else if contains_term(&lower, "viola") {
        (48, 91)
    } else if contains_term(&lower, "cello") {
        (36, 84)
    } else if ["double bass", "contrabass"]
        .iter()
        .any(|word| contains_term(&lower, word))
    {
        (28, 67)
    } else if contains_term(&lower, "piccolo") {
        (74, 108)
    } else if contains_term(&lower, "bass clarinet") {
        (34, 79)
    } else if contains_term(&lower, "clarinet") {
        (50, 94)
    } else if contains_term(&lower, "soprano saxophone") {
        (56, 88)
    } else if contains_term(&lower, "alto saxophone") {
        (49, 80)
    } else if contains_term(&lower, "tenor saxophone") {
        (44, 75)
    } else if contains_term(&lower, "baritone saxophone") {
        (36, 68)
    } else if contains_term(&lower, "oboe") {
        (58, 91)
    } else if contains_term(&lower, "english horn") {
        (52, 81)
    } else if contains_term(&lower, "contrabassoon") {
        (22, 58)
    } else if contains_term(&lower, "bassoon") {
        (34, 75)
    } else if contains_term(&lower, "harmonica") {
        (48, 84)
    } else if [
        "accordion",
        "concertina",
        "bandoneon",
        "melodica",
        "harmonium",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        (36, 96)
    } else if contains_term(&lower, "trumpet") || contains_term(&lower, "cornet") {
        (54, 82)
    } else if contains_term(&lower, "french horn") {
        (34, 77)
    } else if contains_term(&lower, "trombone") {
        (40, 72)
    } else if contains_term(&lower, "tuba") {
        (28, 58)
    } else if contains_term(&lower, "organ") {
        (24, 108)
    } else if contains_term(&lower, "timpani") {
        (36, 57)
    } else {
        match family {
            "bowed_string" => (36, 96),
            "flute" => (60, 96),
            "reed" => (46, 94),
            "brass" => (40, 84),
            "free_reed" => (36, 96),
            "organ" => (24, 108),
            "membrane" => (36, 72),
            "metal" | "wood" => (36, 96),
            "noise_percussion" => (36, 84),
            _ => (36, 96),
        }
    }
}

pub fn family_from_name(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    if ["organ"].iter().any(|word| contains_term(&lower, word)) {
        "organ"
    } else if [
        "harmonica",
        "accordion",
        "concertina",
        "bandoneon",
        "melodica",
        "harmonium",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "free_reed"
    } else if [
        "flute",
        "recorder",
        "piccolo",
        "ocarina",
        "pan flute",
        "shakuhachi",
        "whistle",
        "ney",
        "quena",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "flute"
    } else if [
        "clarinet",
        "sax",
        "saxophone",
        "oboe",
        "bassoon",
        "contrabassoon",
        "english horn",
        "cor anglais",
        "bagpipe",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "reed"
    } else if [
        "trumpet",
        "trombone",
        "french horn",
        "horn",
        "tuba",
        "cornet",
        "euphonium",
        "flugelhorn",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "brass"
    } else if [
        "appalachian dulcimer",
        "mountain dulcimer",
        "banjo",
        "guitar",
        "harp",
        "lute",
        "ukulele",
        "mandolin",
        "mandola",
        "bouzouki",
        "oud",
        "sitar",
        "shamisen",
        "koto",
        "balalaika",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "plucked_string"
    } else if [
        "violin",
        "viola",
        "cello",
        "contrabass",
        "double bass",
        "bowed",
        "fiddle",
        "erhu",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "bowed_string"
    } else if [
        "piano",
        "hammered dulcimer",
        "clavichord",
        "harpsichord",
        "cimbalom",
        "celesta",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "struck_string"
    } else if [
        "drum", "tabla", "timpani", "snare", "tom", "bongo", "bongos", "conga", "djembe", "taiko",
        "kick",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "membrane"
    } else if [
        "bell",
        "cymbal",
        "gong",
        "triangle",
        "vibraphone",
        "glockenspiel",
        "chime",
        "chimes",
        "crotales",
        "steelpan",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "metal"
    } else if [
        "xylophone",
        "marimba",
        "woodblock",
        "wood block",
        "claves",
        "guiro",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "wood"
    } else if [
        "shaker",
        "maraca",
        "maracas",
        "tambourine",
        "cabasa",
        "rainstick",
    ]
    .iter()
    .any(|word| contains_term(&lower, word))
    {
        "noise_percussion"
    } else if ["synth", "synthesizer", "electronic"]
        .iter()
        .any(|word| contains_term(&lower, word))
    {
        "synthesizer"
    } else {
        "unknown"
    }
}

fn contains_term(name: &str, term: &str) -> bool {
    name.match_indices(term).any(|(start, _)| {
        let before = name[..start]
            .chars()
            .next_back()
            .map_or(true, |character| !character.is_ascii_alphanumeric());
        let end = start + term.len();
        let after = name[end..]
            .chars()
            .next()
            .map_or(true, |character| !character.is_ascii_alphanumeric());
        before && after
    })
}

fn learn_modes(
    attacks: &[&Region],
) -> (
    [f32; SYNTH_PARTIALS],
    [f32; SYNTH_PARTIALS],
    [f32; SYNTH_PARTIALS],
) {
    let mut ratios = [0.0_f32; SYNTH_PARTIALS];
    let mut amplitudes = [0.0_f32; SYNTH_PARTIALS];
    let mut decays = [0.0_f32; SYNTH_PARTIALS];
    let Some(region) = attacks
        .iter()
        .copied()
        .max_by_key(|region| region.samples.len())
    else {
        return (ratios, amplitudes, decays);
    };
    let fft_size = region.samples.len().clamp(256, 16_384).next_power_of_two();
    let mut buffer = vec![Complex::new(0.0_f32, 0.0); fft_size];
    for index in 0..fft_size.min(region.samples.len()) {
        let window = 0.5 - 0.5 * (TAU * index as f32 / (fft_size - 1).max(1) as f32).cos();
        buffer[index].re = region.samples[index] * window;
    }
    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(fft_size).process(&mut buffer);
    let magnitudes: Vec<f32> = buffer[..fft_size / 2]
        .iter()
        .map(|value| value.norm())
        .collect();
    let peak = magnitudes.iter().copied().fold(0.0_f32, f32::max).max(1e-9);
    let root =
        440.0 * 2.0_f32.powf((region.root_key as f32 - 69.0) / 12.0 + region.tune_cents / 1_200.0);
    let mut candidates = Vec::new();
    for bin in 2..magnitudes.len().saturating_sub(2) {
        let amplitude = magnitudes[bin];
        if amplitude < peak * 0.015
            || amplitude <= magnitudes[bin - 1]
            || amplitude < magnitudes[bin + 1]
        {
            continue;
        }
        let frequency = bin as f32 * region.sample_rate as f32 / fft_size as f32;
        let ratio = frequency / root.max(1e-6);
        if (0.45..=32.0).contains(&ratio) {
            candidates.push((ratio, amplitude / peak));
        }
    }
    candidates.sort_by(|left, right| right.1.partial_cmp(&left.1).unwrap_or(Ordering::Equal));
    let mut selected: Vec<(f32, f32)> = Vec::new();
    for candidate in candidates {
        if selected
            .iter()
            .all(|existing| (existing.0 - candidate.0).abs() > 0.035 * existing.0.max(1.0))
        {
            selected.push(candidate);
        }
        if selected.len() == SYNTH_PARTIALS {
            break;
        }
    }
    if selected.is_empty() {
        selected.push((1.0, 1.0));
    }
    selected.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap_or(Ordering::Equal));
    let amplitude_peak = selected
        .iter()
        .map(|candidate| candidate.1)
        .fold(0.0_f32, f32::max)
        .max(1e-9);
    for (index, (ratio, amplitude)) in selected.into_iter().enumerate() {
        ratios[index] = ratio;
        amplitudes[index] = (amplitude / amplitude_peak).clamp(0.0, 1.0);
        decays[index] = (region.decay_s.max(0.05) / ratio.sqrt().max(0.5)).clamp(0.02, 30.0);
    }
    (ratios, amplitudes, decays)
}

fn model_for_family(family: &str) -> SynthModel {
    match family {
        "plucked_string" => SynthModel::PluckedString,
        "bowed_string" => SynthModel::BowedString,
        "struck_string" => SynthModel::StruckString,
        "flute" => SynthModel::Wind,
        "reed" => SynthModel::Reed,
        "brass" => SynthModel::Brass,
        "free_reed" => SynthModel::FreeReed,
        "organ" => SynthModel::Organ,
        "membrane" => SynthModel::Membrane,
        "metal" | "wood" => SynthModel::Modal,
        "noise_percussion" => SynthModel::Noise,
        _ => SynthModel::Additive,
    }
}

fn median(values: impl Iterator<Item = f32>) -> f32 {
    let mut values: Vec<f32> = values.filter(|value| value.is_finite()).collect();
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    values[values.len() / 2]
}

fn derived_physical_profiles(
    name: &str,
    family: &str,
) -> (
    Option<KeyboardProfile>,
    Option<WindProfile>,
    Option<PercussionProfile>,
) {
    (
        KeyboardProfile::for_instrument(name, family),
        WindProfile::for_instrument(name, family),
        PercussionProfile::for_instrument(name, family),
    )
}

fn load_v6(bytes: &[u8]) -> io::Result<InstrumentBank> {
    let mut reader = Reader::new(bytes, 5);
    let name = reader.string()?;
    let family = reader.string()?;
    let default_articulation = reader.string()?;
    let polyphony = reader.u16()?;
    let synth = read_synth_v4(&mut reader)?;
    let string_profile = read_string_profile(&mut reader)?;
    let keyboard_profile = read_keyboard_profile(&mut reader)?;
    let wind_profile = read_wind_profile(&mut reader)?;
    let percussion_profile = read_percussion_profile(&mut reader)?;
    let regions = read_regions(&mut reader)?;
    let bank = InstrumentBank {
        name,
        family,
        default_articulation,
        polyphony,
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

fn load_v5(bytes: &[u8]) -> io::Result<InstrumentBank> {
    let mut reader = Reader::new(bytes, 5);
    let name = reader.string()?;
    let family = reader.string()?;
    let default_articulation = reader.string()?;
    let polyphony = reader.u16()?;
    let synth = read_synth_v4(&mut reader)?;
    let string_profile = read_string_profile(&mut reader)?;
    let regions = read_regions(&mut reader)?;
    let (keyboard_profile, wind_profile, percussion_profile) =
        derived_physical_profiles(&name, &family);
    let bank = InstrumentBank {
        name,
        family,
        default_articulation,
        polyphony,
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

fn load_v4(bytes: &[u8]) -> io::Result<InstrumentBank> {
    let mut reader = Reader::new(bytes, 5);
    let name = reader.string()?;
    let family = reader.string()?;
    let default_articulation = reader.string()?;
    let polyphony = reader.u16()?;
    let synth = read_synth_v4(&mut reader)?;
    let regions = read_regions(&mut reader)?;
    let string_profile = StringInstrumentProfile::for_instrument(&name, &family);
    let (keyboard_profile, wind_profile, percussion_profile) =
        derived_physical_profiles(&name, &family);
    let bank = InstrumentBank {
        name,
        family,
        default_articulation,
        polyphony,
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

fn load_v3(bytes: &[u8]) -> io::Result<InstrumentBank> {
    let mut reader = Reader::new(bytes, 5);
    let name = reader.string()?;
    let family = reader.string()?;
    let default_articulation = reader.string()?;
    let polyphony = reader.u16()?;
    let synth = read_synth_v3(&mut reader)?;
    let regions = read_regions(&mut reader)?;
    let string_profile = StringInstrumentProfile::for_instrument(&name, &family);
    let (keyboard_profile, wind_profile, percussion_profile) =
        derived_physical_profiles(&name, &family);
    let bank = InstrumentBank {
        name,
        family,
        default_articulation,
        polyphony,
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

fn load_v2(bytes: &[u8]) -> io::Result<InstrumentBank> {
    let mut reader = Reader::new(bytes, 5);
    let name = reader.string()?;
    let default_articulation = reader.string()?;
    let polyphony = reader.u16()?;
    let regions = read_regions(&mut reader)?;
    let family = family_from_name(&name).to_string();
    let synth = SynthConfig::learn(&family, &regions);
    let string_profile = StringInstrumentProfile::for_instrument(&name, &family);
    let (keyboard_profile, wind_profile, percussion_profile) =
        derived_physical_profiles(&name, &family);
    let bank = InstrumentBank {
        name,
        family,
        default_articulation,
        polyphony,
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

fn load_v1(bytes: &[u8]) -> io::Result<InstrumentBank> {
    let mut reader = Reader::new(bytes, 5);
    let name = reader.string()?;
    let sample_rate = reader.u32()?;
    let root_hz = reader.f32()?;
    let attack_s = reader.f32()?;
    let decay_s = reader.f32()?;
    let sustain = reader.f32()?;
    let release_s = reader.f32()?;
    let noise = reader.f32()?;
    let mut partials = [0.0; PARTIALS];
    for partial in partials.iter_mut().take(32) {
        *partial = reader.f32()?;
    }
    let samples = reader.samples()?;
    let root_key = if root_hz.is_finite() && root_hz > 0.0 {
        (69.0 + 12.0 * (root_hz / 440.0).log2())
            .round()
            .clamp(0.0, 127.0) as u8
    } else {
        69
    };
    let region = Region {
        name: name.clone(),
        articulation: "sustain".into(),
        trigger: Trigger::Attack,
        root_key,
        key_lo: 0,
        key_hi: 127,
        vel_lo: 1,
        vel_hi: 127,
        round_robin: 1,
        sample_rate,
        tune_cents: 0.0,
        gain_db: 0.0,
        attack_s,
        decay_s,
        sustain,
        release_s,
        noise,
        loop_start: None,
        loop_end: None,
        partials,
        samples,
    };
    let family = family_from_name(&name).to_string();
    let synth = SynthConfig::learn(&family, std::slice::from_ref(&region));
    let string_profile = StringInstrumentProfile::for_instrument(&name, &family);
    let (keyboard_profile, wind_profile, percussion_profile) =
        derived_physical_profiles(&name, &family);
    let bank = InstrumentBank {
        name,
        family,
        default_articulation: "sustain".into(),
        polyphony: 64,
        synth,
        string_profile,
        keyboard_profile,
        wind_profile,
        percussion_profile,
        regions: vec![region],
    };
    bank.validate()?;
    Ok(bank)
}

fn read_regions(reader: &mut Reader<'_>) -> io::Result<Vec<Region>> {
    let count = reader.u32()? as usize;
    if count == 0 || count > MAX_REGIONS {
        return Err(invalid_data("invalid region count"));
    }
    let mut regions = Vec::with_capacity(count);
    for _ in 0..count {
        let name = reader.string()?;
        let articulation = reader.string()?;
        let trigger = match reader.u8()? {
            0 => Trigger::Attack,
            1 => Trigger::Release,
            _ => return Err(invalid_data("invalid trigger")),
        };
        let root_key = reader.u8()?;
        let key_lo = reader.u8()?;
        let key_hi = reader.u8()?;
        let vel_lo = reader.u8()?;
        let vel_hi = reader.u8()?;
        let round_robin = reader.u8()?;
        let sample_rate = reader.u32()?;
        let tune_cents = reader.f32()?;
        let gain_db = reader.f32()?;
        let attack_s = reader.f32()?;
        let decay_s = reader.f32()?;
        let sustain = reader.f32()?;
        let release_s = reader.f32()?;
        let noise = reader.f32()?;
        let loop_start = option_index(reader.u32()?);
        let loop_end = option_index(reader.u32()?);
        let mut partials = [0.0; PARTIALS];
        for partial in &mut partials {
            *partial = reader.f32()?;
        }
        let samples = reader.samples()?;
        regions.push(Region {
            name,
            articulation,
            trigger,
            root_key,
            key_lo,
            key_hi,
            vel_lo,
            vel_hi,
            round_robin,
            sample_rate,
            tune_cents,
            gain_db,
            attack_s,
            decay_s,
            sustain,
            release_s,
            noise,
            loop_start,
            loop_end,
            partials,
            samples,
        });
    }
    Ok(regions)
}

fn put_region(bytes: &mut Vec<u8>, region: &Region) -> io::Result<()> {
    put_string(bytes, &region.name)?;
    put_string(bytes, &region.articulation)?;
    bytes.push(match region.trigger {
        Trigger::Attack => 0,
        Trigger::Release => 1,
    });
    bytes.extend([
        region.root_key,
        region.key_lo,
        region.key_hi,
        region.vel_lo,
        region.vel_hi,
        region.round_robin,
    ]);
    bytes.extend(region.sample_rate.to_le_bytes());
    for value in [
        region.tune_cents,
        region.gain_db,
        region.attack_s,
        region.decay_s,
        region.sustain,
        region.release_s,
        region.noise,
    ] {
        bytes.extend(value.to_le_bytes());
    }
    bytes.extend(region.loop_start.unwrap_or(u32::MAX).to_le_bytes());
    bytes.extend(region.loop_end.unwrap_or(u32::MAX).to_le_bytes());
    for partial in region.partials {
        bytes.extend(partial.to_le_bytes());
    }
    bytes.extend(
        u32::try_from(region.samples.len())
            .map_err(|_| invalid_data("sample too large"))?
            .to_le_bytes(),
    );
    for sample in &region.samples {
        bytes.extend(sample.to_le_bytes());
    }
    Ok(())
}

fn put_synth(bytes: &mut Vec<u8>, synth: &SynthConfig) {
    bytes.push(synth.model as u8);
    for value in [
        synth.sample_level,
        synth.synth_level,
        synth.master_gain,
        synth.attack_s,
        synth.decay_s,
        synth.sustain,
        synth.release_s,
        synth.noise,
        synth.brightness,
        synth.damping,
        synth.pluck_position,
    ] {
        bytes.extend(value.to_le_bytes());
    }
    for harmonic in synth.harmonics {
        bytes.extend(harmonic.to_le_bytes());
    }
    for ratio in synth.mode_ratios {
        bytes.extend(ratio.to_le_bytes());
    }
    for amplitude in synth.mode_amplitudes {
        bytes.extend(amplitude.to_le_bytes());
    }
    for decay in synth.mode_decays {
        bytes.extend(decay.to_le_bytes());
    }
    for value in [
        synth.vibrato_rate_hz,
        synth.vibrato_depth_cents,
        synth.transient_mix,
    ] {
        bytes.extend(value.to_le_bytes());
    }
}

fn put_string_profile(bytes: &mut Vec<u8>, profile: Option<&StringInstrumentProfile>) {
    let Some(profile) = profile else {
        bytes.push(0);
        return;
    };
    bytes.extend([
        1,
        profile.kind as u8,
        profile.body as u8,
        profile.material as u8,
        profile.string_count,
        profile.course_voices,
        profile.octave_course_strings,
        profile.open_string_count,
        profile.range_low,
        profile.range_high,
    ]);
    bytes.extend(profile.open_notes);
    for value in [
        profile.scale_length_m,
        profile.tension_scale,
        profile.stiffness,
        profile.high_frequency_loss,
        profile.base_damping,
        profile.course_detune_cents,
        profile.bridge_coupling,
        profile.direct_mix,
        profile.body_mix,
        profile.sympathetic_gain,
        profile.nonlinear_tension,
        profile.pick_noise,
        profile.pluck_position,
        profile.decay_s,
        profile.master_gain,
        profile.brightness,
        profile.transient_mix,
    ] {
        bytes.extend(value.to_le_bytes());
    }
}

fn read_string_profile(reader: &mut Reader<'_>) -> io::Result<Option<StringInstrumentProfile>> {
    match reader.u8()? {
        0 => return Ok(None),
        1 => {}
        _ => return Err(invalid_data("invalid string profile marker")),
    }
    let kind = StringInstrumentKind::from_byte(reader.u8()?)
        .ok_or_else(|| invalid_data("unknown string instrument kind"))?;
    let body = ResonatorBody::from_byte(reader.u8()?)
        .ok_or_else(|| invalid_data("unknown string resonator body"))?;
    let material = StringMaterial::from_byte(reader.u8()?)
        .ok_or_else(|| invalid_data("unknown string material"))?;
    let string_count = reader.u8()?;
    let course_voices = reader.u8()?;
    let octave_course_strings = reader.u8()?;
    let open_string_count = reader.u8()?;
    let range_low = reader.u8()?;
    let range_high = reader.u8()?;
    let mut open_notes = [0; MAX_PROFILE_STRINGS];
    for note in &mut open_notes {
        *note = reader.u8()?;
    }
    let profile = StringInstrumentProfile {
        kind,
        body,
        material,
        string_count,
        course_voices,
        octave_course_strings,
        open_string_count,
        open_notes,
        range_low,
        range_high,
        scale_length_m: reader.f32()?,
        tension_scale: reader.f32()?,
        stiffness: reader.f32()?,
        high_frequency_loss: reader.f32()?,
        base_damping: reader.f32()?,
        course_detune_cents: reader.f32()?,
        bridge_coupling: reader.f32()?,
        direct_mix: reader.f32()?,
        body_mix: reader.f32()?,
        sympathetic_gain: reader.f32()?,
        nonlinear_tension: reader.f32()?,
        pick_noise: reader.f32()?,
        pluck_position: reader.f32()?,
        decay_s: reader.f32()?,
        master_gain: reader.f32()?,
        brightness: reader.f32()?,
        transient_mix: reader.f32()?,
    };
    if !profile.validate() {
        return Err(invalid_data("string construction profile out of range"));
    }
    Ok(Some(profile))
}

fn put_keyboard_profile(bytes: &mut Vec<u8>, profile: Option<&KeyboardProfile>) {
    let Some(profile) = profile else {
        bytes.push(0);
        return;
    };
    bytes.extend([
        1,
        profile.kind as u8,
        profile.exciter as u8,
        profile.range_low,
        profile.range_high,
        profile.strings_per_note,
    ]);
    for value in [
        profile.unison_detune_cents,
        profile.strike_position,
        profile.hammer_hardness,
        profile.hammer_noise,
        profile.string_stiffness,
        profile.base_damping,
        profile.direct_mix,
        profile.soundboard_mix,
        profile.pedal_coupling,
        profile.decay_s,
        profile.release_s,
        profile.brightness,
    ] {
        bytes.extend(value.to_le_bytes());
    }
}

fn read_keyboard_profile(reader: &mut Reader<'_>) -> io::Result<Option<KeyboardProfile>> {
    match reader.u8()? {
        0 => return Ok(None),
        1 => {}
        _ => return Err(invalid_data("invalid keyboard profile marker")),
    }
    let profile = KeyboardProfile {
        kind: KeyboardKind::from_byte(reader.u8()?)
            .ok_or_else(|| invalid_data("unknown keyboard kind"))?,
        exciter: KeyboardExciter::from_byte(reader.u8()?)
            .ok_or_else(|| invalid_data("unknown keyboard exciter"))?,
        range_low: reader.u8()?,
        range_high: reader.u8()?,
        strings_per_note: reader.u8()?,
        unison_detune_cents: reader.f32()?,
        strike_position: reader.f32()?,
        hammer_hardness: reader.f32()?,
        hammer_noise: reader.f32()?,
        string_stiffness: reader.f32()?,
        base_damping: reader.f32()?,
        direct_mix: reader.f32()?,
        soundboard_mix: reader.f32()?,
        pedal_coupling: reader.f32()?,
        decay_s: reader.f32()?,
        release_s: reader.f32()?,
        brightness: reader.f32()?,
    };
    if !profile.validate() {
        return Err(invalid_data("keyboard construction profile out of range"));
    }
    Ok(Some(profile))
}

fn put_wind_profile(bytes: &mut Vec<u8>, profile: Option<&WindProfile>) {
    let Some(profile) = profile else {
        bytes.push(0);
        return;
    };
    bytes.extend([
        1,
        profile.exciter as u8,
        profile.bore as u8,
        profile.range_low,
        profile.range_high,
    ]);
    for value in [
        profile.reflection,
        profile.breath_pressure,
        profile.nonlinearity,
        profile.breath_noise,
        profile.bore_loss,
        profile.bell_mix,
    ] {
        bytes.extend(value.to_le_bytes());
    }
    for value in profile.formant_hz {
        bytes.extend(value.to_le_bytes());
    }
    for value in profile.formant_gain {
        bytes.extend(value.to_le_bytes());
    }
    bytes.extend(profile.brightness.to_le_bytes());
}

fn read_wind_profile(reader: &mut Reader<'_>) -> io::Result<Option<WindProfile>> {
    match reader.u8()? {
        0 => return Ok(None),
        1 => {}
        _ => return Err(invalid_data("invalid wind profile marker")),
    }
    let exciter =
        WindExciter::from_byte(reader.u8()?).ok_or_else(|| invalid_data("unknown wind exciter"))?;
    let bore =
        BoreShape::from_byte(reader.u8()?).ok_or_else(|| invalid_data("unknown bore shape"))?;
    let range_low = reader.u8()?;
    let range_high = reader.u8()?;
    let reflection = reader.f32()?;
    let breath_pressure = reader.f32()?;
    let nonlinearity = reader.f32()?;
    let breath_noise = reader.f32()?;
    let bore_loss = reader.f32()?;
    let bell_mix = reader.f32()?;
    let mut formant_hz = [0.0; 3];
    let mut formant_gain = [0.0; 3];
    for value in &mut formant_hz {
        *value = reader.f32()?;
    }
    for value in &mut formant_gain {
        *value = reader.f32()?;
    }
    let profile = WindProfile {
        exciter,
        bore,
        range_low,
        range_high,
        reflection,
        breath_pressure,
        nonlinearity,
        breath_noise,
        bore_loss,
        bell_mix,
        formant_hz,
        formant_gain,
        brightness: reader.f32()?,
    };
    if !profile.validate() {
        return Err(invalid_data("wind construction profile out of range"));
    }
    Ok(Some(profile))
}

fn put_percussion_profile(bytes: &mut Vec<u8>, profile: Option<&PercussionProfile>) {
    let Some(profile) = profile else {
        bytes.push(0);
        return;
    };
    bytes.extend([
        1,
        profile.resonator as u8,
        profile.exciter as u8,
        profile.range_low,
        profile.range_high,
    ]);
    for value in [
        profile.strike_position,
        profile.hardness,
        profile.damping,
        profile.transient_noise,
        profile.shell_noise,
        profile.resonator_mix,
        profile.decay_s,
        profile.brightness,
    ] {
        bytes.extend(value.to_le_bytes());
    }
}

fn read_percussion_profile(reader: &mut Reader<'_>) -> io::Result<Option<PercussionProfile>> {
    match reader.u8()? {
        0 => return Ok(None),
        1 => {}
        _ => return Err(invalid_data("invalid percussion profile marker")),
    }
    let profile = PercussionProfile {
        resonator: PercussionResonator::from_byte(reader.u8()?)
            .ok_or_else(|| invalid_data("unknown percussion resonator"))?,
        exciter: PercussionExciter::from_byte(reader.u8()?)
            .ok_or_else(|| invalid_data("unknown percussion exciter"))?,
        range_low: reader.u8()?,
        range_high: reader.u8()?,
        strike_position: reader.f32()?,
        hardness: reader.f32()?,
        damping: reader.f32()?,
        transient_noise: reader.f32()?,
        shell_noise: reader.f32()?,
        resonator_mix: reader.f32()?,
        decay_s: reader.f32()?,
        brightness: reader.f32()?,
    };
    if !profile.validate() {
        return Err(invalid_data("percussion construction profile out of range"));
    }
    Ok(Some(profile))
}

fn read_synth_v3(reader: &mut Reader<'_>) -> io::Result<SynthConfig> {
    let model = SynthModel::from_byte(reader.u8()?)?;
    let sample_level = reader.f32()?;
    let synth_level = reader.f32()?;
    let master_gain = reader.f32()?;
    let attack_s = reader.f32()?;
    let decay_s = reader.f32()?;
    let sustain = reader.f32()?;
    let release_s = reader.f32()?;
    let noise = reader.f32()?;
    let brightness = reader.f32()?;
    let damping = reader.f32()?;
    let pluck_position = reader.f32()?;
    let mut harmonics = [0.0; SYNTH_PARTIALS];
    for harmonic in &mut harmonics {
        *harmonic = reader.f32()?;
    }
    let defaults = SynthConfig::default();
    let config = SynthConfig {
        model,
        sample_level,
        synth_level,
        master_gain,
        attack_s,
        decay_s,
        sustain,
        release_s,
        noise,
        brightness,
        damping,
        pluck_position,
        harmonics,
        mode_ratios: defaults.mode_ratios,
        mode_amplitudes: defaults.mode_amplitudes,
        mode_decays: defaults.mode_decays,
        vibrato_rate_hz: defaults.vibrato_rate_hz,
        vibrato_depth_cents: defaults.vibrato_depth_cents,
        transient_mix: defaults.transient_mix,
    };
    config.validate()?;
    Ok(config)
}

fn read_synth_v4(reader: &mut Reader<'_>) -> io::Result<SynthConfig> {
    let mut config = read_synth_v3(reader)?;
    for ratio in &mut config.mode_ratios {
        *ratio = reader.f32()?;
    }
    for amplitude in &mut config.mode_amplitudes {
        *amplitude = reader.f32()?;
    }
    for decay in &mut config.mode_decays {
        *decay = reader.f32()?;
    }
    config.vibrato_rate_hz = reader.f32()?;
    config.vibrato_depth_cents = reader.f32()?;
    config.transient_mix = reader.f32()?;
    config.validate()?;
    Ok(config)
}

fn validate_region(region: &Region) -> io::Result<()> {
    if region.name.len() > 1_000_000
        || region.articulation.is_empty()
        || region.articulation.len() > 1_000_000
        || region.root_key > 127
        || region.key_lo > region.key_hi
        || region.key_hi > 127
        || region.vel_lo == 0
        || region.vel_lo > region.vel_hi
        || region.vel_hi > 127
        || region.round_robin == 0
        || !(8_000..=384_000).contains(&region.sample_rate)
        || region.samples.is_empty()
        || region.samples.len() > MAX_SAMPLES_PER_REGION
    {
        return Err(invalid_data("invalid region metadata"));
    }
    let values = [
        region.tune_cents,
        region.gain_db,
        region.attack_s,
        region.decay_s,
        region.sustain,
        region.release_s,
        region.noise,
    ];
    if values.iter().any(|value| !value.is_finite())
        || region.samples.iter().any(|value| !value.is_finite())
        || region.partials.iter().any(|value| !value.is_finite())
    {
        return Err(invalid_data("non-finite region data"));
    }
    match (region.loop_start, region.loop_end) {
        (None, None) => {}
        (Some(start), Some(end))
            if start < end && end as usize <= region.samples.len() && end - start >= 2 => {}
        _ => return Err(invalid_data("invalid sample loop")),
    }
    Ok(())
}

fn put_string(bytes: &mut Vec<u8>, value: &str) -> io::Result<()> {
    bytes.extend(
        u32::try_from(value.len())
            .map_err(|_| invalid_data("string too large"))?
            .to_le_bytes(),
    );
    bytes.extend(value.as_bytes());
    Ok(())
}

fn option_index(value: u32) -> Option<u32> {
    (value != u32::MAX).then_some(value)
}

fn invalid_data(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8], position: usize) -> Self {
        Self { bytes, position }
    }

    fn take(&mut self, count: usize) -> io::Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(count)
            .ok_or_else(|| invalid_data("profile offset overflow"))?;
        if end > self.bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "truncated .rsti",
            ));
        }
        let value = &self.bytes[self.position..end];
        self.position = end;
        Ok(value)
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn u8(&mut self) -> io::Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> io::Result<u16> {
        let bytes: [u8; 2] = self
            .take(2)?
            .try_into()
            .map_err(|_| invalid_data("bad u16"))?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn u32(&mut self) -> io::Result<u32> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| invalid_data("bad u32"))?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn f32(&mut self) -> io::Result<f32> {
        Ok(f32::from_bits(self.u32()?))
    }

    fn string(&mut self) -> io::Result<String> {
        let count = self.u32()? as usize;
        if count > 1_000_000 || count > self.remaining() {
            return Err(invalid_data("invalid string length"));
        }
        String::from_utf8(self.take(count)?.to_vec()).map_err(|_| invalid_data("invalid UTF-8"))
    }

    fn samples(&mut self) -> io::Result<Vec<f32>> {
        let count = self.u32()? as usize;
        let byte_count = count
            .checked_mul(std::mem::size_of::<f32>())
            .ok_or_else(|| invalid_data("sample size overflow"))?;
        if count == 0 || count > MAX_SAMPLES_PER_REGION || byte_count > self.remaining() {
            return Err(invalid_data("invalid sample count"));
        }
        let bytes = self.take(byte_count)?;
        let mut samples = Vec::with_capacity(count);
        for chunk in bytes.chunks_exact(4) {
            samples.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
        Ok(samples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(frequency: f32, seconds: f32) -> Wav {
        let sample_rate = 44_100;
        let count = (sample_rate as f32 * seconds) as usize;
        let samples = (0..count)
            .map(|index| (TAU * frequency * index as f32 / sample_rate as f32).sin())
            .collect();
        Wav {
            sample_rate,
            samples,
        }
    }

    #[test]
    fn midi_pitch_is_stable() {
        let region = analyze_region("A4", &sine(440.0, 1.0), Some(69));
        assert_eq!(region.root_key, 69);
        assert!(region.tune_cents.abs() < 2.0, "{} cents", region.tune_cents);
    }

    #[test]
    fn single_sample_covers_the_keyboard() {
        let bank = InstrumentBank::single("Banjo", &sine(220.0, 0.2));
        assert_eq!(bank.regions[0].key_lo, 0);
        assert_eq!(bank.regions[0].key_hi, 127);
        assert_eq!(bank.synth.model, SynthModel::Banjo);
    }

    #[test]
    fn names_do_not_confuse_flutes_lutes_or_horns() {
        assert_eq!(family_from_name("Concert Flute"), "flute");
        assert_eq!(family_from_name("Renaissance Lute"), "plucked_string");
        assert_eq!(family_from_name("English Horn"), "reed");
        assert_eq!(family_from_name("French Horn"), "brass");
    }

    #[test]
    fn version_six_memory_round_trip_matches_file_loader() {
        let bank = InstrumentBank::generator("Five-string Banjo");
        let bytes = bank.to_bytes().unwrap();
        assert_eq!(&bytes[..5], b"RSTI\x06");
        let loaded = InstrumentBank::from_bytes(&bytes).unwrap();
        assert_eq!(loaded, bank);
    }

    #[test]
    fn version_six_round_trip_preserves_all_physical_profiles() {
        for name in ["Banjo", "Concert Grand Piano", "Clarinet", "Timpani"] {
            let bank = InstrumentBank::generator(name);
            let path = std::env::temp_dir().join(format!(
                "ruststrument-profile-{}-{}-{}.rsti",
                std::process::id(),
                std::thread::current().name().unwrap_or("test"),
                name.replace(' ', "-")
            ));
            bank.save(&path).unwrap();
            let loaded = InstrumentBank::load(&path).unwrap();
            let _ = fs::remove_file(path);
            assert_eq!(loaded.name, bank.name);
            assert_eq!(loaded.synth.model, bank.synth.model);
            assert_eq!(loaded.string_profile, bank.string_profile);
            assert_eq!(loaded.keyboard_profile, bank.keyboard_profile);
            assert_eq!(loaded.wind_profile, bank.wind_profile);
            assert_eq!(loaded.percussion_profile, bank.percussion_profile);
            assert_eq!(loaded.regions.len(), bank.regions.len());
        }
    }

    #[test]
    fn generator_catalog_covers_major_families_and_uses_no_recorded_audio() {
        assert_eq!(GENERATOR_CATALOG.len(), 94);
        let expected_families = [
            "plucked_string",
            "bowed_string",
            "struck_string",
            "flute",
            "reed",
            "free_reed",
            "brass",
            "organ",
            "membrane",
            "metal",
            "wood",
            "noise_percussion",
            "synthesizer",
        ];
        for family in expected_families {
            assert!(
                GENERATOR_CATALOG
                    .iter()
                    .any(|name| family_from_name(name) == family),
                "missing family {family}"
            );
        }
        for name in GENERATOR_CATALOG {
            let bank = InstrumentBank::generator(name);
            assert_ne!(bank.family, "unknown", "{name}");
            assert_eq!(bank.synth.sample_level, 0.0, "{name}");
            assert_eq!(bank.synth.synth_level, 1.0, "{name}");
            assert!(!bank.articulations().is_empty(), "{name}");
            assert!(bank
                .regions
                .iter()
                .flat_map(|region| &region.samples)
                .all(|sample| *sample == 0.0));
            bank.validate().unwrap();
        }
    }
}
