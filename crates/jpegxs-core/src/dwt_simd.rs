// SIMD-optimized 5/3 DWT implementation for x86_64 and ARM architectures
// Uses portable SIMD operations via the wide crate for cross-platform compatibility

use anyhow::Result;
use wide::f32x4;

/// SIMD-optimized 5/3 DWT forward transform for 2D data
pub fn dwt_53_forward_2d_simd(
    input: &[f32],
    output: &mut [f32],
    width: u32,
    height: u32,
) -> Result<()> {
    if input.len() != (width * height) as usize || output.len() != input.len() {
        return Err(anyhow::anyhow!("Invalid buffer sizes"));
    }

    // Copy input to output for in-place processing
    output.copy_from_slice(input);

    // Apply 1D DWT along rows first (horizontal pass)
    for y in 0..height {
        let row_start = (y * width) as usize;
        let row_end = row_start + width as usize;

        // Use SIMD version for rows with sufficient width
        if width >= 8 {
            dwt_53_forward_1d_simd(&mut output[row_start..row_end]);
        } else {
            super::dwt::dwt_53_forward_1d(&mut output[row_start..row_end]);
        }
    }

    // Apply 1D DWT along columns (vertical pass)
    let mut col_buffer = vec![0.0f32; height as usize];
    for x in 0..width {
        // Extract column
        for y in 0..height {
            col_buffer[y as usize] = output[(y * width + x) as usize];
        }

        // Transform column (use scalar version for columns as they're accessed non-contiguously)
        super::dwt::dwt_53_forward_1d(&mut col_buffer);

        // Put column back
        for y in 0..height {
            output[(y * width + x) as usize] = col_buffer[y as usize];
        }
    }

    Ok(())
}

/// SIMD-optimized 5/3 DWT inverse transform for 2D data
pub fn dwt_53_inverse_2d_simd(
    input: &[f32],
    output: &mut [f32],
    width: u32,
    height: u32,
) -> Result<()> {
    if input.len() != (width * height) as usize || output.len() != input.len() {
        return Err(anyhow::anyhow!("Invalid buffer sizes"));
    }

    // Copy input to output for in-place processing
    output.copy_from_slice(input);

    // Apply 1D inverse DWT along columns first (vertical pass)
    let mut col_buffer = vec![0.0f32; height as usize];
    for x in 0..width {
        // Extract column
        for y in 0..height {
            col_buffer[y as usize] = output[(y * width + x) as usize];
        }

        // Inverse transform column
        super::dwt::dwt_53_inverse_1d(&mut col_buffer);

        // Put column back
        for y in 0..height {
            output[(y * width + x) as usize] = col_buffer[y as usize];
        }
    }

    // Apply 1D inverse DWT along rows (horizontal pass)
    for y in 0..height {
        let row_start = (y * width) as usize;
        let row_end = row_start + width as usize;

        // Use SIMD version for rows with sufficient width
        if width >= 8 {
            dwt_53_inverse_1d_simd(&mut output[row_start..row_end]);
        } else {
            super::dwt::dwt_53_inverse_1d(&mut output[row_start..row_end]);
        }
    }

    Ok(())
}

