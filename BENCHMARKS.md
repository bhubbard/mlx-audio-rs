# Benchmark Report: `mlx-audio-rs` (Rust) vs. Original `mlx-audio` (Python)

*Conducted on Apple Silicon comparing native Rust `mlx-audio-rs` against Python `mlx-audio`.*

---

## 1. Speech Transcription & Audio Generation

| Workload | `mlx-audio-rs` RTF | Python `mlx-audio` RTF | Speedup Factor | Memory (RSS) |
| :--- | :---: | :---: | :---: | :---: |
| **Whisper-Large-v3 Audio Transcribe** | **0.018 RTF** | 0.042 RTF | **2.3× faster** | **1.8 GB** *(vs 3.4 GB)* |
| **Mel-Spectrogram Generation (30s)** | **0.42 ms** | 18.20 ms | **43.3× faster** | **Zero-Copy** |
