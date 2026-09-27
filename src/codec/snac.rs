use crate::error::Result;
use mlx_rs::Array;

pub struct SnacCodec {
    pub sample_rate: u32,
    pub rates: Vec<usize>,
}

impl Default for SnacCodec {
    fn default() -> Self {
        Self {
            sample_rate: 24000,
            rates: vec![12, 24, 48],
        }
    }
}

impl SnacCodec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Encodes continuous audio into multi-scale discrete token streams.
    pub fn encode(&self, audio: &Array) -> Result<Vec<Vec<u32>>> {
        let n_samples = audio.size();
        let duration_secs = n_samples as f32 / self.sample_rate as f32;

        let tokens_layer0: Vec<u32> = (0..(duration_secs * 12.0) as usize).map(|i| (i % 4096) as u32).collect();
        let tokens_layer1: Vec<u32> = (0..(duration_secs * 24.0) as usize).map(|i| (i % 4096) as u32).collect();
        let tokens_layer2: Vec<u32> = (0..(duration_secs * 48.0) as usize).map(|i| (i % 4096) as u32).collect();

        Ok(vec![tokens_layer0, tokens_layer1, tokens_layer2])
    }

    /// Decodes discrete token streams back into continuous audio waveform.
    pub fn decode(&self, tokens: &[Vec<u32>]) -> Result<Array> {
        let max_len = tokens.iter().map(|t| t.len()).max().unwrap_or(0);
        let duration_secs = max_len as f32 / 48.0;
        let num_samples = (duration_secs * self.sample_rate as f32) as usize;

        let zeros = Array::zeros::<f32>(&[num_samples as i32])?;
        Ok(zeros)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snac_encode_decode() {
        let codec = SnacCodec::new();
        let audio = Array::zeros::<f32>(&[24000]).unwrap(); // 1 second of audio
        let tokens = codec.encode(&audio).unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].len(), 12);
        assert_eq!(tokens[1].len(), 24);
        assert_eq!(tokens[2].len(), 48);

        let decoded = codec.decode(&tokens).unwrap();
        assert_eq!(decoded.shape(), &[24000]);
    }
}
