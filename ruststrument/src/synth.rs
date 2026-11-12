use crate::{
    physical::{
        BoreShape, KeyboardExciter, KeyboardKind, KeyboardProfile, PercussionProfile,
        PercussionResonator, WindExciter, WindProfile,
    },
    profile::{InstrumentBank, SynthConfig, SynthModel, Trigger},
    stringed::{ResonatorBody, StringInstrumentKind, StringInstrumentProfile, StringMaterial},
    technique,
    wav::Wav,
};
use std::io;

#[derive(Clone, Debug)]
struct Voice {
    id: u64,
    note: u8,
    midi_velocity: u8,
    region: usize,
    trigger: Trigger,
    position: f64,
    step: f64,
    gain: f32,
    released: bool,
    release_age: usize,
    age: usize,
    auto_release: Option<usize>,
    pending_release_sample: bool,
    phase: f64,
    phase_increment: f64,
    velocity: f32,
    contact_hardness: f32,
    head_strike: f32,
    waveguide: Vec<f32>,
    waveguide_position: usize,
    waveguide_fraction: f32,
    secondary_waveguide: Vec<f32>,
    secondary_waveguide_position: usize,
    tertiary_waveguide: Vec<f32>,
    tertiary_waveguide_position: usize,
    waveguide_filter: f32,
    secondary_waveguide_filter: f32,
    tertiary_waveguide_filter: f32,
    last_string_output: f32,
    resonator_y1: [f32; 16],
    resonator_y2: [f32; 16],
    noise_state: u64,
    filtered_noise: f32,
    pick_filter: f32,
    captured_lowpass: f32,
    captured_highpass_lowpass_2: f32,
    captured_highpass_lowpass_3: f32,
    captured_previous: f32,
    bridge_notches: [BiquadState; 3],
    head_feedback: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct BiquadState {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

#[derive(Clone, Debug, Default)]
struct SharedStringState {
    y1: [f32; 16],
    y2: [f32; 16],
    last_drive: f32,
}

#[derive(Clone, Copy)]
struct BodyMode {
    frequency_hz: f32,
    gain: f32,
    decay_s: f32,
}

#[derive(Clone, Copy)]
struct WaveguideExcitation<'a> {
    model: SynthModel,
    harmonics: &'a [f32; 16],
    contact_hardness: f32,
    string_profile: Option<&'a StringInstrumentProfile>,
}

// These are compact radiating-body approximations, not free-running tone
// generators. They are driven by bridge motion and decay rapidly when the
// string stops delivering energy.
// A banjo head has a rising modal density rather than the sparse, low-pass
// response of a wooden guitar plate. The perceptually important broad
// admittance features are represented by the dense 500--800 Hz group and the
// shorter-lived bridge group around 3.5 kHz. These modes are driven by string
// force: they do not free-run as oscillators.
const BANJO_BODY: [BodyMode; 16] = [
    BodyMode {
        frequency_hz: 265.0,
        gain: 0.07,
        decay_s: 0.090,
    },
    BodyMode {
        frequency_hz: 410.0,
        gain: 0.11,
        decay_s: 0.078,
    },
    BodyMode {
        frequency_hz: 515.0,
        gain: 0.30,
        decay_s: 0.069,
    },
    BodyMode {
        frequency_hz: 625.0,
        gain: 0.38,
        decay_s: 0.061,
    },
    BodyMode {
        frequency_hz: 755.0,
        gain: 0.31,
        decay_s: 0.054,
    },
    BodyMode {
        frequency_hz: 920.0,
        gain: 0.16,
        decay_s: 0.047,
    },
    BodyMode {
        frequency_hz: 1_110.0,
        gain: 0.13,
        decay_s: 0.041,
    },
    BodyMode {
        frequency_hz: 1_360.0,
        gain: 0.15,
        decay_s: 0.036,
    },
    BodyMode {
        frequency_hz: 1_670.0,
        gain: 0.12,
        decay_s: 0.032,
    },
    BodyMode {
        frequency_hz: 2_050.0,
        gain: 0.14,
        decay_s: 0.028,
    },
    BodyMode {
        frequency_hz: 2_470.0,
        gain: 0.16,
        decay_s: 0.024,
    },
    BodyMode {
        frequency_hz: 2_880.0,
        gain: 0.13,
        decay_s: 0.021,
    },
    BodyMode {
        frequency_hz: 3_300.0,
        gain: 0.18,
        decay_s: 0.019,
    },
    BodyMode {
        frequency_hz: 3_620.0,
        gain: 0.25,
        decay_s: 0.017,
    },
    BodyMode {
        frequency_hz: 4_250.0,
        gain: 0.16,
        decay_s: 0.015,
    },
    BodyMode {
        frequency_hz: 5_180.0,
        gain: 0.09,
        decay_s: 0.013,
    },
];

const GUITAR_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 95.0,
        gain: 0.24,
        decay_s: 0.42,
    },
    BodyMode {
        frequency_hz: 190.0,
        gain: 0.34,
        decay_s: 0.36,
    },
    BodyMode {
        frequency_hz: 365.0,
        gain: 0.22,
        decay_s: 0.30,
    },
    BodyMode {
        frequency_hz: 720.0,
        gain: 0.20,
        decay_s: 0.22,
    },
    BodyMode {
        frequency_hz: 1_100.0,
        gain: 0.18,
        decay_s: 0.18,
    },
    BodyMode {
        frequency_hz: 1_850.0,
        gain: 0.14,
        decay_s: 0.12,
    },
    BodyMode {
        frequency_hz: 2_800.0,
        gain: 0.10,
        decay_s: 0.080,
    },
    BodyMode {
        frequency_hz: 4_200.0,
        gain: 0.06,
        decay_s: 0.050,
    },
];

const CLASSICAL_GUITAR_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 92.0,
        gain: 0.30,
        decay_s: 0.52,
    },
    BodyMode {
        frequency_hz: 178.0,
        gain: 0.38,
        decay_s: 0.44,
    },
    BodyMode {
        frequency_hz: 285.0,
        gain: 0.30,
        decay_s: 0.37,
    },
    BodyMode {
        frequency_hz: 515.0,
        gain: 0.22,
        decay_s: 0.29,
    },
    BodyMode {
        frequency_hz: 890.0,
        gain: 0.18,
        decay_s: 0.22,
    },
    BodyMode {
        frequency_hz: 1_480.0,
        gain: 0.13,
        decay_s: 0.16,
    },
    BodyMode {
        frequency_hz: 2_350.0,
        gain: 0.08,
        decay_s: 0.10,
    },
    BodyMode {
        frequency_hz: 3_450.0,
        gain: 0.04,
        decay_s: 0.065,
    },
];

const ELECTRIC_PICKUP_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 115.0,
        gain: 0.07,
        decay_s: 0.35,
    },
    BodyMode {
        frequency_hz: 240.0,
        gain: 0.08,
        decay_s: 0.30,
    },
    BodyMode {
        frequency_hz: 720.0,
        gain: 0.10,
        decay_s: 0.22,
    },
    BodyMode {
        frequency_hz: 1_450.0,
        gain: 0.16,
        decay_s: 0.16,
    },
    BodyMode {
        frequency_hz: 2_350.0,
        gain: 0.20,
        decay_s: 0.12,
    },
    BodyMode {
        frequency_hz: 3_650.0,
        gain: 0.16,
        decay_s: 0.085,
    },
    BodyMode {
        frequency_hz: 5_100.0,
        gain: 0.08,
        decay_s: 0.060,
    },
    BodyMode {
        frequency_hz: 6_700.0,
        gain: 0.03,
        decay_s: 0.040,
    },
];

const MANDOLIN_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 155.0,
        gain: 0.24,
        decay_s: 0.31,
    },
    BodyMode {
        frequency_hz: 315.0,
        gain: 0.34,
        decay_s: 0.27,
    },
    BodyMode {
        frequency_hz: 520.0,
        gain: 0.30,
        decay_s: 0.22,
    },
    BodyMode {
        frequency_hz: 790.0,
        gain: 0.25,
        decay_s: 0.18,
    },
    BodyMode {
        frequency_hz: 1_180.0,
        gain: 0.20,
        decay_s: 0.14,
    },
    BodyMode {
        frequency_hz: 1_920.0,
        gain: 0.16,
        decay_s: 0.10,
    },
    BodyMode {
        frequency_hz: 3_050.0,
        gain: 0.13,
        decay_s: 0.070,
    },
    BodyMode {
        frequency_hz: 4_650.0,
        gain: 0.08,
        decay_s: 0.045,
    },
];

const UKULELE_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 175.0,
        gain: 0.27,
        decay_s: 0.34,
    },
    BodyMode {
        frequency_hz: 355.0,
        gain: 0.36,
        decay_s: 0.29,
    },
    BodyMode {
        frequency_hz: 545.0,
        gain: 0.25,
        decay_s: 0.24,
    },
    BodyMode {
        frequency_hz: 930.0,
        gain: 0.20,
        decay_s: 0.18,
    },
    BodyMode {
        frequency_hz: 1_480.0,
        gain: 0.14,
        decay_s: 0.13,
    },
    BodyMode {
        frequency_hz: 2_350.0,
        gain: 0.09,
        decay_s: 0.090,
    },
    BodyMode {
        frequency_hz: 3_550.0,
        gain: 0.055,
        decay_s: 0.060,
    },
    BodyMode {
        frequency_hz: 5_000.0,
        gain: 0.025,
        decay_s: 0.040,
    },
];

const LUTE_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 105.0,
        gain: 0.29,
        decay_s: 0.48,
    },
    BodyMode {
        frequency_hz: 205.0,
        gain: 0.37,
        decay_s: 0.41,
    },
    BodyMode {
        frequency_hz: 390.0,
        gain: 0.27,
        decay_s: 0.34,
    },
    BodyMode {
        frequency_hz: 680.0,
        gain: 0.21,
        decay_s: 0.26,
    },
    BodyMode {
        frequency_hz: 1_080.0,
        gain: 0.15,
        decay_s: 0.20,
    },
    BodyMode {
        frequency_hz: 1_720.0,
        gain: 0.10,
        decay_s: 0.14,
    },
    BodyMode {
        frequency_hz: 2_650.0,
        gain: 0.06,
        decay_s: 0.09,
    },
    BodyMode {
        frequency_hz: 3_900.0,
        gain: 0.03,
        decay_s: 0.055,
    },
];

const HARP_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 72.0,
        gain: 0.20,
        decay_s: 1.40,
    },
    BodyMode {
        frequency_hz: 145.0,
        gain: 0.28,
        decay_s: 1.15,
    },
    BodyMode {
        frequency_hz: 285.0,
        gain: 0.31,
        decay_s: 0.92,
    },
    BodyMode {
        frequency_hz: 510.0,
        gain: 0.25,
        decay_s: 0.72,
    },
    BodyMode {
        frequency_hz: 880.0,
        gain: 0.19,
        decay_s: 0.54,
    },
    BodyMode {
        frequency_hz: 1_470.0,
        gain: 0.14,
        decay_s: 0.38,
    },
    BodyMode {
        frequency_hz: 2_430.0,
        gain: 0.09,
        decay_s: 0.24,
    },
    BodyMode {
        frequency_hz: 3_900.0,
        gain: 0.05,
        decay_s: 0.14,
    },
];

const JAWARI_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 165.0,
        gain: 0.22,
        decay_s: 0.34,
    },
    BodyMode {
        frequency_hz: 335.0,
        gain: 0.31,
        decay_s: 0.28,
    },
    BodyMode {
        frequency_hz: 625.0,
        gain: 0.35,
        decay_s: 0.22,
    },
    BodyMode {
        frequency_hz: 1_020.0,
        gain: 0.29,
        decay_s: 0.17,
    },
    BodyMode {
        frequency_hz: 1_760.0,
        gain: 0.24,
        decay_s: 0.13,
    },
    BodyMode {
        frequency_hz: 2_850.0,
        gain: 0.20,
        decay_s: 0.095,
    },
    BodyMode {
        frequency_hz: 4_250.0,
        gain: 0.15,
        decay_s: 0.065,
    },
    BodyMode {
        frequency_hz: 6_050.0,
        gain: 0.09,
        decay_s: 0.045,
    },
];

const DULCIMER_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 125.0,
        gain: 0.22,
        decay_s: 0.65,
    },
    BodyMode {
        frequency_hz: 250.0,
        gain: 0.30,
        decay_s: 0.54,
    },
    BodyMode {
        frequency_hz: 475.0,
        gain: 0.28,
        decay_s: 0.44,
    },
    BodyMode {
        frequency_hz: 820.0,
        gain: 0.24,
        decay_s: 0.34,
    },
    BodyMode {
        frequency_hz: 1_320.0,
        gain: 0.18,
        decay_s: 0.25,
    },
    BodyMode {
        frequency_hz: 2_150.0,
        gain: 0.13,
        decay_s: 0.17,
    },
    BodyMode {
        frequency_hz: 3_350.0,
        gain: 0.08,
        decay_s: 0.11,
    },
    BodyMode {
        frequency_hz: 4_850.0,
        gain: 0.04,
        decay_s: 0.070,
    },
];

