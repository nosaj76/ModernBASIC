use ruststrument::{
    analyze,
    creator::{self, note_to_midi},
    gesture,
    profile::{InstrumentBank, SynthModel, GENERATOR_CATALOG},
    quality, reference_match, segment, sf2, sfz,
    stringed::{StringMaterial, MAX_PROFILE_STRINGS},
    synth::Engine,
    technique,
    transcribe::{self, TranscriptionConfig},
    wav::{self, Wav},
};
use std::{
    env, fs,
    io::Write,
    process::{self, Command, Stdio},
};

const SAMPLE_RATE: u32 = 44_100;

fn usage() -> ! {
    eprintln!(
        "Ruststrument Forge 2.0.0

Learn and export:
  ruststrument create <recordings-dir> <name> <bank.rsti>
  ruststrument learn <one-audio-file> <name> <bank.rsti>
  ruststrument generate <instrument-name> <bank.rsti>
  ruststrument catalog
  ruststrument segment <performance.wav> <samples-dir>
  ruststrument extract-notes <samples-dir> <audio...> [options]
  ruststrument configure <bank.rsti> <out.rsti> [options]
  ruststrument soundfont <bank.rsti> <bank.sf2>
  ruststrument verify-sf2 <bank.sf2>
  ruststrument sfz <bank.rsti> <instrument.sfz>
  ruststrument transcribe <audio> <notes.json|notes.csv|notes.mid> [options]
  ruststrument techniques <instrument-name> [family]

Audition and inspect:
  ruststrument play <bank.rsti> <seconds-per-note> <notes> [options]
  ruststrument render <bank.rsti> <out.wav> <seconds> <notes> [options]
  ruststrument perform <bank.rsti> <out.wav> <technique> <seconds> <notes> [options]
  ruststrument song <bank.rsti> <out.wav> [options] <notes:beats|rest:beats> ...
  ruststrument probe <bank.rsti> <seconds-per-note> <notes> [options]
  ruststrument inspect <bank.rsti>
  ruststrument quality <bank.rsti> <report.md>
  ruststrument match <bank.rsti> <report.md>

Analysis:
  ruststrument classify <audio>
  ruststrument analyze <audio>
  ruststrument compare <audio-a> <audio-b>
  ruststrument batch <audio1> <audio2> ...

Transcription options:
  --polyphony <1..16>     --tuning <400..480>
  --min-note <10..2000>   --min-midi <0..127> --max-midi <0..127>

Analyzer extraction options:
  --min-note <10..2000>   --min-midi <0..127> --max-midi <0..127>
  --min-confidence <0..1> --max-cents <0..100>
  --max-regions <1..512>  --round-robins <1..8>
  --preferred-source <1..N>  (repeatable)

Options for configure, play, probe, and render:
  --sample-level <0..2>   --synth-level <0..2>
  --master-gain <0..2>    --articulation <name>
  --velocity <0..1>

Song options:
  --bpm <20..400>          --velocity <0..1>
  --sample-level <0..2>    --synth-level <0..2>
  --master-gain <0..2>     --articulation <name>

Additional configure options:
  --model <sample|additive|plucked|banjo|guitar|piano|bowed|struck|wind|reed|free-reed|organ|brass|membrane|modal|noise>
  --attack <seconds>       --decay <seconds>
  --sustain <0..1>         --release <seconds>
  --noise <0..1>           --brightness <0..1>
  --damping <0.9..1>       --pluck-position <0.01..0.99>
  --polyphony <1..65535>

Physical string profile options:
  --tuning <notes>         --string-material <steel|bronze|nickel|nylon|gut|fluorocarbon|silk-steel>
  --courses <1..3>         --octave-courses <0..16>
  --course-detune <0..50>  --scale-length <0.15..3.0>
  --string-stiffness <0..1> --string-damping <0.9..1>
  --bridge-coupling <0..2> --body-mix <0..5>
  --sympathetic <0..1>     --nonlinearity <0..1>

Keyboard profile options:
  --hammer-hardness <0..1> --hammer-noise <0..1>
  --unison-detune <0..30>  --soundboard-mix <0..5>
  --pedal-coupling <0..1>

Wind profile options:
  --breath-pressure <0..1.5> --breath-noise <0..1>
  --bore-loss <0..1>         --bell-mix <0..2>
  --wind-nonlinearity <0..2>

Percussion profile options:
  --strike-position <0.01..0.99> --hardness <0..1>
  --transient-noise <0..1>       --shell-noise <0..1>
  --resonator-mix <0..4>         --physical-decay <0.03..30>

Notes may be MIDI numbers or names. A comma-separated list, such as
C4,E4,G4, plays a chord with render and plays notes in sequence with play
and probe."
    );
    process::exit(2)
}

fn fail(context: &str, error: impl std::fmt::Display) -> ! {
    eprintln!("{context}: {error}");
    process::exit(1)
}

fn decode_input(path: &str) -> Wav {
    let audio = ruststrument::load_audio(path).unwrap_or_else(|error| fail("decode audio", error));
    Wav {
        sample_rate: audio.sample_rate,
        samples: audio.samples,
    }
}

fn parse_f32(value: &str, name: &str) -> f32 {
    let parsed = value
        .parse::<f32>()
        .unwrap_or_else(|error| fail(name, error));
    if !parsed.is_finite() {
        fail(name, "must be finite");
    }
    parsed
}

fn bounded(value: &str, name: &str, low: f32, high: f32) -> f32 {
    let parsed = parse_f32(value, name);
    if !(low..=high).contains(&parsed) {
        fail(name, format!("must be between {low} and {high}"));
    }
    parsed
}

fn option_value<'a>(arguments: &'a [String], index: &mut usize) -> &'a str {
    *index += 1;
    arguments
        .get(*index)
        .map(String::as_str)
        .unwrap_or_else(|| usage())
}

