use std::{fs, io, path::Path};

#[derive(Clone, Debug, PartialEq)]
pub struct Wav {
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

pub fn read(path: impl AsRef<Path>) -> io::Result<Wav> {
    let bytes = fs::read(path)?;
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err(invalid("not a RIFF/WAVE file"));
    }

    let mut position = 12_usize;
    let mut format_chunk = None;
    let mut data_chunk = None;
    while position
        .checked_add(8)
        .is_some_and(|end| end <= bytes.len())
    {
        let id = &bytes[position..position + 4];
        let length = le_u32(&bytes[position + 4..position + 8])? as usize;
        position += 8;
        let end = position
            .checked_add(length)
            .ok_or_else(|| invalid("WAV chunk length overflow"))?;
        if end > bytes.len() {
            return Err(invalid("truncated WAV chunk"));
        }
        if id == b"fmt " {
            format_chunk = Some(&bytes[position..end]);
        } else if id == b"data" {
            data_chunk = Some(&bytes[position..end]);
        }
        position = end
            .checked_add(length & 1)
            .ok_or_else(|| invalid("WAV padding overflow"))?;
    }

    let format = format_chunk.ok_or_else(|| invalid("missing fmt chunk"))?;
    let data = data_chunk.ok_or_else(|| invalid("missing data chunk"))?;
    if format.len() < 16 {
        return Err(invalid("fmt chunk is shorter than 16 bytes"));
    }

    let original_kind = le_u16(&format[0..2])?;
    let kind = if original_kind == 0xfffe {
        if format.len() < 40 {
            return Err(invalid("truncated WAVE_FORMAT_EXTENSIBLE fmt chunk"));
        }
        le_u16(&format[24..26])?
    } else {
        original_kind
    };
    let channels = le_u16(&format[2..4])? as usize;
    let sample_rate = le_u32(&format[4..8])?;
    let block_align = le_u16(&format[12..14])? as usize;
    let bits = le_u16(&format[14..16])?;
    if channels == 0 || channels > 64 {
        return Err(invalid("invalid channel count"));
    }
    if !(8_000..=384_000).contains(&sample_rate) {
        return Err(invalid("unsupported sample rate"));
    }
    let bytes_per_sample = usize::from(bits.div_ceil(8));
    let minimum_align = bytes_per_sample
        .checked_mul(channels)
        .ok_or_else(|| invalid("sample frame size overflow"))?;
    if bytes_per_sample == 0 || block_align < minimum_align {
        return Err(invalid("invalid sample frame size"));
    }

    let decode = |sample: &[u8]| -> io::Result<f32> {
        let value = match (kind, bits) {
            (1, 8) => (sample[0] as f32 - 128.0) / 128.0,
            (1, 16) => i16::from_le_bytes([sample[0], sample[1]]) as f32 / 32_768.0,
            (1, 24) => {
                let value =
                    ((sample[0] as i32) | ((sample[1] as i32) << 8) | ((sample[2] as i32) << 16))
                        << 8;
                (value >> 8) as f32 / 8_388_608.0
            }
            (1, 32) => {
                i32::from_le_bytes([sample[0], sample[1], sample[2], sample[3]]) as f32
                    / 2_147_483_648.0
            }
            (3, 32) => f32::from_le_bytes([sample[0], sample[1], sample[2], sample[3]]),
            (3, 64) => f64::from_le_bytes([
                sample[0], sample[1], sample[2], sample[3], sample[4], sample[5], sample[6],
                sample[7],
            ]) as f32,
            _ => return Err(invalid("supports PCM8/16/24/32 or float32/64 WAV")),
        };
        if value.is_finite() {
            Ok(value.clamp(-1.0, 1.0))
        } else {
            Err(invalid("non-finite WAV sample"))
        }
    };

    let frame_count = data.len() / block_align;
    if frame_count == 0 {
        return Err(invalid("WAV contains no complete samples"));
    }
    let mut samples = Vec::with_capacity(frame_count);
    for frame in data.chunks_exact(block_align) {
        let mut mono = 0.0;
        for channel in 0..channels {
            let start = channel * bytes_per_sample;
            mono += decode(&frame[start..start + bytes_per_sample])?;
        }
        samples.push(mono / channels as f32);
    }
    Ok(Wav {
        sample_rate,
        samples,
    })
}

pub fn write(path: impl AsRef<Path>, wav: &Wav) -> io::Result<()> {
    if !(8_000..=384_000).contains(&wav.sample_rate) {
        return Err(invalid("unsupported sample rate"));
    }
    let data_length = wav
        .samples
        .len()
        .checked_mul(2)
        .and_then(|length| u32::try_from(length).ok())
        .ok_or_else(|| invalid("WAV is too large for RIFF"))?;
    let riff_length = 36_u32
        .checked_add(data_length)
        .ok_or_else(|| invalid("WAV is too large for RIFF"))?;
    let byte_rate = wav
        .sample_rate
        .checked_mul(2)
        .ok_or_else(|| invalid("WAV byte rate overflow"))?;

    let mut bytes = Vec::with_capacity(44 + data_length as usize);
    bytes.extend(b"RIFF");
    bytes.extend(riff_length.to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(wav.sample_rate.to_le_bytes());
    bytes.extend(byte_rate.to_le_bytes());
    bytes.extend(2_u16.to_le_bytes());
    bytes.extend(16_u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(data_length.to_le_bytes());
    for sample in &wav.samples {
        let sample = if sample.is_finite() { *sample } else { 0.0 };
        let encoded = (sample.clamp(-1.0, 1.0) * 32_767.0).round() as i16;
        bytes.extend(encoded.to_le_bytes());
    }
    fs::write(path, bytes)
}

fn le_u16(bytes: &[u8]) -> io::Result<u16> {
    let value: [u8; 2] = bytes
        .get(..2)
        .ok_or_else(|| invalid("truncated little-endian u16"))?
        .try_into()
        .map_err(|_| invalid("truncated little-endian u16"))?;
    Ok(u16::from_le_bytes(value))
}

fn le_u32(bytes: &[u8]) -> io::Result<u32> {
    let value: [u8; 4] = bytes
        .get(..4)
        .ok_or_else(|| invalid("truncated little-endian u32"))?
        .try_into()
        .map_err(|_| invalid("truncated little-endian u32"))?;
    Ok(u32::from_le_bytes(value))
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_round_trip() {
        let path =
            std::env::temp_dir().join(format!("ruststrument-wav-{}.wav", std::process::id()));
        let source = Wav {
            sample_rate: 48_000,
            samples: vec![0.0, 0.5, -0.5, 1.0],
        };
        write(&path, &source).unwrap();
        let loaded = read(&path).unwrap();
        let _ = fs::remove_file(path);
        assert_eq!(loaded.sample_rate, 48_000);
        assert_eq!(loaded.samples.len(), 4);
        assert!((loaded.samples[1] - 0.5).abs() < 0.001);
    }

    #[test]
    fn short_format_chunk_is_an_error_not_a_panic() {
        let path =
            std::env::temp_dir().join(format!("ruststrument-bad-wav-{}.wav", std::process::id()));
        let mut bytes = b"RIFF".to_vec();
        bytes.extend(38_u32.to_le_bytes());
        bytes.extend(b"WAVEfmt ");
        bytes.extend(2_u32.to_le_bytes());
        bytes.extend([1, 0]);
        bytes.extend(b"data");
        bytes.extend(16_u32.to_le_bytes());
        bytes.extend([0; 16]);
        fs::write(&path, bytes).unwrap();
        let result = read(&path);
        let _ = fs::remove_file(path);
        assert!(result.is_err());
    }
}