const SKIN_STRING_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 230.0,
        gain: 0.16,
        decay_s: 0.14,
    },
    BodyMode {
        frequency_hz: 410.0,
        gain: 0.25,
        decay_s: 0.12,
    },
    BodyMode {
        frequency_hz: 690.0,
        gain: 0.31,
        decay_s: 0.095,
    },
    BodyMode {
        frequency_hz: 1_050.0,
        gain: 0.27,
        decay_s: 0.075,
    },
    BodyMode {
        frequency_hz: 1_680.0,
        gain: 0.22,
        decay_s: 0.058,
    },
    BodyMode {
        frequency_hz: 2_650.0,
        gain: 0.18,
        decay_s: 0.043,
    },
    BodyMode {
        frequency_hz: 3_900.0,
        gain: 0.12,
        decay_s: 0.032,
    },
    BodyMode {
        frequency_hz: 5_300.0,
        gain: 0.07,
        decay_s: 0.024,
    },
];

const BASS_BODY: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 58.0,
        gain: 0.30,
        decay_s: 0.80,
    },
    BodyMode {
        frequency_hz: 110.0,
        gain: 0.37,
        decay_s: 0.66,
    },
    BodyMode {
        frequency_hz: 205.0,
        gain: 0.28,
        decay_s: 0.52,
    },
    BodyMode {
        frequency_hz: 390.0,
        gain: 0.21,
        decay_s: 0.39,
    },
    BodyMode {
        frequency_hz: 720.0,
        gain: 0.16,
        decay_s: 0.28,
    },
    BodyMode {
        frequency_hz: 1_250.0,
        gain: 0.11,
        decay_s: 0.19,
    },
    BodyMode {
        frequency_hz: 2_100.0,
        gain: 0.065,
        decay_s: 0.12,
    },
    BodyMode {
        frequency_hz: 3_300.0,
        gain: 0.03,
        decay_s: 0.075,
    },
];

const PIANO_SOUNDBOARD: [BodyMode; 10] = [
    BodyMode {
        frequency_hz: 88.0,
        gain: 0.20,
        decay_s: 1.80,
    },
    BodyMode {
        frequency_hz: 132.0,
        gain: 0.18,
        decay_s: 1.55,
    },
    BodyMode {
        frequency_hz: 205.0,
        gain: 0.22,
        decay_s: 1.35,
    },
    BodyMode {
        frequency_hz: 310.0,
        gain: 0.20,
        decay_s: 1.10,
    },
    BodyMode {
        frequency_hz: 485.0,
        gain: 0.18,
        decay_s: 0.90,
    },
    BodyMode {
        frequency_hz: 760.0,
        gain: 0.16,
        decay_s: 0.65,
    },
    BodyMode {
        frequency_hz: 1_180.0,
        gain: 0.13,
        decay_s: 0.48,
    },
    BodyMode {
        frequency_hz: 1_850.0,
        gain: 0.10,
        decay_s: 0.34,
    },
    BodyMode {
        frequency_hz: 2_850.0,
        gain: 0.07,
        decay_s: 0.24,
    },
    BodyMode {
        frequency_hz: 4_200.0,
        gain: 0.04,
        decay_s: 0.16,
    },
];

const UPRIGHT_SOUNDBOARD: [BodyMode; 8] = [
    BodyMode {
        frequency_hz: 104.0,
        gain: 0.23,
        decay_s: 1.15,
    },
    BodyMode {
        frequency_hz: 178.0,
        gain: 0.28,
        decay_s: 1.02,
    },
    BodyMode {
        frequency_hz: 285.0,
        gain: 0.25,
        decay_s: 0.82,
    },
    BodyMode {
        frequency_hz: 455.0,
        gain: 0.21,
        decay_s: 0.65,
    },
    BodyMode {
        frequency_hz: 710.0,
        gain: 0.17,
        decay_s: 0.48,
    },
    BodyMode {
        frequency_hz: 1_180.0,
        gain: 0.13,
        decay_s: 0.34,
    },
    BodyMode {
        frequency_hz: 2_050.0,
        gain: 0.085,
        decay_s: 0.23,
    },
    BodyMode {
        frequency_hz: 3_650.0,
        gain: 0.045,
        decay_s: 0.14,
    },
];

const HARPSICHORD_SOUNDBOARD: [BodyMode; 9] = [
    BodyMode {
        frequency_hz: 145.0,
        gain: 0.17,
        decay_s: 0.66,
    },
    BodyMode {
        frequency_hz: 245.0,
        gain: 0.26,
        decay_s: 0.58,
    },
    BodyMode {
        frequency_hz: 390.0,
        gain: 0.30,
        decay_s: 0.49,
    },
    BodyMode {
        frequency_hz: 620.0,
        gain: 0.27,
        decay_s: 0.38,
    },
    BodyMode {
        frequency_hz: 980.0,
        gain: 0.24,
        decay_s: 0.30,
    },
    BodyMode {
        frequency_hz: 1_550.0,
        gain: 0.20,
        decay_s: 0.22,
    },
    BodyMode {
        frequency_hz: 2_450.0,
        gain: 0.15,
        decay_s: 0.16,
    },
    BodyMode {
        frequency_hz: 3_800.0,
        gain: 0.10,
        decay_s: 0.11,
    },
    BodyMode {
        frequency_hz: 5_600.0,
        gain: 0.055,
        decay_s: 0.075,
    },
];

const DULCIMER_SOUNDBOARD: [BodyMode; 9] = [
    BodyMode {
        frequency_hz: 118.0,
        gain: 0.15,
        decay_s: 0.90,
    },
    BodyMode {
        frequency_hz: 225.0,
        gain: 0.22,
        decay_s: 0.78,
    },
    BodyMode {
        frequency_hz: 365.0,
        gain: 0.29,
        decay_s: 0.67,
    },
    BodyMode {
        frequency_hz: 580.0,
        gain: 0.31,
        decay_s: 0.52,
    },
    BodyMode {
        frequency_hz: 925.0,
        gain: 0.26,
        decay_s: 0.40,
    },
    BodyMode {
        frequency_hz: 1_480.0,
        gain: 0.21,
        decay_s: 0.29,
    },
    BodyMode {
        frequency_hz: 2_350.0,
        gain: 0.16,
        decay_s: 0.20,
    },
    BodyMode {
        frequency_hz: 3_650.0,
        gain: 0.11,
        decay_s: 0.14,
    },
    BodyMode {
        frequency_hz: 5_300.0,
        gain: 0.065,
        decay_s: 0.095,
    },
];

pub struct Engine {
    pub bank: InstrumentBank,
    pub sample_rate: u32,
    articulation: String,
    technique_modifiers: technique::TechniqueModifiers,
    voices: Vec<Voice>,
    round_robin: [u8; 128],
    next_id: u64,
    pitch_bend: f32,
    expression: f32,
    sample_level: f32,
    synth_level: f32,
    master_gain: f32,
    shared_strings: SharedStringState,
    has_pending_release_samples: bool,
}

impl Engine {
    pub fn new(bank: InstrumentBank, sample_rate: u32) -> Self {
        let articulation = bank.default_articulation.clone();
        let technique_modifiers = technique::modifiers(&bank.name, &bank.family, &articulation);
        let capacity = bank.polyphony.max(1) as usize;
        let sample_level = bank.synth.sample_level;
        let synth_level = bank.synth.synth_level;
        let master_gain = bank.synth.master_gain;
        Self {
            bank,
            sample_rate: sample_rate.clamp(8_000, 384_000),
            articulation,
            technique_modifiers,
            voices: Vec::with_capacity(capacity),
            round_robin: [1; 128],
            next_id: 1,
            pitch_bend: 0.0,
            expression: 1.0,
            sample_level,
            synth_level,
            master_gain,
            shared_strings: SharedStringState::default(),
            has_pending_release_samples: false,
        }
    }

    pub fn set_articulation(&mut self, name: &str) -> bool {
        let canonical = technique::canonicalize(&self.bank.name, &self.bank.family, name);
        let exact_region = self
            .bank
            .regions
            .iter()
            .any(|region| region.trigger == Trigger::Attack && region.articulation == name);
        let canonical_region =
            self.bank.regions.iter().any(|region| {
                region.trigger == Trigger::Attack && region.articulation == canonical
            });
        let modeled_technique = self.bank.synth.model != SynthModel::SampleOnly
            && technique::recommended(&self.bank.name, &self.bank.family)
                .contains(&canonical.as_str());
        let selected = if exact_region {
            name.to_string()
        } else if canonical_region || modeled_technique {
            canonical
        } else {
            return false;
        };
        self.technique_modifiers =
            technique::modifiers(&self.bank.name, &self.bank.family, &selected);
        self.articulation = selected;
        true
    }

    pub fn articulation(&self) -> &str {
        &self.articulation
    }

    pub fn set_pitch_bend(&mut self, semitones: f32) {
        self.pitch_bend = semitones.clamp(-24.0, 24.0);
        for voice in &mut self.voices {
            let region = &self.bank.regions[voice.region];
            let cents = (voice.note as f32 - region.root_key as f32) * 100.0 - region.tune_cents
                + self.pitch_bend * 100.0;
            voice.step = 2f64.powf(cents as f64 / 1_200.0) * region.sample_rate as f64
                / self.sample_rate as f64;
            let frequency = midi_frequency(voice.note as f32 + self.pitch_bend);
            voice.phase_increment =
                std::f64::consts::TAU * frequency as f64 / self.sample_rate as f64;
        }
    }

    pub fn set_expression(&mut self, value: f32) {
        self.expression = value.clamp(0.0, 1.5);
    }

    pub fn set_sample_level(&mut self, value: f32) {
        self.sample_level = value.clamp(0.0, 2.0);
    }

    pub fn set_synth_level(&mut self, value: f32) {
        self.synth_level = value.clamp(0.0, 2.0);
    }

    pub fn set_master_gain(&mut self, value: f32) {
        self.master_gain = value.clamp(0.0, 2.0);
    }

    pub fn set_mix(&mut self, sample_level: f32, synth_level: f32) {
        self.set_sample_level(sample_level);
        self.set_synth_level(synth_level);
    }

    /// Compatibility control: 0 is sample-only and 1 is model-only.
    pub fn set_synth_blend(&mut self, blend: f32) {
        let blend = blend.clamp(0.0, 1.0);
        self.sample_level = 1.0 - blend;
        self.synth_level = blend;
    }

    pub fn sample_level(&self) -> f32 {
        self.sample_level
    }

    pub fn synth_level(&self) -> f32 {
        self.synth_level
    }

    pub fn master_gain(&self) -> f32 {
        self.master_gain
    }

    fn select(&mut self, note: u8, velocity: u8, trigger: Trigger) -> Option<usize> {
        if note > 127 {
            return None;
        }
        let exact_exists = self.bank.regions.iter().any(|region| {
            region.trigger == trigger
                && note >= region.key_lo
                && note <= region.key_hi
                && region.articulation == self.articulation
        });
        let candidate = |region: &crate::profile::Region| {
            region.trigger == trigger
                && note >= region.key_lo
                && note <= region.key_hi
                && if exact_exists {
                    region.articulation == self.articulation
                } else {
                    region.articulation == self.bank.default_articulation
                        || region.articulation == "sustain"
                }
        };

        // Pitch anchors take precedence over velocity layers. A recording at the
        // requested pitch should not be ignored merely because it was played
        // harder or softer than the requested MIDI velocity. Once the nearest
        // root is known, use the closest available velocity layer at that root.
        let preferred = self
            .bank
            .regions
            .iter()
            .enumerate()
            .filter(|(_, region)| candidate(region))
            .min_by_key(|(_, region)| {
                (
                    note.abs_diff(region.root_key),
                    if velocity < region.vel_lo {
                        region.vel_lo - velocity
                    } else {
                        velocity.saturating_sub(region.vel_hi)
                    },
                    region.root_key,
                    region.vel_lo,
                    region.vel_hi,
                )
            })
            .map(|(index, _)| index)?;
        let anchor = &self.bank.regions[preferred];
        let anchor_root = anchor.root_key;
        let anchor_vel_lo = anchor.vel_lo;
        let anchor_vel_hi = anchor.vel_hi;
        let anchored = |region: &crate::profile::Region| {
            candidate(region)
                && region.root_key == anchor_root
                && region.vel_lo == anchor_vel_lo
                && region.vel_hi == anchor_vel_hi
        };
        let wanted = self.round_robin[note as usize].max(1);
        let maximum = self
            .bank
            .regions
            .iter()
            .filter(|region| anchored(region))
            .map(|region| region.round_robin)
            .max()
            .unwrap_or(1);
        let selected = self
            .bank
            .regions
            .iter()
            .enumerate()
            .filter(|(_, region)| anchored(region))
            .find(|(_, region)| region.round_robin == wanted)
            .map(|(index, _)| index)
            .or_else(|| {
                self.bank
                    .regions
                    .iter()
                    .enumerate()
                    .filter(|(_, region)| anchored(region))
                    .min_by_key(|(_, region)| region.round_robin)
                    .map(|(index, _)| index)
            })?;
        self.round_robin[note as usize] = if wanted >= maximum { 1 } else { wanted + 1 };
        Some(selected)
    }

    fn allocate_voice_id(&mut self) -> u64 {
        loop {
            let id = self.next_id.clamp(1, i32::MAX as u64);
            self.next_id = if id == i32::MAX as u64 { 1 } else { id + 1 };
            if self.voices.iter().all(|voice| voice.id != id) {
                return id;
            }
        }
    }

