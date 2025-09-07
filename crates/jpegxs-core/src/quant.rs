use anyhow::Result;

pub fn quantize(coeffs: &[f32], qp: u8) -> Result<Vec<i32>> {
    if qp == 0 {
        return Err(anyhow::anyhow!("Quantization parameter cannot be zero"));
    }

    let scale = 1.0 / (qp as f32);
    let mut result = Vec::with_capacity(coeffs.len());

    for &coeff in coeffs {
        let quantized = (coeff * scale).round() as i32;
        result.push(quantized);
    }

    Ok(result)
}

pub fn dequantize(coeffs: &[i32], qp: u8) -> Result<Vec<f32>> {
    let scale = qp as f32;
    let mut result = Vec::with_capacity(coeffs.len());

    for &coeff in coeffs {
        let dequantized = coeff as f32 * scale;
        result.push(dequantized);
    }

    Ok(result)
}

pub fn compute_quantization_parameters(quality: f32) -> Result<Vec<u8>> {
    if quality <= 0.0 || quality > 1.0 {
        return Err(anyhow::anyhow!(
            "Invalid quality parameter: {} (must be 0.0 < quality <= 1.0)",
            quality
        ));
    }

    // Proper quality-to-quantization parameter mapping
    // Higher quality -> Lower QP -> Less compression loss
    // Lower quality -> Higher QP -> More compression gain
    let base_qp = if quality >= 0.95 {
        1   // Virtually lossless (minimal compression)
    } else if quality >= 0.9 {
        2   // Very high quality (moderate compression ~2:1)
    } else if quality >= 0.8 {
        3   // High quality (good compression ~3:1)
    } else if quality >= 0.7 {
        4   // Good quality (significant compression ~4:1)
    } else if quality >= 0.6 {
        6   // Medium-high quality (~5:1)
    } else if quality >= 0.5 {
        8   // Medium quality (strong compression ~6:1)
    } else if quality >= 0.4 {
        12  // Medium-low quality (~8:1)
    } else if quality >= 0.3 {
        16  // Low quality (high compression ~10:1)
    } else if quality >= 0.2 {
        24  // Very low quality (~12:1)
    } else {
        32  // Minimum quality (maximum compression ~15:1)
    };

    // For now, return uniform quantization parameters
    // Real JPEG XS would have different QPs for different subbands:
    // - Lower QPs for low-frequency (visually important) subbands
    // - Higher QPs for high-frequency (less visually important) subbands
    Ok(vec![base_qp; 16]) // Assume 4-level DWT = 16 subbands
}