fn configure(arguments: &[String]) {
    if arguments.len() < 4 {
        usage();
    }
    let mut bank = InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
    let mut index = 4;
    while index < arguments.len() {
        let option = arguments[index].as_str();
        match option {
            "--sample-level" => {
                let value = option_value(arguments, &mut index);
                bank.synth.sample_level = bounded(value, option, 0.0, 2.0);
            }
            "--synth-level" => {
                let value = option_value(arguments, &mut index);
                bank.synth.synth_level = bounded(value, option, 0.0, 2.0);
            }
            "--master-gain" => {
                let value = option_value(arguments, &mut index);
                bank.synth.master_gain = bounded(value, option, 0.0, 2.0);
            }
            "--attack" => {
                let value = option_value(arguments, &mut index);
                bank.synth.attack_s = bounded(value, option, 0.0, 10.0);
            }
            "--decay" => {
                let value = option_value(arguments, &mut index);
                bank.synth.decay_s = bounded(value, option, 0.0, 30.0);
            }
            "--sustain" => {
                let value = option_value(arguments, &mut index);
                bank.synth.sustain = bounded(value, option, 0.0, 1.0);
            }
            "--release" => {
                let value = option_value(arguments, &mut index);
                bank.synth.release_s = bounded(value, option, 0.0, 30.0);
            }
            "--noise" => {
                let value = option_value(arguments, &mut index);
                bank.synth.noise = bounded(value, option, 0.0, 1.0);
            }
            "--brightness" => {
                let value = option_value(arguments, &mut index);
                bank.synth.brightness = bounded(value, option, 0.0, 1.0);
            }
            "--damping" => {
                let value = option_value(arguments, &mut index);
                bank.synth.damping = bounded(value, option, 0.9, 1.0);
            }
            "--pluck-position" => {
                let value = option_value(arguments, &mut index);
                bank.synth.pluck_position = bounded(value, option, 0.01, 0.99);
            }
            "--tuning" => {
                let value = option_value(arguments, &mut index);
                let notes: Vec<u8> = value
                    .split(',')
                    .map(|note| {
                        note_to_midi(note).unwrap_or_else(|| {
                            fail(option, format!("invalid tuning note '{note}'"))
                        })
                    })
                    .collect();
                if notes.is_empty() || notes.len() > MAX_PROFILE_STRINGS {
                    fail(option, "requires 1 to 16 comma-separated MIDI notes");
                }
                let profile = bank
                    .string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"));
                profile.open_notes = [0; MAX_PROFILE_STRINGS];
                profile.open_notes[..notes.len()].copy_from_slice(&notes);
                profile.open_string_count = notes.len() as u8;
                profile.string_count =
                    (notes.len() * profile.course_voices as usize).min(u8::MAX as usize) as u8;
            }
            "--string-material" => {
                let value = option_value(arguments, &mut index);
                let material = StringMaterial::parse(value).unwrap_or_else(|| {
                    fail(
                        option,
                        "expected steel, bronze, nickel, nylon, gut, fluorocarbon, or silk-steel",
                    )
                });
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .material = material;
            }
            "--courses" => {
                let value = option_value(arguments, &mut index);
                let courses = value
                    .parse::<u8>()
                    .ok()
                    .filter(|value| (1..=3).contains(value))
                    .unwrap_or_else(|| fail(option, "must be between 1 and 3"));
                let profile = bank
                    .string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"));
                profile.course_voices = courses;
                profile.string_count = profile.open_string_count.saturating_mul(courses);
            }
            "--octave-courses" => {
                let value = option_value(arguments, &mut index);
                let count = value
                    .parse::<u8>()
                    .ok()
                    .filter(|value| *value <= MAX_PROFILE_STRINGS as u8)
                    .unwrap_or_else(|| fail(option, "must be between 0 and 16"));
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .octave_course_strings = count;
            }
            "--course-detune" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .course_detune_cents = bounded(value, option, 0.0, 50.0);
            }
            "--scale-length" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .scale_length_m = bounded(value, option, 0.15, 3.0);
            }
            "--string-stiffness" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .stiffness = bounded(value, option, 0.0, 1.0);
            }
            "--string-damping" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .base_damping = bounded(value, option, 0.9, 1.0);
            }
            "--bridge-coupling" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .bridge_coupling = bounded(value, option, 0.0, 2.0);
            }
            "--body-mix" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .body_mix = bounded(value, option, 0.0, 5.0);
            }
            "--sympathetic" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .sympathetic_gain = bounded(value, option, 0.0, 1.0);
            }
            "--nonlinearity" => {
                let value = option_value(arguments, &mut index);
                bank.string_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical string profile"))
                    .nonlinear_tension = bounded(value, option, 0.0, 1.0);
            }
            "--hammer-hardness" => {
                let value = option_value(arguments, &mut index);
                bank.keyboard_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical keyboard profile"))
                    .hammer_hardness = bounded(value, option, 0.0, 1.0);
            }
            "--hammer-noise" => {
                let value = option_value(arguments, &mut index);
                bank.keyboard_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical keyboard profile"))
                    .hammer_noise = bounded(value, option, 0.0, 1.0);
            }
            "--unison-detune" => {
                let value = option_value(arguments, &mut index);
                bank.keyboard_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical keyboard profile"))
                    .unison_detune_cents = bounded(value, option, 0.0, 30.0);
            }
            "--soundboard-mix" => {
                let value = option_value(arguments, &mut index);
                bank.keyboard_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical keyboard profile"))
                    .soundboard_mix = bounded(value, option, 0.0, 5.0);
            }
            "--pedal-coupling" => {
                let value = option_value(arguments, &mut index);
                bank.keyboard_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical keyboard profile"))
                    .pedal_coupling = bounded(value, option, 0.0, 1.0);
            }
            "--breath-pressure" => {
                let value = option_value(arguments, &mut index);
                bank.wind_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical wind profile"))
                    .breath_pressure = bounded(value, option, 0.0, 1.5);
            }
            "--breath-noise" => {
                let value = option_value(arguments, &mut index);
                bank.wind_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical wind profile"))
                    .breath_noise = bounded(value, option, 0.0, 1.0);
            }
            "--bore-loss" => {
                let value = option_value(arguments, &mut index);
                bank.wind_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical wind profile"))
                    .bore_loss = bounded(value, option, 0.0, 1.0);
            }
            "--bell-mix" => {
                let value = option_value(arguments, &mut index);
                bank.wind_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical wind profile"))
                    .bell_mix = bounded(value, option, 0.0, 2.0);
            }
            "--wind-nonlinearity" => {
                let value = option_value(arguments, &mut index);
                bank.wind_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical wind profile"))
                    .nonlinearity = bounded(value, option, 0.0, 2.0);
            }
            "--strike-position" => {
                let value = option_value(arguments, &mut index);
                bank.percussion_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical percussion profile"))
                    .strike_position = bounded(value, option, 0.01, 0.99);
            }
            "--hardness" => {
                let value = option_value(arguments, &mut index);
                bank.percussion_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical percussion profile"))
                    .hardness = bounded(value, option, 0.0, 1.0);
            }
            "--transient-noise" => {
                let value = option_value(arguments, &mut index);
                bank.percussion_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical percussion profile"))
                    .transient_noise = bounded(value, option, 0.0, 1.0);
            }
            "--shell-noise" => {
                let value = option_value(arguments, &mut index);
                bank.percussion_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical percussion profile"))
                    .shell_noise = bounded(value, option, 0.0, 1.0);
            }
            "--resonator-mix" => {
                let value = option_value(arguments, &mut index);
                bank.percussion_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical percussion profile"))
                    .resonator_mix = bounded(value, option, 0.0, 4.0);
            }
            "--physical-decay" => {
                let value = option_value(arguments, &mut index);
                let decay = bounded(value, option, 0.03, 30.0);
                bank.percussion_profile
                    .as_mut()
                    .unwrap_or_else(|| fail(option, "bank has no physical percussion profile"))
                    .decay_s = decay;
                bank.synth.decay_s = decay;
            }
            "--polyphony" => {
                let value = option_value(arguments, &mut index);
                bank.polyphony = value
                    .parse::<u16>()
                    .ok()
                    .filter(|value| *value > 0)
                    .unwrap_or_else(|| fail(option, "must be between 1 and 65535"));
            }
            "--model" => {
                let value = option_value(arguments, &mut index);
                bank.synth.model = match value {
                    "sample" | "sample_only" => SynthModel::SampleOnly,
                    "additive" => SynthModel::Additive,
                    "plucked" | "plucked_string" => SynthModel::PluckedString,
                    "banjo" | "banjo_physical" => SynthModel::Banjo,
                    "guitar" | "guitar_physical" => SynthModel::Guitar,
                    "piano" | "piano_physical" => SynthModel::Piano,
                    "bowed" | "bowed_string" => SynthModel::BowedString,
                    "struck" | "struck_string" => SynthModel::StruckString,
                    "wind" | "flute" | "flute_air_jet" => SynthModel::Wind,
                    "reed" => SynthModel::Reed,
                    "free-reed" | "free_reed" => SynthModel::FreeReed,
                    "organ" | "organ_additive" => SynthModel::Organ,
                    "brass" => SynthModel::Brass,
                    "membrane" | "drum" => SynthModel::Membrane,
                    "modal" | "metal" | "wood" | "bell" => SynthModel::Modal,
                    "noise" => SynthModel::Noise,
                    _ => fail(
                        option,
                        "expected sample, additive, plucked, banjo, guitar, piano, bowed, struck, wind, reed, free-reed, organ, brass, membrane, modal, or noise",
                    ),
                };
            }
            _ => fail("configure", format!("unknown option {option}")),
        }
        index += 1;
    }
    bank.save(&arguments[3])
        .unwrap_or_else(|error| fail("save", error));
    println!(
        "saved {}: model={} sample={:.2} synth={:.2} master={:.2}",
        arguments[3],
        bank.synth.model.label(),
        bank.synth.sample_level,
        bank.synth.synth_level,
        bank.synth.master_gain
    );
}