    fn push_voice(
        &mut self,
        note: u8,
        velocity: u8,
        region_index: usize,
        trigger: Trigger,
        auto_release: Option<usize>,
    ) -> u64 {
        if note > 127 || region_index >= self.bank.regions.len() {
            return 0;
        }
        if self.voices.len() >= self.bank.polyphony.max(1) as usize {
            let victim = self
                .voices
                .iter()
                .enumerate()
                .max_by_key(|(_, voice)| (voice.released, voice.age))
                .map(|(index, _)| index)
                .unwrap_or(0);
            self.voices.swap_remove(victim);
        }
        let id = self.allocate_voice_id();

        let string_profile = self.bank.string_profile;
        let keyboard_profile = self.bank.keyboard_profile;
        let wind_profile = self.bank.wind_profile;
        let percussion_profile = self.bank.percussion_profile;
        let region = &self.bank.regions[region_index];
        let cents = (note as f32 - region.root_key as f32) * 100.0 - region.tune_cents
            + self.pitch_bend * 100.0;
        let step =
            2f64.powf(cents as f64 / 1_200.0) * region.sample_rate as f64 / self.sample_rate as f64;
        let frequency = midi_frequency(note as f32 + self.pitch_bend);
        let noise_state = 0x9E37_79B9_7F4A_7C15_u64 ^ id.rotate_left(17) ^ u64::from(note);
        let (mut contact_hardness, mut head_strike) = match region.articulation.as_str() {
            "clawhammer" => (0.92, 0.34),
            "strike" => (1.0, 0.48),
            "three_finger" | "crosspick" | "tremolo_pick" | "mizrab" => (0.82, 0.03),
            "single_finger" | "fingerstyle" | "pluck" | "nail" => (0.45, 0.0),
            "strum" => (0.62, 0.04),
            "hammer_on" => (0.16, 0.12),
            "pull_off" => (0.28, 0.02),
            "mute" | "palm_mute" | "chunk" | "chop" | "damped" => (0.38, 0.0),
            "harmonic" => (0.34, 0.0),
            "pick" | "plectrum" | "baji" => (0.78, 0.0),
            _ => (0.56, 0.0),
        };
        if let Some(profile) = keyboard_profile {
            contact_hardness = profile.hammer_hardness;
            head_strike = profile.hammer_noise;
        } else if let Some(profile) = percussion_profile {
            contact_hardness = profile.hardness;
            head_strike = profile.transient_noise;
        }
        let excitation_model = keyboard_profile.map_or(self.bank.synth.model, |profile| {
            if profile.exciter == KeyboardExciter::Plectrum {
                SynthModel::Guitar
            } else {
                SynthModel::Piano
            }
        });
        let needs_waveguide = matches!(
            self.bank.synth.model,
            SynthModel::PluckedString
                | SynthModel::BowedString
                | SynthModel::StruckString
                | SynthModel::Banjo
                | SynthModel::Guitar
                | SynthModel::Piano
        ) || wind_profile.is_some();
        let excitation_position = keyboard_profile
            .map(|profile| profile.strike_position)
            .unwrap_or(self.bank.synth.pluck_position);
        let excitation_noise = keyboard_profile
            .map(|profile| profile.hammer_noise)
            .unwrap_or(self.bank.synth.noise);
        let waveguide = if trigger == Trigger::Attack && needs_waveguide {
            make_waveguide(
                self.sample_rate,
                frequency,
                excitation_position,
                excitation_noise,
                noise_state,
                WaveguideExcitation {
                    model: excitation_model,
                    harmonics: &self.bank.synth.harmonics,
                    contact_hardness,
                    string_profile: string_profile.as_ref(),
                },
            )
        } else {
            Vec::new()
        };
        let waveguide_fraction = if excitation_model == SynthModel::Banjo && !waveguide.is_empty() {
            (waveguide.len() as f32 - self.sample_rate as f32 / frequency.max(8.0)).clamp(0.0, 1.0)
        } else {
            0.5
        };
        let keyboard_detune = keyboard_profile.and_then(|profile| {
            (profile.strings_per_note >= 2)
                .then(|| 2.0_f32.powf(profile.unison_detune_cents / 1_200.0))
        });
        let secondary_ratio = string_profile
            .and_then(|profile| profile.secondary_ratio(note))
            .or(keyboard_detune);
        let secondary_waveguide = if trigger == Trigger::Attack && secondary_ratio.is_some() {
            let detune = secondary_ratio.unwrap_or(1.0);
            make_waveguide(
                self.sample_rate,
                frequency * detune,
                excitation_position,
                excitation_noise,
                noise_state.rotate_left(23),
                WaveguideExcitation {
                    model: excitation_model,
                    harmonics: &self.bank.synth.harmonics,
                    contact_hardness: 0.72,
                    string_profile: string_profile.as_ref(),
                },
            )
        } else {
            Vec::new()
        };
        let tertiary_waveguide = if trigger == Trigger::Attack
            && keyboard_profile.is_some_and(|profile| profile.strings_per_note >= 3)
        {
            let profile = keyboard_profile.expect("checked keyboard profile");
            let detune = 2.0_f32.powf(-profile.unison_detune_cents * 0.73 / 1_200.0);
            make_waveguide(
                self.sample_rate,
                frequency * detune,
                excitation_position,
                excitation_noise,
                noise_state.rotate_left(41),
                WaveguideExcitation {
                    model: excitation_model,
                    harmonics: &self.bank.synth.harmonics,
                    contact_hardness: profile.hammer_hardness * 0.92,
                    string_profile: None,
                },
            )
        } else {
            Vec::new()
        };

        self.voices.push(Voice {
            id,
            note,
            midi_velocity: velocity,
            region: region_index,
            trigger,
            position: 0.0,
            step,
            gain: (velocity.max(1) as f32 / 127.0).powf(1.35) * 10f32.powf(region.gain_db / 20.0),
            released: false,
            release_age: 0,
            age: 0,
            auto_release,
            pending_release_sample: false,
            phase: 0.0,
            phase_increment: std::f64::consts::TAU * frequency as f64 / self.sample_rate as f64,
            velocity: velocity.max(1) as f32 / 127.0,
            contact_hardness,
            head_strike,
            waveguide,
            waveguide_position: 0,
            waveguide_fraction,
            secondary_waveguide,
            secondary_waveguide_position: 0,
            tertiary_waveguide,
            tertiary_waveguide_position: 0,
            waveguide_filter: 0.0,
            secondary_waveguide_filter: 0.0,
            tertiary_waveguide_filter: 0.0,
            last_string_output: 0.0,
            resonator_y1: [0.0; 16],
            resonator_y2: [0.0; 16],
            noise_state,
            filtered_noise: 0.0,
            pick_filter: 0.0,
            captured_lowpass: 0.0,
            captured_highpass_lowpass_2: 0.0,
            captured_highpass_lowpass_3: 0.0,
            captured_previous: 0.0,
            bridge_notches: [BiquadState::default(); 3],
            head_feedback: 0.0,
        });
        id
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) -> u64 {
        self.note_on_for(note, velocity, None)
    }

    pub fn note_on_for(&mut self, note: u8, velocity: u8, duration_frames: Option<usize>) -> u64 {
        if note > 127 {
            return 0;
        }
        if velocity == 0 {
            self.note_off(note);
            return 0;
        }
        match self.select(note, velocity, Trigger::Attack) {
            Some(index) => self.push_voice(note, velocity, index, Trigger::Attack, duration_frames),
            None => 0,
        }
    }

    pub fn note_off(&mut self, note: u8) {
        if note > 127 {
            return;
        }
        let velocities: Vec<u8> = self
            .voices
            .iter_mut()
            .filter(|voice| {
                voice.note == note && voice.trigger == Trigger::Attack && !voice.released
            })
            .map(|voice| {
                voice.released = true;
                voice.release_age = 0;
                voice.auto_release = None;
                voice.midi_velocity
            })
            .collect();
        for velocity in velocities {
            if let Some(index) = self.select(note, velocity, Trigger::Release) {
                self.push_voice(note, velocity, index, Trigger::Release, None);
            }
        }
    }

    pub fn voice_off(&mut self, id: u64) -> bool {
        let Some(index) = self.voices.iter().position(|voice| {
            voice.id == id && voice.trigger == Trigger::Attack && !voice.released
        }) else {
            return false;
        };
        let note = self.voices[index].note;
        let velocity = self.voices[index].midi_velocity;
        self.voices[index].released = true;
        self.voices[index].release_age = 0;
        self.voices[index].auto_release = None;
        if let Some(region) = self.select(note, velocity, Trigger::Release) {
            self.push_voice(note, velocity, region, Trigger::Release, None);
        }
        true
    }

    pub fn all_notes_off(&mut self) {
        let released: Vec<(u8, u8)> = self
            .voices
            .iter_mut()
            .filter(|voice| voice.trigger == Trigger::Attack && !voice.released)
            .map(|voice| {
                voice.released = true;
                voice.release_age = 0;
                voice.auto_release = None;
                (voice.note, voice.midi_velocity)
            })
            .collect();
        for (note, velocity) in released {
            if let Some(region) = self.select(note, velocity, Trigger::Release) {
                self.push_voice(note, velocity, region, Trigger::Release, None);
            }
        }
    }

    fn cubic(samples: &[f32], position: f64) -> f32 {
        let index = position.floor() as isize;
        let fraction = (position - index as f64) as f32;
        let at = |offset: isize| {
            samples[offset.clamp(0, samples.len().saturating_sub(1) as isize) as usize]
        };
        let y0 = at(index - 1);
        let y1 = at(index);
        let y2 = at(index + 1);
        let y3 = at(index + 2);
        let a0 = y3 - y2 - y0 + y1;
        let a1 = y0 - y1 - a0;
        let a2 = y2 - y0;
        ((a0 * fraction + a1) * fraction + a2) * fraction + y1
    }

    fn sample_step(voice: &mut Voice, region: &crate::profile::Region, output_rate: u32) -> f32 {
        if region.samples.len() < 4 {
            return 0.0;
        }
        let looping = voice.trigger == Trigger::Attack
            && !voice.released
            && region.loop_start.is_some()
            && region.loop_end.is_some();
        if looping {
            let start = region.loop_start.unwrap() as f64;
            let end = region.loop_end.unwrap() as f64;
            let length = end - start;
            if voice.position >= end && length >= 2.0 {
                voice.position = start + (voice.position - start) % length;
            }
        }
        if voice.position >= region.samples.len().saturating_sub(2) as f64 {
            return 0.0;
        }

        let mut gain = 1.0;
        let fade_in_frames = (output_rate as f32 * 0.002).max(1.0);
        gain *= (voice.age as f32 / fade_in_frames).min(1.0);
        if !looping {
            let remaining = region.samples.len() as f64 - voice.position;
            let fade_length = (region.sample_rate as f64 * 0.02)
                .max(1.0)
                .min(region.samples.len() as f64);
            if remaining < fade_length {
                gain *= (remaining / fade_length).max(0.0) as f32;
            }
        }
        if voice.trigger == Trigger::Attack && voice.released {
            let release_frames = (region.release_s.max(0.005) * output_rate as f32).max(1.0);
            gain *= (-6.0 * voice.release_age as f32 / release_frames).exp();
        }
        let output = Self::cubic(&region.samples, voice.position) * gain;
        voice.position += voice.step;
        output
    }

    /// Keep the part of a captured performance that the physical model does
    /// not reproduce: the onset, pick/hammer/reed noise, and broadband detail.
    /// Removing the slowly varying tonal component prevents the doubled
    /// fundamental and comb filtering caused by mixing a complete sample with
    /// an independently generated note.
    fn captured_residual_step(
        voice: &mut Voice,
        captured: f32,
        region: &crate::profile::Region,
        output_rate: u32,
        model: SynthModel,
    ) -> f32 {
        let (cutoff_hz, transient_decay_s, base_residual) = match model {
            SynthModel::Banjo
            | SynthModel::Guitar
            | SynthModel::PluckedString
            | SynthModel::Piano
            | SynthModel::StruckString => (1_100.0, 0.055, 0.045),
            SynthModel::Membrane | SynthModel::Modal | SynthModel::Noise => (700.0, 0.090, 0.16),
            SynthModel::Wind
            | SynthModel::Reed
            | SynthModel::Brass
            | SynthModel::FreeReed
            | SynthModel::Organ
            | SynthModel::BowedString => (1_700.0, 0.140, 0.20),
            SynthModel::Additive => (1_300.0, 0.085, 0.10),
            SynthModel::SampleOnly => return captured,
        };
        let alpha = 1.0 - (-std::f32::consts::TAU * cutoff_hz / output_rate.max(1) as f32).exp();
        voice.captured_lowpass += alpha * (captured - voice.captured_lowpass);
        let highpass_1 = captured - voice.captured_lowpass;
        voice.captured_highpass_lowpass_2 +=
            alpha * (highpass_1 - voice.captured_highpass_lowpass_2);
        let highpass_2 = highpass_1 - voice.captured_highpass_lowpass_2;
        voice.captured_highpass_lowpass_3 +=
            alpha * (highpass_2 - voice.captured_highpass_lowpass_3);
        let highpass_3 = highpass_2 - voice.captured_highpass_lowpass_3;
        // Tonal recordings need a steep rejection slope because a performance
        // slice can contain accompaniment below the labeled melody note. Three
        // cascaded high-pass stages preserve hammer/pick detail while removing
        // those unrelated fundamentals. Percussion and noise retain the
        // broader single-stage residual that carries their resonator identity.
        let residual = if matches!(
            model,
            SynthModel::Banjo
                | SynthModel::Guitar
                | SynthModel::PluckedString
                | SynthModel::Piano
                | SynthModel::StruckString
                | SynthModel::Wind
                | SynthModel::Reed
                | SynthModel::Brass
                | SynthModel::FreeReed
                | SynthModel::Organ
                | SynthModel::BowedString
                | SynthModel::Additive
        ) {
            highpass_3
        } else {
            highpass_1
        };
        let derivative = captured - voice.captured_previous;
        voice.captured_previous = captured;

        let time = voice.age as f32 / output_rate.max(1) as f32;
        let transient = (-time / transient_decay_s).exp();
        let detail = base_residual + region.noise.clamp(0.0, 1.0) * 0.55;
        // Captured detail is useful near its measured pitch, but stretching it
        // across several octaves moves body resonances into unrelated notes.
        // Fade to the physical model between one and two octaves from the
        // nearest anchor. Explicit sample-only playback remains unchanged.
        let distance = voice.note.abs_diff(region.root_key) as f32;
        let anchor_weight = (1.0 - (distance - 12.0).max(0.0) / 12.0).clamp(0.0, 1.0);
        (residual * (detail + transient * 0.82) + derivative * transient * 0.32).clamp(-1.0, 1.0)
            * anchor_weight
    }

