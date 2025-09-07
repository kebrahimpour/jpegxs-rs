use anyhow::Result;

/// Color space conversion functions for JPEG XS
/// 
/// Based on ITU-R BT.601 standard for YUV conversion as specified
/// in ISO/IEC 21122-1:2024 for JPEG XS color transforms.

/// ITU-R BT.601 conversion coefficients with higher precision
const RGB_TO_YUV_MATRIX: [[f64; 3]; 3] = [
    [0.299, 0.587, 0.114],              // Y coefficients
    [-0.168736, -0.331264, 0.5],        // U coefficients (Cb)
    [0.5, -0.418688, -0.081312],        // V coefficients (Cr)
];

/// ITU-R BT.601 inverse conversion coefficients with higher precision
const YUV_TO_RGB_MATRIX: [[f64; 3]; 3] = [
    [1.0, 0.0, 1.402],                  // R coefficients
    [1.0, -0.344136, -0.714136],        // G coefficients  
    [1.0, 1.772, 0.0],                  // B coefficients
];

/// Convert RGB to YUV using ITU-R BT.601 standard
/// 
/// Input: RGB interleaved data (RGBRGBRGB...)
/// Output: YUV interleaved data (YUVYUVYUV...)
pub fn rgb_to_yuv(rgb: &[u8], yuv: &mut [u8], width: u32, height: u32) -> Result<()> {
    let pixel_count = (width * height) as usize;
    
    if rgb.len() != pixel_count * 3 {
        return Err(anyhow::anyhow!("RGB buffer size mismatch"));
    }
    if yuv.len() != pixel_count * 3 {
        return Err(anyhow::anyhow!("YUV buffer size mismatch"));
    }
    
    for i in 0..pixel_count {
        let rgb_idx = i * 3;
        let yuv_idx = i * 3;
        
        let r = rgb[rgb_idx] as f64;
        let g = rgb[rgb_idx + 1] as f64;
        let b = rgb[rgb_idx + 2] as f64;
        
        // Apply ITU-R BT.601 conversion matrix
        let y = RGB_TO_YUV_MATRIX[0][0] * r + RGB_TO_YUV_MATRIX[0][1] * g + RGB_TO_YUV_MATRIX[0][2] * b;
        let u = RGB_TO_YUV_MATRIX[1][0] * r + RGB_TO_YUV_MATRIX[1][1] * g + RGB_TO_YUV_MATRIX[1][2] * b + 128.0;
        let v = RGB_TO_YUV_MATRIX[2][0] * r + RGB_TO_YUV_MATRIX[2][1] * g + RGB_TO_YUV_MATRIX[2][2] * b + 128.0;
        
        yuv[yuv_idx] = y.clamp(0.0, 255.0) as u8;
        yuv[yuv_idx + 1] = u.clamp(0.0, 255.0) as u8;
        yuv[yuv_idx + 2] = v.clamp(0.0, 255.0) as u8;
    }
    
    Ok(())
}

/// Convert YUV to RGB using ITU-R BT.601 standard
/// 
/// Input: YUV interleaved data (YUVYUVYUV...)
/// Output: RGB interleaved data (RGBRGBRGB...)
pub fn yuv_to_rgb(yuv: &[u8], rgb: &mut [u8], width: u32, height: u32) -> Result<()> {
    let pixel_count = (width * height) as usize;
    
    if yuv.len() != pixel_count * 3 {
        return Err(anyhow::anyhow!("YUV buffer size mismatch"));
    }
    if rgb.len() != pixel_count * 3 {
        return Err(anyhow::anyhow!("RGB buffer size mismatch"));
    }
    
    for i in 0..pixel_count {
        let yuv_idx = i * 3;
        let rgb_idx = i * 3;
        
        let y = yuv[yuv_idx] as f64;
        let u = yuv[yuv_idx + 1] as f64 - 128.0;
        let v = yuv[yuv_idx + 2] as f64 - 128.0;
        
        // Apply ITU-R BT.601 inverse conversion matrix
        let r = YUV_TO_RGB_MATRIX[0][0] * y + YUV_TO_RGB_MATRIX[0][1] * u + YUV_TO_RGB_MATRIX[0][2] * v;
        let g = YUV_TO_RGB_MATRIX[1][0] * y + YUV_TO_RGB_MATRIX[1][1] * u + YUV_TO_RGB_MATRIX[1][2] * v;
        let b = YUV_TO_RGB_MATRIX[2][0] * y + YUV_TO_RGB_MATRIX[2][1] * u + YUV_TO_RGB_MATRIX[2][2] * v;
        
        rgb[rgb_idx] = r.clamp(0.0, 255.0) as u8;
        rgb[rgb_idx + 1] = g.clamp(0.0, 255.0) as u8;
        rgb[rgb_idx + 2] = b.clamp(0.0, 255.0) as u8;
    }
    
    Ok(())
}

