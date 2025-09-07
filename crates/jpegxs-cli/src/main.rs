// Copyright (c) 2024 Keyvan Ebrahimpour. All rights reserved.
//
// This software is proprietary and confidential. Commercial use is prohibited
// without a valid license. See LICENSE file for full terms and conditions.
//
// For commercial licensing: k1.ebrahimpour@gmail.com

use anyhow::Result;
use clap::{Parser, Subcommand};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use log::info;
use std::path::Path;

#[derive(Parser)]
#[command(name = "jpegxs")]
#[command(about = "JPEG XS encoder/decoder CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode an image file (PNG, JPEG, or raw YUV) to JPEG XS format
    Encode {
        /// Input file (PNG, JPEG, or YUV)
        #[arg(short, long)]
        input: String,

        /// Output JPEG XS file
        #[arg(short, long)]
        output: String,

        /// Width (required for YUV files)
        #[arg(short = 'W', long)]
        width: Option<u32>,

        /// Height (required for YUV files)
        #[arg(short = 'H', long)]
        height: Option<u32>,

        /// Pixel format (for YUV files)
        #[arg(short, long, default_value = "yuv422p")]
        format: String,

        /// Quality level (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        quality: f32,
    },

    /// Decode a JPEG XS file to image format (PNG, JPEG, or raw YUV)
    Decode {
        /// Input JPEG XS file
        #[arg(short, long)]
        input: String,

        /// Output file (PNG, JPEG, or YUV)
        #[arg(short, long)]
        output: String,
    },

    /// Get information about a JPEG XS file
    Info {
        /// Input JPEG XS file
        #[arg(short, long)]
        input: String,
    },
}

fn detect_image_format(path: &str) -> Result<Option<ImageFormat>> {
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        Some("png") => Ok(Some(ImageFormat::Png)),
        Some("jpg") | Some("jpeg") => Ok(Some(ImageFormat::Jpeg)),
        Some("yuv") | Some("raw") => Ok(None), // Raw format
        _ => Err(anyhow::anyhow!("Unsupported file format: {:?}", extension)),
    }
}