    fn additive_step(voice: &mut Voice, config: &SynthConfig, sample_rate: u32) -> f32 {
        let fundamental = voice.phase_increment * sample_rate as f64 / std::f64::consts::TAU;
        let time = voice.age as f32 / sample_rate as f32;
        let vibrato_cents = (std::f32::consts::TAU * config.vibrato_rate_hz * time).sin()
            * config.vibrato_depth_cents;
        let vibrato = 2.0_f64.powf(vibrato_cents as f64 / 1_200.0);
        let mut total = 0.0_f32;
        let mut output = 0.0;
        for (index, amplitude) in config.harmonics.iter().enumerate() {
            if fundamental * (index + 1) as f64 >= sample_rate as f64 * 0.48 {
                break;
            }
            let partial = (index + 1) as f32;
            let tilt = partial.powf((config.brightness - 0.5) * 1.6);
            let amplitude = *amplitude * tilt;
            total += amplitude;
            output += amplitude * (voice.phase * (index + 1) as f64).sin() as f32;
        }
        voice.phase =
            (voice.phase + voice.phase_increment * vibrato).rem_euclid(std::f64::consts::TAU);
        output / total.max(1e-6).sqrt().max(1.0)
    }

    fn delay_step(
        waveguide: &mut [f32],
        position: &mut usize,
        filter_state: &mut f32,
        config: &SynthConfig,
        damping_scale: f32,
    ) -> f32 {
        if waveguide.len() < 3 {
            return 0.0;
        }
        let current_position = *position;
        let next = (current_position + 1) % waveguide.len();
        let current = waveguide[current_position];
        let adjacent = waveguide[next];
        let averaged = (current + adjacent) * 0.5;
        let brightness = config.brightness.clamp(0.0, 1.0);
        let smoothing = 0.12 + (1.0 - brightness) * 0.36;
        *filter_state += (averaged - *filter_state) * (1.0 - smoothing);
        let edge = adjacent - averaged;
        let filtered = *filter_state + edge * brightness * 0.18;
        waveguide[current_position] = filtered * (config.damping * damping_scale).clamp(0.9, 1.0);
        *position = next;
        current
    }

    fn waveguide_step(voice: &mut Voice, config: &SynthConfig) -> f32 {
        Self::delay_step(
            &mut voice.waveguide,
            &mut voice.waveguide_position,
            &mut voice.waveguide_filter,
            config,
            1.0,
        )
    }

    fn notch(input: f32, state: &mut BiquadState, frequency: f32, q: f32, sample_rate: u32) -> f32 {
        let omega = std::f32::consts::TAU * frequency / sample_rate as f32;
        let cosine = omega.cos();
        let alpha = omega.sin() / (2.0 * q.max(0.1));
        let inverse_a0 = 1.0 / (1.0 + alpha);
        let b0 = inverse_a0;
        let b1 = -2.0 * cosine * inverse_a0;
        let b2 = inverse_a0;
        let a1 = -2.0 * cosine * inverse_a0;
        let a2 = (1.0 - alpha) * inverse_a0;
        let output = b0 * input + b1 * state.x1 + b2 * state.x2 - a1 * state.y1 - a2 * state.y2;
        state.x2 = state.x1;
        state.x1 = input;
        state.y2 = state.y1;
        state.y1 = output;
        output
    }

    fn banjo_string_step(
        voice: &mut Voice,
        config: &SynthConfig,
        sample_rate: u32,
        profile: Option<&StringInstrumentProfile>,
    ) -> (f32, f32) {
        if voice.waveguide.len() < 3 {
            return (0.0, 0.0);
        }
        let position = voice.waveguide_position;
        let next = (position + 1) % voice.waveguide.len();
        let following = (position + 2) % voice.waveguide.len();
        let current = voice.waveguide[position];
        let adjacent = voice.waveguide[next];
        // Frequency-dependent bridge admittance is what separates a banjo
        // from a generic Karplus--Strong pluck. The broad 500--800 Hz head
        // formant and bridge hills select the force sent to the membrane.
        let brightness = config.brightness.clamp(0.0, 1.0);
        let adjacent_weight = voice.waveguide_fraction;
        let current_weight = 1.0 - adjacent_weight;
        let incident = current * current_weight + adjacent * adjacent_weight;
        let centers = [640.0_f32, 2_250.0, 3_560.0];
        let qualities = [0.72_f32, 1.10, 1.35];
        let depths = [0.22_f32, 0.09, 0.16];
        let mut admittance_shaped = incident;
        for index in 0..centers.len() {
            let notched = Self::notch(
                admittance_shaped,
                &mut voice.bridge_notches[index],
                centers[index],
                qualities[index],
                sample_rate,
            );
            admittance_shaped += (notched - admittance_shaped) * depths[index];
        }
        let transferred = incident - admittance_shaped;
        let high_frequency_loss = profile.map_or(0.15, |value| value.high_frequency_loss);
        let stiffness = profile.map_or(0.18, Self::string_dispersion);
        let nonlinear_tension = profile.map_or(0.30, |value| value.nonlinear_tension);
        let profile_damping = profile.map_or(config.damping, |value| value.base_damping);
        let loss = (high_frequency_loss * 0.34 + (1.0 - brightness) * 0.20).clamp(0.02, 0.55);
        voice.tertiary_waveguide_filter +=
            (incident - voice.tertiary_waveguide_filter) * (1.0 - loss);
        let curvature = current - 2.0 * adjacent + voice.waveguide[following];
        let dispersion = curvature * stiffness * 0.014;
        let nonlinear = incident * incident * incident * nonlinear_tension * 0.012;
        let feedback = voice.tertiary_waveguide_filter + dispersion + nonlinear;
        let damping = (config.damping * profile_damping)
            .sqrt()
            .clamp(0.90, 0.999_98);
        // A very small return from the moving head makes the coupling
        // bidirectional without allowing a body resonance to ring forever.
        // Keep the bridge admittance's phase rotation outside the delay loop;
        // placing it in the feedback path would detune notes near a head mode.
        // The measured transfer still controls the radiated body signal.
        voice.waveguide[position] = (feedback
            - voice.head_feedback * 0.002 * profile.map_or(0.92, |p| p.bridge_coupling))
            * damping;
        voice.waveguide_position = next;
        (current, transferred)
    }

    fn body_step(voice: &mut Voice, drive: f32, modes: &[BodyMode], sample_rate: u32) -> f32 {
        let mut output = 0.0_f32;
        let mut normalization = 0.0_f32;
        for (index, mode) in modes.iter().enumerate().take(voice.resonator_y1.len()) {
            if mode.frequency_hz >= sample_rate as f32 * 0.47 {
                continue;
            }
            let radius = (-1.0 / (mode.decay_s * sample_rate as f32)).exp();
            let coefficient = 2.0
                * radius
                * (std::f32::consts::TAU * mode.frequency_hz / sample_rate as f32).cos();
            let state = drive + coefficient * voice.resonator_y1[index]
                - radius * radius * voice.resonator_y2[index];
            voice.resonator_y2[index] = voice.resonator_y1[index];
            voice.resonator_y1[index] = state;
            output += state * (1.0 - radius) * mode.gain;
            normalization += mode.gain;
        }
        output / normalization.sqrt().max(1.0)
    }

