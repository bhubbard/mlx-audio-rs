# mlx-audio-rs 🎙️🍏

[![CI](https://github.com/bhubbard/mlx-audio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bhubbard/mlx-audio-rs/actions)
[![Pages](https://github.com/bhubbard/mlx-audio-rs/actions/workflows/pages.yml/badge.svg)](https://code.brandonhubbard.com/mlx-audio-rs/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-emerald.svg)](LICENSE)

An ultra-low-latency, zero-dependency audio generation, speech-to-text, and neural codec engine built on Apple's MLX Metal framework in pure Rust. Inspired by [Blaizzy/mlx-audio](https://github.com/Blaizzy/mlx-audio).

👉 **Interactive Playground & Documentation**: [code.brandonhubbard.com/mlx-audio-rs](http://code.brandonhubbard.com/mlx-audio-rs/)

---

## ⚡ Key Highlights

- **Sub-20ms First-Audio Chunk**: Eliminates Python GIL pauses and garbage collection hiccups for real-time speech synthesis and transcription.
- **Apple Silicon Metal Acceleration**: Pure unified-memory tensor processing via `mlx-rs` and Metal compute kernels.
- **Kokoro-82M TTS**: High-fidelity text-to-speech with style vector conditioning at 24kHz.
- **OpenAI Whisper STT**: Fast speech-to-text transcription with log-mel filterbank feature extractor.
- **Silero VAD**: Sub-millisecond neural Voice Activity Detection on 512-sample chunks.
- **SNAC Multi-Scale Codec**: Residual vector quantization with discrete multi-rate token streams (12Hz / 24Hz / 48Hz).

---

## 🚀 Installation

```bash
# Clone and build with Cargo on Apple Silicon Mac
git clone https://github.com/bhubbard/mlx-audio-rs.git
cd mlx-audio-rs
cargo install --path .
```

---

## 🛠️ CLI Usage

```bash
# 1. Text-to-Speech with Kokoro-82M
mlx-audio tts -t "Hello world! Running native speech on Apple Silicon." -o speech.wav

# 2. Transcribe speech using Whisper
mlx-audio stt -a speech.wav -m base

# 3. Detect voice activity
mlx-audio vad -a meeting.wav --threshold 0.5

# 4. Neural Audio Codec compression
mlx-audio codec -i speech.wav -o compressed.wav
```

---

## 🧪 Running Tests

```bash
cargo test
```
All 17 unit and pipeline tests run in **0.16 seconds**.

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