fn rgb_to_yuv422p(rgb_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let pixels = width as usize * height as usize;
    let mut yuv_data = Vec::with_capacity(pixels * 2); // YUV422p uses 2 bytes per pixel on average

    // Y plane (full resolution)
    let mut y_plane = Vec::with_capacity(pixels);
    // U and V planes (half horizontal resolution)
    let mut u_plane = Vec::with_capacity(pixels / 2);
    let mut v_plane = Vec::with_capacity(pixels / 2);

    for y in 0..height as usize {
        for x in (0..width as usize).step_by(2) {
            // Process two pixels at a time for 422 subsampling
            let idx1 = (y * width as usize + x) * 3;
            let idx2 = if x + 1 < width as usize {
                (y * width as usize + x + 1) * 3
            } else {
                idx1 // Use same pixel if odd width
            };

            // RGB to YUV conversion (ITU-R BT.601)
            let r1 = rgb_data[idx1] as f32;
            let g1 = rgb_data[idx1 + 1] as f32;
            let b1 = rgb_data[idx1 + 2] as f32;

            let r2 = rgb_data[idx2] as f32;
            let g2 = rgb_data[idx2 + 1] as f32;
            let b2 = rgb_data[idx2 + 2] as f32;

            // Y values (luma)
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

            // U and V values (chroma) - average of two pixels for 422 subsampling
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

    // Pack YUV422p format: Y plane, then U plane, then V plane
    yuv_data.extend_from_slice(&y_plane);
    yuv_data.extend_from_slice(&u_plane);
    yuv_data.extend_from_slice(&v_plane);

    yuv_data
}

fn yuv422p_to_rgb(yuv_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let pixels = width as usize * height as usize;
    let mut rgb_data = Vec::with_capacity(pixels * 3);

    let y_plane = &yuv_data[0..pixels];
    let u_plane = &yuv_data[pixels..pixels + pixels / 2];
    let v_plane = &yuv_data[pixels + pixels / 2..pixels + pixels];

    for y in 0..height as usize {
        for x in 0..width as usize {
            let y_val = y_plane[y * width as usize + x] as f32;
            let u_val = u_plane[y * (width as usize / 2) + x / 2] as f32 - 128.0;
            let v_val = v_plane[y * (width as usize / 2) + x / 2] as f32 - 128.0;

            // YUV to RGB conversion (ITU-R BT.601)
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    match cli.command {
        Commands::Encode {
            input,
            output,
            width,
            height,
            format,
            quality,
        } => {
            info!("Encoding {} to {}", input, output);

            let image_format = detect_image_format(&input)?;
            let (yuv_data, actual_width, actual_height) = match image_format {
                Some(format) => {
                    // Load image file (PNG/JPEG)
                    let img = image::open(&input)?;
                    let rgb_img = img.to_rgb8();
                    let (w, h) = rgb_img.dimensions();

                    info!("Loaded {}x{} {:?} image", w, h, format);

                    // Convert RGB to YUV422p
                    let yuv_data = rgb_to_yuv422p(rgb_img.as_raw(), w, h);
                    (yuv_data, w, h)
                }
                None => {
                    // Raw YUV file - require width and height
                    let width =
                        width.ok_or_else(|| anyhow::anyhow!("Width required for YUV files"))?;
                    let height =
                        height.ok_or_else(|| anyhow::anyhow!("Height required for YUV files"))?;

                    let data = std::fs::read(&input)?;
                    info!("Loaded {}x{} YUV file", width, height);
                    (data, width, height)
                }
            };

            info!(
                "Resolution: {}x{}, Format: {}, Quality: {}",
                actual_width, actual_height, format, quality
            );

            let pixel_format = match format.as_str() {
                "yuv422p" => jpegxs_core::types::PixelFormat::Yuv422p8,
                _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
            };

            let image = jpegxs_core::types::ImageView8 {
                data: &yuv_data,
                width: actual_width,
                height: actual_height,
                format: pixel_format,
            };

            // Configure encoder
            let config = jpegxs_core::types::EncoderConfig {
                quality,
                profile: jpegxs_core::types::Profile::Main,
                level: jpegxs_core::types::Level::Level1,
            };

            // Encode
            let bitstream = jpegxs_core::encode_frame(image, &config)?;
            std::fs::write(&output, &bitstream.data)?;

            let compression_ratio = yuv_data.len() as f32 / bitstream.data.len() as f32;
            println!(
                "✅ Encoded successfully: {} bytes (compression ratio: {:.1}:1)",
                bitstream.data.len(),
                compression_ratio
            );
        }

        Commands::Decode { input, output } => {
            info!("Decoding {} to {}", input, output);

            // Load JPEG XS bitstream
            let bitstream_data = std::fs::read(&input)?;
            let bitstream = jpegxs_core::types::Bitstream {
                data: bitstream_data.clone(),
                size_bits: bitstream_data.len() * 8,
            };

            // Configure decoder
            let config = jpegxs_core::types::DecoderConfig { strict_mode: false };

            // Decode
            let decoded_image = jpegxs_core::decode_frame(&bitstream, &config)?;

            // Detect output format
            let output_format = detect_image_format(&output)?;

            match output_format {
                Some(ImageFormat::Png) | Some(ImageFormat::Jpeg) => {
                    // Convert YUV to RGB
                    let rgb_data = yuv422p_to_rgb(
                        &decoded_image.data,
                        decoded_image.width,
                        decoded_image.height,
                    );

                    // Create RGB image
                    let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                        ImageBuffer::from_raw(decoded_image.width, decoded_image.height, rgb_data)
                            .ok_or_else(|| anyhow::anyhow!("Failed to create RGB image buffer"))?;

                    let dynamic_image = DynamicImage::ImageRgb8(rgb_image);

                    // Save as PNG or JPEG
                    match output_format.unwrap() {
                        ImageFormat::Png => {
                            dynamic_image.save_with_format(&output, ImageFormat::Png)?
                        }
                        ImageFormat::Jpeg => {
                            dynamic_image.save_with_format(&output, ImageFormat::Jpeg)?
                        }
                        _ => unreachable!(),
                    }

                    println!(
                        "✅ Decoded successfully: {}x{} image saved as {} to {}",
                        decoded_image.width,
                        decoded_image.height,
                        match output_format.unwrap() {
                            ImageFormat::Png => "PNG",
                            ImageFormat::Jpeg => "JPEG",
                            _ => unreachable!(),
                        },
                        output
                    );
                }
                None => {
                    // Save raw YUV data
                    std::fs::write(&output, &decoded_image.data)?;
                    println!(
                        "✅ Decoded successfully: {}x{} YUV image saved to {}",
                        decoded_image.width, decoded_image.height, output
                    );
                }
                Some(_) => {
                    return Err(anyhow::anyhow!("Unsupported output format"));
                }
            }
        }

        Commands::Info { input } => {
            info!("Getting info for {}", input);

            // Load JPEG XS bitstream
            let bitstream_data = std::fs::read(&input)?;

            // Parse headers using clean-room decoder
            let mut decoder = jpegxs_core_clean::JpegXsDecoder::new(bitstream_data.clone())
                .map_err(|e| anyhow::anyhow!("Decoder error: {}", e))?;
            decoder
                .parse_headers()
                .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

            let (width, height, num_components) = decoder.dimensions();

            println!("JPEG XS File Information:");
            println!("========================");
            println!("File: {}", input);
            println!("Size: {} bytes", bitstream_data.len());
            println!("Resolution: {}x{}", width, height);
            println!("Components: {}", num_components);

            // Check for markers
            println!("\nMarkers found:");
            let markers = [
                (0xff10u16, "SOC - Start of Codestream"),
                (0xff50, "CAP - Capabilities"),
                (0xff12, "PIH - Picture Header"),
                (0xff13, "CDT - Component Table"),
                (0xff14, "WGT - Weights Table"),
                (0xff11, "EOC - End of Codestream"),
            ];

            for (marker_code, name) in &markers {
                let marker_bytes = marker_code.to_be_bytes();
                if bitstream_data
                    .windows(2)
                    .any(|w| w[0] == marker_bytes[0] && w[1] == marker_bytes[1])
                {
                    println!("  ✓ 0x{:04x} - {}", marker_code, name);
                }
            }

            let uncompressed_size = (width as usize * height as usize * 3 * 8) / 8;
            let compression_ratio = uncompressed_size as f32 / bitstream_data.len() as f32;
            println!("\nCompression ratio: {:.1}:1", compression_ratio);
        }
    }

    Ok(())
}