    fn string_body_modes(body: ResonatorBody) -> &'static [BodyMode] {
        match body {
            ResonatorBody::BanjoHead => &BANJO_BODY,
            ResonatorBody::SteelGuitar | ResonatorBody::GenericWood => &GUITAR_BODY,
            ResonatorBody::ClassicalGuitar => &CLASSICAL_GUITAR_BODY,
            ResonatorBody::SolidBodyPickup => &ELECTRIC_PICKUP_BODY,
            ResonatorBody::Mandolin => &MANDOLIN_BODY,
            ResonatorBody::Ukulele => &UKULELE_BODY,
            ResonatorBody::Lute => &LUTE_BODY,
            ResonatorBody::Harp => &HARP_BODY,
            ResonatorBody::GourdJawari => &JAWARI_BODY,
            ResonatorBody::Koto | ResonatorBody::Dulcimer => &DULCIMER_BODY,
            ResonatorBody::ShamisenSkin => &SKIN_STRING_BODY,
            ResonatorBody::Bass => &BASS_BODY,
        }
    }

    fn string_dispersion(profile: &StringInstrumentProfile) -> f32 {
        let length_ratio = 0.65 / profile.scale_length_m.max(0.15);
        (profile.stiffness * length_ratio * length_ratio / profile.tension_scale.max(0.1))
            .clamp(0.0, 4.0)
    }

    fn profiled_delay_step(
        waveguide: &mut [f32],
        position: &mut usize,
        filter_state: &mut f32,
        config: &SynthConfig,
        profile: &StringInstrumentProfile,
        tension_motion: f32,
        damping_scale: f32,
    ) -> f32 {
        if waveguide.len() < 3 {
            return 0.0;
        }
        let current_position = *position;
        let next = (current_position + 1) % waveguide.len();
        let next_two = (current_position + 2) % waveguide.len();
        let current = waveguide[current_position];
        let adjacent = waveguide[next];
        let following = waveguide[next_two];

        // The fractional bridge sample preserves pitch while allowing the
        // moving bridge to create the minute, time-varying tension shift heard
        // in real strings. Stiffness adds a stable dispersive edge term rather
        // than an unrelated additive oscillator.
        let fractional =
            (0.5 + tension_motion * profile.nonlinear_tension * 0.08).clamp(0.40, 0.60);
        let interpolated = current * (1.0 - fractional) + adjacent * fractional;
        let loss = (profile.high_frequency_loss * 0.42 + (1.0 - config.brightness) * 0.24)
            .clamp(0.02, 0.72);
        *filter_state += (interpolated - *filter_state) * (1.0 - loss);
        let curvature = current - 2.0 * adjacent + following;
        let dispersion = curvature * Self::string_dispersion(profile) * 0.018;
        let feedback = *filter_state + dispersion;
        let damping =
            (config.damping * profile.base_damping.sqrt() * damping_scale).clamp(0.90, 0.999_98);
        waveguide[current_position] = feedback * damping;
        *position = next;
        interpolated
    }

    fn profiled_string_step(
        voice: &mut Voice,
        config: &SynthConfig,
        sample_rate: u32,
        profile: &StringInstrumentProfile,
    ) -> f32 {
        let bridge_motion = voice.head_feedback.clamp(-1.0, 1.0);
        let primary = Self::profiled_delay_step(
            &mut voice.waveguide,
            &mut voice.waveguide_position,
            &mut voice.waveguide_filter,
            config,
            profile,
            bridge_motion,
            1.0,
        );
        let secondary = if voice.secondary_waveguide.is_empty() {
            primary
        } else {
            Self::profiled_delay_step(
                &mut voice.secondary_waveguide,
                &mut voice.secondary_waveguide_position,
                &mut voice.secondary_waveguide_filter,
                config,
                profile,
                -bridge_motion,
                0.999_96,
            )
        };
        let course_mix = if voice.secondary_waveguide.is_empty() {
            primary
        } else {
            primary * 0.53 + secondary * 0.47
        };
        let bridge_velocity = course_mix - voice.last_string_output;
        voice.last_string_output = course_mix;

        let time = voice.age as f32 / sample_rate as f32;
        let impact = (-time / (0.0022 + (1.0 - voice.contact_hardness) * 0.0035)).exp();
        let body_drive = bridge_velocity * profile.bridge_coupling
            + course_mix * profile.nonlinear_tension * 0.025
            + voice.head_strike * next_noise(&mut voice.noise_state) * impact * 0.20;
        let body = Self::body_step(
            voice,
            body_drive,
            Self::string_body_modes(profile.body),
            sample_rate,
        );
        voice.head_feedback += (body - voice.head_feedback) * 0.085;

        let raw_contact = next_noise(&mut voice.noise_state);
        voice.pick_filter +=
            (raw_contact - voice.pick_filter) * (0.18 + voice.contact_hardness * 0.34);
        let contact = voice.pick_filter
            * impact
            * profile.pick_noise
            * voice.contact_hardness
            * (0.35 + voice.velocity * 0.65);
        let jawari = if profile.kind == StringInstrumentKind::Sitar {
            // A light asymmetric buzz approximates repeated jawari contact.
            let contact_threshold = 0.16 + 0.04 * (time * 17.0).sin();
            (course_mix.abs() - contact_threshold).max(0.0) * course_mix.signum() * 0.42
        } else {
            0.0
        };
        (course_mix * profile.direct_mix
            + bridge_velocity * profile.bridge_coupling * 0.42
            + body * profile.body_mix
            + contact
            + jawari)
            .tanh()
            * 0.78
    }

    fn shared_string_step(
        state: &mut SharedStringState,
        profile: &StringInstrumentProfile,
        drive: f32,
        sample_rate: u32,
    ) -> f32 {
        if profile.sympathetic_gain <= 0.0 {
            return 0.0;
        }
        let excitation = (drive - state.last_drive) * 0.055;
        state.last_drive = drive;
        let count = profile.open_string_count as usize;
        let mut output = 0.0;
        let mut normalization = 0.0;
        for index in 0..count.min(state.y1.len()) {
            let frequency = midi_frequency(profile.open_notes[index] as f32);
            if frequency >= sample_rate as f32 * 0.45 {
                continue;
            }
            let string_position = index as f32 / count.max(1) as f32;
            let decay_s = (profile.decay_s * (0.58 + string_position * 0.28)).clamp(0.12, 8.0);
            let radius = (-1.0 / (decay_s * sample_rate as f32)).exp();
            let coefficient =
                2.0 * radius * (std::f32::consts::TAU * frequency / sample_rate as f32).cos();
            let next =
                excitation + coefficient * state.y1[index] - radius * radius * state.y2[index];
            state.y2[index] = state.y1[index];
            state.y1[index] = next;
            let weight = 0.75 + 0.25 * (index as f32 * 1.731).sin().abs();
            output += next * (1.0 - radius) * weight;
            normalization += weight;
        }
        output / normalization.sqrt().max(1.0) * profile.sympathetic_gain * 5.0
    }

    fn banjo_step(
        voice: &mut Voice,
        config: &SynthConfig,
        sample_rate: u32,
        profile: Option<&StringInstrumentProfile>,
    ) -> f32 {
        let (raw_string, transferred) =
            Self::banjo_string_step(voice, config, sample_rate, profile);
        let bridge_coupling = profile.map_or(0.92, |value| value.bridge_coupling);
        let direct_mix = profile.map_or(0.18, |value| value.direct_mix);
        let body_mix = profile.map_or(3.8, |value| value.body_mix);
        let pick_noise = profile.map_or(config.noise, |value| {
            value.pick_noise * 0.60 + config.noise * 0.40
        });
        voice.waveguide_filter += (raw_string - voice.waveguide_filter) * 0.42;
        let string = raw_string * 0.28 + voice.waveguide_filter * 0.72;
        let bridge_velocity = string - voice.last_string_output;
        voice.last_string_output = string;
        let normalized_bridge = bridge_velocity / (voice.phase_increment as f32).max(0.012);
        voice.secondary_waveguide_filter +=
            (normalized_bridge - voice.secondary_waveguide_filter) * 0.34;
        let time = voice.age as f32 / sample_rate as f32;
        let initial_force =
            (-time / 0.0032).exp() * raw_string * (0.065 + config.transient_mix * 0.0625);
        let head_strike = next_noise(&mut voice.noise_state)
            * voice.head_strike
            * (-time / 0.0065).exp()
            * (0.30 + voice.velocity * 0.24);
        let body_drive = transferred * bridge_coupling * 5.65
            + bridge_velocity * bridge_coupling * 0.46
            + initial_force
            + head_strike;
        let body = Self::body_step(voice, body_drive, &BANJO_BODY, sample_rate);
        voice.head_feedback += (body - voice.head_feedback) * 0.16;
        let noise = next_noise(&mut voice.noise_state);
        voice.pick_filter += (noise - voice.pick_filter) * 0.28;
        let pick = voice.pick_filter
            * (-time / 0.0038).exp()
            * pick_noise
            * (0.55 + voice.velocity * 0.90)
            * voice.contact_hardness
            * (0.65 + config.transient_mix * 0.55);
        let fundamental_hz =
            voice.phase_increment as f32 * sample_rate as f32 / std::f32::consts::TAU;
        let high_register = ((fundamental_hz - 600.0) / 1_000.0).clamp(0.0, 1.0);
        let direct_register_gain = 1.0 + high_register * 4.0;
        let bridge_register_gain = 1.0 + high_register * 2.0;
        let body_register_gain = 1.0 - high_register * 0.75;
        // The listener mainly hears the head and bridge, not a naked string.
        // Keeping the direct contribution low in the lower registers avoids
        // the generic guitar/harp timbre of a naked delay line. Above the
        // twelfth-fret region, real banjo radiation becomes progressively more
        // string/bridge dominated; attenuating fixed pot modes there also
        // prevents them from masking the played pitch.
        (string * direct_mix * direct_register_gain
            + voice.secondary_waveguide_filter * bridge_coupling * 0.14 * bridge_register_gain
            + body * body_mix * body_register_gain
            + pick)
            .tanh()
            * 0.84
    }

    fn guitar_step(voice: &mut Voice, config: &SynthConfig, sample_rate: u32) -> f32 {
        let string = Self::waveguide_step(voice, config);
        let bridge_velocity = string - voice.last_string_output;
        voice.last_string_output = string;
        let body = Self::body_step(voice, bridge_velocity * 0.55, &GUITAR_BODY, sample_rate);
        (string * 0.62 + bridge_velocity * 0.65 + body * 2.0).tanh() * 0.78
    }

    fn keyboard_body_modes(kind: KeyboardKind) -> &'static [BodyMode] {
        match kind {
            KeyboardKind::GrandPiano => &PIANO_SOUNDBOARD,
            KeyboardKind::UprightPiano | KeyboardKind::Clavichord => &UPRIGHT_SOUNDBOARD,
            KeyboardKind::Harpsichord => &HARPSICHORD_SOUNDBOARD,
            KeyboardKind::HammeredDulcimer | KeyboardKind::Cimbalom | KeyboardKind::Celesta => {
                &DULCIMER_SOUNDBOARD
            }
        }
    }

    fn piano_step(
        voice: &mut Voice,
        config: &SynthConfig,
        sample_rate: u32,
        profile: Option<&KeyboardProfile>,
    ) -> f32 {
        let primary = Self::delay_step(
            &mut voice.waveguide,
            &mut voice.waveguide_position,
            &mut voice.waveguide_filter,
            config,
            1.0,
        );
        let secondary = Self::delay_step(
            &mut voice.secondary_waveguide,
            &mut voice.secondary_waveguide_position,
            &mut voice.secondary_waveguide_filter,
            config,
            0.999_98,
        );
        let tertiary = Self::delay_step(
            &mut voice.tertiary_waveguide,
            &mut voice.tertiary_waveguide_position,
            &mut voice.tertiary_waveguide_filter,
            config,
            0.999_96,
        );
        let strings = match (
            voice.secondary_waveguide.is_empty(),
            voice.tertiary_waveguide.is_empty(),
        ) {
            (true, _) => primary,
            (false, true) => primary * 0.53 + secondary * 0.47,
            (false, false) => primary * 0.38 + secondary * 0.34 + tertiary * 0.28,
        };
        let bridge_velocity = strings - voice.last_string_output;
        voice.last_string_output = strings;
        let profile = profile
            .copied()
            .unwrap_or_else(|| KeyboardProfile::from_kind(KeyboardKind::GrandPiano));
        let soundboard = Self::body_step(
            voice,
            bridge_velocity * (0.34 + profile.pedal_coupling * 0.75),
            Self::keyboard_body_modes(profile.kind),
            sample_rate,
        );
        let time = voice.age as f32 / sample_rate as f32;
        let contact_decay = match profile.exciter {
            KeyboardExciter::Plectrum => 0.0045,
            KeyboardExciter::Tangent => 0.0020,
            KeyboardExciter::HandMallet => 0.0032,
            KeyboardExciter::HardHammer => 0.0022,
            KeyboardExciter::FeltHammer => 0.0038,
        };
        let raw_contact = next_noise(&mut voice.noise_state);
        voice.pick_filter +=
            (raw_contact - voice.pick_filter) * (0.12 + profile.hammer_hardness * 0.44);
        let contact = voice.pick_filter
            * (-time / contact_decay).exp()
            * profile.hammer_noise
            * (0.30 + voice.velocity * 0.70);
        let direct = strings * profile.direct_mix + bridge_velocity * 0.32;
        (direct + soundboard * profile.soundboard_mix + contact).tanh() * 0.72
    }

    fn wind_formants(
        voice: &mut Voice,
        drive: f32,
        profile: &WindProfile,
        sample_rate: u32,
    ) -> f32 {
        let mut output = 0.0;
        let mut normalization = 0.0;
        for index in 0..3 {
            let frequency = profile.formant_hz[index];
            let gain = profile.formant_gain[index];
            if frequency >= sample_rate as f32 * 0.47 || gain <= 0.0 {
                continue;
            }
            let decay_s = 0.010 + gain * 0.075;
            let radius = (-1.0 / (decay_s * sample_rate as f32)).exp();
            let coefficient =
                2.0 * radius * (std::f32::consts::TAU * frequency / sample_rate as f32).cos();
            let next = drive + coefficient * voice.resonator_y1[index]
                - radius * radius * voice.resonator_y2[index];
            voice.resonator_y2[index] = voice.resonator_y1[index];
            voice.resonator_y1[index] = next;
            output += next * (1.0 - radius) * gain;
            normalization += gain;
        }
        output / normalization.sqrt().max(1.0)
    }

    fn wind_step(voice: &mut Voice, profile: &WindProfile, sample_rate: u32) -> f32 {
        if voice.waveguide.len() < 3 {
            return 0.0;
        }
        let position = voice.waveguide_position;
        let next = (position + 1) % voice.waveguide.len();
        let bore_wave = voice.waveguide[position];
        let adjacent = voice.waveguide[next];
        let time = voice.age as f32 / sample_rate as f32;
        let pressure_ramp = 1.0 - (-time / 0.018).exp();
        let breath_noise = next_noise(&mut voice.noise_state)
            * profile.breath_noise
            * (0.15 + pressure_ramp * 0.85);
        let pressure = profile.breath_pressure * voice.velocity.sqrt() * pressure_ramp;
        let oscillator = voice.phase.sin() as f32;
        let delta = pressure + breath_noise - bore_wave * 0.62;
        let source = match profile.exciter {
            WindExciter::AirJet => {
                let jet = oscillator * (0.55 + pressure * 0.45) + adjacent * 0.35;
                (jet * (1.1 + profile.nonlinearity * 1.8)).tanh()
                    - jet * profile.nonlinearity * 0.18
            }
            WindExciter::SingleReed => {
                let opening =
                    (1.0 - delta.abs() * (0.74 + profile.nonlinearity * 0.38)).clamp(0.0, 1.0);
                (delta * opening * 1.8 + oscillator * 0.16).tanh()
            }
            WindExciter::DoubleReed => {
                let opening = (1.0 - delta.abs() * 0.92).clamp(0.0, 1.0);
                (delta * opening * opening * 2.4 + oscillator * 0.20).tanh()
            }
            WindExciter::LipReed => {
                let lip = oscillator + bore_wave * 0.52;
                (lip * (1.2 + pressure * profile.nonlinearity * 2.0)).tanh()
            }
            WindExciter::FreeReed => {
                let reed = oscillator + bore_wave * 0.26;
                (reed * (1.5 + profile.nonlinearity)).tanh()
            }
            WindExciter::Pipe => {
                let jet = oscillator + adjacent * 0.22;
                (jet * (0.85 + pressure)).tanh()
            }
        };
        voice.phase = (voice.phase + voice.phase_increment).rem_euclid(std::f64::consts::TAU);

        let smoothing = (0.10 + profile.bore_loss * 0.48).clamp(0.08, 0.62);
        voice.waveguide_filter +=
            ((bore_wave + adjacent) * 0.5 - voice.waveguide_filter) * (1.0 - smoothing);
        let reflection = profile.reflection.abs();
        let loss = (1.0 - profile.bore_loss * 0.008).clamp(0.97, 0.9998);
        let source_gain = match profile.exciter {
            WindExciter::LipReed => 0.070,
            WindExciter::DoubleReed => 0.060,
            WindExciter::SingleReed | WindExciter::FreeReed => 0.052,
            WindExciter::AirJet | WindExciter::Pipe => 0.042,
        };
        voice.waveguide[position] = (voice.waveguide_filter * reflection * loss
            + source * source_gain
            + breath_noise * 0.025)
            .clamp(-1.0, 1.0);
        voice.waveguide_position = next;

        let bell_velocity = bore_wave - voice.last_string_output;
        voice.last_string_output = bore_wave;
        let formants = Self::wind_formants(voice, bell_velocity * 3.2, profile, sample_rate);
        let bore_colour = match profile.bore {
            BoreShape::CylindricalClosed => bore_wave * 0.82 + source * 0.10,
            BoreShape::Conical => bore_wave * 0.68 + bell_velocity * 0.44,
            BoreShape::OcarinaCavity => voice.waveguide_filter * 0.92,
            BoreShape::FreeRadiating => source * 0.52 + bore_wave * 0.38,
            BoreShape::CylindricalOpen => bore_wave * 0.72 + bell_velocity * 0.30,
        };
        (bore_colour + bell_velocity * profile.bell_mix + formants * 2.4).tanh() * 0.76
    }

    fn percussion_step(
        voice: &mut Voice,
        config: &SynthConfig,
        sample_rate: u32,
        profile: &PercussionProfile,
    ) -> f32 {
        let time = voice.age as f32 / sample_rate as f32;
        let fundamental = voice.phase_increment * sample_rate as f64 / std::f64::consts::TAU;
        let mut modes = 0.0_f32;
        let mut normalization = 0.0_f32;
        for index in 0..config.mode_ratios.len() {
            let ratio = config.mode_ratios[index];
            let amplitude = config.mode_amplitudes[index];
            if ratio <= 0.0
                || amplitude <= 0.0
                || fundamental * ratio as f64 >= sample_rate as f64 * 0.48
            {
                continue;
            }
            let spatial_ratio = match profile.resonator {
                PercussionResonator::Membrane | PercussionResonator::Plate => ratio.sqrt(),
                _ => ratio,
            };
            let spatial = (std::f32::consts::PI * spatial_ratio * profile.strike_position)
                .sin()
                .abs()
                .max(0.04);
            let configured_decay = config.mode_decays[index].max(profile.decay_s * 0.08);
            let decay = configured_decay
                * (profile.decay_s / config.decay_s.max(0.05)).clamp(0.35, 3.0)
                * (1.0 - profile.damping * 0.48);
            let hardness_tilt = ratio.powf((profile.hardness - 0.5) * 0.42);
            let weight = amplitude * spatial * hardness_tilt;
            modes += weight
                * (-time / decay.max(0.025)).exp()
                * (voice.phase * ratio as f64).sin() as f32;
            normalization += weight;
        }
        voice.phase = (voice.phase + voice.phase_increment).rem_euclid(std::f64::consts::TAU);

        let raw_noise = next_noise(&mut voice.noise_state);
        let filter_rate = 0.08 + profile.hardness * 0.62;
        voice.filtered_noise += (raw_noise - voice.filtered_noise) * filter_rate;
        let bright_noise = raw_noise - voice.filtered_noise * (1.0 - profile.brightness * 0.6);
        let impact = bright_noise
            * profile.transient_noise
            * (-time / (0.002 + (1.0 - profile.hardness) * 0.010)).exp();
        let shell = if profile.resonator == PercussionResonator::NoiseShell {
            voice.filtered_noise * profile.shell_noise * (-time / profile.decay_s.max(0.04)).exp()
        } else {
            bright_noise * profile.shell_noise * (-time / (profile.decay_s * 0.42).max(0.03)).exp()
        };
        let resonator = if profile.resonator == PercussionResonator::NoiseShell {
            modes * 0.12
        } else {
            modes / normalization.sqrt().max(1.0)
        };
        (resonator * profile.resonator_mix + impact + shell).tanh() * 0.82
    }

    fn bowed_step(voice: &mut Voice, config: &SynthConfig) -> f32 {
        if voice.waveguide.len() < 3 {
            return 0.0;
        }
        let position = voice.waveguide_position;
        let next = (position + 1) % voice.waveguide.len();
        let current = voice.waveguide[position];
        let adjacent = voice.waveguide[next];
        let string_velocity = current - adjacent;
        let bow_velocity = 0.18 + 0.02 * (voice.age as f32 * 0.000_071).sin();
        let friction = ((bow_velocity - string_velocity) * 6.0).tanh();
        let rosined_noise = next_noise(&mut voice.noise_state) * config.noise * 0.015;
        let feedback = (current + adjacent) * 0.5 * config.damping;
        voice.waveguide[position] = (feedback + friction * 0.006 + rosined_noise).clamp(-1.0, 1.0);
        voice.waveguide_position = next;
        current
    }

    fn modal_step(voice: &mut Voice, config: &SynthConfig, sample_rate: u32) -> f32 {
        let time = voice.age as f32 / sample_rate as f32;
        let fundamental = voice.phase_increment * sample_rate as f64 / std::f64::consts::TAU;
        let mut output = 0.0_f32;
        let mut normalization = 0.0_f32;
        for index in 0..config.mode_ratios.len() {
            let ratio = config.mode_ratios[index];
            let amplitude = config.mode_amplitudes[index];
            let decay = config.mode_decays[index];
            if ratio <= 0.0
                || amplitude <= 0.0
                || decay <= 0.0
                || fundamental * ratio as f64 >= sample_rate as f64 * 0.48
            {
                continue;
            }
            let envelope = (-time / decay).exp();
            output += amplitude * envelope * (voice.phase * ratio as f64).sin() as f32;
            normalization += amplitude;
        }
        voice.phase = (voice.phase + voice.phase_increment).rem_euclid(std::f64::consts::TAU);
        output / normalization.sqrt().max(1.0)
    }

    fn synth_envelope(voice: &Voice, config: &SynthConfig, sample_rate: u32) -> f32 {
        let time = voice.age as f32 / sample_rate as f32;
        let attack = config.attack_s.max(0.001);
        let envelope = if time < attack {
            time / attack
        } else if config.model == SynthModel::Banjo {
            1.0
        } else if matches!(
            config.model,
            SynthModel::PluckedString
                | SynthModel::Guitar
                | SynthModel::Piano
                | SynthModel::StruckString
                | SynthModel::Membrane
                | SynthModel::Modal
                | SynthModel::Noise
        ) {
            (-(time - attack) / config.decay_s.max(0.03)).exp()
        } else {
            let decay = config.decay_s.max(0.01);
            let progress = (time - attack) / decay;
            config.sustain + (1.0 - config.sustain) * (-4.0 * progress).exp()
        };
        if voice.released {
            let release_frames = (config.release_s.max(0.005) * sample_rate as f32).max(1.0);
            envelope * (-6.0 * voice.release_age as f32 / release_frames).exp()
        } else {
            envelope
        }
    }

    fn synth_step(
        voice: &mut Voice,
        config: &SynthConfig,
        sample_rate: u32,
        string_profile: Option<&StringInstrumentProfile>,
        keyboard_profile: Option<&KeyboardProfile>,
        wind_profile: Option<&WindProfile>,
        percussion_profile: Option<&PercussionProfile>,
    ) -> f32 {
        if voice.trigger != Trigger::Attack || config.model == SynthModel::SampleOnly {
            return 0.0;
        }
        let age = voice.age;
        let tonal = match config.model {
            SynthModel::SampleOnly => 0.0,
            SynthModel::Additive => Self::additive_step(voice, config, sample_rate),
            SynthModel::PluckedString => {
                if let Some(profile) = string_profile {
                    Self::profiled_string_step(voice, config, sample_rate, profile)
                } else {
                    let string = Self::waveguide_step(voice, config);
                    let learned = Self::additive_step(voice, config, sample_rate);
                    string * 0.82 + learned * 0.18
                }
            }
            SynthModel::Banjo => Self::banjo_step(voice, config, sample_rate, string_profile),
            SynthModel::Guitar => {
                if let Some(profile) = string_profile {
                    Self::profiled_string_step(voice, config, sample_rate, profile)
                } else {
                    Self::guitar_step(voice, config, sample_rate)
                }
            }
            SynthModel::Piano => Self::piano_step(voice, config, sample_rate, keyboard_profile),
            SynthModel::BowedString => {
                let string = Self::bowed_step(voice, config);
                let learned = Self::additive_step(voice, config, sample_rate);
                string * 0.58 + learned * 0.42
            }
            SynthModel::StruckString => {
                let string = Self::waveguide_step(voice, config);
                let modes = Self::modal_step(voice, config, sample_rate);
                string * 0.6 + modes * 0.4
            }
            SynthModel::Wind
            | SynthModel::Reed
            | SynthModel::FreeReed
            | SynthModel::Organ
            | SynthModel::Brass => {
                if let Some(profile) = wind_profile {
                    Self::wind_step(voice, profile, sample_rate)
                } else {
                    Self::additive_step(voice, config, sample_rate) * 0.82
                }
            }
            SynthModel::Membrane | SynthModel::Modal | SynthModel::Noise => {
                if let Some(profile) = percussion_profile {
                    Self::percussion_step(voice, config, sample_rate, profile)
                } else if config.model == SynthModel::Noise {
                    0.0
                } else {
                    Self::modal_step(voice, config, sample_rate)
                }
            }
        };
        let noise = next_noise(&mut voice.noise_state);
        let noise_alpha = if matches!(
            config.model,
            SynthModel::Wind | SynthModel::Reed | SynthModel::FreeReed
        ) {
            0.18
        } else {
            0.55
        };
        voice.filtered_noise += (noise - voice.filtered_noise) * noise_alpha;
        let shaped_noise = if config.model == SynthModel::Membrane {
            noise - voice.filtered_noise
        } else {
            voice.filtered_noise
        };
        let time = age as f32 / sample_rate as f32;
        let noise_envelope = (-time
            * if config.model == SynthModel::Banjo {
                180.0
            } else if config.model == SynthModel::Guitar {
                90.0
            } else if config.model == SynthModel::Piano {
                55.0
            } else if matches!(
                config.model,
                SynthModel::Noise | SynthModel::Membrane | SynthModel::Modal
            ) {
                5.0
            } else if matches!(
                config.model,
                SynthModel::Wind
                    | SynthModel::Reed
                    | SynthModel::FreeReed
                    | SynthModel::Organ
                    | SynthModel::Brass
            ) {
                1.2
            } else {
                70.0
            })
        .exp();
        let physical_noise_handled =
            keyboard_profile.is_some() || wind_profile.is_some() || percussion_profile.is_some();
        let noise_level = if config.model == SynthModel::Banjo || physical_noise_handled {
            0.0
        } else if config.model == SynthModel::Noise {
            config.noise.max(0.2)
        } else {
            config.noise
        };
        let transient = shaped_noise * noise_level * noise_envelope;
        (tonal + transient * (0.5 + config.transient_mix))
            * Self::synth_envelope(voice, config, sample_rate)
    }

    pub fn render_into(&mut self, output: &mut [f32]) {
        output.fill(0.0);
        let string_profile = self.bank.string_profile;
        let keyboard_profile = self.bank.keyboard_profile;
        let wind_profile = self.bank.wind_profile;
        let percussion_profile = self.bank.percussion_profile;
        let mut config = self.bank.synth.clone();
        let response = self.technique_modifiers;
        config.attack_s = (config.attack_s * response.attack).clamp(0.0, 30.0);
        config.decay_s = (config.decay_s * response.decay).clamp(0.0, 30.0);
        config.sustain = (config.sustain * response.sustain).clamp(0.0, 1.0);
        config.release_s = (config.release_s * response.release).clamp(0.0, 30.0);
        config.noise = (config.noise * response.noise).clamp(0.0, 1.0);
        config.brightness = (config.brightness * response.brightness).clamp(0.0, 1.0);
        config.damping = (config.damping + response.damping_delta).clamp(0.9, 1.0);
        config.transient_mix = (config.transient_mix * response.transient).clamp(0.0, 1.0);
        config.vibrato_depth_cents =
            (config.vibrato_depth_cents * response.vibrato).clamp(0.0, 100.0);
        for sample in output {
            if self.has_pending_release_samples {
                while let Some(index) = self
                    .voices
                    .iter()
                    .enumerate()
                    .filter(|(_, voice)| voice.pending_release_sample)
                    .max_by_key(|(_, voice)| voice.age)
                    .map(|(index, _)| index)
                {
                    let note = self.voices[index].note;
                    let velocity = self.voices[index].midi_velocity;
                    self.voices[index].pending_release_sample = false;
                    if let Some(region) = self.select(note, velocity, Trigger::Release) {
                        self.push_voice(note, velocity, region, Trigger::Release, None);
                    }
                }
                self.has_pending_release_samples = false;
            }
            let mut mix = 0.0;
            let mut release_was_scheduled = false;
            for voice in &mut self.voices {
                if voice.trigger == Trigger::Attack
                    && !voice.released
                    && voice.auto_release.is_some_and(|age| voice.age >= age)
                {
                    voice.released = true;
                    voice.release_age = 0;
                    voice.auto_release = None;
                    voice.pending_release_sample = true;
                    release_was_scheduled = true;
                }
                let region = &self.bank.regions[voice.region];
                let captured = Self::sample_step(voice, region, self.sample_rate);
                let sample_layer =
                    if config.model == SynthModel::SampleOnly || self.synth_level <= f32::EPSILON {
                        captured
                    } else {
                        Self::captured_residual_step(
                            voice,
                            captured,
                            region,
                            self.sample_rate,
                            config.model,
                        )
                    };
                let synth_layer = Self::synth_step(
                    voice,
                    &config,
                    self.sample_rate,
                    string_profile.as_ref(),
                    keyboard_profile.as_ref(),
                    wind_profile.as_ref(),
                    percussion_profile.as_ref(),
                );
                mix += (sample_layer * self.sample_level + synth_layer * self.synth_level)
                    * voice.gain;
                voice.age = voice.age.saturating_add(1);
                if voice.released {
                    voice.release_age = voice.release_age.saturating_add(1);
                }
            }
            self.has_pending_release_samples |= release_was_scheduled;
            if let Some(profile) = string_profile.as_ref() {
                mix += Self::shared_string_step(
                    &mut self.shared_strings,
                    profile,
                    mix,
                    self.sample_rate,
                ) * self.synth_level;
            }
            // Keep normal-level polyphony linear. A tanh applied to the whole
            // chord creates intermodulation difference tones that a note
            // analyzer can mistake for real bass notes.
            *sample = (mix * self.expression * self.master_gain).clamp(-1.0, 1.0);
        }

        self.voices.retain(|voice| {
            let region = &self.bank.regions[voice.region];
            let sample_alive = voice.position < region.samples.len().saturating_sub(2) as f64
                || (voice.trigger == Trigger::Attack
                    && !voice.released
                    && region.loop_start.is_some());
            let release_alive = !voice.released
                || voice.release_age
                    < (config.release_s.max(region.release_s) * self.sample_rate as f32 * 4.0)
                        as usize;
            let synth_alive = match config.model {
                SynthModel::SampleOnly => false,
                SynthModel::PluckedString | SynthModel::Banjo | SynthModel::Guitar => {
                    voice.age
                        < (self.sample_rate as f32 * (config.decay_s * 8.0).clamp(1.0, 12.0))
                            as usize
                        && release_alive
                }
                SynthModel::StruckString
                | SynthModel::Piano
                | SynthModel::Membrane
                | SynthModel::Modal => {
                    voice.age
                        < (self.sample_rate as f32 * (config.decay_s * 8.0).clamp(0.2, 15.0))
                            as usize
                        && release_alive
                }
                SynthModel::Additive
                | SynthModel::Noise
                | SynthModel::BowedString
                | SynthModel::Wind
                | SynthModel::Reed
                | SynthModel::FreeReed
                | SynthModel::Organ
                | SynthModel::Brass => release_alive,
            };
            if voice.trigger == Trigger::Release {
                sample_alive
            } else {
                sample_alive || synth_alive
            }
        });
    }

    pub fn render(&mut self, frames: usize) -> Vec<f32> {
        let mut output = vec![0.0; frames];
        self.render_into(&mut output);
        output
    }

    pub fn active_voice_count(&self) -> usize {
        self.voices.len()
    }
}

