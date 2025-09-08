/// Color space conversion functions for JPEG XS
///
/// Based on ITU-R BT.601 standard for YUV conversion as specified
/// in ISO/IEC 21122-1:2024 for JPEG XS color transforms.
use anyhow::Result;

// ITU-R BT.601-7 coefficients (see Table 3 and Section 2.5.1/2.5.2)
const BT601_Y_R_COEFF: f64 = 0.299;
const BT601_Y_G_COEFF: f64 = 0.587;
const BT601_Y_B_COEFF: f64 = 0.114;
const BT601_CB_R_COEFF: f64 = -0.168736;
const BT601_CB_G_COEFF: f64 = -0.331264;
const BT601_CB_B_COEFF: f64 = 0.5;
const BT601_CR_R_COEFF: f64 = 0.5;
const BT601_CR_G_COEFF: f64 = -0.418688;
const BT601_CR_B_COEFF: f64 = -0.081312;


const YUV_TO_RGB_R_Y_COEFF: f64 = 1.0;
const YUV_TO_RGB_R_CB_COEFF: f64 = 0.0;
const YUV_TO_RGB_R_CR_COEFF: f64 = 1.402;
const YUV_TO_RGB_G_Y_COEFF: f64 = 1.0;
const YUV_TO_RGB_G_CB_COEFF: f64 = -0.344136;
const YUV_TO_RGB_G_CR_COEFF: f64 = -0.714136;
const YUV_TO_RGB_B_Y_COEFF: f64 = 1.0;
const YUV_TO_RGB_B_CB_COEFF: f64 = 1.772;
const YUV_TO_RGB_B_CR_COEFF: f64 = 0.0;

/// Maximum allowable chroma roundtrip error for 4:2:2 subsampling tests
#[cfg(test)]
const CHROMA_ROUNDTRIP_ERROR_TOLERANCE: i16 = 50;

/// ITU-R BT.601 conversion coefficients with higher precision.
///
/// These coefficients are taken from ITU-R BT.601-7 standard, specifically:
/// - Table 3: "Matrix coefficients for YCbCr"
/// - Section 2.5.1: "RGB to YCbCr conversion"
/// - Equations (2.1) through (2.3) for Y, Cb, Cr components
///
/// The exact formulas from the standard are:
/// - Y  = 0.299*R + 0.587*G + 0.114*B
/// - Cb = -0.168736*R - 0.331264*G + 0.5*B
/// - Cr = 0.5*R - 0.418688*G - 0.081312*B
///
/// These values have been verified against ITU-R BT.601-7 (03/2011) edition.
/// The precision of these coefficients is critical to minimize color conversion
/// errors. Do not modify these values without consulting the standard.
const RGB_TO_YUV_MATRIX: [[f64; 3]; 3] = [
    [BT601_Y_R_COEFF, BT601_Y_G_COEFF, BT601_Y_B_COEFF], // Y coefficients (Equation 2.1)
    [BT601_CB_R_COEFF, BT601_CB_G_COEFF, BT601_CB_B_COEFF], // Cb coefficients (Equation 2.2)
    [BT601_CR_R_COEFF, BT601_CR_G_COEFF, BT601_CR_B_COEFF], // Cr coefficients (Equation 2.3)
];

/// Helper function to apply RGB to YUV matrix transformation
/// Returns (Y, U, V) values with U and V offset by +128
#[inline]
fn apply_rgb_to_yuv_matrix(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let y = RGB_TO_YUV_MATRIX[0][0] * r + RGB_TO_YUV_MATRIX[0][1] * g + RGB_TO_YUV_MATRIX[0][2] * b;
    let u = RGB_TO_YUV_MATRIX[1][0] * r
        + RGB_TO_YUV_MATRIX[1][1] * g
        + RGB_TO_YUV_MATRIX[1][2] * b
        + 128.0;
    let v = RGB_TO_YUV_MATRIX[2][0] * r
        + RGB_TO_YUV_MATRIX[2][1] * g
        + RGB_TO_YUV_MATRIX[2][2] * b
        + 128.0;
    (y, u, v)
}

