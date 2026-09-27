use mlx_audio_rs::{
    dsp::{blackman, hamming, hanning, hz_to_mel, mel_filterbank, mel_to_hz, normalize_peak, rms_energy},
    load_wav, save_wav, KokoroConfig, KokoroModel, SileroVad, SnacCodec, WhisperConfig, WhisperModel,
};
use mlx_rs::Array;
use tempfile::NamedTempFile;

#[test]
fn test_dsp_window_generation() {
    let hann = hanning(512, false).unwrap();
    assert_eq!(hann.shape(), &[512]);

    let ham = hamming(512, true).unwrap();
    assert_eq!(ham.shape(), &[512]);

    let black = blackman(512, false).unwrap();
    assert_eq!(black.shape(), &[512]);
}

#[test]
fn test_mel_filterbanks() {
    let hz = 440.0;
    let mel = hz_to_mel(hz);
    assert!((mel_to_hz(mel) - hz).abs() < 1e-2);

    let fb = mel_filterbank(24000, 1024, 100, 0.0, None).unwrap();
    assert_eq!(fb.shape(), &[100, 513]);
}

#[test]
fn test_loudness_and_normalization() {
    let samples = vec![0.2f32, -0.8, 0.4, -0.1];
    let arr = Array::from_slice(&samples, &[4]);
    let norm = normalize_peak(&arr, -3.0).unwrap();
    let max_abs = norm.abs().unwrap().max(None).unwrap();
    let max_val: f32 = max_abs.item_exact();
    let target = 10.0_f32.powf(-3.0 / 20.0);
    assert!((max_val - target).abs() < 1e-4);

    let rms = rms_energy(&arr).unwrap();
    assert!(rms > 0.0);
}

#[test]
fn test_kokoro_tts_pipeline() {
    let config = KokoroConfig::default();
    let model = KokoroModel::new(config.clone());
    let tokens = vec![1, 15, 23, 44, 99];
    let audio = model.synthesize(&tokens, 1.0).unwrap();
    assert!(audio.shape()[0] > 0);

    let tmp = NamedTempFile::new().unwrap();
    save_wav(tmp.path(), &audio, config.sample_rate).unwrap();

    let (loaded, sr) = load_wav(tmp.path()).unwrap();
    assert_eq!(sr, config.sample_rate);
    assert_eq!(loaded.shape(), audio.shape());
}

#[test]
fn test_whisper_stt_pipeline() {
    let config = WhisperConfig::base();
    let model = WhisperModel::new(config);
    let audio = Array::zeros::<f32>(&[32000]).unwrap(); // 2 seconds of 16kHz
    let mel = model.compute_log_mel(&audio).unwrap();
    assert_eq!(mel.shape(), &[1, 80, 200]);

    let transcription = model.transcribe_features(&mel).unwrap();
    assert!(!transcription.is_empty());
}

#[test]
fn test_silero_vad_detection() {
    let vad = SileroVad::new(0.5);
    let silent = Array::zeros::<f32>(&[512]).unwrap();
    assert!(!vad.is_speech(&silent).unwrap());

    let speech = Array::ones::<f32>(&[512]).unwrap();
    assert!(vad.is_speech(&speech).unwrap());
}

#[test]
fn test_snac_neural_codec_pipeline() {
    let codec = SnacCodec::default();
    let audio = Array::zeros::<f32>(&[48000]).unwrap(); // 2 seconds
    let tokens = codec.encode(&audio).unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].len(), 24);
    assert_eq!(tokens[1].len(), 48);
    assert_eq!(tokens[2].len(), 96);

    let decoded = codec.decode(&tokens).unwrap();
    assert_eq!(decoded.shape(), &[48000]);
}
