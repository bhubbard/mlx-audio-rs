use crate::error::Result;
use mlx_rs::Array;

/// Normalizes audio waveform to a specified peak dBFS level (e.g. -1.0 dBFS).
pub fn normalize_peak(audio: &Array, target_db: f32) -> Result<Array> {
    let max_abs = audio.abs()?.max(None)?;
    let max_val: f32 = max_abs.item_exact();

    if max_val <= 1e-9 {
        return Ok(audio.clone());
    }

    let target_linear = 10.0_f32.powf(target_db / 20.0);
    let scale = target_linear / max_val;
    Ok(audio.multiply(&Array::from_f32(scale))?)
}

/// Computes the Root-Mean-Square (RMS) energy of an audio waveform.
pub fn rms_energy(audio: &Array) -> Result<f32> {
    let squared = audio.square()?;
    let mean = squared.mean(None)?;
    let mean_val: f32 = mean.item_exact();
    Ok(mean_val.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_peak() {
        let samples = vec![0.5f32, -0.2, 0.1, -0.4];
        let arr = Array::from_slice(&samples, &[4]);
        let norm = normalize_peak(&arr, 0.0).unwrap(); // 0 dBFS -> peak = 1.0
        let max_abs = norm.abs().unwrap().max(None).unwrap();
        let max_val: f32 = max_abs.item_exact();
        assert!((max_val - 1.0).abs() < 1e-5);
    }
}