struct RenderSettings {
    velocity: f32,
    articulation: Option<String>,
    sample_level: Option<f32>,
    synth_level: Option<f32>,
    master_gain: Option<f32>,
}

fn render_settings(arguments: &[String], start_index: usize, context: &str) -> RenderSettings {
    let mut settings = RenderSettings {
        velocity: 0.8,
        articulation: None,
        sample_level: None,
        synth_level: None,
        master_gain: None,
    };
    let mut index = start_index;
    let mut legacy_position = 0;
    while index < arguments.len() {
        let option = arguments[index].as_str();
        match option {
            "--velocity" => {
                let value = option_value(arguments, &mut index);
                settings.velocity = bounded(value, option, 0.0, 1.0);
            }
            "--articulation" => {
                settings.articulation = Some(option_value(arguments, &mut index).to_string());
            }
            "--sample-level" => {
                let value = option_value(arguments, &mut index);
                settings.sample_level = Some(bounded(value, option, 0.0, 2.0));
            }
            "--synth-level" => {
                let value = option_value(arguments, &mut index);
                settings.synth_level = Some(bounded(value, option, 0.0, 2.0));
            }
            "--master-gain" => {
                let value = option_value(arguments, &mut index);
                settings.master_gain = Some(bounded(value, option, 0.0, 2.0));
            }
            _ if !option.starts_with("--") && legacy_position == 0 => {
                settings.velocity = bounded(option, "velocity", 0.0, 1.0);
                legacy_position += 1;
            }
            _ if !option.starts_with("--") && legacy_position == 1 => {
                settings.articulation = Some(option.to_string());
                legacy_position += 1;
            }
            _ => fail(context, format!("unknown option {option}")),
        }
        index += 1;
    }
    settings
}

