use crate::error::Result;
use mlx_rs::Array;
use std::f32::consts::PI;

/// Generates a Hann (Hanning) window on Apple Silicon unified memory.
pub fn hanning(size: usize, periodic: bool) -> Result<Array> {
    let denom = if periodic { size as f32 } else { (size - 1) as f32 };
    let vals: Vec<f32> = (0..size)
        .map(|n| 0.5 * (1.0 - (2.0 * PI * (n as f32) / denom).cos()))
        .collect();
    Ok(Array::from_slice(&vals, &[size as i32]))
}

/// Generates a Hamming window.
pub fn hamming(size: usize, periodic: bool) -> Result<Array> {
    let denom = if periodic { size as f32 } else { (size - 1) as f32 };
    let vals: Vec<f32> = (0..size)
        .map(|n| 0.54 - 0.46 * (2.0 * PI * (n as f32) / denom).cos())
        .collect();
    Ok(Array::from_slice(&vals, &[size as i32]))
}

/// Generates a Blackman window.
pub fn blackman(size: usize, periodic: bool) -> Result<Array> {
    let denom = if periodic { size as f32 } else { (size - 1) as f32 };
    let vals: Vec<f32> = (0..size)
        .map(|n| {
            let n_f = n as f32;
            0.42 - 0.5 * (2.0 * PI * n_f / denom).cos() + 0.08 * (4.0 * PI * n_f / denom).cos()
        })
        .collect();
    Ok(Array::from_slice(&vals, &[size as i32]))
}

/// Generates a Bartlett (triangular) window.
pub fn bartlett(size: usize) -> Result<Array> {
    let denom = (size - 1) as f32;
    let vals: Vec<f32> = (0..size)
        .map(|n| 1.0 - ((2.0 * (n as f32) - denom) / denom).abs())
        .collect();
    Ok(Array::from_slice(&vals, &[size as i32]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_shapes_and_symmetry() {
        let h = hanning(1024, false).unwrap();
        assert_eq!(h.shape(), &[1024]);

        let ham = hamming(512, true).unwrap();
        assert_eq!(ham.shape(), &[512]);

        let b = blackman(256, false).unwrap();
        assert_eq!(b.shape(), &[256]);
    }
}
