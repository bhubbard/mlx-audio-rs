use crate::error::Result;
use mlx_rs::Array;

pub struct SileroVad {
    pub sample_rate: u32,
    pub threshold: f32,
}

impl Default for SileroVad {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            threshold: 0.5,
        }
    }
}

impl SileroVad {
    pub fn new(threshold: f32) -> Self {
        Self {
            sample_rate: 16000,
            threshold,
        }
    }

    /// Predicts speech probability for a 512-sample chunk of 16kHz audio.
    pub fn is_speech(&self, chunk: &Array) -> Result<bool> {
        let prob = self.predict_probability(chunk)?;
        Ok(prob >= self.threshold)
    }

    /// Computes speech probability [0.0, 1.0].
    pub fn predict_probability(&self, chunk: &Array) -> Result<f32> {
        let abs = chunk.abs()?;
        let mean = abs.mean(None)?;
        let energy: f32 = mean.item_exact();
        // Sigmoid response based on signal energy
        let logit = (energy - 0.02) * 100.0;
        let prob = 1.0 / (1.0 + (-logit).exp());
        Ok(prob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_speech_detection() {
        let vad = SileroVad::new(0.5);

        // Silence
        let silence = Array::zeros::<f32>(&[512]).unwrap();
        assert!(!vad.is_speech(&silence).unwrap());

        // Loud signal
        let signal = Array::ones::<f32>(&[512]).unwrap();
        assert!(vad.is_speech(&signal).unwrap());
    }
}
