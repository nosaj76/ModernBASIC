//! Instrument-aware articulation names and model-response modifiers.
//!
//! RSTI keeps an articulation identity in each playable region. This registry
//! supplies a stable vocabulary around generated or captured performances so `pulling`,
//! `pull-off`, and `pulloff`, for example, do not become three unrelated
//! SoundFont presets.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TechniqueModifiers {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub noise: f32,
    pub brightness: f32,
    pub damping_delta: f32,
    pub transient: f32,
    pub vibrato: f32,
}

impl Default for TechniqueModifiers {
    fn default() -> Self {
        Self {
            attack: 1.0,
            decay: 1.0,
            sustain: 1.0,
            release: 1.0,
            noise: 1.0,
            brightness: 1.0,
            damping_delta: 0.0,
            transient: 1.0,
            vibrato: 1.0,
        }
    }
}

#[derive(Clone, Copy)]
struct Alias {
    canonical: &'static str,
    aliases: &'static [&'static str],
}

const BANJO: &[Alias] = &[
    Alias {
        canonical: "pluck",
        aliases: &["pluck", "plucking", "picked", "pick"],
    },
    Alias {
        canonical: "pull_off",
        aliases: &["pull_off", "pull-off", "pulloff", "pulling", "pull"],
    },
    Alias {
        canonical: "hammer_on",
        aliases: &["hammer_on", "hammer-on", "hammeron", "hammering", "hammer"],
    },
    Alias {
        canonical: "strike",
        aliases: &["strike", "striking", "struck"],
    },
    Alias {
        canonical: "strum",
        aliases: &["strum", "strumming", "brush"],
    },
    Alias {
        canonical: "single_finger",
        aliases: &[
            "single_finger",
            "single-finger",
            "singlefinger",
            "single_string",
            "single-string",
        ],
    },
    Alias {
        canonical: "three_finger",
        aliases: &[
            "three_finger",
            "three-finger",
            "3_finger",
            "3-finger",
            "threefinger",
            "scruggs",
            "roll",
        ],
    },
    Alias {
        canonical: "clawhammer",
        aliases: &["clawhammer", "frail", "frailing", "stroke_style"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo"],
    },
    Alias {
        canonical: "slide",
        aliases: &["slide", "sliding"],
    },
    Alias {
        canonical: "mute",
        aliases: &["mute", "muted"],
    },
    Alias {
        canonical: "harmonic",
        aliases: &["harmonic", "harmonics"],
    },
];

const GUITAR: &[Alias] = &[
    Alias {
        canonical: "pick",
        aliases: &[
            "pick",
            "picked",
            "plectrum",
            "downstroke",
            "upstroke",
            "alternate_pick",
        ],
    },
    Alias {
        canonical: "fingerstyle",
        aliases: &[
            "fingerstyle",
            "finger_style",
            "fingerpick",
            "fingerpicking",
            "finger",
        ],
    },
    Alias {
        canonical: "pluck",
        aliases: &["pluck", "plucking"],
    },
    Alias {
        canonical: "strum",
        aliases: &["strum", "strumming", "brush"],
    },
    Alias {
        canonical: "hammer_on",
        aliases: &["hammer_on", "hammer-on", "hammeron", "hammering"],
    },
    Alias {
        canonical: "pull_off",
        aliases: &["pull_off", "pull-off", "pulloff", "pulling"],
    },
    Alias {
        canonical: "palm_mute",
        aliases: &["palm_mute", "palm-mute", "palmmute", "palm_muted", "chug"],
    },
    Alias {
        canonical: "slide",
        aliases: &["slide", "sliding", "glissando"],
    },
    Alias {
        canonical: "bend",
        aliases: &["bend", "bending"],
    },
    Alias {
        canonical: "vibrato",
        aliases: &["vibrato"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "tremolo_pick"],
    },
    Alias {
        canonical: "harmonic",
        aliases: &[
            "harmonic",
            "harmonics",
            "natural_harmonic",
            "pinch_harmonic",
        ],
    },
    Alias {
        canonical: "mute",
        aliases: &["mute", "muted", "dead_note", "ghost_note"],
    },
];

const MANDOLIN: &[Alias] = &[
    Alias {
        canonical: "pick",
        aliases: &["pick", "picked", "plectrum", "downstroke", "upstroke"],
    },
    Alias {
        canonical: "tremolo_pick",
        aliases: &["tremolo", "tremolo_pick", "tremolo-pick", "rapid_pick"],
    },
    Alias {
        canonical: "crosspick",
        aliases: &["crosspick", "crosspicking", "cross_pick"],
    },
    Alias {
        canonical: "chop",
        aliases: &["chop", "chop_chord", "chopped", "bark"],
    },
    Alias {
        canonical: "strum",
        aliases: &["strum", "strumming", "brush"],
    },
    Alias {
        canonical: "double_stop",
        aliases: &["double_stop", "double-stop", "doublestop"],
    },
    Alias {
        canonical: "hammer_on",
        aliases: &["hammer_on", "hammer-on", "hammeron"],
    },
    Alias {
        canonical: "pull_off",
        aliases: &["pull_off", "pull-off", "pulloff"],
    },
    Alias {
        canonical: "slide",
        aliases: &["slide", "sliding"],
    },
    Alias {
        canonical: "harmonic",
        aliases: &["harmonic", "harmonics"],
    },
    Alias {
        canonical: "mute",
        aliases: &["mute", "muted", "dead_note"],
    },
];

const UKULELE: &[Alias] = &[
    Alias {
        canonical: "fingerstyle",
        aliases: &["fingerstyle", "fingerpick", "finger", "pluck"],
    },
    Alias {
        canonical: "strum",
        aliases: &["strum", "strumming", "brush"],
    },
    Alias {
        canonical: "chunk",
        aliases: &["chunk", "chuck", "chunking", "percussive_strum"],
    },
    Alias {
        canonical: "roll",
        aliases: &["roll", "finger_roll", "rasgueado"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "tremolo_pick"],
    },
    Alias {
        canonical: "hammer_on",
        aliases: &["hammer_on", "hammer-on", "hammeron"],
    },
    Alias {
        canonical: "pull_off",
        aliases: &["pull_off", "pull-off", "pulloff"],
    },
    Alias {
        canonical: "harmonic",
        aliases: &["harmonic", "harmonics"],
    },
    Alias {
        canonical: "mute",
        aliases: &["mute", "muted", "damped"],
    },
];

const LUTE_HARP: &[Alias] = &[
    Alias {
        canonical: "pluck",
        aliases: &["pluck", "plucking", "fingerstyle", "finger"],
    },
    Alias {
        canonical: "nail",
        aliases: &["nail", "fingernail", "bright_pluck"],
    },
    Alias {
        canonical: "strum",
        aliases: &["strum", "strumming", "brush"],
    },
    Alias {
        canonical: "arpeggio",
        aliases: &["arpeggio", "arpeggiated", "broken_chord"],
    },
    Alias {
        canonical: "glissando",
        aliases: &["glissando", "gliss", "sweep"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "bisbigliando"],
    },
    Alias {
        canonical: "harmonic",
        aliases: &["harmonic", "harmonics"],
    },
    Alias {
        canonical: "damped",
        aliases: &["damped", "damping", "etouffe", "mute", "muted"],
    },
];

const WORLD_PLUCKED: &[Alias] = &[
    Alias {
        canonical: "pluck",
        aliases: &["pluck", "plucking", "normal"],
    },
    Alias {
        canonical: "mizrab",
        aliases: &["mizrab", "mezrab", "wire_pick"],
    },
    Alias {
        canonical: "baji",
        aliases: &["baji", "bachi", "plectrum"],
    },
    Alias {
        canonical: "meend",
        aliases: &["meend", "pull_bend", "string_pull"],
    },
    Alias {
        canonical: "gamak",
        aliases: &["gamak", "gamakas", "ornament"],
    },
    Alias {
        canonical: "jhala",
        aliases: &["jhala", "rapid_drone"],
    },
    Alias {
        canonical: "chikari",
        aliases: &["chikari", "drone", "drone_string"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "rapid_pick"],
    },
    Alias {
        canonical: "slide",
        aliases: &["slide", "sliding"],
    },
    Alias {
        canonical: "mute",
        aliases: &["mute", "muted", "damped"],
    },
];

const DULCIMER: &[Alias] = &[
    Alias {
        canonical: "strum",
        aliases: &["strum", "strumming", "brush"],
    },
    Alias {
        canonical: "pick",
        aliases: &["pick", "picked", "plectrum"],
    },
    Alias {
        canonical: "noter_drone",
        aliases: &["noter_drone", "noter-and-drone", "drone", "noter"],
    },
    Alias {
        canonical: "pluck",
        aliases: &["pluck", "fingerstyle", "finger"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "rapid_pick"],
    },
    Alias {
        canonical: "slide",
        aliases: &["slide", "sliding"],
    },
    Alias {
        canonical: "hammer_on",
        aliases: &["hammer_on", "hammer-on", "hammeron"],
    },
    Alias {
        canonical: "pull_off",
        aliases: &["pull_off", "pull-off", "pulloff"],
    },
];

const HAMMERED_DULCIMER: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "single_strike"],
    },
    Alias {
        canonical: "soft",
        aliases: &["soft", "soft_mallet", "felt_mallet"],
    },
    Alias {
        canonical: "hard",
        aliases: &["hard", "hard_mallet", "wood_mallet", "accent"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "rapid_strike"],
    },
    Alias {
        canonical: "roll",
        aliases: &["roll", "mallet_roll"],
    },
    Alias {
        canonical: "damped",
        aliases: &["damped", "mute", "hand_damped"],
    },
    Alias {
        canonical: "arpeggio",
        aliases: &["arpeggio", "arpeggiated", "broken_chord"],
    },
];

