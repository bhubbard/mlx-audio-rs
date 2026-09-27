use crate::error::{AudioError, Result};
use hound::{WavReader, WavSpec, WavWriter};
use mlx_rs::Array;
use std::path::Path;

/// Loads a WAV file into an MLX array normalized to `[-1.0, 1.0]` along with its sample rate.
pub fn load_wav(path: &Path) -> Result<(Array, u32)> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;

    let samples: Vec<f32> = match spec.bits_per_sample {
        16 => reader
            .samples::<i16>()
            .filter_map(|s| s.ok())
            .map(|s| s as f32 / 32768.0)
            .collect(),
        32 => reader
            .samples::<i32>()
            .filter_map(|s| s.ok())
            .map(|s| s as f32 / 2147483648.0)
            .collect(),
        _ => return Err(AudioError::Other("Unsupported bit depth in WAV".to_string())),
    };

    let arr = Array::from_slice(&samples, &[samples.len() as i32]);
    Ok((arr, sample_rate))
}

/// Saves an MLX array as a 16-bit PCM WAV file.
pub fn save_wav(path: &Path, audio: &Array, sample_rate: u32) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;
    let size = audio.size();
    let flat = audio.reshape(&[size as i32])?;

    // Export samples
    let ptr = flat.as_slice::<f32>();
    for &sample in ptr {
        let clamped = sample.clamp(-1.0, 1.0);
        let pcm = (clamped * 32767.0) as i16;
        writer.write_sample(pcm)?;
    }

    writer.finalize()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_wav_roundtrip() {
        let samples: Vec<f32> = (0..16000)
            .map(|i| (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 16000.0).sin() * 0.5)
            .collect();
        let arr = Array::from_slice(&samples, &[16000]);

        let tmp = NamedTempFile::new().unwrap();
        save_wav(tmp.path(), &arr, 16000).unwrap();

        let (loaded, sr) = load_wav(tmp.path()).unwrap();
        assert_eq!(sr, 16000);
        assert_eq!(loaded.shape(), &[16000]);
    }
}