/// SIMD-optimized 1D forward 5/3 DWT
fn dwt_53_forward_1d_simd(data: &mut [f32]) {
    let len = data.len();
    if len < 8 {
        // Fall back to scalar version for small arrays
        super::dwt::dwt_53_forward_1d(data);
        return;
    }

    // Temporary buffer to avoid overwriting data during transform
    let mut temp = data.to_vec();

    // Step 1: Predict step (High-pass coefficients)
    // Process odd indices using SIMD (4 elements at a time)
    let mut i = 1;
    while i + 7 < len {
        // Load 4 odd samples
        let center = f32x4::from([temp[i], temp[i + 2], temp[i + 4], temp[i + 6]]);

        // Load neighboring even samples
        // Note: temp[i - 1] is safe since i starts at 1, so minimum access is temp[0]
        let left = f32x4::from([temp[i - 1], temp[i + 1], temp[i + 3], temp[i + 5]]);
        let right = f32x4::from([temp[i + 1], temp[i + 3], temp[i + 5], temp[i + 7]]);

        // Compute prediction: center - (left + right) / 2
        let avg = (left + right) * f32x4::splat(0.5);
        let result = center - avg;

        // Store results
        data[i] = result.as_array_ref()[0];
        data[i + 2] = result.as_array_ref()[1];
        data[i + 4] = result.as_array_ref()[2];
        data[i + 6] = result.as_array_ref()[3];

        i += 8;
    }

    // Handle remaining odd indices
    while i < len {
        // For odd indices, left neighbor is always i-1 (since i starts at 1)
        let left = temp[i - 1];
        // For right neighbor, use symmetric extension at the boundary
        let right = if i + 1 < len {
            temp[i + 1]
        } else {
            temp[len - 1]
        };
        data[i] = temp[i] - (left + right) / 2.0;
        i += 2;
    }

    // Update temp buffer with predict step results
    for i in (1..len).step_by(2) {
        temp[i] = data[i];
    }

    // Step 2: Update step (Low-pass coefficients)
    // Process even indices using SIMD
    let mut i = 0;
    while i + 7 < len {
        // Load 4 even samples
        let center = f32x4::from([temp[i], temp[i + 2], temp[i + 4], temp[i + 6]]);

        // Load neighboring odd samples (now updated)
        let left = if i > 0 {
            f32x4::from([temp[i - 1], temp[i + 1], temp[i + 3], temp[i + 5]])
        } else {
            f32x4::from([0.0, temp[i + 1], temp[i + 3], temp[i + 5]])
        };

        let right = f32x4::from([
            if i + 1 < len { temp[i + 1] } else { 0.0 },
            if i + 3 < len { temp[i + 3] } else { 0.0 },
            if i + 5 < len { temp[i + 5] } else { 0.0 },
            if i + 7 < len { temp[i + 7] } else { 0.0 },
        ]);

        // Compute update: center + floor((left + right + 2) / 4)
        let sum = left + right + f32x4::splat(2.0);
        let update = (sum * f32x4::splat(0.25)).floor();
        let result = center + update;

        // Store results
        data[i] = result.as_array_ref()[0];
        data[i + 2] = result.as_array_ref()[1];
        data[i + 4] = result.as_array_ref()[2];
        data[i + 6] = result.as_array_ref()[3];

        i += 8;
    }

    // Handle remaining even indices
    while i < len {
        let left = if i > 0 { temp[i - 1] } else { 0.0 };
        let right = if i + 1 < len { temp[i + 1] } else { 0.0 };
        data[i] = temp[i] + ((left + right + 2.0) / 4.0).floor();
        i += 2;
    }

    // Separate into low-frequency and high-frequency subbands
    let mut separated = vec![0.0f32; len];
    let mid = len.div_ceil(2);

    // Pack coefficients
    for i in 0..mid {
        separated[i] = data[i * 2];
    }
    for i in 0..(len / 2) {
        separated[mid + i] = data[i * 2 + 1];
    }

    data.copy_from_slice(&separated);
}