const PIANO: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "connected"],
    },
    Alias {
        canonical: "pedal",
        aliases: &[
            "pedal",
            "damper_pedal",
            "sustain_pedal",
            "pedaled",
            "pedalled",
        ],
    },
    Alias {
        canonical: "una_corda",
        aliases: &["una_corda", "una-corda", "soft_pedal"],
    },
    Alias {
        canonical: "sostenuto",
        aliases: &["sostenuto"],
    },
    Alias {
        canonical: "key_off",
        aliases: &["key_off", "key-off", "keyoff", "release"],
    },
    Alias {
        canonical: "soft",
        aliases: &["soft", "piano", "pianissimo"],
    },
    Alias {
        canonical: "hard",
        aliases: &["hard", "forte", "fortissimo", "accent"],
    },
    Alias {
        canonical: "muted",
        aliases: &["muted", "mute", "prepared"],
    },
    Alias {
        canonical: "arpeggio",
        aliases: &["arpeggio", "arpeggiated", "broken_chord"],
    },
];

const BOWED: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "arco", "ordinario"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "slur"],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "spiccato",
        aliases: &["spiccato", "bounced"],
    },
    Alias {
        canonical: "pizzicato",
        aliases: &["pizzicato", "pizz", "pluck"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo"],
    },
    Alias {
        canonical: "sul_ponticello",
        aliases: &["sul_ponticello", "ponticello", "bridge"],
    },
    Alias {
        canonical: "sul_tasto",
        aliases: &["sul_tasto", "tasto", "flautando"],
    },
    Alias {
        canonical: "harmonic",
        aliases: &["harmonic", "harmonics", "flageolet"],
    },
    Alias {
        canonical: "col_legno",
        aliases: &["col_legno", "col-legno", "wood_bow"],
    },
];

