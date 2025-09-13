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

// Quality-to-QP mapping table for cleaner maintenance and testing
// Each entry: (min_quality, qp, description)
const QUALITY_TO_QP_TABLE: &[(f32, u8, &str)] = &[
    (0.9, 1, "High quality: virtually lossless for 0.9+ quality"),
    (0.8, 2, "Very high quality (moderate compression ~2:1)"),
    (0.7, 3, "High quality (good compression ~3:1)"),
    (0.6, 4, "Good quality (significant compression ~4:1)"),
    (0.5, 6, "Medium-high quality (~5:1)"),
    (0.4, 8, "Medium quality (strong compression ~6:1)"),
    (0.3, 12, "Medium-low quality (~8:1)"),
    (0.2, 16, "Low quality (high compression ~10:1)"),
    (0.1, 24, "Very low quality (~12:1)"),
    (0.0, 32, "Minimum quality (maximum compression ~15:1)"),
];

pub fn compute_quantization_parameters(quality: f32) -> Result<Vec<u8>> {
    if quality <= 0.0 || quality > 1.0 {
        return Err(anyhow::anyhow!(
            "Invalid quality parameter: {} (must be greater than 0.0 and at most 1.0)",
            quality
        ));
    }

    // Find the appropriate QP using the lookup table
    // Higher quality -> Lower QP -> Less compression loss
    // Lower quality -> Higher QP -> More compression gain
    let base_qp = QUALITY_TO_QP_TABLE
        .iter()
        .find(|(min_quality, _, _)| quality >= *min_quality)
        .map(|(_, qp, _)| *qp)
        .unwrap_or(32); // Fallback to maximum compression

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

// 8-bit coefficient quantization functions

pub fn quantize_8bit(coeffs: &[i8], qp: u8) -> Result<Vec<i8>> {
    if qp == 0 {
        return Err(anyhow::anyhow!("Quantization parameter cannot be zero"));
    }

    let mut result = Vec::with_capacity(coeffs.len());

    for &coeff in coeffs {
        // For 8-bit coefficients, quantization is simpler integer division
        let quantized = if qp == 1 {
            coeff // No quantization for QP=1
        } else {
            coeff / (qp as i8)
        };
        result.push(quantized);
    }

    Ok(result)
}

pub fn dequantize_8bit(coeffs: &[i8], qp: u8) -> Result<Vec<i8>> {
    let mut result = Vec::with_capacity(coeffs.len());

    for &coeff in coeffs {
        // Dequantization multiplies by QP
        let dequantized = if qp == 1 {
            coeff // No dequantization for QP=1
        } else {
            coeff.saturating_mul(qp as i8)
        };
        result.push(dequantized);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_to_qp_mapping() {
        // Test specific quality thresholds
        assert_eq!(compute_quantization_parameters(0.95).unwrap()[0], 1);
        assert_eq!(compute_quantization_parameters(0.9).unwrap()[0], 1);
        assert_eq!(compute_quantization_parameters(0.89).unwrap()[0], 2);
        assert_eq!(compute_quantization_parameters(0.8).unwrap()[0], 2);
        assert_eq!(compute_quantization_parameters(0.79).unwrap()[0], 3);
        assert_eq!(compute_quantization_parameters(0.5).unwrap()[0], 6);
        assert_eq!(compute_quantization_parameters(0.1).unwrap()[0], 24);
        assert_eq!(compute_quantization_parameters(0.05).unwrap()[0], 32);
    }

    #[test]
    fn test_invalid_quality_parameters() {
        assert!(compute_quantization_parameters(0.0).is_err());
        assert!(compute_quantization_parameters(-0.1).is_err());
        assert!(compute_quantization_parameters(1.1).is_err());
    }

    #[test]
    fn test_qp_lookup_table_completeness() {
        // Verify the lookup table covers expected quality ranges
        for &(min_quality, qp, _) in QUALITY_TO_QP_TABLE {
            if min_quality > 0.0 {
                let result = compute_quantization_parameters(min_quality).unwrap();
                assert_eq!(
                    result[0], qp,
                    "Quality {} should map to QP {}",
                    min_quality, qp
                );
            }
        }
    }
}
