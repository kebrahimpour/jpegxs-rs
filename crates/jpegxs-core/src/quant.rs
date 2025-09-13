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
            "Invalid quality parameter: {} (must be greater than 0.0 and at most 1.0)",
            quality
        ));
    }

    // Proper quality-to-quantization parameter mapping
    // Higher quality -> Lower QP -> Less compression loss
    // Lower quality -> Higher QP -> More compression gain
    let base_qp = if quality >= 0.9 {
        1 // High quality: virtually lossless for 0.9+ quality
    } else if quality >= 0.8 {
        2 // Very high quality (moderate compression ~2:1)
    } else if quality >= 0.7 {
        3 // High quality (good compression ~3:1)
    } else if quality >= 0.6 {
        4 // Good quality (significant compression ~4:1)
    } else if quality >= 0.5 {
        6 // Medium-high quality (~5:1)
    } else if quality >= 0.4 {
        8 // Medium quality (strong compression ~6:1)
    } else if quality >= 0.3 {
        12 // Medium-low quality (~8:1)
    } else if quality >= 0.2 {
        16 // Low quality (high compression ~10:1)
    } else if quality >= 0.1 {
        24 // Very low quality (~12:1)
    } else {
        32 // Minimum quality (maximum compression ~15:1)
    };

    // Return quantization parameters for DWT subbands
    // Real JPEG XS would have different QPs for different subbands:
    // - Lower QPs for low-frequency (visually important) subbands
    // - Higher QPs for high-frequency (less visually important) subbands

    // TODO: Make DWT levels configurable via encoder config
    // For now, assume 4-level DWT which produces 13 subbands (4*3 + 1 = 13)
    const DWT_LEVELS: usize = 4;
    const NUM_SUBBANDS: usize = 3 * DWT_LEVELS + 1; // 3 detail bands per level + 1 final LL subband

    Ok(vec![base_qp; NUM_SUBBANDS])
}