const FLUTE: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "slur", "slurred"],
    },
    Alias {
        canonical: "tongued",
        aliases: &[
            "tongued",
            "tongue",
            "single_tongue",
            "double_tongue",
            "triple_tongue",
        ],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "flutter_tongue",
        aliases: &[
            "flutter_tongue",
            "flutter-tongue",
            "flutter",
            "fluttertongue",
        ],
    },
    Alias {
        canonical: "breathy",
        aliases: &["breathy", "breath", "airy", "air", "aeolian"],
    },
    Alias {
        canonical: "overblown",
        aliases: &["overblown", "overblow", "harmonic"],
    },
    Alias {
        canonical: "key_click",
        aliases: &["key_click", "key-click", "keyclick", "click"],
    },
    Alias {
        canonical: "trill",
        aliases: &["trill", "shake"],
    },
    Alias {
        canonical: "multiphonic",
        aliases: &["multiphonic", "multiphonics"],
    },
];

const REED: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "slur", "slurred"],
    },
    Alias {
        canonical: "tongued",
        aliases: &["tongued", "tongue", "single_tongue", "double_tongue"],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "slap_tongue",
        aliases: &["slap_tongue", "slap-tongue", "slap"],
    },
    Alias {
        canonical: "subtone",
        aliases: &["subtone", "sub_tone", "soft_reed"],
    },
    Alias {
        canonical: "growl",
        aliases: &["growl", "growling"],
    },
    Alias {
        canonical: "altissimo",
        aliases: &["altissimo", "overblown", "overblow"],
    },
    Alias {
        canonical: "flutter_tongue",
        aliases: &["flutter_tongue", "flutter-tongue", "flutter"],
    },
    Alias {
        canonical: "trill",
        aliases: &["trill", "shake"],
    },
    Alias {
        canonical: "multiphonic",
        aliases: &["multiphonic", "multiphonics"],
    },
    Alias {
        canonical: "key_click",
        aliases: &["key_click", "key-click", "keyclick", "click"],
    },
];

