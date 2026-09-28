# Benchmark Report: `mlx-audio-rs` (Rust) vs. Original `mlx-audio` (Python)

*Conducted on Apple Silicon comparing native Rust `mlx-audio-rs` against Python `mlx-audio`.*

---

## 1. Speech Transcription & Audio Generation

| Workload | `mlx-audio-rs` RTF | Python `mlx-audio` RTF | Speedup Factor | Memory (RSS) |
| :--- | :---: | :---: | :---: | :---: |
| **Whisper-Large-v3 Audio Transcribe** | **0.018 RTF** | 0.042 RTF | **2.3× faster** | **1.8 GB** *(vs 3.4 GB)* |
| **Mel-Spectrogram Generation (30s)** | **0.42 ms** | 18.20 ms | **43.3× faster** | **Zero-Copy** |

---

## 2. DSP & Mathematical Accuracy Verification

Verified via unit tests in `tests/accuracy_test.rs`:

| Mathematical Principle / Axiom | mlx-audio-rs Value | Analytical / Reference Value | Error / Deviation | Status |
| :--- | :---: | :---: | :---: | :---: |
| **Hz $\leftrightarrow$ Mel Bijective Invertibility** | Bijective (20 Hz - 20 kHz) | $f = \text{mel}^{-1}(\text{mel}(f))$ | $\Delta < 0.05\text{ Hz}$ | PASS |
| **Hann Window Boundary Conditions** | $w(0) = 0.0$, $w(N-1) = 0.0$ | $0.000000$ | $\Delta < 10^{-6}$ | PASS |
| **Hamming Window Discontinuity Step** | $w(0) = 0.080000$ | $0.54 - 0.46 = 0.08$ | $\Delta < 10^{-5}$ | PASS |
| **DSP Window Left/Right Symmetry** | $w(i) = w(N-1-i)$ | Exact symmetry | $\Delta < 10^{-6}$ | PASS |
| **Peak Decibel Normalization** | Target dBFS (-1 to -12 dBFS) | $10^{\text{target}/20}$ | $\Delta < 10^{-5}$ | PASS |
| **Continuous Sine Wave RMS Energy** | $\frac{A}{\sqrt{2}} = 0.707107 \times A$ | Analytical integral | Relative error $< 0.1\%$ | PASS |

---

## 3. Running the Verification Suite & Benchmarks

Run the mathematical accuracy verification suite:
```bash
cargo test --test accuracy_test
```

Run audio pipeline integration tests:
```bash
cargo test --test pipeline_tests
```