fn parse_notes(value: &str) -> Vec<u8> {
    let notes: Vec<u8> = value
        .split(',')
        .map(|note| {
            note_to_midi(note).unwrap_or_else(|| fail("note", format!("invalid MIDI note {note}")))
        })
        .collect();
    if notes.is_empty() {
        fail("note", "at least one note is required");
    }
    notes
}

fn apply_render_settings(engine: &mut Engine, settings: &RenderSettings) {
    if let Some(articulation) = settings.articulation.as_deref() {
        if !engine.set_articulation(articulation) {
            fail("articulation", format!("{articulation} was not found"));
        }
    }
    if let Some(level) = settings.sample_level {
        engine.set_sample_level(level);
    }
    if let Some(level) = settings.synth_level {
        engine.set_synth_level(level);
    }
    if let Some(gain) = settings.master_gain {
        engine.set_master_gain(gain);
    }
}

fn settings_velocity(settings: &RenderSettings) -> u8 {
    (settings.velocity * 127.0).round().clamp(0.0, 127.0) as u8
}

fn render_command(arguments: &[String]) {
    if arguments.len() < 6 {
        usage();
    }
    let bank = InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
    let seconds = bounded(&arguments[4], "seconds", 0.01, 600.0);
    let notes = parse_notes(&arguments[5]);
    let settings = render_settings(arguments, 6, "render");
    let mut engine = Engine::new(bank, SAMPLE_RATE);
    apply_render_settings(&mut engine, &settings);
    let velocity = settings_velocity(&settings);
    for &note in &notes {
        if velocity > 0 && engine.note_on(note, velocity) == 0 {
            fail("render", format!("no mapped region for MIDI note {note}"));
        }
    }

    let total_frames = (seconds * SAMPLE_RATE as f32) as usize;
    let held_frames = total_frames * 3 / 4;
    let mut samples = engine.render(held_frames);
    for note in notes {
        engine.note_off(note);
    }
    samples.extend(engine.render(total_frames - held_frames));
    wav::write(
        &arguments[3],
        &Wav {
            sample_rate: SAMPLE_RATE,
            samples,
        },
    )
    .unwrap_or_else(|error| fail("write", error));
    println!(
        "wrote {} (sample={:.2}, synth={:.2}, master={:.2})",
        arguments[3],
        engine.sample_level(),
        engine.synth_level(),
        engine.master_gain()
    );
}

fn perform_command(arguments: &[String]) {
    if arguments.len() < 7 {
        usage();
    }
    let bank = InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
    let technique = technique::canonicalize(&bank.name, &bank.family, &arguments[4]);
    let seconds = bounded(&arguments[5], "seconds", 0.05, 600.0);
    let notes = parse_notes(&arguments[6]);
    let settings = render_settings(arguments, 7, "perform");
    let mut engine = Engine::new(bank, SAMPLE_RATE);
    if !engine.set_articulation(&technique) && !engine.set_articulation(&arguments[4]) {
        fail(
            "perform",
            format!("technique '{technique}' is not mapped in this bank"),
        );
    }
    apply_render_settings(&mut engine, &settings);
    let velocity = settings_velocity(&settings);
    let events = gesture::schedule(&technique, &notes, seconds);
    let mut output = Vec::new();
    let mut cursor = 0_usize;
    for event in events {
        let offset = (event.offset_s * SAMPLE_RATE as f32).round() as usize;
        if offset > cursor {
            output.extend(engine.render(offset - cursor));
            cursor = offset;
        }
        let event_velocity = if velocity == 0 {
            0
        } else {
            (velocity as f32 * event.velocity_scale)
                .round()
                .clamp(1.0, 127.0) as u8
        };
        let duration = (event.duration_s * SAMPLE_RATE as f32).round().max(1.0) as usize;
        if event_velocity > 0 && engine.note_on_for(event.note, event_velocity, Some(duration)) == 0
        {
            fail(
                "perform",
                format!("no mapped region for MIDI note {}", event.note),
            );
        }
    }
    let total = (seconds * SAMPLE_RATE as f32).round() as usize;
    if cursor < total {
        output.extend(engine.render(total - cursor));
    }
    engine.all_notes_off();
    output.extend(engine.render((engine.bank.synth.release_s * SAMPLE_RATE as f32 * 2.0) as usize));
    wav::write(
        &arguments[3],
        &Wav {
            sample_rate: SAMPLE_RATE,
            samples: output,
        },
    )
    .unwrap_or_else(|error| fail("write", error));
    println!("wrote {} using {}", arguments[3], technique);
}