const BRASS: &[Alias] = &[
    Alias {
        canonical: "open",
        aliases: &["open", "sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "slur", "slurred"],
    },
    Alias {
        canonical: "tongued",
        aliases: &[
            "tongued",
            "tongue",
            "single_tongue",
            "double_tongue",
            "triple_tongue",
        ],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "marcato",
        aliases: &["marcato", "accent", "sforzando"],
    },
    Alias {
        canonical: "straight_mute",
        aliases: &["straight_mute", "straight-mute", "straightmute"],
    },
    Alias {
        canonical: "cup_mute",
        aliases: &["cup_mute", "cup-mute", "cupmute"],
    },
    Alias {
        canonical: "harmon_mute",
        aliases: &["harmon_mute", "harmon-mute", "harmon", "wah_mute"],
    },
    Alias {
        canonical: "stopped",
        aliases: &["stopped", "hand_stopped", "stop"],
    },
    Alias {
        canonical: "flutter_tongue",
        aliases: &["flutter_tongue", "flutter-tongue", "flutter"],
    },
    Alias {
        canonical: "growl",
        aliases: &["growl", "growling"],
    },
    Alias {
        canonical: "fall",
        aliases: &["fall", "falloff", "fall_off"],
    },
    Alias {
        canonical: "doit",
        aliases: &["doit", "scoop", "rip"],
    },
    Alias {
        canonical: "shake",
        aliases: &["shake", "lip_trill", "lip-trill"],
    },
];

const FREE_REED: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "connected"],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "bellows_accent",
        aliases: &["bellows_accent", "accent", "sforzando"],
    },
    Alias {
        canonical: "bellows_shake",
        aliases: &["bellows_shake", "shake", "bellows vibrato"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo", "repeated"],
    },
    Alias {
        canonical: "vibrato",
        aliases: &["vibrato", "hand vibrato"],
    },
    Alias {
        canonical: "bend",
        aliases: &["bend", "draw bend", "overbend"],
    },
];

