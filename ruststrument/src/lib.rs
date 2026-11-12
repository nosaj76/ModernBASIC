// Ruststrument — instrument emulator, sample engine, and audio analyzer
#![allow(clippy::needless_range_loop)]
#![allow(clippy::type_complexity)]

pub mod analysis;
pub mod analyze;
pub mod classify;
pub mod creator;
pub mod db;
pub mod gesture;
pub mod physical;
pub mod pitch;
pub mod profile;
pub mod quality;
pub mod reference_match;
pub mod segment;
pub mod sf2;
pub mod sfz;
pub mod stringed;
pub mod synth;
pub mod technique;
pub mod transcribe;
pub mod wav;

// ---- Audio loading (from audio-analyzer-rs) ----
use std::fs::File;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Maximum decoded frames retained by one loader call. This is roughly six
/// minutes at 44.1 kHz and bounds mono analysis to 64 MiB of sample storage.
pub const MAX_DECODED_AUDIO_FRAMES: usize = 16 * 1_048_576;

fn validate_audio_packet(
    samples: &[f32],
    channels: usize,
    retained_frames: usize,
) -> Result<usize, String> {
    if channels == 0 || samples.len() % channels != 0 {
        return Err("Decoded audio has an invalid channel layout".to_string());
    }
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err("Decoded audio contains non-finite samples".to_string());
    }
    let frames = samples.len() / channels;
    let total = retained_frames
        .checked_add(frames)
        .ok_or_else(|| "Decoded audio exceeds the frame limit".to_string())?;
    if total > MAX_DECODED_AUDIO_FRAMES {
        return Err(format!(
            "Decoded audio exceeds the {} frame limit",
            MAX_DECODED_AUDIO_FRAMES
        ));
    }
    Ok(frames)
}

fn validate_audio_sample_rate(sample_rate: u32) -> Result<u32, String> {
    if sample_rate == 0 || sample_rate > 768_000 {
        return Err(format!("Invalid audio sample rate: {sample_rate}"));
    }
    Ok(sample_rate)
}

#[derive(Debug)]
pub struct AudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub duration: f64,
}

#[derive(Debug)]
pub struct StereoAudioData {
    pub left: Vec<f32>,
    pub right: Vec<f32>,
    pub mono: Vec<f32>,
    pub channels: u32,
    pub sample_rate: u32,
    pub duration: f64,
}

impl AudioData {
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

pub fn load_audio(path: &str) -> Result<AudioData, String> {
    let file = File::open(path).map_err(|e| format!("Cannot open file '{}': {}", path, e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = std::path::Path::new(path).extension() {
        if let Some(ext_str) = ext.to_str() {
            hint.with_extension(ext_str);
        }
    }
    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("Failed to probe audio format: {}", e))?;
    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or("No audio tracks found in file")?;
    let track_id = track.id;
    let sample_rate = validate_audio_sample_rate(
        track
            .codec_params
            .sample_rate
            .ok_or("Unknown sample rate")?,
    )?;
    let codec_params = track.codec_params.clone();
    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|e| format!("Failed to create decoder: {}", e))?;
    let mut all_samples: Vec<f32> = Vec::new();
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(error))
                if error.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(error) => return Err(format!("Failed while reading audio packets: {error}")),
        };
        if packet.track_id() != track_id {
            continue;
        }
        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(SymphoniaError::IoError(_) | SymphoniaError::DecodeError(_)) => continue,
            Err(error) => return Err(format!("Failed while decoding audio: {error}")),
        };
        let spec = *decoded.spec();
        let num_channels = spec.channels.count();
        let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
        sample_buf.copy_interleaved_ref(decoded);
        let samples = sample_buf.samples();
        validate_audio_packet(samples, num_channels, all_samples.len())?;
        if num_channels == 1 {
            all_samples.extend_from_slice(samples);
        } else {
            let mono: Vec<f32> = samples
                .chunks(num_channels)
                .map(|chunk| chunk.iter().sum::<f32>() / num_channels as f32)
                .collect();
            all_samples.extend(mono);
        }
    }
    if all_samples.is_empty() {
        return Err("No audio samples decoded".to_string());
    }
    let duration = all_samples.len() as f64 / sample_rate as f64;
    Ok(AudioData {
        samples: all_samples,
        sample_rate,
        duration,
    })
}

