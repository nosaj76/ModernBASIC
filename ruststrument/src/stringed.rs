//! Construction profiles for real-time plucked and strummed string models.
//!
//! A profile describes the physical instrument rather than a finished sound:
//! tuning, courses, string material, scale, loss, bridge coupling, body type,
//! and sympathetic/nonlinear behaviour.  Recorded regions may still calibrate
//! the excitation spectrum, but the profile remains playable at any MIDI note.

pub const MAX_PROFILE_STRINGS: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum StringInstrumentKind {
    Generic = 0,
    Banjo = 1,
    AcousticGuitar = 2,
    ElectricGuitar = 3,
    ClassicalGuitar = 4,
    TwelveStringGuitar = 5,
    Mandolin = 6,
    Mandola = 7,
    Bouzouki = 8,
    Ukulele = 9,
    BaritoneUkulele = 10,
    Lute = 11,
    Harp = 12,
    Sitar = 13,
    Koto = 14,
    Shamisen = 15,
    Balalaika = 16,
    Dulcimer = 17,
    AcousticBass = 18,
    ElectricBass = 19,
}

impl StringInstrumentKind {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Generic,
            1 => Self::Banjo,
            2 => Self::AcousticGuitar,
            3 => Self::ElectricGuitar,
            4 => Self::ClassicalGuitar,
            5 => Self::TwelveStringGuitar,
            6 => Self::Mandolin,
            7 => Self::Mandola,
            8 => Self::Bouzouki,
            9 => Self::Ukulele,
            10 => Self::BaritoneUkulele,
            11 => Self::Lute,
            12 => Self::Harp,
            13 => Self::Sitar,
            14 => Self::Koto,
            15 => Self::Shamisen,
            16 => Self::Balalaika,
            17 => Self::Dulcimer,
            18 => Self::AcousticBass,
            19 => Self::ElectricBass,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Generic => "generic plucked string",
            Self::Banjo => "five-string banjo",
            Self::AcousticGuitar => "steel acoustic guitar",
            Self::ElectricGuitar => "electric guitar",
            Self::ClassicalGuitar => "classical nylon guitar",
            Self::TwelveStringGuitar => "twelve-string guitar",
            Self::Mandolin => "mandolin",
            Self::Mandola => "mandola/octave mandolin",
            Self::Bouzouki => "bouzouki",
            Self::Ukulele => "ukulele",
            Self::BaritoneUkulele => "baritone ukulele",
            Self::Lute => "lute",
            Self::Harp => "harp",
            Self::Sitar => "sitar",
            Self::Koto => "koto",
            Self::Shamisen => "shamisen",
            Self::Balalaika => "balalaika",
            Self::Dulcimer => "Appalachian dulcimer",
            Self::AcousticBass => "acoustic bass guitar",
            Self::ElectricBass => "electric bass guitar",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ResonatorBody {
    GenericWood = 0,
    BanjoHead = 1,
    SteelGuitar = 2,
    ClassicalGuitar = 3,
    SolidBodyPickup = 4,
    Mandolin = 5,
    Ukulele = 6,
    Lute = 7,
    Harp = 8,
    GourdJawari = 9,
    Koto = 10,
    ShamisenSkin = 11,
    Dulcimer = 12,
    Bass = 13,
}

impl ResonatorBody {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::GenericWood,
            1 => Self::BanjoHead,
            2 => Self::SteelGuitar,
            3 => Self::ClassicalGuitar,
            4 => Self::SolidBodyPickup,
            5 => Self::Mandolin,
            6 => Self::Ukulele,
            7 => Self::Lute,
            8 => Self::Harp,
            9 => Self::GourdJawari,
            10 => Self::Koto,
            11 => Self::ShamisenSkin,
            12 => Self::Dulcimer,
            13 => Self::Bass,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::GenericWood => "generic_wood",
            Self::BanjoHead => "banjo_head_and_pot",
            Self::SteelGuitar => "steel_string_guitar_body",
            Self::ClassicalGuitar => "classical_guitar_body",
            Self::SolidBodyPickup => "solid_body_pickup",
            Self::Mandolin => "mandolin_body",
            Self::Ukulele => "ukulele_body",
            Self::Lute => "lute_bowl",
            Self::Harp => "harp_soundbox",
            Self::GourdJawari => "gourd_and_jawari_bridge",
            Self::Koto => "koto_soundboard",
            Self::ShamisenSkin => "shamisen_skin",
            Self::Dulcimer => "dulcimer_box",
            Self::Bass => "bass_body",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum StringMaterial {
    Steel = 0,
    PhosphorBronze = 1,
    Nickel = 2,
    Nylon = 3,
    Gut = 4,
    Fluorocarbon = 5,
    SilkSteel = 6,
}

impl StringMaterial {
    pub fn from_byte(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Steel,
            1 => Self::PhosphorBronze,
            2 => Self::Nickel,
            3 => Self::Nylon,
            4 => Self::Gut,
            5 => Self::Fluorocarbon,
            6 => Self::SilkSteel,
            _ => return None,
        })
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "steel" => Some(Self::Steel),
            "phosphor_bronze" | "bronze" => Some(Self::PhosphorBronze),
            "nickel" => Some(Self::Nickel),
            "nylon" => Some(Self::Nylon),
            "gut" => Some(Self::Gut),
            "fluorocarbon" | "carbon" => Some(Self::Fluorocarbon),
            "silk_steel" | "silk_and_steel" => Some(Self::SilkSteel),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Steel => "steel",
            Self::PhosphorBronze => "phosphor_bronze",
            Self::Nickel => "nickel",
            Self::Nylon => "nylon",
            Self::Gut => "gut",
            Self::Fluorocarbon => "fluorocarbon",
            Self::SilkSteel => "silk_and_steel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StringInstrumentProfile {
    pub kind: StringInstrumentKind,
    pub body: ResonatorBody,
    pub material: StringMaterial,
    pub string_count: u8,
    /// One for single strings, two for paired courses.
    pub course_voices: u8,
    /// The lowest N physical strings use an octave rather than unison mate.
    pub octave_course_strings: u8,
    pub open_string_count: u8,
    pub open_notes: [u8; MAX_PROFILE_STRINGS],
    pub range_low: u8,
    pub range_high: u8,
    pub scale_length_m: f32,
    pub tension_scale: f32,
    pub stiffness: f32,
    pub high_frequency_loss: f32,
    pub base_damping: f32,
    pub course_detune_cents: f32,
    pub bridge_coupling: f32,
    pub direct_mix: f32,
    pub body_mix: f32,
    pub sympathetic_gain: f32,
    pub nonlinear_tension: f32,
    pub pick_noise: f32,
    pub pluck_position: f32,
    pub decay_s: f32,
    pub master_gain: f32,
    pub brightness: f32,
    pub transient_mix: f32,
}

impl StringInstrumentProfile {
    pub fn for_instrument(name: &str, family: &str) -> Option<Self> {
        if family != "plucked_string" {
            return None;
        }
        let lower = name.to_ascii_lowercase();
        let kind = if lower.contains("banjo") {
            StringInstrumentKind::Banjo
        } else if lower.contains("twelve-string")
            || lower.contains("twelve string")
            || lower.contains("12-string")
            || lower.contains("12 string")
        {
            StringInstrumentKind::TwelveStringGuitar
        } else if lower.contains("baritone ukulele") || lower.contains("baritone uke") {
            StringInstrumentKind::BaritoneUkulele
        } else if lower.contains("ukulele") || lower.contains(" uke") {
            StringInstrumentKind::Ukulele
        } else if lower.contains("classical guitar")
            || lower.contains("nylon guitar")
            || lower.contains("flamenco guitar")
        {
            StringInstrumentKind::ClassicalGuitar
        } else if lower.contains("electric bass") || lower.contains("bass guitar") {
            StringInstrumentKind::ElectricBass
        } else if lower.contains("acoustic bass") {
            StringInstrumentKind::AcousticBass
        } else if lower.contains("electric guitar") {
            StringInstrumentKind::ElectricGuitar
        } else if lower.contains("guitar") {
            StringInstrumentKind::AcousticGuitar
        } else if lower.contains("mandola")
            || lower.contains("octave mandolin")
            || lower.contains("mandocello")
        {
            StringInstrumentKind::Mandola
        } else if lower.contains("mandolin") {
            StringInstrumentKind::Mandolin
        } else if lower.contains("bouzouki") {
            StringInstrumentKind::Bouzouki
        } else if lower.contains("lute") || lower.contains("oud") {
            StringInstrumentKind::Lute
        } else if lower.contains("harp") {
            StringInstrumentKind::Harp
        } else if lower.contains("sitar") {
            StringInstrumentKind::Sitar
        } else if lower.contains("koto") {
            StringInstrumentKind::Koto
        } else if lower.contains("shamisen") {
            StringInstrumentKind::Shamisen
        } else if lower.contains("balalaika") {
            StringInstrumentKind::Balalaika
        } else if lower.contains("dulcimer") {
            StringInstrumentKind::Dulcimer
        } else {
            StringInstrumentKind::Generic
        };
        Some(Self::from_kind(kind))
    }

