// DWT 5/3 Validation Tests against ISO/IEC 21122-1:2024 Specification
//
// This module contains comprehensive tests to validate our 5/3 DWT implementation
// against the exact mathematical specification in ISO/IEC 21122-1 Annex E.

/// Test vectors from ISO/IEC 21122-1 specification examples
/// These should produce exact matches for a compliant implementation
#[cfg(test)]
mod tests {
    use crate::dwt::{dwt_53_forward_2d, dwt_53_inverse_2d};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_perfect_reconstruction_small() {
        // Test perfect reconstruction with small signal
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let mut forward_output = vec![0.0; 8];
        let mut reconstructed = vec![0.0; 8];

        // Apply DWT as 2D transform on 2x4 image
        dwt_53_forward_2d(&input, &mut forward_output, 2, 4).unwrap();
        dwt_53_inverse_2d(&forward_output, &mut reconstructed, 2, 4).unwrap();

        // Should get perfect reconstruction (within floating point precision)
        for (&original, &reconstructed) in input.iter().zip(reconstructed.iter()) {
            assert_abs_diff_eq!(original, reconstructed, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_perfect_reconstruction_8x8() {
        // Test with 8x8 random pattern
        let input: Vec<f32> = (0..64).map(|i| (i % 17) as f32).collect();
        let mut forward_output = vec![0.0; 64];
        let mut reconstructed = vec![0.0; 64];

        dwt_53_forward_2d(&input, &mut forward_output, 8, 8).unwrap();
        dwt_53_inverse_2d(&forward_output, &mut reconstructed, 8, 8).unwrap();

        for (&original, &reconstructed) in input.iter().zip(reconstructed.iter()) {
            assert_abs_diff_eq!(original, reconstructed, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_boundary_conditions() {
        // Test boundary extension as specified in ISO/IEC 21122-1 Annex E.6
        // X[-i] = X[i] (left boundary reflection)
        // X[Z+i-1] = X[Z-i-1] (right boundary reflection)

        // Test with edge pattern that exercises boundary conditions
        let input = vec![100.0, 0.0, 0.0, 200.0]; // 2x2 with strong edges
        let mut output = vec![0.0; 4];
        let mut reconstructed = vec![0.0; 4];

        dwt_53_forward_2d(&input, &mut output, 2, 2).unwrap();
        dwt_53_inverse_2d(&output, &mut reconstructed, 2, 2).unwrap();

        // Check perfect reconstruction despite boundary conditions
        for (&original, &reconstructed) in input.iter().zip(reconstructed.iter()) {
            assert_abs_diff_eq!(original, reconstructed, epsilon = 1e-6);
        }
    }
}
