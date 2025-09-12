// DERIVATIVE WORK NOTICE:
// The 5/3 lifting scheme implementation below is derived from concepts in the
// JPEG XS reference software. Copyright holders: intoPIX SA, Fraunhofer IIS, Canon Inc.
// Commercial use requires RAND license from original copyright holders.
// This will be replaced with a clean-room implementation from ISO/IEC 21122 specification.

use anyhow::Result;

pub fn dwt_53_forward_2d(input: &[f32], output: &mut [f32], width: u32, height: u32) -> Result<()> {
    if input.len() != (width * height) as usize || output.len() != input.len() {
        return Err(anyhow::anyhow!("Invalid buffer sizes"));
    }

    // Copy input to output for in-place processing
    output.copy_from_slice(input);

    // Apply 1D DWT along rows
    for y in 0..height {
        let row_start = (y * width) as usize;
        let row_end = row_start + width as usize;
        dwt_53_forward_1d(&mut output[row_start..row_end]);
    }

    // Apply 1D DWT along columns
    let mut col_buffer = vec![0.0f32; height as usize];
    for x in 0..width {
        // Extract column
        for y in 0..height {
            col_buffer[y as usize] = output[(y * width + x) as usize];
        }

        // Transform column
        dwt_53_forward_1d(&mut col_buffer);

        // Put column back
        for y in 0..height {
            output[(y * width + x) as usize] = col_buffer[y as usize];
        }
    }

    Ok(())
}

pub fn dwt_53_inverse_2d(input: &[f32], output: &mut [f32], width: u32, height: u32) -> Result<()> {
    if input.len() != (width * height) as usize || output.len() != input.len() {
        return Err(anyhow::anyhow!("Invalid buffer sizes"));
    }

    // Copy input to output for in-place processing
    output.copy_from_slice(input);

    // Apply 1D inverse DWT along columns first
    let mut col_buffer = vec![0.0f32; height as usize];
    for x in 0..width {
        // Extract column
        for y in 0..height {
            col_buffer[y as usize] = output[(y * width + x) as usize];
        }

        // Inverse transform column
        dwt_53_inverse_1d(&mut col_buffer);

        // Put column back
        for y in 0..height {
            output[(y * width + x) as usize] = col_buffer[y as usize];
        }
    }

    // Apply 1D inverse DWT along rows
    for y in 0..height {
        let row_start = (y * width) as usize;
        let row_end = row_start + width as usize;
        dwt_53_inverse_1d(&mut output[row_start..row_end]);
    }

    Ok(())
}

// Note: JPEG XS standard (ISO/IEC 21122-1:2024) only specifies 5/3 reversible DWT.
// The 9/7 irreversible DWT is not part of the JPEG XS specification.

// 1D 5/3 lifting-based DWT forward transform
fn dwt_53_forward_1d(data: &mut [f32]) {
    let len = data.len();
    if len < 2 {
        return;
    }

    // ISO/IEC 21122-1 Annex E.7: 5/3 lifting scheme
    // Predict step: odd[i] -= (even[i-1] + even[i+1]) / 2
    for i in (1..len).step_by(2) {
        let left = if i > 0 { data[i - 1] } else { data[i + 1] };
        let right = if i + 1 < len {
            data[i + 1]
        } else {
            data[i - 1]
        };
        data[i] -= (left + right) / 2.0;
    }

    // Update step: even[i] += (odd[i-1] + odd[i+1]) / 4
    for i in (0..len).step_by(2) {
        let left = if i > 0 { data[i - 1] } else { 0.0 };
        let right = if i + 1 < len { data[i + 1] } else { 0.0 };
        data[i] += (left + right) / 4.0;
    }


    // Separate low and high frequencies
    let mut temp = vec![0.0f32; len];
    let mut low_idx = 0;
    let mut high_idx = len.div_ceil(2);

    for (i, &value) in data.iter().enumerate().take(len) {
        if i % 2 == 0 {
            temp[low_idx] = value;
            low_idx += 1;
        } else {
            temp[high_idx] = value;
            high_idx += 1;
        }
    }

    data.copy_from_slice(&temp);
}

// 1D 5/3 lifting-based DWT inverse transform
fn dwt_53_inverse_1d(data: &mut [f32]) {
    let len = data.len();
    if len < 2 {
        return;
    }

    // Reconstruct interleaved signal
    let mut temp = vec![0.0f32; len];
    let low_len = len.div_ceil(2);
    let high_len = len / 2;

    for i in 0..low_len {
        temp[i * 2] = data[i];
    }
    for i in 0..high_len {
        temp[i * 2 + 1] = data[low_len + i];
    }

    data.copy_from_slice(&temp);


    // Reverse update step
    for i in (0..len).step_by(2) {
        let left = if i > 0 { data[i - 1] } else { 0.0 };
        let right = if i + 1 < len { data[i + 1] } else { 0.0 };
        data[i] -= (left + right) / 4.0;
    }

    // Reverse predict step
    for i in (1..len).step_by(2) {
        let left = if i > 0 { data[i - 1] } else { data[i + 1] };
        let right = if i + 1 < len {
            data[i + 1]
        } else {
            data[i - 1]
        };
        data[i] += (left + right) / 2.0;
    }
}