pub fn load_audio_stereo(path: &str) -> Result<StereoAudioData, String> {
    let file = File::open(path).map_err(|e| format!("Cannot open file '{}': {}", path, e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = std::path::Path::new(path).extension() {
        if let Some(ext_str) = ext.to_str() {
            hint.with_extension(ext_str);
        }
    }
    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("Failed to probe audio format: {}", e))?;
    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or("No audio tracks found in file")?;
    let track_id = track.id;
    let sample_rate = validate_audio_sample_rate(
        track
            .codec_params
            .sample_rate
            .ok_or("Unknown sample rate")?,
    )?;
    let codec_params = track.codec_params.clone();
    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|e| format!("Failed to create decoder: {}", e))?;
    let mut left_samples: Vec<f32> = Vec::new();
    let mut right_samples: Vec<f32> = Vec::new();
    let mut file_channels: u32 = 1;
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(error))
                if error.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(error) => return Err(format!("Failed while reading audio packets: {error}")),
        };
        if packet.track_id() != track_id {
            continue;
        }
        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(SymphoniaError::IoError(_) | SymphoniaError::DecodeError(_)) => continue,
            Err(error) => return Err(format!("Failed while decoding audio: {error}")),
        };
        let spec = *decoded.spec();
        let num_channels = spec.channels.count();
        file_channels = num_channels as u32;
        let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
        sample_buf.copy_interleaved_ref(decoded);
        let samples = sample_buf.samples();
        validate_audio_packet(samples, num_channels, left_samples.len())?;
        if num_channels == 1 {
            left_samples.extend_from_slice(samples);
            right_samples.extend_from_slice(samples);
        } else {
            for chunk in samples.chunks(num_channels) {
                left_samples.push(chunk[0]);
                right_samples.push(if chunk.len() > 1 { chunk[1] } else { chunk[0] });
            }
        }
    }
    if left_samples.is_empty() {
        return Err("No audio samples decoded".to_string());
    }
    let mono: Vec<f32> = left_samples
        .iter()
        .zip(right_samples.iter())
        .map(|(l, r)| (l + r) / 2.0)
        .collect();
    let duration = mono.len() as f64 / sample_rate as f64;
    Ok(StereoAudioData {
        left: left_samples,
        right: right_samples,
        mono,
        channels: file_channels,
        sample_rate,
        duration,
    })
}
#[cfg(test)]
mod audio_loading_tests {
    use super::*;

    #[test]
    fn decoded_packets_reject_non_finite_and_oversized_audio() {
        assert!(validate_audio_packet(&[f32::NAN], 1, 0)
            .unwrap_err()
            .contains("non-finite"));
        assert!(validate_audio_packet(&[0.0], 0, 0)
            .unwrap_err()
            .contains("channel layout"));
        assert!(validate_audio_packet(&[0.0], 1, MAX_DECODED_AUDIO_FRAMES)
            .unwrap_err()
            .contains("frame limit"));
    }

    #[test]
    fn decoded_sample_rates_are_bounded_and_nonzero() {
        assert!(validate_audio_sample_rate(0).is_err());
        assert!(validate_audio_sample_rate(768_001).is_err());
        assert_eq!(validate_audio_sample_rate(44_100).unwrap(), 44_100);
    }
}

// ---- Stable C ABI for embedding the live hybrid engine ----

// ---- Stable C ABI for embedding the live hybrid engine ----