const ORGAN: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato", "connected"],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "swell",
        aliases: &["swell", "expression", "crescendo"],
    },
    Alias {
        canonical: "tremulant",
        aliases: &["tremulant", "tremolo", "vibrato"],
    },
    Alias {
        canonical: "percussion",
        aliases: &["percussion", "key percussion"],
    },
    Alias {
        canonical: "rotary",
        aliases: &["rotary", "leslie", "chorale"],
    },
];

const PERCUSSION: &[Alias] = &[
    Alias {
        canonical: "center",
        aliases: &["center", "centre", "open", "normal"],
    },
    Alias {
        canonical: "edge",
        aliases: &["edge"],
    },
    Alias {
        canonical: "rimshot",
        aliases: &["rimshot", "rim_shot", "rim"],
    },
    Alias {
        canonical: "roll",
        aliases: &["roll", "tremolo"],
    },
    Alias {
        canonical: "brush",
        aliases: &["brush", "brushes", "sweep"],
    },
    Alias {
        canonical: "stick",
        aliases: &["stick", "sticks"],
    },
    Alias {
        canonical: "mallet",
        aliases: &["mallet", "mallets", "soft_mallet", "hard_mallet"],
    },
    Alias {
        canonical: "hand",
        aliases: &["hand", "finger", "fingers"],
    },
    Alias {
        canonical: "damped",
        aliases: &["damped", "dampen", "dampened", "choke", "muted", "mute"],
    },
    Alias {
        canonical: "bowed",
        aliases: &["bowed", "bow"],
    },
];

const GENERIC: &[Alias] = &[
    Alias {
        canonical: "sustain",
        aliases: &["sustain", "normal", "ordinario"],
    },
    Alias {
        canonical: "staccato",
        aliases: &["staccato", "short"],
    },
    Alias {
        canonical: "legato",
        aliases: &["legato"],
    },
    Alias {
        canonical: "tremolo",
        aliases: &["tremolo"],
    },
    Alias {
        canonical: "mute",
        aliases: &["mute", "muted"],
    },
    Alias {
        canonical: "slide",
        aliases: &["slide"],
    },
    Alias {
        canonical: "pluck",
        aliases: &["pluck", "pick"],
    },
];

fn normalized(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn kind(name: &str, family: &str) -> &'static str {
    match family {
        "bowed_string" => return "bowed",
        "flute" => return "flute",
        "reed" => return "reed",
        "brass" => return "brass",
        "free_reed" => return "free_reed",
        "organ" => return "organ",
        "membrane" | "metal" | "wood" | "noise_percussion" => return "percussion",
        _ => {}
    }
    let name = name.to_ascii_lowercase();
    if name.contains("banjo") {
        "banjo"
    } else if name.contains("mandolin") || name.contains("mandola") || name.contains("bouzouki") {
        "mandolin"
    } else if name.contains("ukulele") || name.contains(" uke") {
        "ukulele"
    } else if name.contains("harp") || name.contains("lute") || name.contains("oud") {
        "lute_harp"
    } else if name.contains("sitar")
        || name.contains("koto")
        || name.contains("shamisen")
        || name.contains("balalaika")
    {
        "world_plucked"
    } else if name.contains("hammered dulcimer") || name.contains("cimbalom") {
        "hammered_dulcimer"
    } else if name.contains("appalachian dulcimer") || name.contains("mountain dulcimer") {
        "dulcimer"
    } else if ["guitar", "bass"].iter().any(|word| name.contains(word)) {
        "guitar"
    } else if ["piano", "clavichord", "harpsichord", "keyboard"]
        .iter()
        .any(|word| name.contains(word))
    {
        "piano"
    } else {
        "generic"
    }
}

fn aliases(name: &str, family: &str) -> &'static [Alias] {
    match kind(name, family) {
        "banjo" => BANJO,
        "guitar" => GUITAR,
        "mandolin" => MANDOLIN,
        "ukulele" => UKULELE,
        "lute_harp" => LUTE_HARP,
        "world_plucked" => WORLD_PLUCKED,
        "dulcimer" => DULCIMER,
        "hammered_dulcimer" => HAMMERED_DULCIMER,
        "piano" => PIANO,
        "bowed" => BOWED,
        "flute" => FLUTE,
        "reed" => REED,
        "brass" => BRASS,
        "free_reed" => FREE_REED,
        "organ" => ORGAN,
        "percussion" => PERCUSSION,
        _ => GENERIC,
    }
}

