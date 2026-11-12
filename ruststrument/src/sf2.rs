//! SoundFont 2.04 writer.
//!
//! SoundFont players cannot run Ruststrument's physical model, so every
//! attack region is rendered with its configured sample and model layers
//! before it is written to the SF2. Key and velocity zones remain editable
//! in the RSTI bank and are preserved here.

use crate::{
    profile::{InstrumentBank, Region, Trigger},
    synth,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::Path,
};

const SAMPLE_GUARD: usize = 46;

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn malformed(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn checked_u16(value: usize, message: &'static str) -> io::Result<u16> {
    u16::try_from(value).map_err(|_| invalid(message))
}

fn checked_u32(value: usize, message: &'static str) -> io::Result<u32> {
    u32::try_from(value).map_err(|_| invalid(message))
}

fn chunk(id: &[u8; 4], mut data: Vec<u8>) -> io::Result<Vec<u8>> {
    let length = checked_u32(data.len(), "SoundFont chunk exceeds 4 GiB")?;
    let mut output = Vec::with_capacity(data.len() + 9);
    output.extend(id);
    output.extend(length.to_le_bytes());
    output.append(&mut data);
    if output.len() & 1 == 1 {
        output.push(0);
    }
    Ok(output)
}

fn info_string(id: &[u8; 4], value: &str) -> io::Result<Vec<u8>> {
    let mut data = value.as_bytes().to_vec();
    data.push(0);
    if data.len() & 1 == 1 {
        data.push(0);
    }
    chunk(id, data)
}

fn list(kind: &[u8; 4], parts: Vec<Vec<u8>>) -> io::Result<Vec<u8>> {
    let mut data = kind.to_vec();
    for part in parts {
        data.extend(part);
    }
    chunk(b"LIST", data)
}

fn name20(name: &str) -> [u8; 20] {
    let mut output = [0; 20];
    let bytes = name.as_bytes();
    output[..bytes.len().min(19)].copy_from_slice(&bytes[..bytes.len().min(19)]);
    output
}

fn preset_header(name: &str, preset: u16, bank: u16, bag_index: u16) -> Vec<u8> {
    let mut output = name20(name).to_vec();
    output.extend(preset.to_le_bytes());
    output.extend(bank.to_le_bytes());
    output.extend(bag_index.to_le_bytes());
    output.extend([0; 12]);
    output
}

fn instrument_header(name: &str, bag_index: u16) -> Vec<u8> {
    let mut output = name20(name).to_vec();
    output.extend(bag_index.to_le_bytes());
    output
}

fn bag(generator_index: u16) -> Vec<u8> {
    let mut output = generator_index.to_le_bytes().to_vec();
    output.extend(0_u16.to_le_bytes());
    output
}

fn generator(operator: u16, amount: u16) -> [u8; 4] {
    let mut output = [0_u8; 4];
    output[..2].copy_from_slice(&operator.to_le_bytes());
    output[2..].copy_from_slice(&amount.to_le_bytes());
    output
}

fn sample_header(
    name: &str,
    start: u32,
    end: u32,
    loop_start: u32,
    loop_end: u32,
    sample_rate: u32,
    root: u8,
) -> Vec<u8> {
    let mut output = name20(name).to_vec();
    for value in [start, end, loop_start, loop_end] {
        output.extend(value.to_le_bytes());
    }
    output.extend(sample_rate.to_le_bytes());
    output.push(root);
    output.push(0);
    output.extend(0_u16.to_le_bytes());
    output.extend(1_u16.to_le_bytes());
    output
}

fn zone_range(low: u8, high: u8) -> u16 {
    u16::from(low) | (u16::from(high) << 8)
}

fn transformed_loop(region: &Region, baked_length: usize) -> Option<(u32, u32)> {
    let (start, end) = (region.loop_start?, region.loop_end?);
    let correction = 2_f64.powf(-region.tune_cents as f64 / 1_200.0);
    let baked_start = (start as f64 / correction).round() as usize;
    let baked_end = (end as f64 / correction).round() as usize;
    if baked_start + 2 <= baked_end && baked_end <= baked_length {
        Some((baked_start as u32, baked_end as u32))
    } else {
        None
    }
}

fn gm_program_for_name(name: &str) -> Option<u16> {
    let name = name.to_ascii_lowercase();
    let mappings: &[(&[&str], u16)] = &[
        (&["banjo"], 105),
        (&["sitar"], 104),
        (&["shamisen"], 106),
        (&["koto"], 107),
        (&["kalimba"], 108),
        (&["bagpipe"], 109),
        (&["fiddle"], 110),
        (&["steel guitar"], 25),
        (&["guitar"], 24),
        (&["ukulele", "mandolin", "lute"], 24),
        (&["violin"], 40),
        (&["viola"], 41),
        (&["cello"], 42),
        (&["contrabass", "double bass"], 43),
        (&["harp"], 46),
        (&["timpani"], 47),
        (&["trumpet"], 56),
        (&["trombone"], 57),
        (&["tuba"], 58),
        (&["english horn"], 69),
        (&["french horn", "horn"], 60),
        (&["soprano sax"], 64),
        (&["tenor sax"], 66),
        (&["baritone sax"], 67),
        (&["alto sax", "saxophone", "sax"], 65),
        (&["oboe"], 68),
        (&["bassoon"], 70),
        (&["clarinet"], 71),
        (&["piccolo"], 72),
        (&["pan flute"], 75),
        (&["flute"], 73),
        (&["recorder"], 74),
        (&["piano"], 0),
    ];
    mappings
        .iter()
        .find(|(words, _)| words.iter().any(|word| contains_term(&name, word)))
        .map(|(_, program)| *program)
}

fn contains_term(name: &str, term: &str) -> bool {
    name.match_indices(term).any(|(start, _)| {
        let before_is_boundary = name[..start]
            .chars()
            .next_back()
            .map_or(true, |character| !character.is_ascii_alphanumeric());
        let end = start + term.len();
        let after_is_boundary = name[end..]
            .chars()
            .next()
            .map_or(true, |character| !character.is_ascii_alphanumeric());
        before_is_boundary && after_is_boundary
    })
}

#[derive(Clone, Debug)]
struct Preset {
    program: u16,
    bank: u16,
    name: String,
    articulation: String,
}

fn presets(bank: &InstrumentBank, articulations: &[String]) -> io::Result<Vec<Preset>> {
    let gm_program = gm_program_for_name(&bank.name);
    let mut used = BTreeSet::from([0_u16]);
    if let Some(program) = gm_program {
        used.insert(program);
    }
    let mut presets = vec![Preset {
        program: 0,
        bank: 0,
        name: bank.name.clone(),
        articulation: bank.default_articulation.clone(),
    }];
    let mut available_programs = (1_u16..=127).filter(|program| !used.contains(program));
    for articulation in articulations
        .iter()
        .filter(|name| **name != bank.default_articulation)
    {
        let next_program = available_programs
            .next()
            .ok_or_else(|| invalid("SoundFont supports at most 128 MIDI programs per bank"))?;
        presets.push(Preset {
            program: next_program,
            bank: 0,
            name: format!("{} {}", bank.name, articulation),
            articulation: articulation.clone(),
        });
    }
    if let Some(program) = gm_program.filter(|program| *program != 0) {
        presets.push(Preset {
            program,
            bank: 0,
            name: bank.name.clone(),
            articulation: bank.default_articulation.clone(),
        });
    }
    if matches!(bank.family.as_str(), "membrane" | "metal" | "wood") {
        presets.push(Preset {
            program: 0,
            bank: 128,
            name: format!("{} Percussion", bank.name),
            articulation: bank.default_articulation.clone(),
        });
    }
    presets.sort_by_key(|preset| (preset.bank, preset.program));
    Ok(presets)
}

pub fn write(bank: &InstrumentBank, path: impl AsRef<Path>) -> io::Result<()> {
    bank.validate()?;
    let attack_regions: Vec<usize> = bank
        .regions
        .iter()
        .enumerate()
        .filter(|(_, region)| region.trigger == Trigger::Attack && region.round_robin == 1)
        .map(|(index, _)| index)
        .collect();
    if attack_regions.is_empty() {
        return Err(invalid("bank has no attack samples"));
    }

    let mut articulations = bank.articulations();
    articulations.sort();
    if let Some(position) = articulations
        .iter()
        .position(|name| name == &bank.default_articulation)
    {
        articulations.swap(0, position);
    }
    let presets = presets(bank, &articulations)?;
    if presets.len() > u16::MAX as usize {
        return Err(invalid("too many SoundFont presets"));
    }

    let mut sample_data = Vec::new();
    let mut sample_headers = Vec::new();
    let mut sample_ids = BTreeMap::new();
    let mut loop_points = BTreeMap::new();
    for &region_index in &attack_regions {
        let region = &bank.regions[region_index];
        let baked = synth::bake_region(bank, region_index)?;
        let start = checked_u32(sample_data.len() / 2, "SoundFont sample pool exceeds 4 GiB")?;
        for sample in &baked.samples {
            let pcm = (sample.clamp(-1.0, 1.0) * 32_767.0).round() as i16;
            sample_data.extend(pcm.to_le_bytes());
        }
        let end = start
            .checked_add(checked_u32(
                baked.samples.len(),
                "SoundFont sample exceeds 4 GiB",
            )?)
            .ok_or_else(|| invalid("SoundFont sample offsets overflow"))?;
        let transformed = transformed_loop(region, baked.samples.len());
        let (loop_start, loop_end) = transformed
            .map(|(loop_start, loop_end)| {
                (
                    start.saturating_add(loop_start),
                    start.saturating_add(loop_end),
                )
            })
            .unwrap_or((end, end));
        for _ in 0..SAMPLE_GUARD {
            sample_data.extend(0_i16.to_le_bytes());
        }

        let sample_id = checked_u16(sample_ids.len(), "too many SoundFont samples")?;
        sample_ids.insert(region_index, sample_id);
        loop_points.insert(region_index, transformed.is_some());
        sample_headers.extend(sample_header(
            &region.name,
            start,
            end,
            loop_start,
            loop_end,
            baked.sample_rate,
            region.root_key,
        ));
    }
    let end_of_samples = checked_u32(sample_data.len() / 2, "SoundFont sample pool exceeds 4 GiB")?;
    sample_headers.extend(sample_header(
        "EOS",
        end_of_samples,
        end_of_samples,
        end_of_samples,
        end_of_samples,
        44_100,
        0,
    ));

    let mut preset_headers = Vec::new();
    let mut preset_bags = Vec::new();
    let mut preset_generators: Vec<[u8; 4]> = Vec::new();
    let mut instrument_headers = Vec::new();
    let mut instrument_bags = Vec::new();
    let mut instrument_generators: Vec<[u8; 4]> = Vec::new();

    for (instrument_index, preset) in presets.iter().enumerate() {
        let instrument_index = checked_u16(instrument_index, "too many SoundFont instruments")?;
        preset_headers.extend(preset_header(
            &preset.name,
            preset.program,
            preset.bank,
            checked_u16(preset_bags.len() / 4, "too many preset bags")?,
        ));
        preset_bags.extend(bag(checked_u16(
            preset_generators.len(),
            "too many preset generators",
        )?));
        preset_generators.push(generator(41, instrument_index));

        instrument_headers.extend(instrument_header(
            &preset.name,
            checked_u16(instrument_bags.len() / 4, "too many instrument bags")?,
        ));
        for &region_index in attack_regions
            .iter()
            .filter(|&&region_index| bank.regions[region_index].articulation == preset.articulation)
        {
            let region = &bank.regions[region_index];
            instrument_bags.extend(bag(checked_u16(
                instrument_generators.len(),
                "too many instrument generators",
            )?));
            instrument_generators.push(generator(43, zone_range(region.key_lo, region.key_hi)));
            instrument_generators.push(generator(44, zone_range(region.vel_lo, region.vel_hi)));
            if loop_points.get(&region_index) == Some(&true) {
                instrument_generators.push(generator(54, 1));
            }
            instrument_generators.push(generator(58, u16::from(region.root_key)));
            instrument_generators.push(generator(
                53,
                *sample_ids
                    .get(&region_index)
                    .ok_or_else(|| invalid("missing SoundFont sample"))?,
            ));
        }
    }

    preset_headers.extend(preset_header(
        "EOP",
        0,
        0,
        checked_u16(preset_bags.len() / 4, "too many preset bags")?,
    ));
    preset_bags.extend(bag(checked_u16(
        preset_generators.len(),
        "too many preset generators",
    )?));
    instrument_headers.extend(instrument_header(
        "EOI",
        checked_u16(instrument_bags.len() / 4, "too many instrument bags")?,
    ));
    instrument_bags.extend(bag(checked_u16(
        instrument_generators.len(),
        "too many instrument generators",
    )?));
    // SF2 hydra generator tables end with a terminal record. Bag indices point
    // to the number of real generators, so append the terminator only after
    // every terminal bag index has been recorded.
    preset_generators.push(generator(0, 0));
    instrument_generators.push(generator(0, 0));

    let information = list(
        b"INFO",
        vec![
            chunk(b"ifil", [2_u16.to_le_bytes(), 4_u16.to_le_bytes()].concat())?,
            info_string(b"isng", "EMU8000")?,
            info_string(b"INAM", &bank.name)?,
            info_string(
                b"ISFT",
                &format!("Ruststrument Forge {}", env!("CARGO_PKG_VERSION")),
            )?,
        ],
    )?;
    let sample_section = list(b"sdta", vec![chunk(b"smpl", sample_data)?])?;
    let preset_generators: Vec<u8> = preset_generators.into_iter().flatten().collect();
    let instrument_generators: Vec<u8> = instrument_generators.into_iter().flatten().collect();
    let preset_data = list(
        b"pdta",
        vec![
            chunk(b"phdr", preset_headers)?,
            chunk(b"pbag", preset_bags)?,
            chunk(b"pmod", vec![0; 10])?,
            chunk(b"pgen", preset_generators)?,
            chunk(b"inst", instrument_headers)?,
            chunk(b"ibag", instrument_bags)?,
            chunk(b"imod", vec![0; 10])?,
            chunk(b"igen", instrument_generators)?,
            chunk(b"shdr", sample_headers)?,
        ],
    )?;
    let mut body = b"sfbk".to_vec();
    body.extend(information);
    body.extend(sample_section);
    body.extend(preset_data);
    let bytes = chunk(b"RIFF", body)?;
    validate_bytes(&bytes)?;
    fs::write(path, bytes)
}

/// Validate the RIFF/SF2 structure and the index bounds used by the SoundFont
/// hydra tables. This is intentionally stricter than merely checking for four
/// chunk names.
pub fn validate_file(path: impl AsRef<Path>) -> io::Result<()> {
    validate_bytes(&fs::read(path)?)
}

pub fn validate_bytes(bytes: &[u8]) -> io::Result<()> {
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"sfbk" {
        return Err(malformed("not a SoundFont RIFF bank"));
    }
    let declared = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    if declared
        .checked_add(8)
        .map_or(true, |length| length > bytes.len())
    {
        return Err(malformed("RIFF size exceeds the file"));
    }
    let riff_end = declared + 8;
    let info =
        find_list(bytes, riff_end, b"INFO")?.ok_or_else(|| malformed("missing INFO list"))?;
    let sdta =
        find_list(bytes, riff_end, b"sdta")?.ok_or_else(|| malformed("missing sdta list"))?;
    let pdta =
        find_list(bytes, riff_end, b"pdta")?.ok_or_else(|| malformed("missing pdta list"))?;
    let ifil = find_chunk(bytes, info, b"ifil")?.ok_or_else(|| malformed("missing ifil"))?;
    if ifil.len() != 4 {
        return Err(malformed("invalid ifil record"));
    }
    let major = u16::from_le_bytes(ifil[..2].try_into().unwrap());
    if major != 2 {
        return Err(malformed("unsupported SoundFont major version"));
    }
    let sample_data =
        find_chunk(bytes, sdta, b"smpl")?.ok_or_else(|| malformed("missing 16-bit sample pool"))?;
    if sample_data.len() < SAMPLE_GUARD * 2 || sample_data.len() & 1 == 1 {
        return Err(malformed("invalid SoundFont sample pool"));
    }

    let required = [
        (b"phdr", 38_usize),
        (b"pbag", 4),
        (b"pmod", 10),
        (b"pgen", 4),
        (b"inst", 22),
        (b"ibag", 4),
        (b"imod", 10),
        (b"igen", 4),
        (b"shdr", 46),
    ];
    let mut tables = BTreeMap::new();
    for (id, record_size) in required {
        let table = find_chunk(bytes, pdta, id)?.ok_or_else(|| malformed("missing hydra table"))?;
        if table.is_empty() || table.len() % record_size != 0 {
            return Err(malformed("invalid hydra record size"));
        }
        tables.insert(*id, table);
    }
    let phdr = tables[b"phdr"];
    let pbag = tables[b"pbag"];
    let pgen = tables[b"pgen"];
    let inst = tables[b"inst"];
    let ibag = tables[b"ibag"];
    let igen = tables[b"igen"];
    let shdr = tables[b"shdr"];
    if phdr.len() < 38 || &phdr[phdr.len() - 38..phdr.len() - 35] != b"EOP" {
        return Err(malformed("missing terminal preset record"));
    }
    if inst.len() < 22 || &inst[inst.len() - 22..inst.len() - 19] != b"EOI" {
        return Err(malformed("missing terminal instrument record"));
    }
    if shdr.len() < 46 || &shdr[shdr.len() - 46..shdr.len() - 43] != b"EOS" {
        return Err(malformed("missing terminal sample record"));
    }
    if pgen.len() < 4 || pgen[pgen.len() - 4..] != [0; 4] {
        return Err(malformed("missing terminal preset generator record"));
    }
    if igen.len() < 4 || igen[igen.len() - 4..] != [0; 4] {
        return Err(malformed("missing terminal instrument generator record"));
    }
    validate_bags(pbag, pgen.len() / 4 - 1)?;
    validate_bags(ibag, igen.len() / 4 - 1)?;
    let preset_terminal_bag = u16::from_le_bytes(
        phdr[phdr.len() - 38 + 24..phdr.len() - 38 + 26]
            .try_into()
            .unwrap(),
    ) as usize;
    let instrument_terminal_bag =
        u16::from_le_bytes(inst[inst.len() - 22 + 20..inst.len()].try_into().unwrap()) as usize;
    if preset_terminal_bag + 1 != pbag.len() / 4 || instrument_terminal_bag + 1 != ibag.len() / 4 {
        return Err(malformed("terminal bag index is inconsistent"));
    }
    let sample_count = sample_data.len() / 2;
    for header in shdr[..shdr.len() - 46].chunks_exact(46) {
        let start = u32::from_le_bytes(header[20..24].try_into().unwrap()) as usize;
        let end = u32::from_le_bytes(header[24..28].try_into().unwrap()) as usize;
        let loop_start = u32::from_le_bytes(header[28..32].try_into().unwrap()) as usize;
        let loop_end = u32::from_le_bytes(header[32..36].try_into().unwrap()) as usize;
        if start > end
            || end > sample_count
            || loop_start < start
            || loop_start > end
            || loop_end < loop_start
            || loop_end > end
        {
            return Err(malformed("sample header points outside the sample pool"));
        }
    }
    Ok(())
}

fn validate_bags(bags: &[u8], generator_count: usize) -> io::Result<()> {
    let mut previous = 0;
    for bag in bags.chunks_exact(4) {
        let index = u16::from_le_bytes(bag[..2].try_into().unwrap()) as usize;
        if index < previous || index > generator_count {
            return Err(malformed("bag generator index is invalid"));
        }
        previous = index;
    }
    Ok(())
}

fn find_list(bytes: &[u8], riff_end: usize, kind: &[u8; 4]) -> io::Result<Option<(usize, usize)>> {
    for (id, start, end) in chunk_ranges(bytes, (12, riff_end))? {
        if &id == b"LIST" && end.saturating_sub(start) >= 4 && &bytes[start..start + 4] == kind {
            return Ok(Some((start + 4, end)));
        }
    }
    Ok(None)
}

fn find_chunk<'a>(
    bytes: &'a [u8],
    range: (usize, usize),
    wanted: &[u8; 4],
) -> io::Result<Option<&'a [u8]>> {
    for (id, start, end) in chunk_ranges(bytes, range)? {
        if &id == wanted {
            return Ok(Some(&bytes[start..end]));
        }
    }
    Ok(None)
}

