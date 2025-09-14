// JPEG XS Educational Implementation - Copyright (c) 2025 Keyvan Ebrahimpour
//
// Educational use only. This software is provided for learning and research purposes.
// See LICENSE file for complete educational use terms and conditions.

pub mod accel;
pub mod colors;
pub mod dwt;
pub mod dwt_validation;
pub mod entropy;
pub mod gpu_dwt;
pub mod neon_dwt;
pub mod packet;
pub mod profile;
pub mod quant;
pub mod types;

use anyhow::Result;
pub use types::{Bitstream, DecoderConfig, EncoderConfig, ImageOwned8, ImageView8, PixelFormat};

/// Default quantization parameter used when QP values cannot be extracted from bitstream
/// This provides a moderate quality fallback that balances compression and visual quality
const DEFAULT_FALLBACK_QP: u8 = 8;

/// Encode an image frame using JPEG XS compression
///
/// This function supports multiple pixel formats and automatically handles format conversion
/// to the internal YUV444 representation used by JPEG XS.
///
/// # Supported Formats
/// - `Yuv444p8`: Full resolution YUV (most efficient, no conversion needed)
/// - `Yuv422p8`: Horizontally subsampled chroma, upsampled to 4:4:4 internally
/// - `Yuv420p8`: Vertically and horizontally subsampled chroma, upsampled to 4:4:4
/// - `Rgb8`: Interleaved RGB, converted using ITU-R BT.601 color matrix
/// - `Bgr8`: Interleaved BGR, converted using ITU-R BT.601 color matrix
/// - `Rgb8Planar`: Planar RGB (separate R, G, B planes), converted using ITU-R BT.601
///
/// # Example
/// ```rust,ignore
/// use jpegxs_core::{encode_frame, types::{EncoderConfig, ImageView8, PixelFormat}};
///
/// // RGB8 input data
/// let width = 128;
/// let height = 96;
/// let rgb_data = vec![0u8; (width * height * 3) as usize];
///
/// let input = ImageView8 {
///     data: &rgb_data,
///     width,
///     height,
///     format: PixelFormat::Rgb8,
/// };
///
/// let config = EncoderConfig {
///     quality: 0.95, // High quality
///     ..Default::default()
/// };
///
/// let bitstream = encode_frame(input, &config)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn encode_frame(input: ImageView8, config: &EncoderConfig) -> Result<Bitstream> {
    // Convert input image to YUV planar format for processing
    let (y_data, u_data, v_data) = match input.format {
        PixelFormat::Yuv444p8 => {
            // Direct YUV444 - most efficient path
            let pixel_count = (input.width * input.height) as usize;

            if input.data.len() < pixel_count * 3 {
                return Err(anyhow::anyhow!("Insufficient data for YUV444p8 format"));
            }

            let y = &input.data[0..pixel_count];
            let u = &input.data[pixel_count..pixel_count * 2];
            let v = &input.data[pixel_count * 2..pixel_count * 3];
            (y.to_vec(), u.to_vec(), v.to_vec())
        }
        PixelFormat::Yuv422p8 => {
            // YUV422 - upsample chroma to 444
            if input.width % 2 != 0 {
                return Err(anyhow::anyhow!("Width must be even for YUV422p8 format"));
            }
            let y_size = (input.width * input.height) as usize;
            let uv_size = (input.width / 2 * input.height) as usize;

            if input.data.len() < y_size + uv_size * 2 {
                return Err(anyhow::anyhow!("Insufficient data for YUV422p8 format"));
            }

            let y = &input.data[0..y_size];
            let u = &input.data[y_size..y_size + uv_size];
            let v = &input.data[y_size + uv_size..y_size + uv_size * 2];

            // Upsample to 444
            colors::upsample_422_to_444(y, u, v, input.width, input.height)?
        }
        PixelFormat::Yuv420p8 => {
            // YUV420 - upsample chroma to 444
            if input.width % 2 != 0 || input.height % 2 != 0 {
                return Err(anyhow::anyhow!(
                    "Width and height must be even for YUV420p8 format"
                ));
            }
            let y_size = (input.width * input.height) as usize;
            let uv_size = (input.width / 2 * input.height / 2) as usize;

            if input.data.len() < y_size + uv_size * 2 {
                return Err(anyhow::anyhow!("Insufficient data for YUV420p8 format"));
            }

            let y = &input.data[0..y_size];
            let u = &input.data[y_size..y_size + uv_size];
            let v = &input.data[y_size + uv_size..y_size + uv_size * 2];

            // Upsample to 444
            colors::upsample_420_to_444(y, u, v, input.width, input.height)?
        }
        PixelFormat::Rgb8 => {
            // RGB interleaved - convert to YUV
            colors::rgb_to_yuv_planar(input.data, input.width, input.height)?
        }
        PixelFormat::Bgr8 => {
            // BGR interleaved - convert to YUV
            colors::bgr_to_yuv_planar(input.data, input.width, input.height)?
        }
        PixelFormat::Rgb8Planar => {
            // RGB planar - convert to YUV
            let pixel_count = (input.width * input.height) as usize;

            if input.data.len() < pixel_count * 3 {
                return Err(anyhow::anyhow!("Insufficient data for RGB8Planar format"));
            }

            let r = &input.data[0..pixel_count];
            let g = &input.data[pixel_count..pixel_count * 2];
            let b = &input.data[pixel_count * 2..pixel_count * 3];
            colors::rgb_planar_to_yuv_planar(r, g, b, input.width, input.height)?
        }
    };

    // Convert to floating point and center around 0
    let mut y_plane = vec![0.0f32; y_data.len()];
    let mut u_plane = vec![0.0f32; u_data.len()];
    let mut v_plane = vec![0.0f32; v_data.len()];

    for (i, &val) in y_data.iter().enumerate() {
        y_plane[i] = val as f32 - 128.0;
    }
    for (i, &val) in u_data.iter().enumerate() {
        u_plane[i] = val as f32 - 128.0;
    }
    for (i, &val) in v_data.iter().enumerate() {
        v_plane[i] = val as f32 - 128.0;
    }

    // Log pre-DWT statistics for precision analysis
    log::info!(
        "DWT_ANALYSIS: Pre-DWT Y coefficients - min: {:.3}, max: {:.3}, mean: {:.3}, std: {:.3}",
        y_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
        y_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
        y_plane.iter().sum::<f32>() / y_plane.len() as f32,
        {
            let mean = y_plane.iter().sum::<f32>() / y_plane.len() as f32;
            (y_plane.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / y_plane.len() as f32).sqrt()
        }
    );
    log::info!("DWT_ANALYSIS: Pre-DWT UV coefficients - U_min: {:.3}, U_max: {:.3}, V_min: {:.3}, V_max: {:.3}",
               u_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               u_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
               v_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               v_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)));

    // Apply DWT to each plane using Apple Silicon acceleration (all are now 444)
    let mut y_dwt = vec![0.0f32; y_plane.len()];
    let mut u_dwt = vec![0.0f32; u_plane.len()];
    let mut v_dwt = vec![0.0f32; v_plane.len()];

    // Initialize unified acceleration (GPU → NEON → Scalar fallback)
    let accel = accel::AccelDwt::new();

    accel.dwt_53_forward_2d(&y_plane, &mut y_dwt, input.width, input.height)?;
    accel.dwt_53_forward_2d(&u_plane, &mut u_dwt, input.width, input.height)?;
    accel.dwt_53_forward_2d(&v_plane, &mut v_dwt, input.width, input.height)?;

    // Log post-DWT statistics for precision analysis
    log::info!(
        "DWT_ANALYSIS: Post-DWT Y coefficients - min: {:.3}, max: {:.3}, mean: {:.3}, std: {:.3}",
        y_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
        y_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
        y_dwt.iter().sum::<f32>() / y_dwt.len() as f32,
        {
            let mean = y_dwt.iter().sum::<f32>() / y_dwt.len() as f32;
            (y_dwt.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / y_dwt.len() as f32).sqrt()
        }
    );
    log::info!("DWT_ANALYSIS: Post-DWT UV coefficients - U_min: {:.3}, U_max: {:.3}, V_min: {:.3}, V_max: {:.3}",
               u_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               u_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
               v_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               v_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)));

    // Quantize coefficients using corrected quality mapping
    let qps = quant::compute_quantization_parameters(config.quality)?;
    let qp_y = qps[0];
    let qp_uv = qps.get(1).copied().unwrap_or(qp_y);

    let y_quantized = quant::quantize(&y_dwt, qp_y)?;
    let u_quantized = quant::quantize(&u_dwt, qp_uv)?;
    let v_quantized = quant::quantize(&v_dwt, qp_uv)?;

    // Log post-quantization statistics for precision analysis
    log::info!(
        "DWT_ANALYSIS: Post-Quantization Y coefficients - min: {}, max: {}, mean: {:.3}, QP: {}",
        y_quantized.iter().min().unwrap_or(&0),
        y_quantized.iter().max().unwrap_or(&0),
        y_quantized.iter().sum::<i32>() as f32 / y_quantized.len() as f32,
        qp_y
    );
    log::info!("DWT_ANALYSIS: Post-Quantization UV coefficients - U_min: {}, U_max: {}, V_min: {}, V_max: {}, QP: {}",
               u_quantized.iter().min().unwrap_or(&0),
               u_quantized.iter().max().unwrap_or(&0),
               v_quantized.iter().min().unwrap_or(&0),
               v_quantized.iter().max().unwrap_or(&0),
               qp_uv);

    // Use clean-room JPEG XS bitstream format from ISO/IEC 21122-1:2024
    let mut jxs_bitstream = jpegxs_core_clean::JpegXsBitstream::new();

    // Add Capabilities marker (mandatory second marker per ISO A.4.3)
    jxs_bitstream.write_cap_marker();

    // Add PIH (Picture Header) marker according to ISO A.7 specification
    // Third mandatory marker providing image dimensions and decoder configuration
    let num_components = 3; // All formats are converted to YUV with 3 components
    jxs_bitstream.write_pih_marker(input.width as u16, input.height as u16, num_components);

    // Add CDT (Component Table) marker according to ISO A.4.5 specification
    // Fourth mandatory marker providing component precision and sampling factors
    jxs_bitstream.write_cdt_marker(num_components);

    // Add WGT (Weights Table) marker according to ISO A.4.12 specification
    // Fifth mandatory marker providing band gain parameters for quantization
    // Pass all subband QP values computed from quality setting
    jxs_bitstream.write_wgt_marker(Some(&qps));

    // Add entropy coded data per ISO Annex C specification
    // Combine all quantized coefficients for entropy coding
    let mut all_coefficients = Vec::new();
    all_coefficients.extend_from_slice(&y_quantized);
    all_coefficients.extend_from_slice(&u_quantized);
    all_coefficients.extend_from_slice(&v_quantized);

    // Apply basic entropy coding for significant compression
    jxs_bitstream.add_entropy_coded_data(&all_coefficients);

    // Finalize with EOC marker
    jxs_bitstream.finalize();
    let final_data = jxs_bitstream.into_bytes();

    let size_bits = final_data.len() * 8;
    Ok(Bitstream {
        data: final_data,
        size_bits,
    })
}