pub fn bake_region(bank: &InstrumentBank, region_index: usize) -> io::Result<Wav> {
    let region = bank
        .regions
        .get(region_index)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "region index out of range"))?;
    if region.trigger == Trigger::Release {
        let output_rate = region.sample_rate;
        let mut engine = Engine::new(bank.clone(), output_rate);
        engine.push_voice(region.root_key, 127, region_index, Trigger::Release, None);
        return Ok(Wav {
            sample_rate: output_rate,
            samples: engine.render(region.samples.len().max(64)),
        });
    }

    let output_rate = region.sample_rate;
    let sample_frames = region.samples.len();
    let modeled_frames = if bank.synth.synth_level <= f32::EPSILON {
        0
    } else {
        match bank.synth.model {
            SynthModel::SampleOnly => 0,
            SynthModel::PluckedString | SynthModel::Banjo | SynthModel::Guitar => {
                (output_rate as f32 * (bank.synth.decay_s * 6.0).clamp(0.5, 8.0)) as usize
            }
            SynthModel::StruckString
            | SynthModel::Piano
            | SynthModel::Membrane
            | SynthModel::Modal => {
                (output_rate as f32 * (bank.synth.decay_s * 8.0).clamp(0.3, 15.0)) as usize
            }
            SynthModel::Additive
            | SynthModel::Noise
            | SynthModel::BowedString
            | SynthModel::Wind
            | SynthModel::Reed
            | SynthModel::FreeReed
            | SynthModel::Organ
            | SynthModel::Brass => {
                (output_rate as f32
                    * (bank.synth.attack_s + bank.synth.decay_s * 2.0 + bank.synth.release_s * 4.0)
                        .clamp(0.5, 8.0)) as usize
            }
        }
    };
    let hold_frames = if region.loop_start.is_some() {
        sample_frames
    } else {
        sample_frames.max(modeled_frames / 2)
    };
    let total_frames = sample_frames.max(modeled_frames).max(64);
    let mut engine = Engine::new(bank.clone(), output_rate);
    // SoundFont baking addresses regions directly, so use the region's own
    // technique response rather than the bank's default articulation.
    let _ = engine.set_articulation(&region.articulation);
    engine.push_voice(
        region.root_key,
        127,
        region_index,
        Trigger::Attack,
        Some(hold_frames),
    );
    Ok(Wav {
        sample_rate: output_rate,
        samples: engine.render(total_frames),
    })
}

