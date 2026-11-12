//! Persisted construction profiles for struck keyboards, winds, and percussion.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum KeyboardKind {
    GrandPiano = 0,
    UprightPiano = 1,
    Harpsichord = 2,
    Clavichord = 3,
    HammeredDulcimer = 4,
    Cimbalom = 5,
    Celesta = 6,
}

impl KeyboardKind {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::GrandPiano,
            1 => Self::UprightPiano,
            2 => Self::Harpsichord,
            3 => Self::Clavichord,
            4 => Self::HammeredDulcimer,
            5 => Self::Cimbalom,
            6 => Self::Celesta,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::GrandPiano => "concert_grand",
            Self::UprightPiano => "upright_piano",
            Self::Harpsichord => "harpsichord",
            Self::Clavichord => "clavichord",
            Self::HammeredDulcimer => "hammered_dulcimer",
            Self::Cimbalom => "cimbalom",
            Self::Celesta => "celesta",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum KeyboardExciter {
    FeltHammer = 0,
    HardHammer = 1,
    Plectrum = 2,
    Tangent = 3,
    HandMallet = 4,
}

impl KeyboardExciter {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::FeltHammer,
            1 => Self::HardHammer,
            2 => Self::Plectrum,
            3 => Self::Tangent,
            4 => Self::HandMallet,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::FeltHammer => "felt_hammer",
            Self::HardHammer => "hard_hammer",
            Self::Plectrum => "plectrum",
            Self::Tangent => "tangent",
            Self::HandMallet => "hand_mallet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyboardProfile {
    pub kind: KeyboardKind,
    pub exciter: KeyboardExciter,
    pub range_low: u8,
    pub range_high: u8,
    pub strings_per_note: u8,
    pub unison_detune_cents: f32,
    pub strike_position: f32,
    pub hammer_hardness: f32,
    pub hammer_noise: f32,
    pub string_stiffness: f32,
    pub base_damping: f32,
    pub direct_mix: f32,
    pub soundboard_mix: f32,
    pub pedal_coupling: f32,
    pub decay_s: f32,
    pub release_s: f32,
    pub brightness: f32,
}

impl KeyboardProfile {
    pub fn for_instrument(name: &str, family: &str) -> Option<Self> {
        if family != "struck_string" {
            return None;
        }
        let lower = name.to_ascii_lowercase();
        let kind = if lower.contains("harpsichord") {
            KeyboardKind::Harpsichord
        } else if lower.contains("clavichord") {
            KeyboardKind::Clavichord
        } else if lower.contains("hammered dulcimer") {
            KeyboardKind::HammeredDulcimer
        } else if lower.contains("cimbalom") {
            KeyboardKind::Cimbalom
        } else if lower.contains("celesta") {
            KeyboardKind::Celesta
        } else if lower.contains("upright") {
            KeyboardKind::UprightPiano
        } else {
            KeyboardKind::GrandPiano
        };
        Some(Self::from_kind(kind))
    }