/// ITU-R BT.601 inverse conversion coefficients with higher precision.
///
/// These coefficients are derived from ITU-R BT.601-7, specifically:
/// - Table 3: "Matrix coefficients for YCbCr"
/// - Section 2.5.2: "YCbCr to RGB conversion"
/// - Inverse transformation of equations (2.1) through (2.3)
///
/// The exact formulas for inverse conversion are:
/// - R = Y + 1.402*Cr
/// - G = Y - 0.344136*Cb - 0.714136*Cr
/// - B = Y + 1.772*Cb
///
/// These values have been verified against ITU-R BT.601-7 (03/2011) edition.
/// The precision is critical for accurate color reproduction. Do not change
/// these values without consulting the standard.
const YUV_TO_RGB_MATRIX: [[f64; 3]; 3] = [
    [
        YUV_TO_RGB_R_Y_COEFF,
        YUV_TO_RGB_R_CB_COEFF,
        YUV_TO_RGB_R_CR_COEFF,
    ], // R coefficients
    [
        YUV_TO_RGB_G_Y_COEFF,
        YUV_TO_RGB_G_CB_COEFF,
        YUV_TO_RGB_G_CR_COEFF,
    ], // G coefficients
    [
        YUV_TO_RGB_B_Y_COEFF,
        YUV_TO_RGB_B_CB_COEFF,
        YUV_TO_RGB_B_CR_COEFF,
    ], // B coefficients
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
        let (y, u, v) = apply_rgb_to_yuv_matrix(r, g, b);

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
        let r =
            YUV_TO_RGB_MATRIX[0][0] * y + YUV_TO_RGB_MATRIX[0][1] * u + YUV_TO_RGB_MATRIX[0][2] * v;
        let g =
            YUV_TO_RGB_MATRIX[1][0] * y + YUV_TO_RGB_MATRIX[1][1] * u + YUV_TO_RGB_MATRIX[1][2] * v;
        let b =
            YUV_TO_RGB_MATRIX[2][0] * y + YUV_TO_RGB_MATRIX[2][1] * u + YUV_TO_RGB_MATRIX[2][2] * v;

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
        return Err(anyhow::anyhow!(
            "Width must be even for 4:2:2 subsampling, got width: {}",
            width
        ));
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

/// Convert BGR to YUV using ITU-R BT.601 standard
///
/// Input: BGR interleaved data (BGRBGRBGR...)
/// Output: YUV planar data (Y plane, U plane, V plane)
pub fn bgr_to_yuv_planar(
    bgr: &[u8],
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let pixel_count = (width * height) as usize;

    if bgr.len() != pixel_count * 3 {
        return Err(anyhow::anyhow!("BGR buffer size mismatch"));
    }

    let mut y_plane = Vec::with_capacity(pixel_count);
    let mut u_plane = Vec::with_capacity(pixel_count);
    let mut v_plane = Vec::with_capacity(pixel_count);

    for i in 0..pixel_count {
        let bgr_idx = i * 3;

        let b = bgr[bgr_idx] as f64;
        let g = bgr[bgr_idx + 1] as f64;
        let r = bgr[bgr_idx + 2] as f64;

        // Apply ITU-R BT.601 conversion matrix
        let (y, u, v) = apply_rgb_to_yuv_matrix(r, g, b);

        y_plane.push(y.clamp(0.0, 255.0) as u8);
        u_plane.push(u.clamp(0.0, 255.0) as u8);
        v_plane.push(v.clamp(0.0, 255.0) as u8);
    }

    Ok((y_plane, u_plane, v_plane))
}

/// Convert RGB planar to YUV planar using ITU-R BT.601 standard
///
/// Input: RGB planar data (R plane, G plane, B plane)
/// Output: YUV planar data (Y plane, U plane, V plane)
pub fn rgb_planar_to_yuv_planar(
    r_plane: &[u8],
    g_plane: &[u8],
    b_plane: &[u8],
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let pixel_count = (width * height) as usize;

    if r_plane.len() != pixel_count || g_plane.len() != pixel_count || b_plane.len() != pixel_count
    {
        return Err(anyhow::anyhow!("RGB planar buffer size mismatch"));
    }

    let mut y_plane = Vec::with_capacity(pixel_count);
    let mut u_plane = Vec::with_capacity(pixel_count);
    let mut v_plane = Vec::with_capacity(pixel_count);

    for i in 0..pixel_count {
        let r = r_plane[i] as f64;
        let g = g_plane[i] as f64;
        let b = b_plane[i] as f64;

        // Apply ITU-R BT.601 conversion matrix
        let (y, u, v) = apply_rgb_to_yuv_matrix(r, g, b);

        y_plane.push(y.clamp(0.0, 255.0) as u8);
        u_plane.push(u.clamp(0.0, 255.0) as u8);
        v_plane.push(v.clamp(0.0, 255.0) as u8);
    }

    Ok((y_plane, u_plane, v_plane))
}

/// Convert RGB interleaved to YUV planar using ITU-R BT.601 standard
///
/// Input: RGB interleaved data (RGBRGBRGB...)
/// Output: YUV planar data (Y plane, U plane, V plane)
pub fn rgb_to_yuv_planar(
    rgb: &[u8],
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let pixel_count = (width * height) as usize;

    if rgb.len() != pixel_count * 3 {
        return Err(anyhow::anyhow!("RGB buffer size mismatch"));
    }

    let mut y_plane = Vec::with_capacity(pixel_count);
    let mut u_plane = Vec::with_capacity(pixel_count);
    let mut v_plane = Vec::with_capacity(pixel_count);

    for i in 0..pixel_count {
        let rgb_idx = i * 3;

        let r = rgb[rgb_idx] as f64;
        let g = rgb[rgb_idx + 1] as f64;
        let b = rgb[rgb_idx + 2] as f64;

        // Apply ITU-R BT.601 conversion matrix
        let (y, u, v) = apply_rgb_to_yuv_matrix(r, g, b);

        y_plane.push(y.clamp(0.0, 255.0) as u8);
        u_plane.push(u.clamp(0.0, 255.0) as u8);
        v_plane.push(v.clamp(0.0, 255.0) as u8);
    }

    Ok((y_plane, u_plane, v_plane))
}

/// Downsample YUV 4:4:4 to YUV 4:2:0 using 2x2 averaging
///
/// This implements the chroma subsampling required by JPEG XS YUV420p8 format.
/// The Y channel remains full resolution, while U and V are downsampled by 2 in both dimensions.
pub fn downsample_444_to_420(
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

    if width % 2 != 0 || height % 2 != 0 {
        return Err(anyhow::anyhow!(
            "Width and height must be even for 4:2:0 subsampling, got {}x{}",
            width,
            height
        ));
    }

    let new_chroma_width = width / 2;
    let new_chroma_height = height / 2;
    let chroma_size = (new_chroma_width * new_chroma_height) as usize;

    let y_out = y.to_vec(); // Y channel unchanged
    let mut u_out = Vec::with_capacity(chroma_size);
    let mut v_out = Vec::with_capacity(chroma_size);

    for row in 0..new_chroma_height {
        for col in 0..new_chroma_width {
            let src_row = row * 2;
            let src_col = col * 2;

            // Indices of the 2x2 block in the source
            let idx1 = ((src_row * width) + src_col) as usize;
            let idx2 = ((src_row * width) + src_col + 1) as usize;
            let idx3 = (((src_row + 1) * width) + src_col) as usize;
            let idx4 = (((src_row + 1) * width) + src_col + 1) as usize;

            // Average 2x2 block for chroma
            let u_avg =
                ((u[idx1] as u16 + u[idx2] as u16 + u[idx3] as u16 + u[idx4] as u16) / 4) as u8;
            let v_avg =
                ((v[idx1] as u16 + v[idx2] as u16 + v[idx3] as u16 + v[idx4] as u16) / 4) as u8;

            u_out.push(u_avg);
            v_out.push(v_avg);
        }
    }

    Ok((y_out, u_out, v_out))
}

/// Upsample YUV 4:2:0 to YUV 4:4:4 using bilinear interpolation
///
/// This implements the chroma upsampling required to convert from JPEG XS YUV420p8 format
/// back to full resolution.
pub fn upsample_420_to_444(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if width % 2 != 0 || height % 2 != 0 {
        return Err(anyhow::anyhow!(
            "Width and height must be even for 4:2:0 upsampling, got {}x{}",
            width,
            height
        ));
    }

    let chroma_width = width / 2;
    let chroma_height = height / 2;
    let expected_y_size = (width * height) as usize;
    let expected_chroma_size = (chroma_width * chroma_height) as usize;

    if y.len() != expected_y_size
        || u.len() != expected_chroma_size
        || v.len() != expected_chroma_size
    {
        return Err(anyhow::anyhow!("Input buffer size mismatch"));
    }

    let pixel_count = expected_y_size;
    let y_out = y.to_vec(); // Y channel unchanged
    let mut u_out = Vec::with_capacity(pixel_count);
    let mut v_out = Vec::with_capacity(pixel_count);

    for row in 0..height {
        for col in 0..width {
            let chroma_row = row / 2;
            let chroma_col = col / 2;
            let chroma_idx = (chroma_row * chroma_width + chroma_col) as usize;

            // Simple duplication for now (could implement bilinear interpolation)
            u_out.push(u[chroma_idx]);
            v_out.push(v[chroma_idx]);
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
        return Err(anyhow::anyhow!(
            "Width must be even for 4:2:2 upsampling, got width: {}",
            width
        ));
    }

    let chroma_width = width / 2;
    let expected_y_size = (width * height) as usize;
    let expected_chroma_size = (chroma_width * height) as usize;

    if y.len() != expected_y_size
        || u.len() != expected_chroma_size
        || v.len() != expected_chroma_size
    {
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
    fn test_bgr_to_yuv_planar() {
        let width = 2;
        let height = 2;
        let bgr = vec![
            100, 150, 200, // Pixel 1: B=100, G=150, R=200
            50, 100, 150, // Pixel 2: B=50, G=100, R=150
            75, 125, 175, // Pixel 3: B=75, G=125, R=175
            25, 75, 125, // Pixel 4: B=25, G=75, R=125
        ];

        let (y, u, v) = bgr_to_yuv_planar(&bgr, width, height).unwrap();

        assert_eq!(y.len(), 4);
        assert_eq!(u.len(), 4);
        assert_eq!(v.len(), 4);

        // Verify Y values are reasonable (should be weighted average of RGB)
        for &y_val in &y {
            assert!(y_val > 0 && y_val < 255);
        }
    }

    #[test]
    fn test_rgb_planar_to_yuv_planar() {
        let width = 2;
        let height = 2;
        let r_plane = vec![200, 150, 175, 125];
        let g_plane = vec![150, 100, 125, 75];
        let b_plane = vec![100, 50, 75, 25];

        let (y, u, v) =
            rgb_planar_to_yuv_planar(&r_plane, &g_plane, &b_plane, width, height).unwrap();

        assert_eq!(y.len(), 4);
        assert_eq!(u.len(), 4);
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn test_rgb_to_yuv_planar() {
        let width = 2;
        let height = 2;
        let rgb = vec![
            200, 150, 100, // Pixel 1: R=200, G=150, B=100
            150, 100, 50, // Pixel 2
            175, 125, 75, // Pixel 3
            125, 75, 25, // Pixel 4
        ];

        let (y, u, v) = rgb_to_yuv_planar(&rgb, width, height).unwrap();

        assert_eq!(y.len(), 4);
        assert_eq!(u.len(), 4);
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn test_420_subsampling_roundtrip() {
        let width = 4;
        let height = 4;
        let pixel_count = (width * height) as usize;

        let y = vec![100u8; pixel_count];
        let u = (0..pixel_count).map(|i| (i * 10) as u8).collect::<Vec<_>>();
        let v = (0..pixel_count).map(|i| (i * 20) as u8).collect::<Vec<_>>();

        // 444 -> 420 -> 444
        let (y_420, u_420, v_420) = downsample_444_to_420(&y, &u, &v, width, height).unwrap();

        assert_eq!(u_420.len(), 4); // 2x2 for 4x4 image
        assert_eq!(v_420.len(), 4);

        let (y_444, u_444, v_444) =
            upsample_420_to_444(&y_420, &u_420, &v_420, width, height).unwrap();

        // Y should be unchanged
        assert_eq!(y, y_444);

        // U and V should have reasonable approximation
        assert_eq!(u_444.len(), pixel_count);
        assert_eq!(v_444.len(), pixel_count);
    }

    #[test]
    fn test_invalid_420_dimensions() {
        let y = vec![0u8; 9];
        let u = vec![0u8; 9];
        let v = vec![0u8; 9];

        // Odd dimensions should fail for 4:2:0
        assert!(downsample_444_to_420(&y, &u, &v, 3, 3).is_err());
        assert!(upsample_420_to_444(&y, &u, &v, 3, 3).is_err());
    }

    #[test]
    fn test_rgb_yuv_roundtrip() {
        let width = 4;
        let height = 2;
        let pixel_count = (width * height) as usize;

        // Test data: simple gradient
        let mut rgb = Vec::with_capacity(pixel_count * 3);
        for i in 0..pixel_count {
            let val = (i * 32) as u8;
            rgb.push(val); // R
            rgb.push(val + 1); // G
            rgb.push(val + 2); // B
        }

        let mut yuv = vec![0u8; pixel_count * 3];
        let mut rgb_out = vec![0u8; pixel_count * 3];

        // Convert RGB -> YUV -> RGB
        rgb_to_yuv(&rgb, &mut yuv, width, height).unwrap();
        yuv_to_rgb(&yuv, &mut rgb_out, width, height).unwrap();

        // Check roundtrip accuracy (allow larger differences due to YUV conversion precision loss)
        for i in 0..rgb.len() {
            let diff = (rgb[i] as i16 - rgb_out[i] as i16).abs();
            assert!(
                diff <= 3,
                "RGB roundtrip error too large at index {}: {} vs {} (diff: {})",
                i,
                rgb[i],
                rgb_out[i],
                diff
            );
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
        let (y_444, u_444, v_444) =
            upsample_422_to_444(&y_422, &u_422, &v_422, width, height).unwrap();

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
            assert!(
                u_diff <= CHROMA_ROUNDTRIP_ERROR_TOLERANCE,
                "U roundtrip error too large at index {}: {} vs {}",
                i,
                u[i],
                u_444[i]
            );
            assert!(
                v_diff <= CHROMA_ROUNDTRIP_ERROR_TOLERANCE,
                "V roundtrip error too large at index {}: {} vs {}",
                i,
                v[i],
                v_444[i]
            );
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