    pub fn from_kind(kind: StringInstrumentKind) -> Self {
        let mut profile = Self {
            kind,
            body: ResonatorBody::GenericWood,
            material: StringMaterial::Steel,
            string_count: 6,
            course_voices: 1,
            octave_course_strings: 0,
            open_string_count: 6,
            open_notes: notes(&[40, 45, 50, 55, 59, 64]),
            range_low: 40,
            range_high: 88,
            scale_length_m: 0.648,
            tension_scale: 1.0,
            stiffness: 0.12,
            high_frequency_loss: 0.28,
            base_damping: 0.9964,
            course_detune_cents: 0.0,
            bridge_coupling: 0.58,
            direct_mix: 0.54,
            body_mix: 1.85,
            sympathetic_gain: 0.025,
            nonlinear_tension: 0.05,
            pick_noise: 0.07,
            pluck_position: 0.18,
            decay_s: 2.2,
            master_gain: 0.76,
            brightness: 0.68,
            transient_mix: 0.62,
        };
        match kind {
            StringInstrumentKind::Generic | StringInstrumentKind::AcousticGuitar => {
                profile.body = ResonatorBody::SteelGuitar;
                profile.material = StringMaterial::PhosphorBronze;
            }
            StringInstrumentKind::Banjo => {
                profile.body = ResonatorBody::BanjoHead;
                profile.material = StringMaterial::Steel;
                profile.string_count = 5;
                profile.open_string_count = 5;
                profile.open_notes = notes(&[50, 55, 59, 62, 67]);
                profile.range_low = 50;
                profile.range_high = 91;
                profile.scale_length_m = 0.670;
                profile.tension_scale = 0.88;
                profile.stiffness = 0.18;
                profile.high_frequency_loss = 0.15;
                profile.base_damping = 0.9895;
                profile.bridge_coupling = 0.92;
                profile.direct_mix = 0.18;
                profile.body_mix = 3.8;
                profile.sympathetic_gain = 0.060;
                profile.nonlinear_tension = 0.30;
                profile.pick_noise = 0.055;
                profile.pluck_position = 0.14;
                profile.decay_s = 0.48;
                profile.master_gain = 0.86;
                profile.brightness = 0.58;
                profile.transient_mix = 0.72;
            }
            StringInstrumentKind::ElectricGuitar => {
                profile.body = ResonatorBody::SolidBodyPickup;
                profile.material = StringMaterial::Nickel;
                profile.stiffness = 0.16;
                profile.high_frequency_loss = 0.20;
                profile.base_damping = 0.9972;
                profile.bridge_coupling = 0.32;
                profile.direct_mix = 0.88;
                profile.body_mix = 0.38;
                profile.sympathetic_gain = 0.018;
                profile.decay_s = 3.4;
                profile.master_gain = 0.68;
                profile.brightness = 0.72;
            }
            StringInstrumentKind::ClassicalGuitar => {
                profile.body = ResonatorBody::ClassicalGuitar;
                profile.material = StringMaterial::Nylon;
                profile.scale_length_m = 0.650;
                profile.tension_scale = 0.72;
                profile.stiffness = 0.035;
                profile.high_frequency_loss = 0.48;
                profile.base_damping = 0.9960;
                profile.bridge_coupling = 0.52;
                profile.direct_mix = 0.42;
                profile.body_mix = 2.15;
                profile.sympathetic_gain = 0.030;
                profile.nonlinear_tension = 0.025;
                profile.pick_noise = 0.032;
                profile.pluck_position = 0.22;
                profile.decay_s = 2.45;
                profile.brightness = 0.49;
                profile.transient_mix = 0.45;
            }
            StringInstrumentKind::TwelveStringGuitar => {
                profile.body = ResonatorBody::SteelGuitar;
                profile.material = StringMaterial::PhosphorBronze;
                profile.string_count = 12;
                profile.course_voices = 2;
                profile.octave_course_strings = 4;
                profile.course_detune_cents = 3.5;
                profile.high_frequency_loss = 0.20;
                profile.base_damping = 0.9962;
                profile.direct_mix = 0.46;
                profile.body_mix = 1.70;
                profile.sympathetic_gain = 0.050;
                profile.pick_noise = 0.085;
                profile.decay_s = 2.35;
                profile.master_gain = 0.65;
                profile.brightness = 0.78;
            }
            StringInstrumentKind::Mandolin => {
                profile.body = ResonatorBody::Mandolin;
                profile.material = StringMaterial::PhosphorBronze;
                profile.string_count = 8;
                profile.course_voices = 2;
                profile.open_string_count = 4;
                profile.open_notes = notes(&[55, 62, 69, 76]);
                profile.range_low = 55;
                profile.range_high = 100;
                profile.scale_length_m = 0.350;
                profile.tension_scale = 1.10;
                profile.stiffness = 0.20;
                profile.high_frequency_loss = 0.13;
                profile.base_damping = 0.9938;
                profile.course_detune_cents = 2.6;
                profile.bridge_coupling = 0.74;
                profile.direct_mix = 0.42;
                profile.body_mix = 2.35;
                profile.sympathetic_gain = 0.040;
                profile.nonlinear_tension = 0.09;
                profile.pick_noise = 0.10;
                profile.pluck_position = 0.12;
                profile.decay_s = 1.15;
                profile.master_gain = 0.70;
                profile.brightness = 0.86;
                profile.transient_mix = 0.78;
            }
            StringInstrumentKind::Mandola => {
                profile = Self::from_kind(StringInstrumentKind::Mandolin);
                profile.kind = kind;
                profile.open_notes = notes(&[48, 55, 62, 69]);
                profile.range_low = 48;
                profile.range_high = 91;
                profile.scale_length_m = 0.510;
                profile.base_damping = 0.9950;
                profile.brightness = 0.73;
                profile.decay_s = 1.55;
            }
            StringInstrumentKind::Bouzouki => {
                profile = Self::from_kind(StringInstrumentKind::Mandolin);
                profile.kind = kind;
                profile.open_notes = notes(&[43, 50, 57, 62]);
                profile.range_low = 43;
                profile.range_high = 88;
                profile.scale_length_m = 0.660;
                profile.base_damping = 0.9955;
                profile.course_detune_cents = 3.2;
                profile.brightness = 0.76;
                profile.decay_s = 1.85;
            }
            StringInstrumentKind::Ukulele => {
                profile.body = ResonatorBody::Ukulele;
                profile.material = StringMaterial::Fluorocarbon;
                profile.string_count = 4;
                profile.open_string_count = 4;
                profile.open_notes = notes(&[67, 60, 64, 69]);
                profile.range_low = 60;
                profile.range_high = 93;
                profile.scale_length_m = 0.381;
                profile.tension_scale = 0.66;
                profile.stiffness = 0.025;
                profile.high_frequency_loss = 0.44;
                profile.base_damping = 0.9952;
                profile.bridge_coupling = 0.52;
                profile.direct_mix = 0.44;
                profile.body_mix = 2.10;
                profile.sympathetic_gain = 0.022;
                profile.nonlinear_tension = 0.035;
                profile.pick_noise = 0.028;
                profile.pluck_position = 0.23;
                profile.decay_s = 1.25;
                profile.master_gain = 0.74;
                profile.brightness = 0.55;
                profile.transient_mix = 0.48;
            }
            StringInstrumentKind::BaritoneUkulele => {
                profile = Self::from_kind(StringInstrumentKind::Ukulele);
                profile.kind = kind;
                profile.open_notes = notes(&[50, 55, 59, 64]);
                profile.range_low = 50;
                profile.range_high = 86;
                profile.scale_length_m = 0.510;
                profile.base_damping = 0.9958;
                profile.decay_s = 1.55;
                profile.brightness = 0.50;
            }
            StringInstrumentKind::Lute => {
                profile.body = ResonatorBody::Lute;
                profile.material = StringMaterial::Gut;
                profile.string_count = 11;
                profile.course_voices = 2;
                profile.octave_course_strings = 3;
                profile.open_string_count = 6;
                profile.open_notes = notes(&[41, 46, 50, 55, 59, 64]);
                profile.range_low = 41;
                profile.range_high = 88;
                profile.scale_length_m = 0.600;
                profile.tension_scale = 0.64;
                profile.stiffness = 0.025;
                profile.high_frequency_loss = 0.52;
                profile.base_damping = 0.9956;
                profile.course_detune_cents = 2.2;
                profile.bridge_coupling = 0.48;
                profile.direct_mix = 0.38;
                profile.body_mix = 2.25;
                profile.sympathetic_gain = 0.032;
                profile.nonlinear_tension = 0.025;
                profile.pick_noise = 0.040;
                profile.pluck_position = 0.21;
                profile.decay_s = 1.70;
                profile.master_gain = 0.70;
                profile.brightness = 0.45;
                profile.transient_mix = 0.48;
            }
            StringInstrumentKind::Harp => {
                profile.body = ResonatorBody::Harp;
                profile.material = StringMaterial::Gut;
                profile.string_count = 47;
                profile.open_string_count = 16;
                profile.open_notes = notes(&[
                    24, 29, 33, 36, 41, 45, 48, 53, 57, 60, 65, 69, 72, 77, 81, 84,
                ]);
                profile.range_low = 24;
                profile.range_high = 103;
                profile.scale_length_m = 1.20;
                profile.tension_scale = 0.90;
                profile.stiffness = 0.045;
                profile.high_frequency_loss = 0.34;
                profile.base_damping = 0.9982;
                profile.bridge_coupling = 0.44;
                profile.direct_mix = 0.46;
                profile.body_mix = 2.50;
                profile.sympathetic_gain = 0.085;
                profile.nonlinear_tension = 0.020;
                profile.pick_noise = 0.025;
                profile.pluck_position = 0.24;
                profile.decay_s = 4.2;
                profile.master_gain = 0.64;
                profile.brightness = 0.61;
                profile.transient_mix = 0.36;
            }
            StringInstrumentKind::Sitar => {
                profile.body = ResonatorBody::GourdJawari;
                profile.material = StringMaterial::Steel;
                profile.string_count = 18;
                profile.open_string_count = 7;
                profile.open_notes = notes(&[48, 55, 60, 67, 72, 74, 79]);
                profile.range_low = 48;
                profile.range_high = 96;
                profile.scale_length_m = 0.860;
                profile.tension_scale = 0.82;
                profile.stiffness = 0.21;
                profile.high_frequency_loss = 0.09;
                profile.base_damping = 0.9964;
                profile.bridge_coupling = 0.80;
                profile.direct_mix = 0.38;
                profile.body_mix = 2.15;
                profile.sympathetic_gain = 0.16;
                profile.nonlinear_tension = 0.12;
                profile.pick_noise = 0.13;
                profile.pluck_position = 0.09;
                profile.decay_s = 2.25;
                profile.master_gain = 0.63;
                profile.brightness = 0.92;
                profile.transient_mix = 0.82;
            }
            StringInstrumentKind::Koto => {
                profile.body = ResonatorBody::Koto;
                profile.material = StringMaterial::SilkSteel;
                profile.string_count = 13;
                profile.open_string_count = 13;
                profile.open_notes = notes(&[48, 50, 53, 55, 57, 60, 62, 65, 67, 69, 72, 74, 77]);
                profile.range_low = 48;
                profile.range_high = 96;
                profile.scale_length_m = 1.80;
                profile.tension_scale = 0.78;
                profile.stiffness = 0.08;
                profile.high_frequency_loss = 0.30;
                profile.base_damping = 0.9970;
                profile.bridge_coupling = 0.58;
                profile.direct_mix = 0.42;
                profile.body_mix = 2.25;
                profile.sympathetic_gain = 0.060;
                profile.nonlinear_tension = 0.06;
                profile.pick_noise = 0.075;
                profile.pluck_position = 0.14;
                profile.decay_s = 2.5;
                profile.brightness = 0.72;
                profile.transient_mix = 0.65;
            }
            StringInstrumentKind::Shamisen => {
                profile.body = ResonatorBody::ShamisenSkin;
                profile.material = StringMaterial::SilkSteel;
                profile.string_count = 3;
                profile.open_string_count = 3;
                profile.open_notes = notes(&[48, 55, 60]);
                profile.range_low = 48;
                profile.range_high = 88;
                profile.scale_length_m = 1.00;
                profile.tension_scale = 0.62;
                profile.stiffness = 0.07;
                profile.high_frequency_loss = 0.22;
                profile.base_damping = 0.9925;
                profile.bridge_coupling = 0.88;
                profile.direct_mix = 0.24;
                profile.body_mix = 3.0;
                profile.sympathetic_gain = 0.025;
                profile.nonlinear_tension = 0.10;
                profile.pick_noise = 0.18;
                profile.pluck_position = 0.10;
                profile.decay_s = 0.85;
                profile.master_gain = 0.68;
                profile.brightness = 0.82;
                profile.transient_mix = 0.90;
            }
            StringInstrumentKind::Balalaika => {
                profile.body = ResonatorBody::GenericWood;
                profile.material = StringMaterial::Steel;
                profile.string_count = 3;
                profile.open_string_count = 3;
                profile.open_notes = notes(&[64, 64, 69]);
                profile.range_low = 52;
                profile.range_high = 91;
                profile.scale_length_m = 0.430;
                profile.base_damping = 0.9948;
                profile.bridge_coupling = 0.66;
                profile.direct_mix = 0.46;
                profile.body_mix = 2.25;
                profile.pick_noise = 0.065;
                profile.decay_s = 1.35;
                profile.brightness = 0.70;
            }
            StringInstrumentKind::Dulcimer => {
                profile.body = ResonatorBody::Dulcimer;
                profile.material = StringMaterial::Steel;
                profile.string_count = 4;
                profile.course_voices = 2;
                profile.open_string_count = 3;
                profile.open_notes = notes(&[50, 57, 62]);
                profile.range_low = 50;
                profile.range_high = 86;
                profile.scale_length_m = 0.710;
                profile.stiffness = 0.14;
                profile.high_frequency_loss = 0.17;
                profile.base_damping = 0.9960;
                profile.course_detune_cents = 2.0;
                profile.bridge_coupling = 0.65;
                profile.direct_mix = 0.40;
                profile.body_mix = 2.35;
                profile.sympathetic_gain = 0.10;
                profile.nonlinear_tension = 0.055;
                profile.pick_noise = 0.06;
                profile.pluck_position = 0.16;
                profile.decay_s = 2.0;
                profile.brightness = 0.72;
            }
            StringInstrumentKind::AcousticBass => {
                profile.body = ResonatorBody::Bass;
                profile.material = StringMaterial::PhosphorBronze;
                profile.string_count = 4;
                profile.open_string_count = 4;
                profile.open_notes = notes(&[28, 33, 38, 43]);
                profile.range_low = 28;
                profile.range_high = 67;
                profile.scale_length_m = 0.864;
                profile.tension_scale = 1.12;
                profile.stiffness = 0.24;
                profile.high_frequency_loss = 0.46;
                profile.base_damping = 0.9978;
                profile.bridge_coupling = 0.58;
                profile.direct_mix = 0.52;
                profile.body_mix = 2.20;
                profile.sympathetic_gain = 0.032;
                profile.nonlinear_tension = 0.08;
                profile.pick_noise = 0.045;
                profile.pluck_position = 0.20;
                profile.decay_s = 3.6;
                profile.master_gain = 0.70;
                profile.brightness = 0.42;
                profile.transient_mix = 0.50;
            }
            StringInstrumentKind::ElectricBass => {
                profile = Self::from_kind(StringInstrumentKind::AcousticBass);
                profile.kind = kind;
                profile.body = ResonatorBody::SolidBodyPickup;
                profile.material = StringMaterial::Nickel;
                profile.bridge_coupling = 0.28;
                profile.direct_mix = 0.92;
                profile.body_mix = 0.32;
                profile.base_damping = 0.9982;
                profile.decay_s = 4.5;
                profile.brightness = 0.48;
            }
        }
        profile
    }

