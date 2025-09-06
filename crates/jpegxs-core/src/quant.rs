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

pub fn compute_quantization_parameters(target_bpp: f32) -> Result<Vec<u8>> {
    if target_bpp <= 0.0 || target_bpp > 32.0 {
        return Err(anyhow::anyhow!("Invalid target bits per pixel: {}", target_bpp));
    }
    
    // Simple quantization parameter computation
    // In practice, this would be much more sophisticated
    let base_qp = if target_bpp >= 8.0 {
        1  // High quality
    } else if target_bpp >= 4.0 {
        2  // Medium quality
    } else if target_bpp >= 2.0 {
        4  // Low quality
    } else {
        8  // Very low quality
    };
    
    // For now, return uniform quantization parameters
    // Real JPEG XS would have different QPs for different subbands
    Ok(vec![base_qp; 16]) // Assume 4-level DWT = 16 subbands
}
