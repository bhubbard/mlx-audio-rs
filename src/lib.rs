pub mod audio_io;
pub mod codec;
pub mod dsp;
pub mod error;
pub mod stt;
pub mod tts;
pub mod vad;

pub use audio_io::{load_wav, save_wav};
pub use codec::SnacCodec;
pub use dsp::{hz_to_mel, mel_filterbank, mel_to_hz, normalize_peak, rms_energy};
pub use error::{AudioError, Result};
pub use stt::{WhisperConfig, WhisperModel};
pub use tts::{KokoroConfig, KokoroModel};
pub use vad::SileroVad;
