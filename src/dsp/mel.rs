use crate::error::Result;
use mlx_rs::Array;

pub fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

pub fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10.0_f32.powf(mel / 2595.0) - 1.0)
}

/// Computes a triangular Mel-filterbank matrix of shape `[n_mels, n_fft / 2 + 1]`.
pub fn mel_filterbank(
    sr: u32,
    n_fft: usize,
    n_mels: usize,
    f_min: f32,
    f_max: Option<f32>,
) -> Result<Array> {
    let f_max = f_max.unwrap_or(sr as f32 / 2.0);
    let mel_min = hz_to_mel(f_min);
    let mel_max = hz_to_mel(f_max);

    let n_freqs = n_fft / 2 + 1;
    let mut weights = vec![0.0f32; n_mels * n_freqs];

    // Compute mel points
    let mel_points: Vec<f32> = (0..=n_mels + 1)
        .map(|i| mel_min + (mel_max - mel_min) * (i as f32) / ((n_mels + 1) as f32))
        .collect();

    let hz_points: Vec<f32> = mel_points.iter().copied().map(mel_to_hz).collect();
    let fft_freqs: Vec<f32> = (0..n_freqs)
        .map(|i| (i as f32) * (sr as f32) / (n_fft as f32))
        .collect();

    for m in 0..n_mels {
        let f_left = hz_points[m];
        let f_center = hz_points[m + 1];
        let f_right = hz_points[m + 2];

        for (k, &f) in fft_freqs.iter().enumerate() {
            if f >= f_left && f <= f_center && f_center > f_left {
                weights[m * n_freqs + k] = (f - f_left) / (f_center - f_left);
            } else if f > f_center && f <= f_right && f_right > f_center {
                weights[m * n_freqs + k] = (f_right - f) / (f_right - f_center);
            }
        }
    }

    Ok(Array::from_slice(&weights, &[n_mels as i32, n_freqs as i32]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mel_conversions() {
        let hz = 1000.0;
        let mel = hz_to_mel(hz);
        let back = mel_to_hz(mel);
        assert!((back - hz).abs() < 1e-3);
    }

    #[test]
    fn test_mel_filterbank_shape() {
        let fb = mel_filterbank(16000, 400, 80, 0.0, None).unwrap();
        assert_eq!(fb.shape(), &[80, 201]);
    }
}