    pub fn from_kind(kind: KeyboardKind) -> Self {
        let mut profile = Self {
            kind,
            exciter: KeyboardExciter::FeltHammer,
            range_low: 21,
            range_high: 108,
            strings_per_note: 3,
            unison_detune_cents: 1.1,
            strike_position: 0.12,
            hammer_hardness: 0.48,
            hammer_noise: 0.055,
            string_stiffness: 0.22,
            base_damping: 0.9991,
            direct_mix: 0.55,
            soundboard_mix: 2.3,
            pedal_coupling: 0.14,
            decay_s: 4.8,
            release_s: 1.3,
            brightness: 0.66,
        };
        match kind {
            KeyboardKind::GrandPiano => {}
            KeyboardKind::UprightPiano => {
                profile.strings_per_note = 2;
                profile.unison_detune_cents = 1.8;
                profile.hammer_hardness = 0.58;
                profile.string_stiffness = 0.25;
                profile.base_damping = 0.9987;
                profile.soundboard_mix = 2.0;
                profile.pedal_coupling = 0.09;
                profile.decay_s = 3.4;
                profile.brightness = 0.61;
            }
            KeyboardKind::Harpsichord => {
                profile.exciter = KeyboardExciter::Plectrum;
                profile.range_low = 29;
                profile.range_high = 89;
                profile.strings_per_note = 2;
                profile.unison_detune_cents = 2.8;
                profile.strike_position = 0.18;
                profile.hammer_hardness = 0.92;
                profile.hammer_noise = 0.12;
                profile.string_stiffness = 0.19;
                profile.base_damping = 0.9954;
                profile.direct_mix = 0.62;
                profile.soundboard_mix = 1.85;
                profile.pedal_coupling = 0.0;
                profile.decay_s = 1.25;
                profile.release_s = 0.18;
                profile.brightness = 0.92;
            }
            KeyboardKind::Clavichord => {
                profile.exciter = KeyboardExciter::Tangent;
                profile.range_low = 36;
                profile.range_high = 84;
                profile.strings_per_note = 2;
                profile.unison_detune_cents = 1.4;
                profile.hammer_hardness = 0.36;
                profile.hammer_noise = 0.025;
                profile.base_damping = 0.9970;
                profile.direct_mix = 0.40;
                profile.soundboard_mix = 1.55;
                profile.decay_s = 1.6;
                profile.release_s = 0.38;
                profile.brightness = 0.50;
            }
            KeyboardKind::HammeredDulcimer | KeyboardKind::Cimbalom => {
                profile.exciter = KeyboardExciter::HandMallet;
                profile.range_low = 36;
                profile.range_high = 96;
                profile.strings_per_note = if kind == KeyboardKind::Cimbalom { 3 } else { 2 };
                profile.unison_detune_cents = 3.0;
                profile.strike_position = 0.16;
                profile.hammer_hardness = 0.82;
                profile.hammer_noise = 0.10;
                profile.string_stiffness = 0.18;
                profile.base_damping = 0.9970;
                profile.direct_mix = 0.50;
                profile.soundboard_mix = 2.15;
                profile.pedal_coupling = 0.08;
                profile.decay_s = 2.4;
                profile.release_s = 0.55;
                profile.brightness = 0.82;
            }
            KeyboardKind::Celesta => {
                profile.exciter = KeyboardExciter::HardHammer;
                profile.range_low = 48;
                profile.range_high = 96;
                profile.strings_per_note = 1;
                profile.unison_detune_cents = 0.0;
                profile.strike_position = 0.20;
                profile.hammer_hardness = 0.88;
                profile.hammer_noise = 0.045;
                profile.string_stiffness = 0.32;
                profile.base_damping = 0.9975;
                profile.direct_mix = 0.46;
                profile.soundboard_mix = 1.9;
                profile.pedal_coupling = 0.06;
                profile.decay_s = 2.2;
                profile.release_s = 0.65;
                profile.brightness = 0.86;
            }
        }
        profile
    }

