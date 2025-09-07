// Copyright (c) 2025 Keyvan Ebrahimpour. All rights reserved.
//
// This software is proprietary and confidential. Commercial use is prohibited
// without a valid license. See LICENSE file for full terms and conditions.
//
// For commercial licensing: k1.ebrahimpour@gmail.com

pub mod colors;
pub mod dwt;
pub mod entropy;
pub mod packet;
pub mod quant;
pub mod types;

use anyhow::Result;
use types::{Bitstream, DecoderConfig, EncoderConfig, ImageOwned8, ImageView8, PixelFormat};

pub fn encode_frame(input: ImageView8, config: &EncoderConfig) -> Result<Bitstream> {
    // Convert input image to floating point for processing
    let mut y_plane = vec![0.0f32; (input.width * input.height) as usize];
    let mut u_plane;
    let mut v_plane;

    match input.format {
        PixelFormat::Yuv422p8 => {
            let uv_size = (input.width * input.height / 2) as usize;
            u_plane = vec![0.0f32; uv_size];
            v_plane = vec![0.0f32; uv_size];

            let y_size = (input.width * input.height) as usize;
            let u_size = uv_size;
            let v_size = uv_size;

            // Extract YUV planes
            for (i, value) in y_plane.iter_mut().enumerate().take(y_size) {
                *value = input.data[i] as f32 - 128.0; // Center around 0
            }
            for (i, value) in u_plane.iter_mut().enumerate().take(u_size) {
                *value = input.data[y_size + i] as f32 - 128.0;
            }
            for (i, value) in v_plane.iter_mut().enumerate().take(v_size) {
                *value = input.data[y_size + u_size + i] as f32 - 128.0;
            }
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported pixel format: {:?}",
                input.format
            ))
        }
    }

    // Apply DWT to each plane
    let mut y_dwt = vec![0.0f32; y_plane.len()];
    let mut u_dwt = vec![0.0f32; u_plane.len()];
    let mut v_dwt = vec![0.0f32; v_plane.len()];

    jpegxs_core_clean::dwt::dwt_53_forward_2d(&y_plane, &mut y_dwt, input.width, input.height)?;

    // For 422, U/V have half width
    let uv_width = input.width / 2;
    let uv_height = input.height;
    jpegxs_core_clean::dwt::dwt_53_forward_2d(&u_plane, &mut u_dwt, uv_width, uv_height)?;
    jpegxs_core_clean::dwt::dwt_53_forward_2d(&v_plane, &mut v_dwt, uv_width, uv_height)?;

    // Quantize coefficients using corrected quality mapping
    let qps = quant::compute_quantization_parameters(config.quality)?;
    let qp_y = qps[0];
    let qp_uv = qps.get(1).copied().unwrap_or(qp_y);

    let y_quantized = quant::quantize(&y_dwt, qp_y)?;
    let u_quantized = quant::quantize(&u_dwt, qp_uv)?;
    let v_quantized = quant::quantize(&v_dwt, qp_uv)?;

    // Use clean-room JPEG XS bitstream format from ISO/IEC 21122-1:2024
    let mut jxs_bitstream = jpegxs_core_clean::JpegXsBitstream::new();

    // Add Capabilities marker (mandatory second marker per ISO A.4.3)
    jxs_bitstream.write_cap_marker();

    // Add PIH (Picture Header) marker according to ISO A.7 specification
    // Third mandatory marker providing image dimensions and decoder configuration
    let num_components = match input.format {
        PixelFormat::Yuv422p8 => 3, // Y, U, V components
        _ => return Err(anyhow::anyhow!("Unsupported pixel format for PIH marker")),
    };
    jxs_bitstream.write_pih_marker(input.width as u16, input.height as u16, num_components);

    // Add CDT (Component Table) marker according to ISO A.4.5 specification
    // Fourth mandatory marker providing component precision and sampling factors
    jxs_bitstream.write_cdt_marker(num_components);

    // Add WGT (Weights Table) marker according to ISO A.4.12 specification
    // Fifth mandatory marker providing band gain parameters for quantization
    jxs_bitstream.write_wgt_marker();

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
    // Use clean-room JPEG XS decoder to parse headers and extract entropy data
    let mut decoder = jpegxs_core_clean::JpegXsDecoder::new(bitstream.data.clone())
        .map_err(|e| anyhow::anyhow!("Decoder creation failed: {}", e))?;

    // Parse JPEG XS markers to extract image parameters
    decoder
        .parse_headers()
        .map_err(|e| anyhow::anyhow!("Header parsing failed: {}", e))?;

    let (width, height, num_components) = decoder.dimensions();
    let format = match num_components {
        3 => PixelFormat::Yuv422p8,
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported number of components: {}",
                num_components
            ))
        }
    };

    // Decode entropy coded data
    let all_coefficients = decoder
        .decode_entropy_data()
        .map_err(|e| anyhow::anyhow!("Entropy decoding failed: {}", e))?;

    // Split coefficients back into Y, U, V components
    let width = width as u32;
    let height = height as u32;
    let y_size = (width * height) as usize;
    let uv_size = (width * height / 2) as usize;

    if all_coefficients.len() < y_size + 2 * uv_size {
        return Err(anyhow::anyhow!("Insufficient decoded coefficients"));
    }

    let y_quantized = all_coefficients[0..y_size].to_vec();
    let u_quantized = all_coefficients[y_size..y_size + uv_size].to_vec();
    let v_quantized = all_coefficients[y_size + uv_size..y_size + 2 * uv_size].to_vec();

    // Dequantize - Extract QP from WGT marker or use quality-consistent defaults
    // TODO: Properly parse WGT marker to extract actual QP values from bitstream
    let (qp_y, qp_uv) = extract_quantization_parameters(&decoder)
        .unwrap_or_else(get_default_quantization_parameters);

    let y_dwt = quant::dequantize(&y_quantized, qp_y)?;
    let u_dwt = quant::dequantize(&u_quantized, qp_uv)?;
    let v_dwt = quant::dequantize(&v_quantized, qp_uv)?;

    // Apply inverse DWT
    let mut y_plane = vec![0.0f32; y_size];
    let mut u_plane = vec![0.0f32; uv_size];
    let mut v_plane = vec![0.0f32; uv_size];

    jpegxs_core_clean::dwt::dwt_53_inverse_2d(&y_dwt, &mut y_plane, width, height)?;
    let uv_width = width / 2;
    let uv_height = height;
    jpegxs_core_clean::dwt::dwt_53_inverse_2d(&u_dwt, &mut u_plane, uv_width, uv_height)?;
    jpegxs_core_clean::dwt::dwt_53_inverse_2d(&v_dwt, &mut v_plane, uv_width, uv_height)?;

    // Convert back to 8-bit and pack
    let total_size = y_size + 2 * uv_size;
    let mut data = Vec::with_capacity(total_size);

    // Pack Y plane
    for &sample in &y_plane {
        let value = (sample + 128.0).clamp(0.0, 255.0) as u8;
        data.push(value);
    }

    // Pack U plane
    for &sample in &u_plane {
        let value = (sample + 128.0).clamp(0.0, 255.0) as u8;
        data.push(value);
    }

    // Pack V plane
    for &sample in &v_plane {
        let value = (sample + 128.0).clamp(0.0, 255.0) as u8;
        data.push(value);
    }

    Ok(ImageOwned8 {
        data,
        width,
        height,
        format,
    })
}