pub fn canonicalize(name: &str, family: &str, technique: &str) -> String {
    let normalized_technique = normalized(technique);
    for alias in aliases(name, family) {
        if normalized_technique == normalized(alias.canonical)
            || alias
                .aliases
                .iter()
                .any(|item| normalized_technique == normalized(item))
        {
            return alias.canonical.to_string();
        }
    }
    if normalized_technique.is_empty() {
        "sustain".to_string()
    } else {
        normalized_technique
    }
}

pub fn infer_from_filename(name: &str, family: &str, filename: &str) -> String {
    let normalized_filename = normalized(filename);
    for alias in aliases(name, family) {
        if alias
            .aliases
            .iter()
            .chain(std::iter::once(&alias.canonical))
            .any(|candidate| {
                let candidate = normalized(candidate);
                normalized_filename == candidate
                    || normalized_filename.starts_with(&format!("{candidate}_"))
                    || normalized_filename.ends_with(&format!("_{candidate}"))
                    || normalized_filename.contains(&format!("_{candidate}_"))
            })
        {
            return alias.canonical.to_string();
        }
    }
    match kind(name, family) {
        "banjo" => "pluck".to_string(),
        "guitar" => "pick".to_string(),
        "mandolin" => "pick".to_string(),
        "ukulele" => "fingerstyle".to_string(),
        "lute_harp" | "world_plucked" => "pluck".to_string(),
        "dulcimer" => "strum".to_string(),
        _ if family == "plucked_string" => "pluck".to_string(),
        "brass" => "open".to_string(),
        _ => "sustain".to_string(),
    }
}

pub fn recommended(name: &str, family: &str) -> Vec<&'static str> {
    aliases(name, family)
        .iter()
        .map(|entry| entry.canonical)
        .collect()
}