    pub fn validate(self) -> bool {
        self.range_low <= self.range_high
            && self.range_high <= 127
            && (1..=3).contains(&self.strings_per_note)
            && finite(self.unison_detune_cents, 0.0, 30.0)
            && finite(self.strike_position, 0.01, 0.99)
            && finite(self.hammer_hardness, 0.0, 1.0)
            && finite(self.hammer_noise, 0.0, 1.0)
            && finite(self.string_stiffness, 0.0, 1.0)
            && finite(self.base_damping, 0.9, 1.0)
            && finite(self.direct_mix, 0.0, 2.0)
            && finite(self.soundboard_mix, 0.0, 5.0)
            && finite(self.pedal_coupling, 0.0, 1.0)
            && finite(self.decay_s, 0.03, 30.0)
            && finite(self.release_s, 0.01, 30.0)
            && finite(self.brightness, 0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum WindExciter {
    AirJet = 0,
    SingleReed = 1,
    DoubleReed = 2,
    LipReed = 3,
    FreeReed = 4,
    Pipe = 5,
}

impl WindExciter {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::AirJet,
            1 => Self::SingleReed,
            2 => Self::DoubleReed,
            3 => Self::LipReed,
            4 => Self::FreeReed,
            5 => Self::Pipe,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::AirJet => "air_jet",
            Self::SingleReed => "single_reed",
            Self::DoubleReed => "double_reed",
            Self::LipReed => "lip_reed",
            Self::FreeReed => "free_reed",
            Self::Pipe => "organ_pipe",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BoreShape {
    CylindricalOpen = 0,
    CylindricalClosed = 1,
    Conical = 2,
    OcarinaCavity = 3,
    FreeRadiating = 4,
}

impl BoreShape {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::CylindricalOpen,
            1 => Self::CylindricalClosed,
            2 => Self::Conical,
            3 => Self::OcarinaCavity,
            4 => Self::FreeRadiating,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::CylindricalOpen => "cylindrical_open",
            Self::CylindricalClosed => "cylindrical_closed",
            Self::Conical => "conical",
            Self::OcarinaCavity => "ocarina_cavity",
            Self::FreeRadiating => "free_radiating",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindProfile {
    pub exciter: WindExciter,
    pub bore: BoreShape,
    pub range_low: u8,
    pub range_high: u8,
    pub reflection: f32,
    pub breath_pressure: f32,
    pub nonlinearity: f32,
    pub breath_noise: f32,
    pub bore_loss: f32,
    pub bell_mix: f32,
    pub formant_hz: [f32; 3],
    pub formant_gain: [f32; 3],
    pub brightness: f32,
}

impl WindProfile {
    pub fn for_instrument(name: &str, family: &str) -> Option<Self> {
        if !matches!(family, "flute" | "reed" | "brass" | "free_reed" | "organ") {
            return None;
        }
        let lower = name.to_ascii_lowercase();
        let mut profile = match family {
            "flute" => Self::air_jet(),
            "reed" => Self::single_reed(),
            "brass" => Self::brass(),
            "free_reed" => Self::free_reed(),
            "organ" => Self::organ(),
            _ => unreachable!(),
        };
        if lower.contains("piccolo") {
            profile.range_low = 74;
            profile.range_high = 108;
            profile.bore_loss = 0.12;
            profile.bell_mix = 0.62;
            profile.brightness = 0.86;
            profile.formant_hz = [1_200.0, 3_000.0, 5_800.0];
        } else if lower.contains("recorder") || lower.contains("whistle") {
            profile.breath_noise = 0.045;
            profile.brightness = 0.66;
        } else if lower.contains("ocarina") {
            profile.bore = BoreShape::OcarinaCavity;
            profile.reflection = -0.72;
            profile.bore_loss = 0.30;
            profile.bell_mix = 0.34;
            profile.brightness = 0.42;
        } else if lower.contains("shakuhachi") || lower.contains("ney") {
            profile.breath_noise = 0.18;
            profile.nonlinearity = 0.72;
            profile.brightness = 0.50;
        } else if lower.contains("oboe") || lower.contains("english horn") {
            profile = Self::double_reed();
            if lower.contains("english") {
                profile.range_low = 52;
                profile.range_high = 81;
                profile.brightness = 0.62;
            }
        } else if lower.contains("bassoon") {
            profile = Self::double_reed();
            profile.range_low = if lower.contains("contra") { 22 } else { 34 };
            profile.range_high = if lower.contains("contra") { 58 } else { 75 };
            profile.formant_hz = [420.0, 1_150.0, 2_350.0];
            profile.brightness = 0.46;
        } else if lower.contains("sax") {
            profile.bore = BoreShape::Conical;
            profile.reflection = 0.80;
            profile.nonlinearity = 0.80;
            profile.bell_mix = 0.68;
            profile.formant_hz = [650.0, 1_450.0, 2_750.0];
            profile.brightness = 0.70;
            if lower.contains("baritone") {
                profile.range_low = 36;
                profile.range_high = 68;
            } else if lower.contains("tenor") {
                profile.range_low = 44;
                profile.range_high = 75;
            } else if lower.contains("alto") {
                profile.range_low = 49;
                profile.range_high = 80;
            } else {
                profile.range_low = 56;
                profile.range_high = 88;
            }
        } else if lower.contains("clarinet") {
            profile.bore = BoreShape::CylindricalClosed;
            profile.reflection = 0.88;
            profile.formant_hz = [520.0, 1_550.0, 3_050.0];
            if lower.contains("bass") {
                profile.range_low = 34;
                profile.range_high = 79;
                profile.brightness = 0.52;
            }
        } else if family == "brass" {
            if lower.contains("tuba") {
                profile.range_low = 28;
                profile.range_high = 58;
                profile.formant_hz = [280.0, 720.0, 1_600.0];
                profile.brightness = 0.42;
            } else if lower.contains("horn") {
                profile.range_low = 34;
                profile.range_high = 77;
                profile.formant_hz = [420.0, 980.0, 2_050.0];
                profile.brightness = 0.52;
            } else if lower.contains("trombone") {
                profile.range_low = 40;
                profile.range_high = 72;
                profile.formant_hz = [520.0, 1_250.0, 2_650.0];
                profile.brightness = 0.68;
            }
        } else if family == "free_reed" && lower.contains("harmonica") {
            profile.range_low = 48;
            profile.range_high = 84;
            profile.breath_noise = 0.055;
            profile.formant_hz = [750.0, 1_900.0, 3_600.0];
            profile.brightness = 0.73;
        }
        Some(profile)
    }

    fn air_jet() -> Self {
        Self {
            exciter: WindExciter::AirJet,
            bore: BoreShape::CylindricalOpen,
            range_low: 60,
            range_high: 96,
            reflection: -0.82,
            breath_pressure: 0.58,
            nonlinearity: 0.68,
            breath_noise: 0.095,
            bore_loss: 0.18,
            bell_mix: 0.52,
            formant_hz: [850.0, 2_100.0, 4_250.0],
            formant_gain: [0.18, 0.12, 0.06],
            brightness: 0.60,
        }
    }

    fn single_reed() -> Self {
        Self {
            exciter: WindExciter::SingleReed,
            bore: BoreShape::CylindricalClosed,
            range_low: 50,
            range_high: 94,
            reflection: 0.86,
            breath_pressure: 0.66,
            nonlinearity: 0.82,
            breath_noise: 0.050,
            bore_loss: 0.14,
            bell_mix: 0.58,
            formant_hz: [600.0, 1_650.0, 3_150.0],
            formant_gain: [0.24, 0.18, 0.10],
            brightness: 0.65,
        }
    }

    fn double_reed() -> Self {
        let mut value = Self::single_reed();
        value.exciter = WindExciter::DoubleReed;
        value.bore = BoreShape::Conical;
        value.range_low = 58;
        value.range_high = 91;
        value.reflection = 0.78;
        value.nonlinearity = 0.92;
        value.breath_noise = 0.035;
        value.formant_hz = [900.0, 2_200.0, 3_900.0];
        value.formant_gain = [0.34, 0.24, 0.15];
        value.brightness = 0.80;
        value
    }

    fn brass() -> Self {
        Self {
            exciter: WindExciter::LipReed,
            bore: BoreShape::Conical,
            range_low: 54,
            range_high: 82,
            reflection: 0.91,
            breath_pressure: 0.72,
            nonlinearity: 0.86,
            breath_noise: 0.022,
            bore_loss: 0.10,
            bell_mix: 0.74,
            formant_hz: [700.0, 1_800.0, 3_600.0],
            formant_gain: [0.26, 0.22, 0.14],
            brightness: 0.80,
        }
    }

    fn free_reed() -> Self {
        Self {
            exciter: WindExciter::FreeReed,
            bore: BoreShape::FreeRadiating,
            range_low: 36,
            range_high: 96,
            reflection: 0.52,
            breath_pressure: 0.60,
            nonlinearity: 0.78,
            breath_noise: 0.030,
            bore_loss: 0.16,
            bell_mix: 0.55,
            formant_hz: [650.0, 1_550.0, 3_000.0],
            formant_gain: [0.18, 0.15, 0.08],
            brightness: 0.64,
        }
    }

    fn organ() -> Self {
        Self {
            exciter: WindExciter::Pipe,
            bore: BoreShape::CylindricalOpen,
            range_low: 24,
            range_high: 108,
            reflection: -0.90,
            breath_pressure: 0.52,
            nonlinearity: 0.46,
            breath_noise: 0.008,
            bore_loss: 0.08,
            bell_mix: 0.60,
            formant_hz: [500.0, 1_250.0, 2_700.0],
            formant_gain: [0.14, 0.10, 0.06],
            brightness: 0.58,
        }
    }

    pub fn validate(self) -> bool {
        self.range_low <= self.range_high
            && self.range_high <= 127
            && finite(self.reflection, -1.0, 1.0)
            && finite(self.breath_pressure, 0.0, 1.5)
            && finite(self.nonlinearity, 0.0, 2.0)
            && finite(self.breath_noise, 0.0, 1.0)
            && finite(self.bore_loss, 0.0, 1.0)
            && finite(self.bell_mix, 0.0, 2.0)
            && self
                .formant_hz
                .iter()
                .all(|value| finite(*value, 20.0, 20_000.0))
            && self
                .formant_gain
                .iter()
                .all(|value| finite(*value, 0.0, 1.0))
            && finite(self.brightness, 0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PercussionResonator {
    Membrane = 0,
    WoodBar = 1,
    MetalBar = 2,
    Bell = 3,
    Plate = 4,
    NoiseShell = 5,
}

impl PercussionResonator {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Membrane,
            1 => Self::WoodBar,
            2 => Self::MetalBar,
            3 => Self::Bell,
            4 => Self::Plate,
            5 => Self::NoiseShell,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Membrane => "membrane",
            Self::WoodBar => "wood_bar",
            Self::MetalBar => "metal_bar",
            Self::Bell => "bell",
            Self::Plate => "plate",
            Self::NoiseShell => "noise_shell",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PercussionExciter {
    Hand = 0,
    Stick = 1,
    SoftMallet = 2,
    HardMallet = 3,
    Brush = 4,
    Shake = 5,
}

impl PercussionExciter {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Hand,
            1 => Self::Stick,
            2 => Self::SoftMallet,
            3 => Self::HardMallet,
            4 => Self::Brush,
            5 => Self::Shake,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Hand => "hand",
            Self::Stick => "stick",
            Self::SoftMallet => "soft_mallet",
            Self::HardMallet => "hard_mallet",
            Self::Brush => "brush",
            Self::Shake => "shake",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PercussionProfile {
    pub resonator: PercussionResonator,
    pub exciter: PercussionExciter,
    pub range_low: u8,
    pub range_high: u8,
    pub strike_position: f32,
    pub hardness: f32,
    pub damping: f32,
    pub transient_noise: f32,
    pub shell_noise: f32,
    pub resonator_mix: f32,
    pub decay_s: f32,
    pub brightness: f32,
}

impl PercussionProfile {
    pub fn for_instrument(name: &str, family: &str) -> Option<Self> {
        if !matches!(family, "membrane" | "metal" | "wood" | "noise_percussion") {
            return None;
        }
        let lower = name.to_ascii_lowercase();
        let mut profile = match family {
            "membrane" => Self::membrane(),
            "wood" => Self::wood(),
            "metal" => Self::metal(),
            "noise_percussion" => Self::noise(),
            _ => unreachable!(),
        };
        if lower.contains("snare") {
            profile.exciter = PercussionExciter::Stick;
            profile.hardness = 0.84;
            profile.transient_noise = 0.44;
            profile.shell_noise = 0.62;
            profile.decay_s = 0.32;
            profile.brightness = 0.88;
        } else if lower.contains("timpani") {
            profile.exciter = PercussionExciter::SoftMallet;
            profile.range_low = 36;
            profile.range_high = 57;
            profile.hardness = 0.34;
            profile.damping = 0.18;
            profile.transient_noise = 0.08;
            profile.decay_s = 1.8;
            profile.brightness = 0.38;
        } else if lower.contains("bass drum") || lower.contains("taiko") {
            profile.exciter = PercussionExciter::SoftMallet;
            profile.hardness = 0.30;
            profile.damping = 0.30;
            profile.decay_s = 1.1;
            profile.brightness = 0.34;
        } else if lower.contains("bongo")
            || lower.contains("conga")
            || lower.contains("djembe")
            || lower.contains("tabla")
        {
            profile.exciter = PercussionExciter::Hand;
            profile.strike_position = 0.38;
            profile.hardness = 0.58;
            profile.transient_noise = 0.16;
            profile.decay_s = 0.72;
            profile.brightness = 0.66;
        } else if lower.contains("marimba") {
            profile.exciter = PercussionExciter::SoftMallet;
            profile.hardness = 0.36;
            profile.decay_s = 2.4;
            profile.brightness = 0.46;
        } else if lower.contains("xylophone") || lower.contains("woodblock") {
            profile.exciter = PercussionExciter::HardMallet;
            profile.hardness = 0.88;
            profile.decay_s = 0.72;
            profile.brightness = 0.82;
        } else if lower.contains("vibraphone") {
            profile.exciter = PercussionExciter::SoftMallet;
            profile.resonator = PercussionResonator::MetalBar;
            profile.decay_s = 4.2;
            profile.brightness = 0.58;
        } else if lower.contains("glockenspiel") || lower.contains("crotales") {
            profile.exciter = PercussionExciter::HardMallet;
            profile.resonator = PercussionResonator::MetalBar;
            profile.hardness = 0.94;
            profile.decay_s = 3.8;
            profile.brightness = 0.94;
        } else if lower.contains("bell") || lower.contains("chime") {
            profile.resonator = PercussionResonator::Bell;
            profile.decay_s = 5.2;
            profile.brightness = 0.82;
        } else if lower.contains("cymbal") || lower.contains("gong") {
            profile.resonator = PercussionResonator::Plate;
            profile.hardness = 0.72;
            profile.shell_noise = 0.34;
            profile.decay_s = 6.2;
            profile.brightness = 0.80;
        } else if family == "noise_percussion" {
            profile.exciter = if lower.contains("tambourine") {
                PercussionExciter::Hand
            } else {
                PercussionExciter::Shake
            };
        }
        Some(profile)
    }

    fn membrane() -> Self {
        Self {
            resonator: PercussionResonator::Membrane,
            exciter: PercussionExciter::Stick,
            range_low: 36,
            range_high: 72,
            strike_position: 0.22,
            hardness: 0.64,
            damping: 0.24,
            transient_noise: 0.18,
            shell_noise: 0.08,
            resonator_mix: 1.0,
            decay_s: 0.68,
            brightness: 0.60,
        }
    }

    fn wood() -> Self {
        Self {
            resonator: PercussionResonator::WoodBar,
            exciter: PercussionExciter::HardMallet,
            range_low: 36,
            range_high: 96,
            strike_position: 0.29,
            hardness: 0.68,
            damping: 0.22,
            transient_noise: 0.10,
            shell_noise: 0.02,
            resonator_mix: 1.0,
            decay_s: 1.4,
            brightness: 0.64,
        }
    }

    fn metal() -> Self {
        Self {
            resonator: PercussionResonator::MetalBar,
            exciter: PercussionExciter::HardMallet,
            range_low: 36,
            range_high: 96,
            strike_position: 0.26,
            hardness: 0.80,
            damping: 0.08,
            transient_noise: 0.12,
            shell_noise: 0.05,
            resonator_mix: 1.0,
            decay_s: 3.5,
            brightness: 0.80,
        }
    }

    fn noise() -> Self {
        Self {
            resonator: PercussionResonator::NoiseShell,
            exciter: PercussionExciter::Shake,
            range_low: 36,
            range_high: 84,
            strike_position: 0.5,
            hardness: 0.62,
            damping: 0.42,
            transient_noise: 0.85,
            shell_noise: 0.72,
            resonator_mix: 0.36,
            decay_s: 0.42,
            brightness: 0.78,
        }
    }

    pub fn validate(self) -> bool {
        self.range_low <= self.range_high
            && self.range_high <= 127
            && finite(self.strike_position, 0.0, 1.0)
            && finite(self.hardness, 0.0, 1.0)
            && finite(self.damping, 0.0, 1.0)
            && finite(self.transient_noise, 0.0, 1.0)
            && finite(self.shell_noise, 0.0, 1.0)
            && finite(self.resonator_mix, 0.0, 2.0)
            && finite(self.decay_s, 0.02, 30.0)
            && finite(self.brightness, 0.0, 1.0)
    }
}

fn finite(value: f32, low: f32, high: f32) -> bool {
    value.is_finite() && (low..=high).contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn major_families_select_different_physical_profiles() {
        let piano =
            KeyboardProfile::for_instrument("Concert Grand Piano", "struck_string").unwrap();
        let harpsichord = KeyboardProfile::for_instrument("Harpsichord", "struck_string").unwrap();
        let flute = WindProfile::for_instrument("Concert Flute", "flute").unwrap();
        let clarinet = WindProfile::for_instrument("Clarinet", "reed").unwrap();
        let snare = PercussionProfile::for_instrument("Snare Drum", "membrane").unwrap();
        assert!(piano.validate() && harpsichord.validate());
        assert!(flute.validate() && clarinet.validate() && snare.validate());
        assert_ne!(piano.exciter, harpsichord.exciter);
        assert_ne!(flute.exciter, clarinet.exciter);
        assert!(snare.shell_noise > 0.5);
    }
}