    pub fn validate(self) -> bool {
        let count = self.open_string_count as usize;
        self.string_count > 0
            && self.course_voices > 0
            && self.course_voices <= 3
            && count > 0
            && count <= MAX_PROFILE_STRINGS
            && self.octave_course_strings as usize <= count
            && self.open_notes[..count].iter().all(|note| *note <= 127)
            && self.range_low <= self.range_high
            && self.range_high <= 127
            && finite_in(self.scale_length_m, 0.15, 3.0)
            && finite_in(self.tension_scale, 0.1, 3.0)
            && finite_in(self.stiffness, 0.0, 1.0)
            && finite_in(self.high_frequency_loss, 0.0, 1.0)
            && finite_in(self.base_damping, 0.9, 1.0)
            && finite_in(self.course_detune_cents, 0.0, 50.0)
            && finite_in(self.bridge_coupling, 0.0, 2.0)
            && finite_in(self.direct_mix, 0.0, 2.0)
            && finite_in(self.body_mix, 0.0, 5.0)
            && finite_in(self.sympathetic_gain, 0.0, 1.0)
            && finite_in(self.nonlinear_tension, 0.0, 1.0)
            && finite_in(self.pick_noise, 0.0, 1.0)
            && finite_in(self.pluck_position, 0.01, 0.99)
            && finite_in(self.decay_s, 0.03, 30.0)
            && finite_in(self.master_gain, 0.0, 2.0)
            && finite_in(self.brightness, 0.0, 1.0)
            && finite_in(self.transient_mix, 0.0, 1.0)
    }