fn chunk_ranges(bytes: &[u8], range: (usize, usize)) -> io::Result<Vec<([u8; 4], usize, usize)>> {
    let (mut position, limit) = range;
    if position > limit || limit > bytes.len() {
        return Err(malformed("chunk range is outside the file"));
    }
    let mut chunks = Vec::new();
    while position < limit {
        if position + 8 > limit {
            return Err(malformed("truncated RIFF chunk header"));
        }
        let id: [u8; 4] = bytes[position..position + 4].try_into().unwrap();
        let length =
            u32::from_le_bytes(bytes[position + 4..position + 8].try_into().unwrap()) as usize;
        let start = position + 8;
        let end = start
            .checked_add(length)
            .filter(|end| *end <= limit)
            .ok_or_else(|| malformed("RIFF chunk exceeds its parent"))?;
        chunks.push((id, start, end));
        position = end + (length & 1);
    }
    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wav::Wav;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn writes_named_soundfont_with_required_sections() {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let path = std::env::temp_dir().join(format!(
            "ruststrument-sf2-{}-{}.sf2",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let samples: Vec<f32> = (0..2_048)
            .map(|index| (std::f32::consts::TAU * 220.0 * index as f32 / 44_100.0).sin())
            .collect();
        let bank = InstrumentBank::single(
            "Test Banjo",
            &Wav {
                sample_rate: 44_100,
                samples,
            },
        );
        write(&bank, &path).unwrap();
        let bytes = fs::read(&path).unwrap();
        let _ = fs::remove_file(path);
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"sfbk");
        assert!(bytes.windows(10).any(|window| window == b"Test Banjo"));
        for required in [b"INFO", b"sdta", b"pdta", b"phdr", b"shdr"] {
            assert!(bytes.windows(4).any(|window| window == required));
        }
        for id in [b"isng", b"INAM", b"ISFT"] {
            let offset = bytes
                .windows(4)
                .position(|window| window == id)
                .expect("missing INFO string");
            let length =
                u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap()) as usize;
            assert_eq!(length & 1, 0, "INFO string chunks must have even sizes");
            assert_eq!(bytes[offset + 8 + length - 1], 0);
        }
        let declared = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        let pdta = find_list(&bytes, declared + 8, b"pdta")
            .unwrap()
            .expect("missing pdta list");
        for (bag_id, generator_id) in [(b"pbag", b"pgen"), (b"ibag", b"igen")] {
            let bags = find_chunk(&bytes, pdta, bag_id)
                .unwrap()
                .expect("missing bag table");
            let generators = find_chunk(&bytes, pdta, generator_id)
                .unwrap()
                .expect("missing generator table");
            assert_eq!(&generators[generators.len() - 4..], &[0; 4]);
            let terminal_index =
                u16::from_le_bytes(bags[bags.len() - 4..bags.len() - 2].try_into().unwrap());
            assert_eq!(usize::from(terminal_index), generators.len() / 4 - 1);
        }
    }

    #[test]
    fn maps_names_to_modern64_general_midi_programs() {
        assert_eq!(gm_program_for_name("My Banjo"), Some(105));
        assert_eq!(gm_program_for_name("Concert Flute"), Some(73));
        assert_eq!(gm_program_for_name("Unknown Thing"), None);
    }

    #[test]
    fn percussion_family_gets_midi_bank_128() {
        let samples: Vec<f32> = (0..2_048)
            .map(|index| (std::f32::consts::TAU * 110.0 * index as f32 / 44_100.0).sin())
            .collect();
        let bank = InstrumentBank::single(
            "Concert Timpani",
            &Wav {
                sample_rate: 44_100,
                samples,
            },
        );
        let result = presets(&bank, &bank.articulations()).unwrap();
        assert!(result.iter().any(|preset| preset.bank == 128));
    }

    #[test]
    fn validator_rejects_a_truncated_riff() {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend(100_u32.to_le_bytes());
        bytes.extend(b"sfbk");
        assert!(validate_bytes(&bytes).is_err());
    }
}