fn midi_frequency(note: f32) -> f32 {
    440.0 * 2.0_f32.powf((note - 69.0) / 12.0)
}

fn next_noise(state: &mut u64) -> f32 {
    let mut value = *state;
    value ^= value << 13;
    value ^= value >> 7;
    value ^= value << 17;
    *state = value;
    ((value >> 40) as u32 as f32 / 8_388_608.0) - 1.0
}

fn make_waveguide(
    sample_rate: u32,
    frequency: f32,
    pluck_position: f32,
    noise: f32,
    mut state: u64,
    excitation_profile: WaveguideExcitation<'_>,
) -> Vec<f32> {
    let model = excitation_profile.model;
    let harmonics = excitation_profile.harmonics;
    let contact_hardness = excitation_profile.contact_hardness;
    let string_profile = excitation_profile.string_profile;
    let phase_correction = if matches!(
        model,
        SynthModel::PluckedString
            | SynthModel::Banjo
            | SynthModel::Guitar
            | SynthModel::Piano
            | SynthModel::StruckString
    ) {
        0.5
    } else {
        0.0
    };
    let ideal_length = sample_rate as f32 / frequency.max(8.0);
    let length = if model == SynthModel::Banjo {
        ideal_length.ceil()
    } else {
        (ideal_length - phase_correction).round()
    }
    .clamp(2.0, sample_rate as f32 / 8.0) as usize;
    let pluck_position = pluck_position.clamp(0.01, 0.99);
    let mut waveguide = Vec::with_capacity(length);
    let mut previous_noise = 0.0_f32;
    for index in 0..length {
        let position = index as f32 / length as f32;
        let triangle = if position <= pluck_position {
            position / pluck_position
        } else {
            (1.0 - position) / (1.0 - pluck_position)
        };
        let random = next_noise(&mut state);
        let high_passed = random - previous_noise * 0.78;
        previous_noise = random;
        let excitation = match model {
            SynthModel::Banjo => {
                let distance = (position - pluck_position) / 0.028;
                let pick_contact = (-distance * distance).exp();
                // Initial displacement and velocity are spatial profiles of a
                // real pluck, so all partial phases are coherent. Noise only
                // represents finite finger/pick contact and never becomes a
                // sustained source.
                let hardness = contact_hardness.clamp(0.0, 1.0);
                let learned_weight = harmonics.iter().sum::<f32>().max(1e-6);
                let learned_excitation = harmonics
                    .iter()
                    .enumerate()
                    .take_while(|(index, _)| {
                        frequency * ((*index + 1) as f32) < sample_rate as f32 * 0.45
                    })
                    .map(|(index, amplitude)| {
                        let partial = (index + 1) as f32;
                        amplitude
                            * (std::f32::consts::TAU * partial * (position - pluck_position)).cos()
                    })
                    .sum::<f32>()
                    / learned_weight;
                triangle * (0.30 - hardness * 0.05)
                    + high_passed * (0.13 + hardness * 0.10 + noise * 0.08)
                    + pick_contact * (random * (0.03 + hardness * 0.06) + 0.25 + hardness * 0.08)
                    + learned_excitation * 0.34
            }
            SynthModel::Guitar => {
                let material_tone = string_profile.map_or(0.65, |profile| match profile.material {
                    StringMaterial::Steel | StringMaterial::PhosphorBronze => 0.78,
                    StringMaterial::Nickel => 0.72,
                    StringMaterial::Nylon | StringMaterial::Fluorocarbon => 0.42,
                    StringMaterial::Gut => 0.34,
                    StringMaterial::SilkSteel => 0.55,
                });
                let contact_width = 0.020 + (1.0 - contact_hardness) * 0.060;
                let distance = (position - pluck_position) / contact_width;
                let contact = (-distance * distance).exp();
                triangle * (0.72 - material_tone * 0.22)
                    + high_passed * (0.035 + material_tone * 0.14 + noise * 0.24)
                    + contact * random * (0.025 + contact_hardness * 0.075)
            }
            SynthModel::Piano | SynthModel::StruckString => {
                let distance = (position - pluck_position) / 0.055;
                let hammer = (-distance * distance).exp();
                hammer * (random * 0.82 + 0.38) + triangle * 0.10
            }
            SynthModel::BowedString => random * 0.025,
            _ => triangle + random * noise,
        };
        waveguide.push(excitation);
    }
    let mean = waveguide.iter().sum::<f32>() / waveguide.len() as f32;
    let peak = waveguide
        .iter()
        .map(|value| (*value - mean).abs())
        .fold(0.0_f32, f32::max)
        .max(1e-6);
    for value in &mut waveguide {
        *value = (*value - mean) * (0.82 / peak);
    }
    waveguide
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::InstrumentBank;

    fn bank() -> InstrumentBank {
        let samples: Vec<f32> = (0..4_410)
            .map(|index| {
                (std::f32::consts::TAU * 220.0 * index as f32 / 44_100.0).sin()
                    * (1.0 - index as f32 / 4_410.0)
            })
            .collect();
        InstrumentBank::single(
            "test banjo",
            &Wav {
                sample_rate: 44_100,
                samples,
            },
        )
    }

    fn energy(samples: &[f32]) -> f32 {
        samples.iter().map(|sample| sample * sample).sum()
    }

    #[test]
    fn sample_and_model_layers_work_independently() {
        let bank = bank();
        let mut samples = Engine::new(bank.clone(), 44_100);
        samples.set_mix(1.0, 0.0);
        assert_ne!(samples.note_on(57, 100), 0);
        let sample_audio = samples.render(2_000);

        let mut model = Engine::new(bank, 44_100);
        model.set_mix(0.0, 1.0);
        assert_ne!(model.note_on(57, 100), 0);
        let model_audio = model.render(2_000);

        assert!(energy(&sample_audio) > 0.01);
        assert!(energy(&model_audio) > 0.01);
        assert_ne!(sample_audio, model_audio);
    }

    #[test]
    fn modeled_bank_accepts_recommended_techniques_without_dedicated_samples() {
        let mut bank = bank();
        bank.name = "Hammered Dulcimer".into();
        bank.family = "struck_string".into();
        bank.synth.model = SynthModel::Piano;
        bank.default_articulation = "sustain".into();
        bank.regions[0].articulation = "sustain".into();

        let mut engine = Engine::new(bank, 44_100);
        assert!(engine.set_articulation("wood mallet"));
        assert_eq!(engine.articulation(), "hard");
        assert_ne!(engine.note_on(57, 100), 0);
        assert!(energy(&engine.render(512)) > 0.0);
    }

    #[test]
    fn notes_and_chords_are_polyphonic() {
        let mut engine = Engine::new(bank(), 44_100);
        assert_ne!(engine.note_on(60, 100), 0);
        assert_ne!(engine.note_on(64, 100), 0);
        assert_ne!(engine.note_on(67, 100), 0);
        assert_eq!(engine.active_voice_count(), 3);
        assert!(energy(&engine.render(512)) > 0.0);
    }

    #[test]
    fn invalid_midi_byte_does_not_index_round_robin_table() {
        let mut engine = Engine::new(bank(), 44_100);
        assert_eq!(engine.note_on(200, 100), 0);
        engine.note_off(200);
    }

    #[test]
    fn voice_ids_wrap_without_colliding_in_the_stable_c_range() {
        let mut engine = Engine::new(bank(), 44_100);
        assert_eq!(engine.note_on(57, 100), 1);
        engine.next_id = i32::MAX as u64;
        let first = engine.note_on(57, 100);
        let second = engine.note_on(57, 100);
        assert_eq!(first, i32::MAX as u64);
        assert_eq!(second, 2);
    }

    #[test]
    fn release_samples_require_a_live_note_and_follow_its_velocity() {
        let mut bank = bank();
        let mut soft_release = bank.regions[0].clone();
        soft_release.name = "soft release".into();
        soft_release.trigger = Trigger::Release;
        soft_release.vel_lo = 1;
        soft_release.vel_hi = 63;
        let mut hard_release = soft_release.clone();
        hard_release.name = "hard release".into();
        hard_release.vel_lo = 64;
        hard_release.vel_hi = 127;
        bank.regions.extend([soft_release, hard_release]);

        let mut engine = Engine::new(bank, 44_100);
        engine.note_off(57);
        assert_eq!(engine.active_voice_count(), 0);

        assert_ne!(engine.note_on(57, 40), 0);
        engine.note_off(57);
        let release = engine
            .voices
            .iter()
            .find(|voice| voice.trigger == Trigger::Release)
            .expect("release voice");
        assert_eq!(release.midi_velocity, 40);
        assert_eq!(engine.bank.regions[release.region].name, "soft release");
    }

    #[test]
    fn voice_off_selects_the_matching_release_layer() {
        let mut bank = bank();
        let mut soft_release = bank.regions[0].clone();
        soft_release.name = "soft release".into();
        soft_release.trigger = Trigger::Release;
        soft_release.vel_lo = 1;
        soft_release.vel_hi = 63;
        let mut hard_release = soft_release.clone();
        hard_release.name = "hard release".into();
        hard_release.vel_lo = 64;
        hard_release.vel_hi = 127;
        bank.regions.extend([soft_release, hard_release]);

        let mut engine = Engine::new(bank, 44_100);
        let id = engine.note_on(57, 112);
        assert!(engine.voice_off(id));
        let release = engine
            .voices
            .iter()
            .find(|voice| voice.trigger == Trigger::Release)
            .expect("release voice");
        assert_eq!(release.midi_velocity, 112);
        assert_eq!(engine.bank.regions[release.region].name, "hard release");
    }

    #[test]
    fn scheduled_note_duration_triggers_a_release_sample() {
        let mut bank = bank();
        let mut release = bank.regions[0].clone();
        release.name = "scheduled release".into();
        release.trigger = Trigger::Release;
        bank.regions.push(release);

        let mut engine = Engine::new(bank, 44_100);
        assert_ne!(engine.note_on_for(57, 96, Some(1)), 0);
        engine.render_into(&mut [0.0; 4]);
        assert!(engine.voices.iter().any(|voice| {
            voice.trigger == Trigger::Release
                && engine.bank.regions[voice.region].name == "scheduled release"
        }));
    }

    #[test]
    fn baked_release_follows_configured_sample_level() {
        let mut full_level = bank();
        let mut release = full_level.regions[0].clone();
        release.name = "baked release".into();
        release.trigger = Trigger::Release;
        full_level.regions.push(release);
        full_level.synth.sample_level = 1.0;
        full_level.synth.master_gain = 1.0;
        let full = bake_region(&full_level, 1).unwrap();

        let mut quarter_level = full_level.clone();
        quarter_level.synth.sample_level = 0.25;
        let quiet = bake_region(&quarter_level, 1).unwrap();
        let maximum = |samples: &[f32]| {
            samples
                .iter()
                .fold(0.0_f32, |peak, sample| peak.max(sample.abs()))
        };
        let ratio = maximum(&quiet.samples) / maximum(&full.samples);

        assert!((ratio - 0.25).abs() < 1e-4, "{ratio}");
    }

    #[test]
    fn hybrid_capture_removes_the_steady_pitched_sample_component() {
        let mut physical_bank = bank();
        physical_bank.synth.model = SynthModel::Guitar;
        // A captured attack contains a short broadband contact burst in
        // addition to its pitched decay. A pure sine onset is tonal content,
        // so a deliberately steep residual filter should remove it.
        for index in 96..192 {
            let decay = 1.0 - (index - 96) as f32 / 96.0;
            physical_bank.regions[0].samples[index] +=
                if index % 2 == 0 { 0.35 } else { -0.35 } * decay;
        }

        let mut full_sample = Engine::new(physical_bank.clone(), 44_100);
        full_sample.set_mix(1.0, 0.0);
        assert_ne!(full_sample.note_on(57, 100), 0);
        let full_audio = full_sample.render(4_000);

        let mut residual = Engine::new(physical_bank, 44_100);
        residual.set_mix(1.0, 0.000_1);
        assert_ne!(residual.note_on(57, 100), 0);
        let residual_audio = residual.render(4_000);

        let full_tail = energy(&full_audio[2_200..3_500]);
        let residual_tail = energy(&residual_audio[2_200..3_500]);
        assert!(energy(&residual_audio[..700]) > 0.01);
        assert!(
            residual_tail < full_tail * 0.10,
            "full_tail={full_tail}, residual_tail={residual_tail}"
        );
    }

    #[test]
    fn hybrid_capture_strips_lower_accompaniment_from_tonal_regions() {
        let mut physical_bank = bank();
        physical_bank.synth.model = SynthModel::Piano;
        for (index, sample) in physical_bank.regions[0].samples.iter_mut().enumerate() {
            let phase = std::f32::consts::TAU * index as f32 / 44_100.0;
            *sample = 0.55 * (220.0 * phase).sin() + 0.45 * (110.0 * phase).sin();
        }

        let mut full_sample = Engine::new(physical_bank.clone(), 44_100);
        full_sample.set_mix(1.0, 0.0);
        assert_ne!(full_sample.note_on(57, 100), 0);
        let full_audio = full_sample.render(4_000);

        let mut residual = Engine::new(physical_bank, 44_100);
        residual.set_mix(1.0, 0.000_1);
        assert_ne!(residual.note_on(57, 100), 0);
        let residual_audio = residual.render(4_000);

        let full_tail = energy(&full_audio[2_500..3_700]);
        let residual_tail = energy(&residual_audio[2_500..3_700]);
        assert!(
            residual_tail < full_tail * 0.005,
            "full_tail={full_tail}, residual_tail={residual_tail}"
        );
    }

    #[test]
    fn hybrid_capture_falls_back_to_the_model_far_from_its_pitch_anchor() {
        let mut physical_bank = bank();
        physical_bank.synth.model = SynthModel::Piano;
        physical_bank.regions[0].key_lo = 0;
        physical_bank.regions[0].key_hi = 127;

        let far_note = physical_bank.regions[0].root_key.saturating_add(24);
        let mut hybrid = Engine::new(physical_bank.clone(), 44_100);
        hybrid.set_mix(1.0, 1.0);
        assert_ne!(hybrid.note_on(far_note, 100), 0);
        let hybrid_audio = hybrid.render(4_000);

        let mut model = Engine::new(physical_bank, 44_100);
        model.set_mix(0.0, 1.0);
        assert_ne!(model.note_on(far_note, 100), 0);
        let model_audio = model.render(4_000);

        let maximum_difference = hybrid_audio
            .iter()
            .zip(&model_audio)
            .map(|(left, right)| (left - right).abs())
            .fold(0.0_f32, f32::max);
        assert!(maximum_difference < 1e-6, "{maximum_difference}");
    }

    #[test]
    fn nearest_pitch_anchor_wins_over_a_closer_velocity_layer() {
        let mut bank = bank();
        let mut recorded_g1 = bank.regions[0].clone();
        recorded_g1.name = "recorded G1".into();
        recorded_g1.root_key = 31;
        recorded_g1.key_lo = 0;
        recorded_g1.key_hi = 127;
        recorded_g1.vel_lo = 100;
        recorded_g1.vel_hi = 127;

        let mut medium_f_sharp_2 = bank.regions[0].clone();
        medium_f_sharp_2.name = "medium F#2".into();
        medium_f_sharp_2.root_key = 42;
        medium_f_sharp_2.key_lo = 0;
        medium_f_sharp_2.key_hi = 127;
        medium_f_sharp_2.vel_lo = 1;
        medium_f_sharp_2.vel_hi = 99;

        bank.regions = vec![medium_f_sharp_2, recorded_g1];
        let mut engine = Engine::new(bank, 44_100);
        assert_ne!(engine.note_on(31, 80), 0);
        assert_eq!(engine.bank.regions[engine.voices[0].region].root_key, 31);
    }

    #[test]
    fn baked_region_contains_hybrid_audio() {
        let bank = bank();
        let baked = bake_region(&bank, 0).unwrap();
        assert_eq!(baked.sample_rate, 44_100);
        assert!(energy(&baked.samples) > 0.01);
        assert!(baked.samples.len() >= bank.regions[0].samples.len());
    }

    #[test]
    fn muted_model_does_not_pad_the_baked_sample_tail() {
        let mut bank = bank();
        bank.synth.model = SynthModel::Piano;
        bank.synth.sample_level = 1.0;
        bank.synth.synth_level = 0.0;
        bank.synth.decay_s = 0.1;
        let sample_frames = bank.regions[0].samples.len().max(64);

        let sample_only = bake_region(&bank, 0).unwrap();
        assert_eq!(sample_only.samples.len(), sample_frames);

        bank.synth.synth_level = 1.0;
        let hybrid = bake_region(&bank, 0).unwrap();
        assert!(hybrid.samples.len() > sample_only.samples.len());
    }

    #[test]
    fn banjo_contacts_and_baked_articulations_are_not_relabels() {
        let bank = InstrumentBank::generator("Five-string Banjo");
        let root = 62;
        let index_for = |articulation: &str| {
            bank.regions
                .iter()
                .position(|region| region.articulation == articulation && region.root_key == root)
                .unwrap()
        };
        let pluck = bake_region(&bank, index_for("pluck")).unwrap();
        let clawhammer = bake_region(&bank, index_for("clawhammer")).unwrap();
        let difference: f32 = pluck
            .samples
            .iter()
            .zip(&clawhammer.samples)
            .take(4_410)
            .map(|(left, right)| (left - right).abs())
            .sum();
        assert!(difference > 1.0, "difference={difference}");
    }

    #[test]
    fn banjo_physical_and_contact_controls_change_the_audio() {
        let base = InstrumentBank::generator("Five-string Banjo");
        let render = |bank: InstrumentBank| {
            let mut engine = Engine::new(bank, 44_100);
            assert_ne!(engine.note_on_for(62, 112, Some(4_410)), 0);
            engine.render(8_820)
        };
        let difference = |left: &[f32], right: &[f32]| {
            left.iter()
                .zip(right)
                .map(|(left, right)| (left - right).abs())
                .sum::<f32>()
        };

        let mut dry_body = base.clone();
        dry_body.string_profile.as_mut().unwrap().body_mix = 0.2;
        let mut resonant_body = base.clone();
        resonant_body.string_profile.as_mut().unwrap().body_mix = 4.8;
        assert!(difference(&render(dry_body), &render(resonant_body)) > 1.0);

        let mut short_scale = base.clone();
        short_scale.string_profile.as_mut().unwrap().scale_length_m = 0.35;
        let mut long_scale = base.clone();
        long_scale.string_profile.as_mut().unwrap().scale_length_m = 1.0;
        assert!(difference(&render(short_scale), &render(long_scale)) > 1.0);

        let mut clean_contact = base.clone();
        clean_contact.synth.noise = 0.0;
        let mut noisy_contact = base;
        noisy_contact.synth.noise = 0.30;
        assert!(difference(&render(clean_contact), &render(noisy_contact)) > 0.1);
    }

    #[test]
    fn high_banjo_notes_are_not_reported_an_octave_low() {
        let mut engine = Engine::new(InstrumentBank::generator("Five-string Banjo"), 44_100);
        assert_ne!(engine.note_on_for(91, 110, Some(26_460)), 0);
        let rendered = engine.render(35_280);
        let detected = crate::analyze::identify_pitch(&rendered, 44_100).expect("stable pitch");
        assert_eq!(detected.midi, 91, "{detected:?}");
        assert!(detected.cents.abs() <= 50.0, "{detected:?}");
    }

    #[test]
    fn generator_bank_is_audible_without_recorded_samples() {
        let bank = InstrumentBank::generator("Five-string Banjo");
        assert_eq!(bank.synth.sample_level, 0.0);
        assert_eq!(bank.synth.synth_level, 1.0);
        assert!(bank
            .regions
            .iter()
            .flat_map(|region| &region.samples)
            .all(|sample| *sample == 0.0));

        let mut silent_layer = Engine::new(bank.clone(), 44_100);
        silent_layer.set_mix(1.0, 0.0);
        assert_ne!(silent_layer.note_on(67, 110), 0);
        assert_eq!(energy(&silent_layer.render(22_050)), 0.0);

        let mut generator = Engine::new(bank, 44_100);
        generator.set_mix(0.0, 1.0);
        assert_ne!(generator.note_on(67, 110), 0);
        let rendered = generator.render(44_100);
        let early = energy(&rendered[..4_410]);
        let late = energy(&rendered[22_050..35_280]);
        assert!(early > 0.1);
        assert!(early > late * 4.0, "early={early}, late={late}");
    }

    #[test]
    fn major_generator_models_produce_distinct_audio() {
        let names = [
            "Five-string Banjo",
            "Acoustic Guitar",
            "Concert Grand Piano",
            "Concert Flute",
            "Clarinet",
            "Trumpet",
            "Accordion",
            "Pipe Organ",
            "Timpani",
            "Marimba",
            "Shaker",
        ];
        let mut fingerprints = Vec::new();
        for name in names {
            let bank = InstrumentBank::generator(name);
            let mut engine = Engine::new(bank, 44_100);
            assert_ne!(engine.note_on_for(60, 100, Some(8_820)), 0, "{name}");
            let audio = engine.render(13_230);
            assert!(energy(&audio) > 0.001, "{name}");
            fingerprints.push(
                audio
                    .chunks_exact(441)
                    .take(16)
                    .map(energy)
                    .collect::<Vec<_>>(),
            );
        }
        for pair in fingerprints.windows(2) {
            assert_ne!(pair[0], pair[1]);
        }
    }

    #[test]
    fn plucked_instrument_constructions_produce_distinct_audio() {
        let names = [
            "Acoustic Guitar",
            "Classical Guitar",
            "Mandolin",
            "Ukulele",
            "Lute",
        ];
        let mut fingerprints = Vec::new();
        for name in names {
            let bank = InstrumentBank::generator(name);
            assert!(bank.string_profile.is_some(), "{name}");
            let mut engine = Engine::new(bank, 44_100);
            assert_ne!(engine.note_on_for(64, 105, Some(13_230)), 0, "{name}");
            let audio = engine.render(17_640);
            assert!(energy(&audio) > 0.01, "{name}");
            fingerprints.push(
                audio
                    .chunks_exact(441)
                    .take(24)
                    .map(energy)
                    .collect::<Vec<_>>(),
            );
        }
        for pair in fingerprints.windows(2) {
            assert_ne!(pair[0], pair[1]);
        }
    }
}
