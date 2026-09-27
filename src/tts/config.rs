use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KokoroConfig {
    pub vocab_size: usize,
    pub hidden_dim: usize,
    pub num_layers: usize,
    pub style_dim: usize,
    pub sample_rate: u32,
    pub num_heads: usize,
}

impl Default for KokoroConfig {
    fn default() -> Self {
        Self {
            vocab_size: 178,
            hidden_dim: 512,
            num_layers: 6,
            style_dim: 128,
            sample_rate: 24000,
            num_heads: 8,
        }
    }
}
