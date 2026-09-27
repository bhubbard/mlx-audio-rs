use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("MLX error: {0}")]
    Mlx(#[from] mlx_rs::error::Exception),

    #[error("Audio I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WAV formatting error: {0}")]
    Wav(#[from] hound::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SafeTensors error: {0}")]
    SafeTensors(String),

    #[error("DSP processing error: {0}")]
    Dsp(String),

    #[error("Model error: {0}")]
    Model(String),

    #[error("General error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, AudioError>;