/// Downsample YUV 4:4:4 to YUV 4:2:2 using horizontal averaging
/// 
/// This implements the chroma subsampling required by JPEG XS YUV422p8 format.
/// The Y channel remains full resolution, while U and V are horizontally downsampled by 2.
pub fn downsample_444_to_422(
    y: &[u8],
    u: &[u8], 
    v: &[u8],
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let pixel_count = (width * height) as usize;
    
    if y.len() != pixel_count || u.len() != pixel_count || v.len() != pixel_count {
        return Err(anyhow::anyhow!("Input buffer size mismatch"));
    }
    
    if width % 2 != 0 {
        return Err(anyhow::anyhow!("Width must be even for 4:2:2 subsampling"));
    }
    
    let new_chroma_width = width / 2;
    let chroma_size = (new_chroma_width * height) as usize;
    
    let y_out = y.to_vec(); // Y channel unchanged
    let mut u_out = Vec::with_capacity(chroma_size);
    let mut v_out = Vec::with_capacity(chroma_size);
    
    for row in 0..height {
        for col in 0..new_chroma_width {
            let src_col = col * 2;
            let src_idx1 = (row * width + src_col) as usize;
            let src_idx2 = (row * width + src_col + 1) as usize;
            
            // Average two horizontally adjacent chroma samples
            let u_avg = ((u[src_idx1] as u16 + u[src_idx2] as u16) / 2) as u8;
            let v_avg = ((v[src_idx1] as u16 + v[src_idx2] as u16) / 2) as u8;
            
            u_out.push(u_avg);
            v_out.push(v_avg);
        }
    }
    
    Ok((y_out, u_out, v_out))
}

/// Upsample YUV 4:2:2 to YUV 4:4:4 using horizontal duplication
/// 
/// This implements the chroma upsampling required to convert from JPEG XS YUV422p8 format
/// back to full resolution. Each chroma sample is duplicated horizontally.
pub fn upsample_422_to_444(
    y: &[u8],
    u: &[u8],
    v: &[u8], 
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if width % 2 != 0 {
        return Err(anyhow::anyhow!("Width must be even for 4:2:2 upsampling"));
    }
    
    let chroma_width = width / 2;
    let expected_y_size = (width * height) as usize;
    let expected_chroma_size = (chroma_width * height) as usize;
    
    if y.len() != expected_y_size || u.len() != expected_chroma_size || v.len() != expected_chroma_size {
        return Err(anyhow::anyhow!("Input buffer size mismatch"));
    }
    
    let pixel_count = expected_y_size;
    let y_out = y.to_vec(); // Y channel unchanged
    let mut u_out = Vec::with_capacity(pixel_count);
    let mut v_out = Vec::with_capacity(pixel_count);
    
    for row in 0..height {
        for col in 0..width {
            let chroma_col = col / 2;
            let chroma_idx = (row * chroma_width + chroma_col) as usize;
            
            // Duplicate chroma samples horizontally
            u_out.push(u[chroma_idx]);
            v_out.push(v[chroma_idx]);
        }
    }
    
    Ok((y_out, u_out, v_out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_yuv_roundtrip() {
        let width = 4;
        let height = 2;
        let pixel_count = (width * height) as usize;
        
        // Test data: simple gradient
        let mut rgb = Vec::with_capacity(pixel_count * 3);
        for i in 0..pixel_count {
            let val = (i * 32) as u8;
            rgb.push(val);      // R
            rgb.push(val + 1);  // G  
            rgb.push(val + 2);  // B
        }
        
        let mut yuv = vec![0u8; pixel_count * 3];
        let mut rgb_out = vec![0u8; pixel_count * 3];
        
        // Convert RGB -> YUV -> RGB
        rgb_to_yuv(&rgb, &mut yuv, width, height).unwrap();
        yuv_to_rgb(&yuv, &mut rgb_out, width, height).unwrap();
        
        // Check roundtrip accuracy (allow larger differences due to YUV conversion precision loss)
        for i in 0..rgb.len() {
            let diff = (rgb[i] as i16 - rgb_out[i] as i16).abs();
            assert!(diff <= 3, "RGB roundtrip error too large at index {}: {} vs {} (diff: {})", 
                   i, rgb[i], rgb_out[i], diff);
        }
    }

    #[test]
    fn test_422_subsampling_roundtrip() {
        let width = 4;
        let height = 2;
        let pixel_count = (width * height) as usize;
        
        let y = vec![100u8; pixel_count];
        let u = (0..pixel_count).map(|i| (i * 10) as u8).collect::<Vec<_>>();
        let v = (0..pixel_count).map(|i| (i * 20) as u8).collect::<Vec<_>>();
        
        // 444 -> 422 -> 444
        let (y_422, u_422, v_422) = downsample_444_to_422(&y, &u, &v, width, height).unwrap();
        let (y_444, u_444, v_444) = upsample_422_to_444(&y_422, &u_422, &v_422, width, height).unwrap();
        
        // Y should be unchanged
        assert_eq!(y, y_444);
        
        // U and V should have reasonable approximation
        assert_eq!(u_444.len(), pixel_count);
        assert_eq!(v_444.len(), pixel_count);
        
        // Check that chroma values are reasonable (allowing for subsampling loss)
        for i in 0..pixel_count {
            let u_diff = (u[i] as i16 - u_444[i] as i16).abs();
            let v_diff = (v[i] as i16 - v_444[i] as i16).abs();
            
            // Allow larger differences due to 4:2:2 subsampling
            assert!(u_diff <= 50, "U roundtrip error too large at index {}: {} vs {}", 
                   i, u[i], u_444[i]);
            assert!(v_diff <= 50, "V roundtrip error too large at index {}: {} vs {}", 
                   i, v[i], v_444[i]);
        }
    }
    
    #[test] 
    fn test_invalid_dimensions() {
        let rgb = vec![0u8; 12];
        let mut yuv = vec![0u8; 12];
        
        // Wrong buffer size should fail
        assert!(rgb_to_yuv(&rgb, &mut yuv, 2, 3).is_err()); // 2*3*3 = 18, not 12
        
        // Odd width for 4:2:2 should fail
        let y = vec![0u8; 6];
        let u = vec![0u8; 6];
        let v = vec![0u8; 6];
        assert!(downsample_444_to_422(&y, &u, &v, 3, 2).is_err()); // Width 3 is odd
    }
}