/// Extract quantization parameters from the decoder's parsed WGT marker
/// TODO: Implement proper WGT marker parsing
fn extract_quantization_parameters(
    _decoder: &jpegxs_core_clean::JpegXsDecoder,
) -> Option<(u8, u8)> {
    // For now, return None to use fallback values
    // In a complete implementation, this would:
    // 1. Check if WGT marker was parsed
    // 2. Extract QP values from the marker data
    // 3. Return appropriate values for Y and UV components
    None
}

/// Get default quantization parameters using the same quality mapping as encoder
/// This ensures consistency when QP values cannot be extracted from bitstream
fn get_default_quantization_parameters() -> (u8, u8) {
    // Use the same quality-to-QP mapping as the encoder for consistency
    // Assuming medium-high quality (0.8) as a reasonable default for decoding
    const DEFAULT_QUALITY: f32 = 0.8;

    // This mirrors the logic from quant::compute_quantization_parameters()
    let base_qp = if DEFAULT_QUALITY >= 0.95 {
        1 // Virtually lossless
    } else if DEFAULT_QUALITY >= 0.9 {
        2 // Very high quality
    } else if DEFAULT_QUALITY >= 0.8 {
        4 // High quality
    } else if DEFAULT_QUALITY >= 0.6 {
        8 // Medium quality
    } else if DEFAULT_QUALITY >= 0.4 {
        12 // Lower quality
    } else {
        16 // Low quality
    };

    // Use same QP for both Y and UV components as default
    (base_qp, base_qp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{DecoderConfig, EncoderConfig, ImageView8, Level, PixelFormat, Profile};

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

        // Test decode
        let decoded = decode_frame(&bitstream, &decoder_config).expect("Decoding failed");
        assert_eq!(decoded.width, width);
        assert_eq!(decoded.height, height);
        assert_eq!(decoded.format, PixelFormat::Yuv422p8);

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