use profile::InstrumentBank;
use std::{
    ffi::{c_char, CStr},
    mem::size_of,
    slice,
    sync::{Mutex, MutexGuard, TryLockError},
};
use synth::Engine;

static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);

fn engine() -> MutexGuard<'static, Option<Engine>> {
    ENGINE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

unsafe fn c_string(pointer: *const c_char) -> Option<String> {
    if pointer.is_null() {
        return None;
    }
    // SAFETY: The C ABI requires callers to supply a readable NUL-terminated
    // string for the duration of this call.
    unsafe { CStr::from_ptr(pointer) }
        .to_str()
        .ok()
        .map(str::to_owned)
}

#[no_mangle]
/// Load an RSTI bank into the process-global C engine.
///
/// # Safety
///
/// The path pointer must reference a readable NUL-terminated string for this call.
pub unsafe extern "C" fn rsti_load(path: *const c_char, sample_rate: u32) -> i32 {
    if !(8_000..=384_000).contains(&sample_rate) {
        return -3;
    }
    // SAFETY: Forwarded from this function's C string contract.
    let Some(path) = (unsafe { c_string(path) }) else {
        return -1;
    };
    match InstrumentBank::load(path) {
        Ok(bank) => {
            *engine() = Some(Engine::new(bank, sample_rate));
            0
        }
        Err(_) => -2,
    }
}

#[no_mangle]
pub extern "C" fn rsti_unload() {
    *engine() = None;
}

#[no_mangle]
/// Select a named articulation in the process-global C engine.
///
/// # Safety
///
/// The name pointer must reference a readable NUL-terminated string for this call.
pub unsafe extern "C" fn rsti_set_articulation(name: *const c_char) -> i32 {
    // SAFETY: Forwarded from this function's C string contract.
    let Some(name) = (unsafe { c_string(name) }) else {
        return -1;
    };
    match engine().as_mut() {
        Some(engine) => {
            if engine.set_articulation(&name) {
                0
            } else {
                -2
            }
        }
        None => -3,
    }
}

#[no_mangle]
pub extern "C" fn rsti_note_on(midi: u8, velocity: f32, duration_ms: u32) -> i32 {
    if midi > 127 || !velocity.is_finite() || !(0.0..=1.0).contains(&velocity) {
        return -2;
    }
    let mut guard = engine();
    let Some(engine) = guard.as_mut() else {
        return -1;
    };
    let duration_frames = (duration_ms > 0)
        .then(|| (duration_ms as usize).saturating_mul(engine.sample_rate as usize) / 1_000);
    if velocity == 0.0 {
        engine.note_off(midi);
        return 0;
    }
    let voice_id = engine.note_on_for(
        midi,
        (velocity * 127.0).round().clamp(0.0, 127.0) as u8,
        duration_frames,
    );
    if voice_id == 0 {
        -3
    } else {
        voice_id as i32
    }
}