/// SIMD-optimized 1D inverse 5/3 DWT
fn dwt_53_inverse_1d_simd(data: &mut [f32]) {
    let len = data.len();
    if len < 8 {
        // Fall back to scalar version for small arrays
        super::dwt::dwt_53_inverse_1d(data);
        return;
    }

    // Reconstruct interleaved signal from subbands
    let mut temp = vec![0.0f32; len];
    let mid = len.div_ceil(2);

    // Unpack coefficients
    for i in 0..mid {
        if i * 2 < len {
            temp[i * 2] = data[i];
        }
    }
    for i in 0..(len / 2) {
        temp[i * 2 + 1] = data[mid + i];
    }

    // Step 1: Inverse update step
    // Process even indices using SIMD
    let mut i = 0;
    while i + 7 < len {
        // Load 4 even samples
        let center = f32x4::from([temp[i], temp[i + 2], temp[i + 4], temp[i + 6]]);

        // Load neighboring odd samples
        let left = if i > 0 {
            f32x4::from([temp[i - 1], temp[i + 1], temp[i + 3], temp[i + 5]])
        } else {
            f32x4::from([0.0, temp[i + 1], temp[i + 3], temp[i + 5]])
        };

        let right = f32x4::from([
            if i + 1 < len { temp[i + 1] } else { 0.0 },
            if i + 3 < len { temp[i + 3] } else { 0.0 },
            if i + 5 < len { temp[i + 5] } else { 0.0 },
            if i + 7 < len { temp[i + 7] } else { 0.0 },
        ]);

        // Compute inverse update: center - floor((left + right + 2) / 4)
        let sum = left + right + f32x4::splat(2.0);
        let update = (sum * f32x4::splat(0.25)).floor();
        let result = center - update;

        // Store results
        data[i] = result.as_array_ref()[0];
        data[i + 2] = result.as_array_ref()[1];
        data[i + 4] = result.as_array_ref()[2];
        data[i + 6] = result.as_array_ref()[3];

        i += 8;
    }

    // Handle remaining even indices
    while i < len {
        let left = if i > 0 { temp[i - 1] } else { 0.0 };
        let right = if i + 1 < len { temp[i + 1] } else { 0.0 };
        data[i] = temp[i] - ((left + right + 2.0) / 4.0).floor();
        i += 2;
    }

    // Update temp buffer
    for i in (0..len).step_by(2) {
        temp[i] = data[i];
    }

    // Step 2: Inverse predict step
    // Process odd indices using SIMD
    let mut i = 1;
    while i + 7 < len {
        // Load 4 odd samples
        let center = f32x4::from([temp[i], temp[i + 2], temp[i + 4], temp[i + 6]]);

        // Load neighboring even samples (now updated)
        let left = f32x4::from([temp[i - 1], temp[i + 1], temp[i + 3], temp[i + 5]]);
        let right = f32x4::from([
            if i + 1 < len {
                temp[i + 1]
            } else {
                temp[len - 2]
            },
            if i + 3 < len {
                temp[i + 3]
            } else {
                temp[len - 2]
            },
            if i + 5 < len {
                temp[i + 5]
            } else {
                temp[len - 2]
            },
            if i + 7 < len {
                temp[i + 7]
            } else {
                temp[len - 2]
            },
        ]);

        // Compute inverse predict: center + (left + right) / 2
        let avg = (left + right) * f32x4::splat(0.5);
        let result = center + avg;

        // Store results
        data[i] = result.as_array_ref()[0];
        data[i + 2] = result.as_array_ref()[1];
        data[i + 4] = result.as_array_ref()[2];
        data[i + 6] = result.as_array_ref()[3];

        i += 8;
    }

    // Handle remaining odd indices
    while i < len {
        let left = if i > 0 { temp[i - 1] } else { temp[1] };
        let right = if i + 1 < len {
            temp[i + 1]
        } else {
            temp[len - 2]
        };
        data[i] = temp[i] + (left + right) / 2.0;
        i += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_dwt_roundtrip() {
        let width = 64u32;
        let height = 64u32;
        let size = (width * height) as usize;

        // Create test pattern
        let original: Vec<f32> = (0..size).map(|i| (i as f32) * 0.1).collect();
        let mut data = original.clone();
        let mut temp = vec![0.0f32; size];

        // Forward transform with SIMD
        dwt_53_forward_2d_simd(&data, &mut temp, width, height).unwrap();

        // Inverse transform with SIMD
        dwt_53_inverse_2d_simd(&temp, &mut data, width, height).unwrap();

        // Check roundtrip accuracy
        for (orig, reconstructed) in original.iter().zip(data.iter()) {
            assert!(
                (orig - reconstructed).abs() < 1e-5,
                "SIMD roundtrip error too large: {} vs {}",
                orig,
                reconstructed
            );
        }
    }

    #[test]
    fn test_simd_matches_scalar() {
        let width = 32u32;
        let height = 32u32;
        let size = (width * height) as usize;

        // Create test data
        let input: Vec<f32> = (0..size).map(|i| (i as f32) * 0.5).collect();

        // Scalar version
        let mut scalar_output = vec![0.0f32; size];
        crate::dwt::dwt_53_forward_2d(&input, &mut scalar_output, width, height).unwrap();

        // SIMD version
        let mut simd_output = vec![0.0f32; size];
        dwt_53_forward_2d_simd(&input, &mut simd_output, width, height).unwrap();

        // Compare results
        for (scalar, simd) in scalar_output.iter().zip(simd_output.iter()) {
            assert!(
                (scalar - simd).abs() < 1e-5,
                "SIMD doesn't match scalar: {} vs {}",
                scalar,
                simd
            );
        }
    }
}