    pub fn best_string(self, note: u8) -> usize {
        let count = self.open_string_count as usize;
        let mut best = 0;
        let mut best_distance = u8::MAX;
        for (index, open) in self.open_notes[..count].iter().copied().enumerate() {
            let distance = if note >= open {
                note - open
            } else {
                (open - note).saturating_add(48)
            };
            if distance < best_distance {
                best = index;
                best_distance = distance;
            }
        }
        best
    }

    pub fn secondary_ratio(self, note: u8) -> Option<f32> {
        if self.course_voices < 2 {
            return None;
        }
        let string = self.best_string(note);
        let detune = 2.0_f32.powf(self.course_detune_cents / 1_200.0);
        if string < self.octave_course_strings as usize {
            Some(2.0 * detune)
        } else {
            Some(detune)
        }
    }
}

fn notes(values: &[u8]) -> [u8; MAX_PROFILE_STRINGS] {
    let mut output = [0; MAX_PROFILE_STRINGS];
    let count = values.len().min(output.len());
    output[..count].copy_from_slice(&values[..count]);
    output
}

fn finite_in(value: f32, low: f32, high: f32) -> bool {
    value.is_finite() && (low..=high).contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn major_profiles_have_real_construction_differences() {
        let guitar =
            StringInstrumentProfile::for_instrument("Acoustic Guitar", "plucked_string").unwrap();
        let mandolin =
            StringInstrumentProfile::for_instrument("Mandolin", "plucked_string").unwrap();
        let ukulele = StringInstrumentProfile::for_instrument("Ukulele", "plucked_string").unwrap();
        assert!(guitar.validate() && mandolin.validate() && ukulele.validate());
        assert_eq!(mandolin.course_voices, 2);
        assert_eq!(mandolin.open_notes[..4], [55, 62, 69, 76]);
        assert_eq!(ukulele.open_notes[..4], [67, 60, 64, 69]);
        assert_ne!(guitar.body, mandolin.body);
        assert_ne!(guitar.material, ukulele.material);
    }

    #[test]
    fn twelve_string_uses_octave_and_unison_courses() {
        let profile = StringInstrumentProfile::from_kind(StringInstrumentKind::TwelveStringGuitar);
        assert!(profile.secondary_ratio(40).unwrap() > 1.9);
        assert!(profile.secondary_ratio(64).unwrap() < 1.02);
    }
}