#[no_mangle]
pub extern "C" fn rsti_note_off(midi: u8) -> i32 {
    if midi > 127 {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.note_off(midi);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_voice_off(voice_id: u64) -> i32 {
    match engine().as_mut() {
        Some(engine) => {
            if engine.voice_off(voice_id) {
                0
            } else {
                -2
            }
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_all_notes_off() -> i32 {
    match engine().as_mut() {
        Some(engine) => {
            engine.all_notes_off();
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_pitch_bend(semitones: f32) -> i32 {
    if !semitones.is_finite() {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.set_pitch_bend(semitones);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_expression(value: f32) -> i32 {
    if !value.is_finite() {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.set_expression(value);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_set_sample_level(value: f32) -> i32 {
    if !value.is_finite() || !(0.0..=2.0).contains(&value) {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.set_sample_level(value);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_set_synth_level(value: f32) -> i32 {
    if !value.is_finite() || !(0.0..=2.0).contains(&value) {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.set_synth_level(value);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_set_master_gain(value: f32) -> i32 {
    if !value.is_finite() || !(0.0..=2.0).contains(&value) {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.set_master_gain(value);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_set_mix(sample_level: f32, synth_level: f32) -> i32 {
    if !sample_level.is_finite()
        || !synth_level.is_finite()
        || !(0.0..=2.0).contains(&sample_level)
        || !(0.0..=2.0).contains(&synth_level)
    {
        return -2;
    }
    match engine().as_mut() {
        Some(engine) => {
            engine.set_mix(sample_level, synth_level);
            0
        }
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn rsti_active_voices() -> usize {
    engine().as_ref().map_or(0, Engine::active_voice_count)
}

#[no_mangle]
/// Render mono floating-point frames into caller-owned memory.
///
/// # Safety
///
/// The output pointer must be writable for the requested number of contiguous
/// f32 values and must not be accessed concurrently during this call.
pub unsafe extern "C" fn rsti_render(mono_output: *mut f32, frames: usize) -> usize {
    if mono_output.is_null() || frames == 0 || frames > isize::MAX as usize / size_of::<f32>() {
        return 0;
    }
    // SAFETY: The C ABI requires a writable array of at least the requested
    // number of f32 values with no concurrent access.
    let output = unsafe { slice::from_raw_parts_mut(mono_output, frames) };
    let mut guard = match ENGINE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            output.fill(0.0);
            return frames;
        }
        Err(TryLockError::Poisoned(error)) => error.into_inner(),
    };
    let Some(engine) = guard.as_mut() else {
        output.fill(0.0);
        return 0;
    };
    engine.render_into(output);
    frames
}

#[cfg(test)]
mod ffi_tests {
    use super::*;
    use std::ffi::CString;

    static FFI_TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn render_callback_returns_silence_instead_of_waiting_for_the_control_lock() {
        let _test_lock = FFI_TEST_MUTEX.lock().unwrap();
        let _control_lock = engine();
        let mut output = [1.0_f32; 16];
        // SAFETY: `output` is writable and contains exactly 16 contiguous f32 values.
        let written = unsafe { rsti_render(output.as_mut_ptr(), output.len()) };
        assert_eq!(written, output.len());
        assert_eq!(output, [0.0; 16]);
    }

    #[test]
    fn c_abi_loads_controls_and_renders_a_bank() {
        let _test_lock = FFI_TEST_MUTEX.lock().unwrap();
        let path = std::env::temp_dir().join(format!(
            "ruststrument-ffi-banjo-{}.rsti",
            std::process::id()
        ));
        InstrumentBank::generator("Five-string Banjo")
            .save(&path)
            .unwrap();
        let path = CString::new(path.to_string_lossy().as_bytes()).unwrap();
        // SAFETY: Both CString pointers remain valid and NUL-terminated for each call.
        unsafe {
            assert_eq!(rsti_load(path.as_ptr(), 44_100), 0);
            let articulation = CString::new("pluck").unwrap();
            assert_eq!(rsti_set_articulation(articulation.as_ptr()), 0);
        }
        assert_eq!(rsti_set_mix(0.0, 1.0), 0);
        assert_eq!(rsti_expression(0.9), 0);
        assert_eq!(rsti_pitch_bend(0.0), 0);
        let voice = rsti_note_on(62, 0.8, 100);
        assert!(voice > 0);
        assert_eq!(rsti_active_voices(), 1);
        let mut output = [0.0_f32; 4_096];
        // SAFETY: `output` is writable and contains exactly 4,096 contiguous f32 values.
        assert_eq!(
            unsafe { rsti_render(output.as_mut_ptr(), output.len()) },
            output.len()
        );
        assert!(output.iter().any(|sample| sample.abs() > 1e-4));
        assert_eq!(rsti_voice_off(voice as u64), 0);
        assert_eq!(rsti_all_notes_off(), 0);
        rsti_unload();
        let _ = std::fs::remove_file(path.to_string_lossy().as_ref());
    }
}
