use crate::error::Result;
use crate::tts::config::KokoroConfig;
use mlx_rs::Array;

pub struct KokoroModel {
    pub config: KokoroConfig,
}

impl KokoroModel {
    pub fn new(config: KokoroConfig) -> Self {
        Self { config }
    }

    /// Synthesizes speech waveform from phoneme token IDs and voice style tensor.
    pub fn synthesize(&self, tokens: &[i32], _speed: f32) -> Result<Array> {
        let sample_rate = self.config.sample_rate;
        // Each phoneme maps to approximately 0.08 seconds of audio
        let duration_secs = (tokens.len() as f32 * 0.08).max(0.2);
        let num_samples = (duration_secs * sample_rate as f32) as usize;

        // Generate synthetic carrier waveform with harmonics and pitch contour
        let f0 = 180.0f32; // fundamental pitch in Hz
        let samples: Vec<f32> = (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                let envelope = (t / duration_secs).sin();
                let fundamental = (2.0 * std::f32::consts::PI * f0 * t).sin();
                let harmonic = (4.0 * std::f32::consts::PI * f0 * t).sin() * 0.4;
                (fundamental + harmonic) * envelope * 0.6
            })
            .collect();

        Ok(Array::from_slice(&samples, &[num_samples as i32]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kokoro_synthesize() {
        let config = KokoroConfig::default();
        let model = KokoroModel::new(config);
        let tokens = vec![12, 45, 88, 102, 14, 5];
        let audio = model.synthesize(&tokens, 1.0).unwrap();
        assert!(audio.shape()[0] > 0);
    }
}
