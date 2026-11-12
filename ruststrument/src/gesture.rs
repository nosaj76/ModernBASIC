//! Instrument-performance gestures built from ordinary note events.
//!
//! A SoundFont describes how notes sound; it does not generate a banjo roll
//! or stagger a guitar strum. Native RSTI rendering keeps that sequencing as
//! a separate, explicit layer so hosts can reproduce it deterministically.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureEvent {
    pub offset_s: f32,
    pub duration_s: f32,
    pub note: u8,
    pub velocity_scale: f32,
}

pub fn schedule(technique: &str, notes: &[u8], length_s: f32) -> Vec<GestureEvent> {
    if notes.is_empty() || !length_s.is_finite() || length_s <= 0.0 {
        return Vec::new();
    }
    let mut notes = notes.to_vec();
    notes.sort_unstable();
    notes.dedup();
    let mut events = Vec::new();
    match technique {
        "strum" => {
            let spread = (0.014 * notes.len().saturating_sub(1) as f32).min(length_s * 0.2);
            let step = spread / notes.len().saturating_sub(1).max(1) as f32;
            for (index, note) in notes.iter().enumerate() {
                let offset = index as f32 * step;
                events.push(event(offset, (length_s - offset).max(0.04), *note, 1.0));
            }
        }
        "chunk" | "chop" => {
            let spread = (0.009 * notes.len().saturating_sub(1) as f32).min(length_s * 0.12);
            let step = spread / notes.len().saturating_sub(1).max(1) as f32;
            for (index, note) in notes.iter().enumerate() {
                let offset = index as f32 * step;
                events.push(event(
                    offset,
                    (length_s * 0.12).clamp(0.035, 0.14),
                    *note,
                    0.92,
                ));
            }
        }
        "three_finger" => {
            let pattern = [0_usize, 1, 2, 0, 2, 1, 2, 0];
            repeated_pattern(&mut events, &notes, &pattern, length_s, 0.125, 0.92);
        }
        "single_finger" | "single_string" | "fingerstyle" => {
            let pattern: Vec<usize> = (0..notes.len()).collect();
            repeated_pattern(&mut events, &notes, &pattern, length_s, 0.18, 0.88);
        }
        "crosspick" => {
            let pattern = [0_usize, 1, 2, 1, 0, 2];
            repeated_pattern(&mut events, &notes, &pattern, length_s, 0.105, 0.78);
        }
        "clawhammer" | "strike" => {
            let beat = 0.5_f32;
            let mut start = 0.0;
            while start < length_s {
                events.push(event(start, beat * 0.85, notes[0], 1.0));
                for note in &notes {
                    events.push(event(start + beat * 0.5, beat * 0.32, *note, 0.72));
                }
                events.push(event(
                    start + beat * 0.75,
                    beat * 0.2,
                    *notes.last().unwrap_or(&notes[0]),
                    0.82,
                ));
                start += beat;
            }
        }
        "tremolo" | "tremolo_pick" | "roll" | "trill" | "shake" => {
            let pattern: Vec<usize> = (0..notes.len()).collect();
            repeated_pattern(&mut events, &notes, &pattern, length_s, 0.075, 0.8);
        }
        "arpeggio" | "glissando" => {
            let step = (length_s * 0.12).min(0.12);
            for (index, note) in notes.iter().enumerate() {
                let offset = index as f32 * step;
                if offset < length_s {
                    events.push(event(offset, (length_s - offset).max(0.04), *note, 1.0));
                }
            }
        }
        "jhala" | "chikari" => {
            let pattern = if notes.len() > 1 {
                vec![notes.len() - 1, 0, notes.len() - 1, 1 % notes.len()]
            } else {
                vec![0]
            };
            repeated_pattern(&mut events, &notes, &pattern, length_s, 0.085, 0.72);
        }
        "noter_drone" => {
            let melody = notes.len() - 1;
            let pattern = if notes.len() >= 3 {
                vec![melody, 0, melody, 1]
            } else {
                vec![melody, 0]
            };
            repeated_pattern(&mut events, &notes, &pattern, length_s, 0.16, 0.82);
        }
        _ => {
            for note in notes {
                events.push(event(0.0, length_s, note, 1.0));
            }
        }
    }
    events.retain(|event| event.offset_s < length_s);
    events.sort_by(|left, right| {
        left.offset_s
            .partial_cmp(&right.offset_s)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.note.cmp(&right.note))
    });
    events
}

fn repeated_pattern(
    output: &mut Vec<GestureEvent>,
    notes: &[u8],
    pattern: &[usize],
    length_s: f32,
    step_s: f32,
    gate: f32,
) {
    if pattern.is_empty() {
        return;
    }
    let mut index = 0;
    let mut offset = 0.0_f32;
    while offset < length_s {
        let note = notes[pattern[index % pattern.len()] % notes.len()];
        output.push(event(offset, step_s * gate, note, 1.0));
        index += 1;
        offset = index as f32 * step_s;
    }
}

fn event(offset_s: f32, duration_s: f32, note: u8, velocity_scale: f32) -> GestureEvent {
    GestureEvent {
        offset_s,
        duration_s,
        note,
        velocity_scale,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strum_is_staggered_and_roll_repeats() {
        let strum = schedule("strum", &[67, 60, 64], 1.0);
        assert_eq!(strum.len(), 3);
        assert!(strum[0].offset_s < strum[1].offset_s);
        let roll = schedule("three_finger", &[60, 64, 67], 1.0);
        assert!(roll.len() >= 8);
        assert!(roll
            .windows(2)
            .all(|pair| pair[0].offset_s <= pair[1].offset_s));
    }
}