fn write_engine_frames(
    engine: &mut Engine,
    output: &mut impl Write,
    mut frames: usize,
) -> std::io::Result<()> {
    const BLOCK_FRAMES: usize = 512;
    let mut bytes = Vec::with_capacity(BLOCK_FRAMES * std::mem::size_of::<f32>());
    while frames > 0 {
        let block = frames.min(BLOCK_FRAMES);
        let samples = engine.render(block);
        bytes.clear();
        for sample in samples {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        output.write_all(&bytes)?;
        frames -= block;
    }
    Ok(())
}

fn play_command(arguments: &[String]) {
    if arguments.len() < 5 {
        usage();
    }
    let bank = InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
    let seconds = bounded(&arguments[3], "seconds-per-note", 0.05, 30.0);
    let notes = parse_notes(&arguments[4]);
    let settings = render_settings(arguments, 5, "play");
    let mut engine = Engine::new(bank, SAMPLE_RATE);
    apply_render_settings(&mut engine, &settings);
    let velocity = settings_velocity(&settings);

    let mut child = Command::new("ffplay")
        .args([
            "-nodisp",
            "-autoexit",
            "-loglevel",
            "error",
            "-f",
            "f32le",
            "-ar",
        ])
        .arg(SAMPLE_RATE.to_string())
        .args(["-ac", "1", "-i", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap_or_else(|error| fail("play", format!("could not start ffplay: {error}")));
    let mut input = child
        .stdin
        .take()
        .unwrap_or_else(|| fail("play", "ffplay stdin was unavailable"));
    let total_frames = (seconds * SAMPLE_RATE as f32) as usize;
    let held_frames = total_frames * 3 / 4;
    for note in notes {
        println!("playing {}", analyze::midi_note_name(note));
        if velocity > 0 && engine.note_on(note, velocity) == 0 {
            fail("play", format!("no mapped region for MIDI note {note}"));
        }
        write_engine_frames(&mut engine, &mut input, held_frames)
            .unwrap_or_else(|error| fail("play", error));
        engine.note_off(note);
        write_engine_frames(&mut engine, &mut input, total_frames - held_frames)
            .unwrap_or_else(|error| fail("play", error));
    }
    drop(input);
    let status = child
        .wait()
        .unwrap_or_else(|error| fail("play", format!("could not wait for ffplay: {error}")));
    if !status.success() {
        fail("play", format!("ffplay exited with {status}"));
    }
}

fn probe_command(arguments: &[String]) {
    if arguments.len() < 5 {
        usage();
    }
    let bank = InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
    let seconds = bounded(&arguments[3], "seconds-per-note", 0.05, 30.0);
    let notes = parse_notes(&arguments[4]);
    let settings = render_settings(arguments, 5, "probe");
    let velocity = settings_velocity(&settings);
    if velocity == 0 {
        fail("probe", "--velocity must be greater than zero");
    }
    let total_frames = (seconds * SAMPLE_RATE as f32) as usize;
    let mut passed = true;

    for note in notes {
        let mut engine = Engine::new(bank.clone(), SAMPLE_RATE);
        apply_render_settings(&mut engine, &settings);
        if engine.note_on(note, velocity) == 0 {
            fail("probe", format!("no mapped region for MIDI note {note}"));
        }
        let samples = engine.render(total_frames);
        let expected = analyze::midi_note_name(note);
        match analyze::identify_pitch(&samples, SAMPLE_RATE) {
            Some(pitch) => {
                let error_cents = (pitch.midi as i16 - note as i16) as f32 * 100.0 + pitch.cents;
                let result = if error_cents.abs() <= 50.0 {
                    "PASS"
                } else {
                    passed = false;
                    "FAIL"
                };
                println!(
                    "{expected}: detected={} {:.1}Hz error={error_cents:+.1} cents {result}",
                    analyze::midi_note_name(pitch.midi),
                    pitch.hz
                );
            }
            None => {
                passed = false;
                println!("{expected}: no stable pitch detected FAIL");
            }
        }
    }
    if !passed {
        fail(
            "probe",
            "one or more generated notes failed pitch verification",
        );
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SongEvent {
    notes: Vec<u8>,
    beats: f32,
}

fn parse_song_event(value: &str) -> Result<SongEvent, String> {
    let (notes, beats) = value
        .rsplit_once(':')
        .ok_or_else(|| "expected notes:beats".to_string())?;
    let beats = beats
        .parse::<f32>()
        .map_err(|error| format!("invalid beat length: {error}"))?;
    if !beats.is_finite() || !(0.01..=128.0).contains(&beats) {
        return Err("beats must be between 0.01 and 128".to_string());
    }
    if notes.eq_ignore_ascii_case("r") || notes.eq_ignore_ascii_case("rest") {
        return Ok(SongEvent {
            notes: Vec::new(),
            beats,
        });
    }
    let notes = notes
        .split(',')
        .map(|note| note_to_midi(note).ok_or_else(|| format!("invalid MIDI note {note}")))
        .collect::<Result<Vec<_>, _>>()?;
    if notes.is_empty() {
        return Err("at least one note or a rest is required".to_string());
    }
    Ok(SongEvent { notes, beats })
}

fn song(arguments: &[String]) {
    if arguments.len() < 5 {
        usage();
    }
    let bank = InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
    let release_s = bank.synth.release_s;
    let mut engine = Engine::new(bank, SAMPLE_RATE);
    let mut settings = RenderSettings {
        velocity: 0.8,
        articulation: None,
        sample_level: None,
        synth_level: None,
        master_gain: None,
    };
    let mut bpm = 120.0_f32;
    let mut events = Vec::new();
    let mut index = 4;
    while index < arguments.len() {
        let value = arguments[index].as_str();
        match value {
            "--bpm" | "--tempo" => {
                let option = value;
                bpm = bounded(option_value(arguments, &mut index), option, 20.0, 400.0);
            }
            "--velocity" => {
                settings.velocity =
                    bounded(option_value(arguments, &mut index), "--velocity", 0.0, 1.0);
            }
            "--articulation" => {
                settings.articulation = Some(option_value(arguments, &mut index).to_string());
            }
            "--sample-level" => {
                settings.sample_level = Some(bounded(
                    option_value(arguments, &mut index),
                    "--sample-level",
                    0.0,
                    2.0,
                ));
            }
            "--synth-level" => {
                settings.synth_level = Some(bounded(
                    option_value(arguments, &mut index),
                    "--synth-level",
                    0.0,
                    2.0,
                ));
            }
            "--master-gain" => {
                settings.master_gain = Some(bounded(
                    option_value(arguments, &mut index),
                    "--master-gain",
                    0.0,
                    2.0,
                ));
            }
            _ if value.starts_with("--") => fail("song", format!("unknown option {value}")),
            _ => events
                .push(parse_song_event(value).unwrap_or_else(|error| fail("song event", error))),
        }
        index += 1;
    }
    if events.is_empty() {
        fail(
            "song",
            "at least one notes:beats or rest:beats event is required",
        );
    }
    apply_render_settings(&mut engine, &settings);
    let velocity = settings_velocity(&settings);
    let mut output = Vec::new();
    for event in events {
        let event_frames = (event.beats * 60.0 / bpm * SAMPLE_RATE as f32)
            .round()
            .max(1.0) as usize;
        let duration = event_frames * 4 / 5;
        for note in event.notes {
            if velocity > 0 && engine.note_on_for(note, velocity, Some(duration)) == 0 {
                fail("song", format!("no mapped region for MIDI note {note}"));
            }
        }
        output.extend(engine.render(event_frames));
    }
    engine.all_notes_off();
    let tail_frames = ((release_s * 2.0 + 0.5).clamp(0.5, 15.0) * SAMPLE_RATE as f32) as usize;
    output.extend(engine.render(tail_frames));
    wav::write(
        &arguments[3],
        &Wav {
            sample_rate: SAMPLE_RATE,
            samples: output,
        },
    )
    .unwrap_or_else(|error| fail("write", error));
    println!(
        "wrote {} at {:.1} BPM (sample={:.2}, synth={:.2}, master={:.2})",
        arguments[3],
        bpm,
        engine.sample_level(),
        engine.synth_level(),
        engine.master_gain()
    );
}

fn inspect(path: &str) {
    let bank = InstrumentBank::load(path).unwrap_or_else(|error| fail("load", error));
    println!("name={}", bank.name);
    println!("family={}", bank.family);
    println!("model={}", bank.synth.model.label());
    println!("regions={}", bank.regions.len());
    println!("polyphony={}", bank.polyphony);
    println!("default_articulation={}", bank.default_articulation);
    println!("articulations={}", bank.articulations().join(","));
    println!(
        "sample_level={:.3} synth_level={:.3} master_gain={:.3}",
        bank.synth.sample_level, bank.synth.synth_level, bank.synth.master_gain
    );
    println!(
        "attack={:.4}s decay={:.4}s sustain={:.3} release={:.4}s",
        bank.synth.attack_s, bank.synth.decay_s, bank.synth.sustain, bank.synth.release_s
    );
    println!(
        "noise={:.3} brightness={:.3} damping={:.5} pluck_position={:.3}",
        bank.synth.noise, bank.synth.brightness, bank.synth.damping, bank.synth.pluck_position
    );
    if let Some(profile) = bank.string_profile {
        let tuning = profile.open_notes[..profile.open_string_count as usize]
            .iter()
            .map(|note| analyze::midi_note_name(*note))
            .collect::<Vec<_>>()
            .join(",");
        println!("string_kind={}", profile.kind.label());
        println!(
            "strings={} courses={} octave_courses={} tuning={}",
            profile.string_count, profile.course_voices, profile.octave_course_strings, tuning
        );
        println!(
            "material={} body={} scale_length={:.3}m stiffness={:.3}",
            profile.material.label(),
            profile.body.label(),
            profile.scale_length_m,
            profile.stiffness
        );
        println!(
            "bridge={:.3} body_mix={:.3} sympathetic={:.3} nonlinear={:.3}",
            profile.bridge_coupling,
            profile.body_mix,
            profile.sympathetic_gain,
            profile.nonlinear_tension
        );
    }
    if let Some(profile) = bank.keyboard_profile {
        println!(
            "keyboard_kind={} exciter={} range={}-{} strings_per_note={}",
            profile.kind.label(),
            profile.exciter.label(),
            profile.range_low,
            profile.range_high,
            profile.strings_per_note
        );
        println!(
            "strike_position={:.3} hardness={:.3} unison_detune={:.2}c soundboard={:.3} pedal={:.3}",
            profile.strike_position,
            profile.hammer_hardness,
            profile.unison_detune_cents,
            profile.soundboard_mix,
            profile.pedal_coupling
        );
    }
    if let Some(profile) = bank.wind_profile {
        println!(
            "wind_exciter={} bore={} range={}-{} pressure={:.3}",
            profile.exciter.label(),
            profile.bore.label(),
            profile.range_low,
            profile.range_high,
            profile.breath_pressure
        );
        println!(
            "reflection={:.3} bore_loss={:.3} breath_noise={:.3} bell_mix={:.3} nonlinearity={:.3}",
            profile.reflection,
            profile.bore_loss,
            profile.breath_noise,
            profile.bell_mix,
            profile.nonlinearity
        );
    }
    if let Some(profile) = bank.percussion_profile {
        println!(
            "percussion_resonator={} exciter={} range={}-{}",
            profile.resonator.label(),
            profile.exciter.label(),
            profile.range_low,
            profile.range_high
        );
        println!(
            "strike_position={:.3} hardness={:.3} damping={:.3} resonator_mix={:.3} decay={:.3}s",
            profile.strike_position,
            profile.hardness,
            profile.damping,
            profile.resonator_mix,
            profile.decay_s
        );
    }
    println!(
        "harmonics={}",
        bank.synth
            .harmonics
            .iter()
            .map(|value| format!("{value:.3}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    for region in &bank.regions {
        println!(
            "{} art={} trigger={:?} key={}-{} root={} vel={}-{} rr={} rate={} samples={}",
            region.name,
            region.articulation,
            region.trigger,
            region.key_lo,
            region.key_hi,
            region.root_key,
            region.vel_lo,
            region.vel_hi,
            region.round_robin,
            region.sample_rate,
            region.samples.len()
        );
    }
}

fn extract_notes_command(arguments: &[String]) {
    if arguments.len() < 4 {
        usage();
    }
    let first_option = arguments
        .iter()
        .enumerate()
        .skip(3)
        .find(|(_, argument)| argument.starts_with("--"))
        .map(|(index, _)| index)
        .unwrap_or(arguments.len());
    let inputs: Vec<std::path::PathBuf> = arguments[3..first_option]
        .iter()
        .map(std::path::PathBuf::from)
        .collect();
    if inputs.is_empty() {
        usage();
    }

    let mut config = segment::AnalyzedExtractionConfig::default();
    let mut index = first_option;
    while index < arguments.len() {
        let option = arguments[index].as_str();
        match option {
            "--min-midi" => {
                config.min_midi = option_value(arguments, &mut index)
                    .parse::<u8>()
                    .ok()
                    .filter(|value| *value <= 127)
                    .unwrap_or_else(|| fail(option, "must be between 0 and 127"));
            }
            "--max-midi" => {
                config.max_midi = option_value(arguments, &mut index)
                    .parse::<u8>()
                    .ok()
                    .filter(|value| *value <= 127)
                    .unwrap_or_else(|| fail(option, "must be between 0 and 127"));
            }
            "--min-note" => {
                config.min_note_ms =
                    bounded(option_value(arguments, &mut index), option, 10.0, 2_000.0);
            }
            "--min-confidence" => {
                config.min_confidence =
                    bounded(option_value(arguments, &mut index), option, 0.0, 1.0);
            }
            "--max-cents" => {
                config.max_cents = bounded(option_value(arguments, &mut index), option, 0.0, 100.0);
            }
            "--max-regions" => {
                config.max_regions = option_value(arguments, &mut index)
                    .parse::<usize>()
                    .ok()
                    .filter(|value| (1..=512).contains(value))
                    .unwrap_or_else(|| fail(option, "must be between 1 and 512"));
            }
            "--round-robins" => {
                config.round_robins = option_value(arguments, &mut index)
                    .parse::<usize>()
                    .ok()
                    .filter(|value| (1..=8).contains(value))
                    .unwrap_or_else(|| fail(option, "must be between 1 and 8"));
            }
            "--preferred-source" => {
                let source = option_value(arguments, &mut index)
                    .parse::<usize>()
                    .ok()
                    .filter(|value| *value >= 1 && *value <= inputs.len())
                    .map(|value| value - 1)
                    .unwrap_or_else(|| fail(option, "must identify one listed recording"));
                config.preferred_sources.insert(source);
            }
            _ => fail("extract-notes", format!("unknown option {option}")),
        }
        index += 1;
    }
    config
        .validate()
        .unwrap_or_else(|error| fail("extract-notes", error));
    let count = segment::extract_analyzed(&inputs, &arguments[2], &config)
        .unwrap_or_else(|error| fail("extract-notes", error));
    println!(
        "extracted {count} analyzer-labeled samples from {} recordings",
        inputs.len()
    );
}

fn transcribe_command(arguments: &[String]) {
    if arguments.len() < 4 {
        usage();
    }
    let mut config = TranscriptionConfig::default();
    let mut index = 4;
    while index < arguments.len() {
        let option = arguments[index].as_str();
        match option {
            "--polyphony" => {
                let value = option_value(arguments, &mut index);
                config.max_polyphony = value
                    .parse::<usize>()
                    .ok()
                    .filter(|value| (1..=16).contains(value))
                    .unwrap_or_else(|| fail(option, "must be between 1 and 16"));
            }
            "--tuning" => {
                let value = option_value(arguments, &mut index);
                config.tuning_hz = bounded(value, option, 400.0, 480.0);
            }
            "--min-note" => {
                let value = option_value(arguments, &mut index);
                config.min_note_ms = bounded(value, option, 10.0, 2_000.0);
            }
            "--min-midi" => {
                let value = option_value(arguments, &mut index);
                config.min_midi = value
                    .parse::<u8>()
                    .ok()
                    .filter(|value| *value <= 127)
                    .unwrap_or_else(|| fail(option, "must be between 0 and 127"));
            }
            "--max-midi" => {
                let value = option_value(arguments, &mut index);
                config.max_midi = value
                    .parse::<u8>()
                    .ok()
                    .filter(|value| *value <= 127)
                    .unwrap_or_else(|| fail(option, "must be between 0 and 127"));
            }
            _ => fail("transcribe", format!("unknown option {option}")),
        }
        index += 1;
    }
    config
        .validate()
        .unwrap_or_else(|error| fail("transcribe", error));
    let audio =
        ruststrument::load_audio(&arguments[2]).unwrap_or_else(|error| fail("decode audio", error));
    let result = transcribe::transcribe(&audio.samples, audio.sample_rate, &config)
        .unwrap_or_else(|error| fail("transcribe", error));
    let extension = std::path::Path::new(&arguments[3])
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match extension.as_str() {
        "json" => result.write_json(&arguments[3]),
        "csv" => result.write_csv(&arguments[3]),
        "mid" | "midi" => result.write_midi(&arguments[3]),
        _ => fail(
            "transcribe",
            "output must end in .json, .csv, .mid, or .midi",
        ),
    }
    .unwrap_or_else(|error| fail("write transcription", error));
    println!(
        "wrote {}: {} notes, {} chord spans, {} percussion onsets",
        arguments[3],
        result.notes.len(),
        result.chords.len(),
        result.percussion.len()
    );
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    match arguments.get(1).map(String::as_str) {
        Some("extract-notes") => extract_notes_command(&arguments),
        Some("segment") if arguments.len() == 4 => {
            let count = segment::segment(&arguments[2], &arguments[3])
                .unwrap_or_else(|error| fail("segment", error));
            println!("extracted {count} labeled samples");
        }
        Some("create") if arguments.len() == 5 => {
            let bank = creator::create(&arguments[2], &arguments[3])
                .unwrap_or_else(|error| fail("create", error));
            bank.save(&arguments[4])
                .unwrap_or_else(|error| fail("save", error));
            println!(
                "created {}: family={}, model={}, {} regions, {} articulations",
                bank.name,
                bank.family,
                bank.synth.model.label(),
                bank.regions.len(),
                bank.articulations().len()
            );
        }
        Some("learn") if arguments.len() == 5 => {
            let wav = decode_input(&arguments[2]);
            let bank = InstrumentBank::single(&arguments[3], &wav);
            bank.save(&arguments[4])
                .unwrap_or_else(|error| fail("save", error));
            println!(
                "created {}: one sample mapped across MIDI 0..127 with model={}",
                bank.name,
                bank.synth.model.label()
            );
        }
        Some("generate") if arguments.len() == 4 => {
            let bank = InstrumentBank::generator(&arguments[2]);
            bank.save(&arguments[3])
                .unwrap_or_else(|error| fail("save", error));
            println!(
                "generated {}: family={}, model={}, sample={:.2}, synth={:.2}, {} modeled articulations",
                bank.name,
                bank.family,
                bank.synth.model.label(),
                bank.synth.sample_level,
                bank.synth.synth_level,
                bank.articulations().len()
            );
        }
        Some("catalog") if arguments.len() == 2 => {
            println!("instrument\tfamily\tmodel\trange\tarticulations");
            for name in GENERATOR_CATALOG {
                let bank = InstrumentBank::generator(name);
                let low = bank
                    .regions
                    .iter()
                    .map(|region| region.root_key)
                    .min()
                    .unwrap_or(0);
                let high = bank
                    .regions
                    .iter()
                    .map(|region| region.root_key)
                    .max()
                    .unwrap_or(127);
                println!(
                    "{}\t{}\t{}\t{}..{}\t{}",
                    name,
                    bank.family,
                    bank.synth.model.label(),
                    low,
                    high,
                    bank.articulations().len()
                );
            }
        }
        Some("configure") => configure(&arguments),
        Some("soundfont") if arguments.len() == 4 => {
            let bank =
                InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
            sf2::write(&bank, &arguments[3]).unwrap_or_else(|error| fail("SoundFont", error));
            println!(
                "wrote {} with the configured playback layers baked in (program 0 plus a GM alias when recognized)",
                arguments[3]
            );
        }
        Some("verify-sf2") if arguments.len() == 3 => {
            sf2::validate_file(&arguments[2]).unwrap_or_else(|error| fail("SoundFont", error));
            println!("{} is a structurally valid SoundFont 2 bank", arguments[2]);
        }
        Some("sfz") if arguments.len() == 4 => {
            let bank =
                InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
            sfz::write(&bank, &arguments[3]).unwrap_or_else(|error| fail("SFZ", error));
            println!(
                "wrote {} with the configured playback layers baked in",
                arguments[3]
            );
        }
        Some("transcribe") => transcribe_command(&arguments),
        Some("techniques") if (3..=4).contains(&arguments.len()) => {
            let family = arguments
                .get(3)
                .map(String::as_str)
                .unwrap_or_else(|| ruststrument::profile::family_from_name(&arguments[2]));
            println!(
                "{}",
                technique::recommended(&arguments[2], family).join("\n")
            );
        }
        Some("play") => play_command(&arguments),
        Some("probe") => probe_command(&arguments),
        Some("render") => render_command(&arguments),
        Some("perform") => perform_command(&arguments),
        Some("song") => song(&arguments),
        Some("inspect") if arguments.len() == 3 => inspect(&arguments[2]),
        Some("quality") if arguments.len() == 4 => {
            let bank =
                InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
            fs::write(&arguments[3], quality::report(&bank))
                .unwrap_or_else(|error| fail("report", error));
            println!("wrote {}", arguments[3]);
        }
        Some("match") if arguments.len() == 4 => {
            let bank =
                InstrumentBank::load(&arguments[2]).unwrap_or_else(|error| fail("load", error));
            let report = reference_match::report(&bank);
            fs::write(&arguments[3], &report.text)
                .unwrap_or_else(|error| fail("reference match", error));
            println!(
                "wrote {}: {} passed, {} failed, {} noisy/contaminated, {} unverifiable, {} unrecorded mappings",
                arguments[3],
                report.passed_regions,
                report.failed_regions,
                report.contaminated_regions,
                report.unverifiable_regions,
                report.unmatched_mappings
            );
            if !report.passed() {
                process::exit(1);
            }
        }
        Some("classify") if arguments.len() == 3 => {
            let wav = decode_input(&arguments[2]);
            let features = ruststrument::classify::analyze_recording(&wav);
            let result = ruststrument::classify::classify_ranked(&features);
            eprintln!(
                "centroid={:.0}Hz rolloff={:.0}Hz flatness={:.3} zcr={:.3} clarity={:.3} \
                 attack={:.1}ms decay={:.2}s sustain={:.3} inharmonicity={:.4} \
                 tristimulus={:.3}/{:.3}/{:.3} odd_harmonics={:.3} pitched={} confidence={:.2}",
                features.spectral_centroid,
                features.spectral_rolloff,
                features.spectral_flatness,
                features.zero_crossing_rate,
                features.pitch_clarity,
                features.attack_time_ms,
                features.decay_time_s,
                features.sustain_ratio,
                features.inharmonicity,
                features.harmonic_tristimulus[0],
                features.harmonic_tristimulus[1],
                features.harmonic_tristimulus[2],
                features.odd_harmonic_ratio,
                features.has_clear_pitch,
                result.confidence
            );
            println!("{}", result.family.label());
        }
        Some("analyze") if arguments.len() == 3 => {
            let report =
                analyze::analyze_wav(&arguments[2]).unwrap_or_else(|error| fail("analyze", error));
            analyze::print_report(&report);
        }
        Some("compare") if arguments.len() == 4 => {
            println!(
                "{}",
                ruststrument::analysis::compare::compare_tracks(&arguments[2], &arguments[3])
            );
        }
        Some("batch") if arguments.len() >= 3 => {
            analyze::analyze_batch(&arguments[2..]).unwrap_or_else(|error| fail("batch", error));
        }
        _ => usage(),
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn song_events_accept_chords_and_rests() {
        assert_eq!(
            parse_song_event("G3,B3,D4:0.5").unwrap(),
            SongEvent {
                notes: vec![55, 59, 62],
                beats: 0.5,
            }
        );
        assert_eq!(
            parse_song_event("rest:1").unwrap(),
            SongEvent {
                notes: Vec::new(),
                beats: 1.0,
            }
        );
        assert!(parse_song_event("C4").is_err());
        assert!(parse_song_event("H4:1").is_err());
    }

    #[test]
    fn zero_render_velocity_remains_silent() {
        let settings = RenderSettings {
            velocity: 0.0,
            articulation: None,
            sample_level: None,
            synth_level: None,
            master_gain: None,
        };
        assert_eq!(settings_velocity(&settings), 0);
    }
}
