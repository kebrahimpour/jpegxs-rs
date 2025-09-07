// Copyright (c) 2024 Keyvan Ebrahimpour. All rights reserved.

//! Color space conversion utilities for benchmarking
//! 
//! Provides RGB <-> YUV conversion functions used across multiple benchmark tools.

/// Convert RGB data to YUV422p format
/// 
/// # Arguments
/// * `rgb_data` - Input RGB data (R, G, B bytes)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// 
/// # Returns
/// YUV422p data with layout: [Y plane][U plane][V plane]
pub fn rgb_to_yuv422p(rgb_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let pixels = width as usize * height as usize;
    let mut yuv_data = Vec::with_capacity(pixels * 2);

    let mut y_plane = Vec::with_capacity(pixels);
    let mut u_plane = Vec::with_capacity(pixels / 2);
    let mut v_plane = Vec::with_capacity(pixels / 2);

    for y in 0..height as usize {
        for x in (0..width as usize).step_by(2) {
            let idx1 = (y * width as usize + x) * 3;
            let idx2 = if x + 1 < width as usize {
                (y * width as usize + x + 1) * 3
            } else {
                idx1
            };

            // RGB to YUV conversion (ITU-R BT.601)
            let r1 = rgb_data[idx1] as f32;
            let g1 = rgb_data[idx1 + 1] as f32;
            let b1 = rgb_data[idx1 + 2] as f32;

            let r2 = rgb_data[idx2] as f32;
            let g2 = rgb_data[idx2 + 1] as f32;
            let b2 = rgb_data[idx2 + 2] as f32;

            let y1 = (0.299 * r1 + 0.587 * g1 + 0.114 * b1)
                .round()
                .clamp(0.0, 255.0) as u8;
            let y2 = (0.299 * r2 + 0.587 * g2 + 0.114 * b2)
                .round()
                .clamp(0.0, 255.0) as u8;

            y_plane.push(y1);
            if x + 1 < width as usize {
                y_plane.push(y2);
            }

            let avg_r = (r1 + r2) / 2.0;
            let avg_g = (g1 + g2) / 2.0;
            let avg_b = (b1 + b2) / 2.0;

            let u = (-0.14713 * avg_r - 0.28886 * avg_g + 0.436 * avg_b + 128.0)
                .round()
                .clamp(0.0, 255.0) as u8;
            let v = (0.615 * avg_r - 0.51499 * avg_g - 0.10001 * avg_b + 128.0)
                .round()
                .clamp(0.0, 255.0) as u8;

            u_plane.push(u);
            v_plane.push(v);
        }
    }

    yuv_data.extend_from_slice(&y_plane);
    yuv_data.extend_from_slice(&u_plane);
    yuv_data.extend_from_slice(&v_plane);

    yuv_data
}

/// Convert YUV422p data back to RGB format
/// 
/// # Arguments
/// * `yuv_data` - Input YUV422p data
/// * `width` - Image width in pixels  
/// * `height` - Image height in pixels
/// 
/// # Returns
/// RGB data with layout: [R, G, B, R, G, B, ...]
pub fn yuv422p_to_rgb(yuv_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let y_size = (width * height) as usize;
    let uv_size = y_size / 2;
    let mut rgb_data = Vec::with_capacity(y_size * 3);

    let y_plane = &yuv_data[0..y_size];
    let u_plane = &yuv_data[y_size..y_size + uv_size];
    let v_plane = &yuv_data[y_size + uv_size..y_size + 2 * uv_size];

    for y in 0..height as usize {
        for x in 0..width as usize {
            let y_val = y_plane[y * width as usize + x] as f32;
            let u_val = u_plane[y * (width as usize / 2) + x / 2] as f32 - 128.0;
            let v_val = v_plane[y * (width as usize / 2) + x / 2] as f32 - 128.0;

            let r = (y_val + 1.402 * v_val).round().clamp(0.0, 255.0) as u8;
            let g = (y_val - 0.34414 * u_val - 0.71414 * v_val)
                .round()
                .clamp(0.0, 255.0) as u8;
            let b = (y_val + 1.772 * u_val).round().clamp(0.0, 255.0) as u8;

            rgb_data.push(r);
            rgb_data.push(g);
            rgb_data.push(b);
        }
    }

    rgb_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_yuv_roundtrip() {
        // Create a small test image
        let width = 4u32;
        let height = 2u32;
        let rgb_data = vec![
            255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, // First row: R, G, B, W
            128, 128, 128, 64, 64, 64, 192, 192, 192, 0, 0, 0, // Second row: grays and black
        ];

        // Convert to YUV and back
        let yuv_data = rgb_to_yuv422p(&rgb_data, width, height);
        let rgb_result = yuv422p_to_rgb(&yuv_data, width, height);

        // Should be approximately equal (some precision loss expected)
        assert_eq!(rgb_data.len(), rgb_result.len());
        
        // Check that conversion doesn't completely destroy the data
        for (original, result) in rgb_data.iter().zip(rgb_result.iter()) {
            let diff = (*original as i16 - *result as i16).abs();
            assert!(diff <= 5, "Color difference too large: {} vs {}", original, result);
        }
    }
}