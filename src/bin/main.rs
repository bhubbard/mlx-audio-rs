use clap::{Parser, Subcommand};
use mlx_audio_rs::{
    load_wav, save_wav, KokoroConfig, KokoroModel, Result, SileroVad, SnacCodec, WhisperConfig,
    WhisperModel,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mlx-audio")]
#[command(author = "Brandon Hubbard <brandon@brandonhubbard.com>")]
#[command(version = "0.0.1")]
#[command(about = "Native Apple Silicon Rust audio engine for TTS, STT, codecs, and VAD using MLX", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Synthesize speech from text using Kokoro-82M on Apple Silicon Metal
    Tts {
        /// Text prompt to synthesize
        #[arg(short = 't', long = "text", required = true)]
        text: String,

        /// Output WAV destination path
        #[arg(short = 'o', long = "output", default_value = "output.wav")]
        output: PathBuf,

        /// Speech rate multiplier
        #[arg(short = 's', long = "speed", default_value = "1.0")]
        speed: f32,
    },

    /// Transcribe speech audio to text using OpenAI Whisper on Apple Silicon
    Stt {
        /// Path to input audio WAV file (16kHz recommended)
        #[arg(short = 'a', long = "audio", required = true)]
        audio: PathBuf,

        /// Whisper model size: tiny, base, small, large-v3
        #[arg(short = 'm', long = "model", default_value = "base")]
        model: String,
    },

    /// Run Silero Voice Activity Detection (VAD) on audio
    Vad {
        /// Input WAV audio path
        #[arg(short = 'a', long = "audio", required = true)]
        audio: PathBuf,

        /// Speech probability detection threshold (0.0 to 1.0)
        #[arg(short = 't', long = "threshold", default_value = "0.5")]
        threshold: f32,
    },

    /// Neural audio codec encode/decode using SNAC
    Codec {
        /// Input audio WAV file
        #[arg(short = 'i', long = "input", required = true)]
        input: PathBuf,

        /// Output reconstructed WAV file
        #[arg(short = 'o', long = "output", default_value = "reconstructed.wav")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tts { text, output, speed } => {
            println!("Synthesizing TTS with Kokoro-82M: \"{}\"...", text);
            let config = KokoroConfig::default();
            let model = KokoroModel::new(config.clone());

            // Convert string chars to token IDs
            let tokens: Vec<i32> = text.chars().map(|c| (c as i32) % 178).collect();
            let audio_arr = model.synthesize(&tokens, speed)?;

            save_wav(&output, &audio_arr, config.sample_rate)?;
            println!("✓ Audio generated: {}", output.display());
        }

        Commands::Stt { audio, model } => {
            println!("Transcribing '{}' using Whisper ({}) on Apple Silicon...", audio.display(), model);
            let config = match model.as_str() {
                "tiny" => WhisperConfig::tiny(),
                "small" => WhisperConfig::small(),
                "large-v3" => WhisperConfig::large_v3(),
                _ => WhisperConfig::base(),
            };

            let whisper = WhisperModel::new(config);
            let (audio_arr, _) = load_wav(&audio)?;
            let mel = whisper.compute_log_mel(&audio_arr)?;
            let text = whisper.transcribe_features(&mel)?;
            println!("Transcription: \"{}\"", text);
        }

        Commands::Vad { audio, threshold } => {
            println!("Evaluating VAD on '{}' (threshold: {})...", audio.display(), threshold);
            let (audio_arr, _) = load_wav(&audio)?;
            let vad = SileroVad::new(threshold);
            let has_speech = vad.is_speech(&audio_arr)?;
            println!("Voice activity detected: {}", has_speech);
        }

        Commands::Codec { input, output } => {
            println!("Encoding and decoding '{}' through SNAC multi-scale codec...", input.display());
            let (audio_arr, sr) = load_wav(&input)?;
            let codec = SnacCodec::new();
            let tokens = codec.encode(&audio_arr)?;
            println!("Encoded into 3 token layers (total tokens: {})", tokens.iter().map(|t| t.len()).sum::<usize>());

            let decoded = codec.decode(&tokens)?;
            save_wav(&output, &decoded, sr)?;
            println!("✓ Reconstructed output saved to {}", output.display());
        }
    }

    Ok(())
}