pub fn decode_frame(bitstream: &Bitstream, _config: &DecoderConfig) -> Result<ImageOwned8> {
    decode_frame_to_format(bitstream, _config, PixelFormat::Yuv444p8)
}

/// Decode a JPEG XS bitstream to a specific pixel format
///
/// This function decodes a JPEG XS bitstream and converts the result to the specified
/// output format. The internal representation is always YUV444, so format conversion
/// is applied as needed.
///
/// # Supported Output Formats
/// - `Yuv444p8`: Full resolution YUV (most efficient, no conversion needed)
/// - `Yuv422p8`: Horizontally subsampled chroma, downsampled from 4:4:4
/// - `Yuv420p8`: Vertically and horizontally subsampled chroma, downsampled from 4:4:4
/// - `Rgb8`: Interleaved RGB, converted using ITU-R BT.601 inverse color matrix
/// - `Bgr8`: Interleaved BGR, converted using ITU-R BT.601 inverse color matrix
/// - `Rgb8Planar`: Planar RGB (separate R, G, B planes), converted using ITU-R BT.601
///
/// # Example
/// ```rust,ignore
/// use jpegxs_core::{decode_frame_to_format, types::{DecoderConfig, PixelFormat}};
///
/// // Decode to RGB8 format
/// let decoder_config = DecoderConfig::default();
/// let decoded = decode_frame_to_format(&bitstream, &decoder_config, PixelFormat::Rgb8)?;
///
/// assert_eq!(decoded.format, PixelFormat::Rgb8);
/// assert_eq!(decoded.data.len(), (decoded.width * decoded.height * 3) as usize);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn decode_frame_to_format(
    bitstream: &Bitstream,
    _config: &DecoderConfig,
    output_format: PixelFormat,
) -> Result<ImageOwned8> {
    // Use clean-room JPEG XS decoder to parse headers and extract entropy data
    let mut decoder = jpegxs_core_clean::JpegXsDecoder::new(bitstream.data.clone())
        .map_err(|e| anyhow::anyhow!("Decoder creation failed: {}", e))?;

    // Parse JPEG XS markers to extract image parameters
    decoder
        .parse_headers()
        .map_err(|e| anyhow::anyhow!("Header parsing failed: {}", e))?;

    let (width, height, num_components) = decoder.dimensions();
    if num_components != 3 {
        return Err(anyhow::anyhow!(
            "Unsupported number of components: {}",
            num_components
        ));
    }

    // Decode entropy coded data
    let all_coefficients = decoder
        .decode_entropy_data()
        .map_err(|e| anyhow::anyhow!("Entropy decoding failed: {}", e))?;

    // Split coefficients back into Y, U, V components (all 444 now)
    let width = width as u32;
    let height = height as u32;
    let y_size = (width * height) as usize;
    let uv_size = y_size; // Full resolution chroma

    if all_coefficients.len() < y_size + 2 * uv_size {
        return Err(anyhow::anyhow!("Insufficient decoded coefficients"));
    }

    let y_quantized = all_coefficients[0..y_size].to_vec();
    let u_quantized = all_coefficients[y_size..y_size + uv_size].to_vec();
    let v_quantized = all_coefficients[y_size + uv_size..y_size + 2 * uv_size].to_vec();

    // Dequantize - Extract QP from WGT marker
    let qp_values = decoder.get_qp_values();
    let qp_y = qp_values.first().copied().unwrap_or(DEFAULT_FALLBACK_QP);
    let qp_uv = qp_values.get(1).copied().unwrap_or(qp_y);

    let y_dwt = quant::dequantize(&y_quantized, qp_y)?;
    let u_dwt = quant::dequantize(&u_quantized, qp_uv)?;
    let v_dwt = quant::dequantize(&v_quantized, qp_uv)?;

    // Log post-dequantization statistics for precision analysis
    log::info!(
        "DWT_ANALYSIS: Post-Dequantization Y coefficients - min: {:.3}, max: {:.3}, mean: {:.3}",
        y_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
        y_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
        y_dwt.iter().sum::<f32>() / y_dwt.len() as f32
    );
    log::info!("DWT_ANALYSIS: Post-Dequantization UV coefficients - U_min: {:.3}, U_max: {:.3}, V_min: {:.3}, V_max: {:.3}",
               u_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               u_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
               v_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               v_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)));

    // Apply inverse DWT using Apple Silicon acceleration (all planes are 444)
    let mut y_plane = vec![0.0f32; y_size];
    let mut u_plane = vec![0.0f32; uv_size];
    let mut v_plane = vec![0.0f32; uv_size];

    // Initialize unified acceleration (GPU → NEON → Scalar fallback)
    let accel = accel::AccelDwt::new();

    accel.dwt_53_inverse_2d(&y_dwt, &mut y_plane, width, height)?;
    accel.dwt_53_inverse_2d(&u_dwt, &mut u_plane, width, height)?;
    accel.dwt_53_inverse_2d(&v_dwt, &mut v_plane, width, height)?;

    // Log post-inverse-DWT statistics for precision analysis
    log::info!("DWT_ANALYSIS: Post-Inverse-DWT Y coefficients - min: {:.3}, max: {:.3}, mean: {:.3}, std: {:.3}",
               y_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               y_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
               y_plane.iter().sum::<f32>() / y_plane.len() as f32,
               {
                   let mean = y_plane.iter().sum::<f32>() / y_plane.len() as f32;
                   (y_plane.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / y_plane.len() as f32).sqrt()
               });
    log::info!("DWT_ANALYSIS: Post-Inverse-DWT UV coefficients - U_min: {:.3}, U_max: {:.3}, V_min: {:.3}, V_max: {:.3}",
               u_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               u_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
               v_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
               v_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)));

    // Convert back to 8-bit
    let mut y_data = Vec::with_capacity(y_size);
    let mut u_data = Vec::with_capacity(uv_size);
    let mut v_data = Vec::with_capacity(uv_size);

    for &sample in &y_plane {
        y_data.push((sample + 128.0).clamp(0.0, 255.0) as u8);
    }
    for &sample in &u_plane {
        u_data.push((sample + 128.0).clamp(0.0, 255.0) as u8);
    }
    for &sample in &v_plane {
        v_data.push((sample + 128.0).clamp(0.0, 255.0) as u8);
    }

    // Convert to desired output format
    let data = match output_format {
        PixelFormat::Yuv444p8 => {
            // Direct YUV444 output
            let mut out = Vec::with_capacity(y_size * 3);
            out.extend_from_slice(&y_data);
            out.extend_from_slice(&u_data);
            out.extend_from_slice(&v_data);
            out
        }
        PixelFormat::Yuv422p8 => {
            // Downsample to 422
            let (y_out, u_out, v_out) =
                colors::downsample_444_to_422(&y_data, &u_data, &v_data, width, height)?;
            let mut out = Vec::with_capacity(y_out.len() + u_out.len() + v_out.len());
            out.extend_from_slice(&y_out);
            out.extend_from_slice(&u_out);
            out.extend_from_slice(&v_out);
            out
        }
        PixelFormat::Yuv420p8 => {
            // Downsample to 420
            let (y_out, u_out, v_out) =
                colors::downsample_444_to_420(&y_data, &u_data, &v_data, width, height)?;
            let mut out = Vec::with_capacity(y_out.len() + u_out.len() + v_out.len());
            out.extend_from_slice(&y_out);
            out.extend_from_slice(&u_out);
            out.extend_from_slice(&v_out);
            out
        }
        PixelFormat::Rgb8 => {
            // Convert YUV planar to RGB interleaved
            let mut yuv_interleaved = Vec::with_capacity(y_size * 3);
            for i in 0..y_size {
                yuv_interleaved.push(y_data[i]);
                yuv_interleaved.push(u_data[i]);
                yuv_interleaved.push(v_data[i]);
            }
            let mut rgb = vec![0u8; y_size * 3];
            colors::yuv_to_rgb(&yuv_interleaved, &mut rgb, width, height)?;
            rgb
        }
        PixelFormat::Bgr8 => {
            // Convert YUV to RGB then swap R and B
            let mut yuv_interleaved = Vec::with_capacity(y_size * 3);
            for i in 0..y_size {
                yuv_interleaved.push(y_data[i]);
                yuv_interleaved.push(u_data[i]);
                yuv_interleaved.push(v_data[i]);
            }
            let mut rgb = vec![0u8; y_size * 3];
            colors::yuv_to_rgb(&yuv_interleaved, &mut rgb, width, height)?;
            // Convert RGB to BGR in-place by swapping R and B channels
            for i in (0..rgb.len()).step_by(3) {
                rgb.swap(i, i + 2); // Swap R and B channels
            }
            rgb
        }
        PixelFormat::Rgb8Planar => {
            // Convert YUV to RGB planar
            let mut yuv_interleaved = Vec::with_capacity(y_size * 3);
            for i in 0..y_size {
                yuv_interleaved.push(y_data[i]);
                yuv_interleaved.push(u_data[i]);
                yuv_interleaved.push(v_data[i]);
            }
            let mut rgb = vec![0u8; y_size * 3];
            colors::yuv_to_rgb(&yuv_interleaved, &mut rgb, width, height)?;
            // Convert interleaved RGB to planar
            let mut r_plane = Vec::with_capacity(y_size);
            let mut g_plane = Vec::with_capacity(y_size);
            let mut b_plane = Vec::with_capacity(y_size);
            for i in (0..rgb.len()).step_by(3) {
                r_plane.push(rgb[i]);
                g_plane.push(rgb[i + 1]);
                b_plane.push(rgb[i + 2]);
            }
            let mut out = Vec::with_capacity(y_size * 3);
            out.extend_from_slice(&r_plane);
            out.extend_from_slice(&g_plane);
            out.extend_from_slice(&b_plane);
            out
        }
    };

    Ok(ImageOwned8 {
        data,
        width,
        height,
        format: output_format,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{DecoderConfig, EncoderConfig, ImageView8, Level, PixelFormat, Profile};

    #[test]
    fn test_profile_level_combinations() {
        // Test valid combinations
        use crate::profile::validate_profile_level_combination;

        // Light profile valid combinations
        assert!(validate_profile_level_combination(Profile::Light, Level::Level1).is_ok());
        assert!(validate_profile_level_combination(Profile::Light, Level::Level2).is_ok());

        // Light profile invalid combinations
        assert!(validate_profile_level_combination(Profile::Light, Level::Level3).is_err());
        assert!(validate_profile_level_combination(Profile::Light, Level::Level4).is_err());
        assert!(validate_profile_level_combination(Profile::Light, Level::Level5).is_err());

        // Main profile valid combinations
        assert!(validate_profile_level_combination(Profile::Main, Level::Level1).is_ok());
        assert!(validate_profile_level_combination(Profile::Main, Level::Level4).is_ok());

        // Main profile invalid combination
        assert!(validate_profile_level_combination(Profile::Main, Level::Level5).is_err());

        // High profile - all levels valid
        assert!(validate_profile_level_combination(Profile::High, Level::Level1).is_ok());
        assert!(validate_profile_level_combination(Profile::High, Level::Level5).is_ok());
    }

    #[test]
    fn test_encode_with_different_profiles() {
        let width = 64u32;
        let height = 64u32;
        let y_size = (width * height) as usize;
        let uv_size = y_size / 2;

        // Create test image data
        let mut test_data = Vec::with_capacity(y_size + 2 * uv_size);
        for y in 0..height {
            for x in 0..width {
                test_data.push(((x + y) % 256) as u8);
            }
        }
        test_data.extend(std::iter::repeat_n(128, uv_size));
        test_data.extend(std::iter::repeat_n(128, uv_size));

        let input = ImageView8 {
            data: &test_data,
            width,
            height,
            format: PixelFormat::Yuv422p8,
        };

        // Test Light Profile Level 1
        let light_config = EncoderConfig {
            quality: 0.9,
            profile: Profile::Light,
            level: Level::Level1,
        };
        let light_bitstream = encode_frame(input, &light_config).expect("Light encoding failed");
        assert!(!light_bitstream.data.is_empty());

        // Test Main Profile Level 3
        let main_config = EncoderConfig {
            quality: 0.9,
            profile: Profile::Main,
            level: Level::Level3,
        };
        let main_bitstream = encode_frame(input, &main_config).expect("Main encoding failed");
        assert!(!main_bitstream.data.is_empty());

        // Test High Profile Level 5
        let high_config = EncoderConfig {
            quality: 0.9,
            profile: Profile::High,
            level: Level::Level5,
        };
        let high_bitstream = encode_frame(input, &high_config).expect("High encoding failed");
        assert!(!high_bitstream.data.is_empty());
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let width = 64u32;
        let height = 64u32;
        let y_size = (width * height) as usize;
        let uv_size = y_size / 2;

        // Create test image data
        let mut test_data = Vec::with_capacity(y_size + 2 * uv_size);

        // Y plane: gradient
        for y in 0..height {
            for x in 0..width {
                test_data.push(((x + y) % 256) as u8);
            }
        }

        // U plane: constant
        test_data.extend(std::iter::repeat_n(128, uv_size));

        // V plane: constant
        test_data.extend(std::iter::repeat_n(128, uv_size));

        let input = ImageView8 {
            data: &test_data,
            width,
            height,
            format: PixelFormat::Yuv422p8,
        };

        let encoder_config = EncoderConfig {
            quality: 0.9,
            profile: Profile::Main,
            level: Level::Level1,
        };

        let decoder_config = DecoderConfig::default();

        // Test encode
        let bitstream = encode_frame(input, &encoder_config).expect("Encoding failed");
        assert!(!bitstream.data.is_empty(), "Bitstream should not be empty");

        // Test decode - now defaults to YUV444p8
        let decoded = decode_frame(&bitstream, &decoder_config).expect("Decoding failed");
        assert_eq!(decoded.width, width);
        assert_eq!(decoded.height, height);
        assert_eq!(decoded.format, PixelFormat::Yuv444p8);

        // Also test decoding to original format
        let decoded_422 =
            decode_frame_to_format(&bitstream, &decoder_config, PixelFormat::Yuv422p8)
                .expect("Decoding to 422 failed");
        assert_eq!(decoded_422.width, width);
        assert_eq!(decoded_422.height, height);
        assert_eq!(decoded_422.format, PixelFormat::Yuv422p8);

        println!("Roundtrip test completed successfully");
    }

    #[test]
    fn test_dwt_roundtrip() {
        let width = 8u32;
        let height = 8u32;
        let size = (width * height) as usize;

        // Create test signal
        let mut input = vec![0.0f32; size];
        for (i, value) in input.iter_mut().enumerate().take(size) {
            *value = (i as f32).sin();
        }

        let mut forward_output = vec![0.0f32; size];
        let mut inverse_output = vec![0.0f32; size];

        // Forward DWT
        jpegxs_core_clean::dwt::dwt_53_forward_2d(&input, &mut forward_output, width, height)
            .expect("Forward DWT failed");

        // Inverse DWT
        jpegxs_core_clean::dwt::dwt_53_inverse_2d(
            &forward_output,
            &mut inverse_output,
            width,
            height,
        )
        .expect("Inverse DWT failed");

        // Check reconstruction quality
        let mut max_error = 0.0f32;
        for i in 0..size {
            let error = (input[i] - inverse_output[i]).abs();
            max_error = max_error.max(error);
        }

        assert!(
            max_error < 0.1,
            "DWT roundtrip error too large: {}",
            max_error
        );
    }

    #[test]
    fn test_quantization_roundtrip() {
        let coeffs = vec![1.5, -2.3, 0.8, -0.1, 3.7];
        let qp = 2;

        let quantized = quant::quantize(&coeffs, qp).expect("Quantization failed");
        let dequantized = quant::dequantize(&quantized, qp).expect("Dequantization failed");

        assert_eq!(coeffs.len(), dequantized.len());

        // Check that quantization introduces some loss but maintains reasonable fidelity
        for (orig, reconstructed) in coeffs.iter().zip(dequantized.iter()) {
            let error = (orig - reconstructed).abs();
            assert!(error < 5.0, "Quantization error too large: {}", error);
        }
    }
}
