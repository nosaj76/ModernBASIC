use crate::{
    profile::{InstrumentBank, Trigger},
    technique,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

fn similarity(left: &[f32], right: &[f32]) -> f32 {
    let dot: f32 = left
        .iter()
        .zip(right)
        .map(|(left, right)| left * right)
        .sum();
    let left_energy: f32 = left.iter().map(|value| value * value).sum();
    let right_energy: f32 = right.iter().map(|value| value * value).sum();
    dot / (left_energy * right_energy).sqrt().max(1e-9)
}

fn coverage(bank: &InstrumentBank) -> (usize, usize) {
    let mut covered = 0;
    let total = 128 * 127;
    for key in 0..=127_u8 {
        for velocity in 1..=127_u8 {
            if bank.regions.iter().any(|region| {
                region.trigger == Trigger::Attack
                    && region.articulation == bank.default_articulation
                    && key >= region.key_lo
                    && key <= region.key_hi
                    && velocity >= region.vel_lo
                    && velocity <= region.vel_hi
            }) {
                covered += 1;
            }
        }
    }
    (covered, total)
}

pub fn report(bank: &InstrumentBank) -> String {
    let mut output = String::new();
    let generator_only = bank.synth.sample_level == 0.0 && bank.synth.synth_level > 0.0;
    writeln!(output, "# Ruststrument quality report: {}\n", bank.name).unwrap();
    writeln!(output, "- Family: {}", bank.family).unwrap();
    writeln!(output, "- Model: {}", bank.synth.model.label()).unwrap();
    writeln!(output, "- Regions: {}", bank.regions.len()).unwrap();
    writeln!(output, "- Polyphony: {}", bank.polyphony).unwrap();
    writeln!(
        output,
        "- Articulations: {}",
        bank.articulations().join(", ")
    )
    .unwrap();
    let captured: BTreeSet<String> = bank
        .articulations()
        .into_iter()
        .map(|name| technique::canonicalize(&bank.name, &bank.family, &name))
        .collect();
    let recommended = technique::recommended(&bank.name, &bank.family);
    let missing: Vec<_> = recommended
        .iter()
        .copied()
        .filter(|name| !captured.contains(*name))
        .collect();
    writeln!(
        output,
        "- Instrument-aware technique palette: {}",
        recommended.join(", ")
    )
    .unwrap();
    writeln!(
        output,
        "- Technique coverage: {}/{}{}",
        recommended.len().saturating_sub(missing.len()),
        recommended.len(),
        if missing.is_empty() {
            String::new()
        } else {
            format!(" (not recorded: {})", missing.join(", "))
        }
    )
    .unwrap();
    writeln!(
        output,
        "- Playback levels: sample {:.2}, synth {:.2}, master {:.2}\n",
        bank.synth.sample_level, bank.synth.synth_level, bank.synth.master_gain
    )
    .unwrap();
    if generator_only {
        writeln!(
            output,
            "- Sound source: generated model only; silent region samples provide zoning and analysis metadata, not audible playback.\n"
        )
        .unwrap();
    }
    writeln!(output, "## Stored analysis/model configuration\n").unwrap();
    writeln!(
        output,
        "| Attack | Decay | Sustain | Release | Noise | Brightness | Damping | Pluck position | Vibrato | Transient | Modes |"
    )
    .unwrap();
    writeln!(
        output,
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    )
    .unwrap();
    let active_modes = bank
        .synth
        .mode_amplitudes
        .iter()
        .filter(|amplitude| **amplitude > 0.0)
        .count();
    writeln!(
        output,
        "| {:.4}s | {:.4}s | {:.3} | {:.4}s | {:.3} | {:.3} | {:.5} | {:.3} | {:.1}Hz/{:.1}¢ | {:.3} | {} |\n",
        bank.synth.attack_s,
        bank.synth.decay_s,
        bank.synth.sustain,
        bank.synth.release_s,
        bank.synth.noise,
        bank.synth.brightness,
        bank.synth.damping,
        bank.synth.pluck_position,
        bank.synth.vibrato_rate_hz,
        bank.synth.vibrato_depth_cents,
        bank.synth.transient_mix,
        active_modes
    )
    .unwrap();

    let mut groups: BTreeMap<(String, u8), Vec<_>> = BTreeMap::new();
    for region in &bank.regions {
        groups
            .entry((
                region.articulation.clone(),
                u8::from(region.trigger == Trigger::Release),
            ))
            .or_default()
            .push(region);
    }
    let mut lowest_similarity = 1.0_f32;
    let mut similarity_pairs = 0;
    for ((articulation, trigger), mut regions) in groups {
        regions.sort_by_key(|region| (region.root_key, region.vel_lo, region.round_robin));
        for pair in regions.windows(2) {
            lowest_similarity =
                lowest_similarity.min(similarity(&pair[0].partials, &pair[1].partials));
            similarity_pairs += 1;
        }
        writeln!(
            output,
            "## {} / {}\n",
            articulation,
            if trigger == 0 { "attack" } else { "release" }
        )
        .unwrap();
        writeln!(
            output,
            "| Root | Keys | Velocity | RR | Tuning | Length | Loop |"
        )
        .unwrap();
        writeln!(output, "|---:|---:|---:|---:|---:|---:|:---:|").unwrap();
        for region in regions {
            writeln!(
                output,
                "| {} | {}–{} | {}–{} | {} | {:+.1}¢ | {:.2}s | {} |",
                region.root_key,
                region.key_lo,
                region.key_hi,
                region.vel_lo,
                region.vel_hi,
                region.round_robin,
                region.tune_cents,
                region.samples.len() as f32 / region.sample_rate as f32,
                if region.loop_start.is_some() {
                    "yes"
                } else {
                    "no"
                }
            )
            .unwrap();
        }
        output.push('\n');
    }

    let attacks = bank
        .regions
        .iter()
        .filter(|region| region.trigger == Trigger::Attack)
        .count();
    let releases = bank
        .regions
        .iter()
        .filter(|region| region.trigger == Trigger::Release)
        .count();
    let (covered, total) = coverage(bank);
    writeln!(output, "## Evaluation\n").unwrap();
    writeln!(
        output,
        "- Default-articulation key/velocity coverage: {:.1}% ({covered}/{total})",
        covered as f32 / total as f32 * 100.0
    )
    .unwrap();
    if similarity_pairs > 0 {
        writeln!(
            output,
            "- Lowest adjacent-region spectral similarity: {:.1}%",
            lowest_similarity * 100.0
        )
        .unwrap();
    }
    if covered != total {
        writeln!(
            output,
            "- **Coverage error:** some MIDI key/velocity combinations have no attack region."
        )
        .unwrap();
    }
    if attacks < 12 && !generator_only {
        writeln!(
            output,
            "- **Authenticity warning:** only {attacks} attack regions are available. \
             The mapper prevents silence, but widely stretched samples are less realistic."
        )
        .unwrap();
    }
    if releases == 0 && !generator_only {
        writeln!(
            output,
            "- **Release note:** no release-trigger recordings are present; the learned release envelope is used."
        )
        .unwrap();
    }
    if bank
        .regions
        .iter()
        .any(|region| region.tune_cents.abs() > 35.0)
    {
        writeln!(
            output,
            "- **Tuning warning:** at least one sample differs from its root label by over 35 cents."
        )
        .unwrap();
    }
    if similarity_pairs > 0 && lowest_similarity < 0.55 {
        writeln!(
            output,
            "- **Timbre warning:** adjacent regions differ sharply; check microphone position, articulation labels, and roots."
        )
        .unwrap();
    }
    if bank.regions.iter().all(|region| region.round_robin == 1) && !generator_only {
        writeln!(
            output,
            "- **Variation warning:** there are no round-robin alternatives, so repeated notes may sound mechanical."
        )
        .unwrap();
    }
    writeln!(
        output,
        "- SF2 export bakes the current playback levels into ordinary PCM zones. \
         Native RSTI playback retains adjustable layers and every round robin."
    )
    .unwrap();
    output
}