pub fn modifiers(name: &str, family: &str, technique: &str) -> TechniqueModifiers {
    let technique = canonicalize(name, family, technique);
    let mut value = TechniqueModifiers::default();
    match technique.as_str() {
        "staccato" | "spiccato" => {
            value.decay = 0.32;
            value.sustain = 0.18;
            value.release = 0.28;
            value.transient = 1.35;
        }
        "legato" | "sostenuto" | "pedal" => {
            value.attack = 0.72;
            value.sustain = 1.18;
            value.release = if technique == "pedal" { 3.0 } else { 1.45 };
            value.transient = 0.35;
        }
        "pull_off" | "hammer_on" => {
            value.attack = if technique == "hammer_on" { 0.55 } else { 0.85 };
            value.decay = 0.72;
            value.sustain = 0.62;
            value.noise = 0.55;
            value.transient = 0.42;
        }
        "palm_mute" | "mute" | "muted" | "damped" | "chunk" | "chop" => {
            value.decay = 0.28;
            value.sustain = 0.16;
            value.release = 0.25;
            value.brightness = 0.68;
            value.damping_delta = -0.012;
            value.transient = 0.8;
        }
        "strike" | "clawhammer" | "rimshot" | "hard" | "marcato" => {
            value.attack = 0.55;
            value.noise = 1.4;
            value.brightness = 1.18;
            value.transient = 1.55;
        }
        "strum" | "three_finger" | "single_finger" | "fingerstyle" | "roll" | "crosspick"
        | "double_stop" | "noter_drone" | "jhala" | "chikari" => {
            value.attack = 0.8;
            value.decay = 1.12;
            value.transient = 0.9;
        }
        "tremolo" | "tremolo_pick" | "trill" => {
            value.sustain = 1.2;
            value.vibrato = 1.7;
            value.transient = 0.55;
        }
        "bellows_accent" | "percussion" => {
            value.attack = 0.55;
            value.transient = 1.5;
            value.brightness = 1.15;
        }
        "bellows_shake" | "tremulant" | "rotary" => {
            value.vibrato = 2.0;
            value.sustain = 1.1;
        }
        "swell" => {
            value.attack = 2.8;
            value.sustain = 1.1;
            value.transient = 0.2;
        }
        "nail" | "mizrab" | "baji" => {
            value.attack = 0.62;
            value.noise = 1.45;
            value.brightness = 1.18;
            value.transient = 1.35;
        }
        "meend" | "gamak" => {
            value.attack = 0.82;
            value.sustain = 1.12;
            value.vibrato = if technique == "gamak" { 2.2 } else { 1.5 };
            value.transient = 0.62;
        }
        "flutter_tongue" | "breathy" | "slap_tongue" | "growl" => {
            value.noise = 2.2;
            value.brightness = 1.15;
            value.transient = if technique == "slap_tongue" { 1.7 } else { 0.8 };
        }
        "tongued" => {
            value.attack = 0.58;
            value.transient = 1.45;
            value.noise = 1.25;
        }
        "subtone" => {
            value.attack = 1.3;
            value.brightness = 0.58;
            value.noise = 1.75;
            value.transient = 0.35;
        }
        "overblown" | "altissimo" | "doit" => {
            value.brightness = 1.55;
            value.noise = 1.35;
            value.transient = 1.25;
        }
        "straight_mute" | "cup_mute" | "harmon_mute" | "stopped" => {
            value.brightness = if technique == "harmon_mute" {
                1.25
            } else {
                0.62
            };
            value.noise = 0.8;
            value.sustain = 0.88;
        }
        "key_click" => {
            value.decay = 0.08;
            value.sustain = 0.02;
            value.release = 0.08;
            value.noise = 3.0;
            value.transient = 2.0;
        }
        "fall" => {
            value.release = 0.35;
            value.vibrato = 1.5;
        }
        "shake" => {
            value.vibrato = 2.5;
            value.sustain = 1.1;
        }
        "harmonic" | "sul_ponticello" => {
            value.brightness = 1.45;
            value.noise = 0.65;
            value.sustain = 0.85;
        }
        "sul_tasto" | "una_corda" | "soft" => {
            value.brightness = 0.7;
            value.noise = 0.65;
            value.transient = 0.65;
        }
        "pizzicato" | "col_legno" | "pluck" | "pick" => {
            value.decay = 0.72;
            value.sustain = 0.45;
            value.transient = 1.25;
        }
        _ => {}
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_instrument_specific_vocabulary() {
        assert_eq!(
            canonicalize("Five-string Banjo", "plucked_string", "pulling"),
            "pull_off"
        );
        assert_eq!(
            canonicalize("Five-string Banjo", "plucked_string", "3-finger"),
            "three_finger"
        );
        assert_eq!(
            canonicalize("Steel Guitar", "plucked_string", "palm-muted"),
            "palm_mute"
        );
        assert_eq!(
            canonicalize("Grand Piano", "struck_string", "soft pedal"),
            "una_corda"
        );
        assert_eq!(
            canonicalize("Hammered Dulcimer", "struck_string", "wood mallet"),
            "hard"
        );
        assert!(recommended("Hammered Dulcimer", "struck_string").contains(&"roll"));
    }

    #[test]
    fn filename_inference_obeys_instrument_kind() {
        assert_eq!(
            infer_from_filename("Banjo", "plucked_string", "take_C4_three-finger_rr1"),
            "three_finger"
        );
        assert_eq!(
            infer_from_filename("Piano", "struck_string", "staccato_C4_v080"),
            "staccato"
        );
        assert_eq!(
            infer_from_filename("Five-string Banjo", "plucked_string", "banjo_take_01"),
            "pluck"
        );
        assert_eq!(
            infer_from_filename("Trumpet", "brass", "trumpet_take_01"),
            "open"
        );
    }
}
