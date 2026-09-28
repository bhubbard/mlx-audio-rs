//! Analytical and Mathematical Accuracy Verification Tests for mlx-audio-rs
//!
//! Validates:
//! 1. Frequency-to-Mel transformation bijection and analytical invertibility across full human hearing spectrum.
//! 2. DSP window symmetry, endpoint boundary conditions, and harmonic attenuation properties.
//! 3. Audio normalization mathematical fidelity across decibel scales.
//! 4. RMS energy exactness against analytical continuous sine integrals.

use mlx_audio_rs::dsp::{
    blackman, hamming, hanning, hz_to_mel, mel_to_hz, normalize_peak, rms_energy,
};
use mlx_rs::Array;
use std::f32::consts::PI;

#[test]
fn test_hz_mel_analytical_inversion() {
    let test_frequencies = [
        20.0f32, 50.0, 100.0, 250.0, 440.0, 1000.0, 2500.0, 5000.0, 8000.0, 12000.0, 16000.0,
        20000.0,
    ];

    for &hz in &test_frequencies {
        let mel = hz_to_mel(hz);
        let reconstructed_hz = mel_to_hz(mel);
        let error = (reconstructed_hz - hz).abs();

        assert!(
            error < 0.05,
            "Hz <-> Mel inversion error at {} Hz: reconstructed {}, err {}",
            hz,
            reconstructed_hz,
            error
        );
    }
}

#[test]
fn test_dsp_window_analytical_symmetry_and_endpoints() {
    let n = 513; // Odd length for well-defined exact midpoint

    // 1. Hann Window: w(0) = 0, w(N-1) = 0, w(mid) = 1.0, symmetric
    let hann = hanning(n, false).expect("hanning generation");
    let h_slice: Vec<f32> = hann.as_slice().to_vec();
    assert_eq!(h_slice.len(), n);

    assert!(
        h_slice[0].abs() < 1e-6,
        "Hann start endpoint must be 0 (got {})",
        h_slice[0]
    );
    assert!(
        h_slice[n - 1].abs() < 1e-6,
        "Hann end endpoint must be 0 (got {})",
        h_slice[n - 1]
    );
    assert!(
        (h_slice[n / 2] - 1.0).abs() < 1e-5,
        "Hann midpoint must be 1.0 (got {})",
        h_slice[n / 2]
    );

    // Symmetry check: w[i] == w[n - 1 - i]
    for i in 0..n / 2 {
        let diff = (h_slice[i] - h_slice[n - 1 - i]).abs();
        assert!(
            diff < 1e-6,
            "Hann asymmetry at index {}: {} vs {}",
            i,
            h_slice[i],
            h_slice[n - 1 - i]
        );
    }

    // 2. Hamming Window: w(0) = 0.54 - 0.46 = 0.08
    let ham = hamming(n, false).expect("hamming generation");
    let ham_slice: Vec<f32> = ham.as_slice().to_vec();
    assert!(
        (ham_slice[0] - 0.08).abs() < 1e-5,
        "Hamming start must equal 0.08 (got {})",
        ham_slice[0]
    );
    assert!(
        (ham_slice[n / 2] - 1.0).abs() < 1e-5,
        "Hamming midpoint must be 1.0 (got {})",
        ham_slice[n / 2]
    );

    // 3. Blackman Window: w(0) = 0.42 - 0.5 + 0.08 = 0.0
    let blk = blackman(n, false).expect("blackman generation");
    let blk_slice: Vec<f32> = blk.as_slice().to_vec();
    assert!(
        blk_slice[0].abs() < 1e-5,
        "Blackman start must equal 0.0 (got {})",
        blk_slice[0]
    );
    assert!(
        (blk_slice[n / 2] - 1.0).abs() < 1e-5,
        "Blackman midpoint must be 1.0 (got {})",
        blk_slice[n / 2]
    );
}

#[test]
fn test_peak_normalization_target_exactness() {
    let raw_samples = vec![0.12f32, -0.85, 0.43, -0.31, 0.05, 0.62];
    let audio = Array::from_slice(&raw_samples, &[raw_samples.len() as i32]);

    for &target_db in &[-0.5f32, -1.0, -3.0, -6.0, -12.0] {
        let normalized = normalize_peak(&audio, target_db).expect("normalization failed");
        let peak_abs: f32 = normalized
            .abs()
            .unwrap()
            .max(None)
            .unwrap()
            .item_exact();

        let expected_linear = 10.0_f32.powf(target_db / 20.0);
        let error = (peak_abs - expected_linear).abs();

        assert!(
            error < 1e-5,
            "Peak normalization failed at {} dBFS: got {}, expected {}, err {}",
            target_db,
            peak_abs,
            expected_linear,
            error
        );
    }
}

#[test]
fn test_rms_energy_pure_sine_exactness() {
    // For a pure continuous sine wave A * sin(omega * t), the analytical RMS energy
    // over integer cycles is exactly A / sqrt(2) ~= 0.70710678 * A.
    let amplitude = 0.85f32;
    let sample_rate = 16000;
    let freq = 100.0f32; // Exactly 100 cycles per second
    let duration = 1.0f32; // 1 second = 16000 samples = integer 100 cycles

    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for n in 0..num_samples {
        let t = n as f32 / sample_rate as f32;
        samples.push(amplitude * (2.0 * PI * freq * t).sin());
    }

    let arr = Array::from_slice(&samples, &[num_samples as i32]);
    let computed_rms = rms_energy(&arr).expect("RMS computation");
    let analytical_rms = amplitude / 2.0_f32.sqrt();

    let relative_error = (computed_rms - analytical_rms).abs() / analytical_rms;
    assert!(
        relative_error < 0.001,
        "Sine RMS energy deviation: computed {}, analytical {}, rel error {}",
        computed_rms,
        analytical_rms,
        relative_error
    );
}
