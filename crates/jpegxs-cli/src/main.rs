// Copyright (c) 2024 Keyvan Ebrahimpour. All rights reserved.
//
// This software is proprietary and confidential. Commercial use is prohibited
// without a valid license. See LICENSE file for full terms and conditions.
//
// For commercial licensing: k1.ebrahimpour@gmail.com

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

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
    /// Encode a raw YUV file to JPEG XS format
    Encode {
        /// Input YUV file
        #[arg(short, long)]
        input: String,

        /// Output JPEG XS file
        #[arg(short, long)]
        output: String,

        /// Width
        #[arg(short = 'W', long)]
        width: u32,

        /// Height
        #[arg(short = 'H', long)]
        height: u32,

        /// Pixel format
        #[arg(short, long, default_value = "yuv422p")]
        format: String,

        /// Quality level (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        quality: f32,
    },

    /// Decode a JPEG XS file to raw YUV
    Decode {
        /// Input JPEG XS file
        #[arg(short, long)]
        input: String,

        /// Output YUV file
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
            info!(
                "Resolution: {}x{}, Format: {}, Quality: {}",
                width, height, format, quality
            );

            // Load raw YUV data
            let data = std::fs::read(&input)?;
            let pixel_format = match format.as_str() {
                "yuv422p" => jpegxs_core::types::PixelFormat::Yuv422p8,
                _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
            };

            let image = jpegxs_core::types::ImageView8 {
                data: &data,
                width,
                height,
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
            
            let compression_ratio = data.len() as f32 / bitstream.data.len() as f32;
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
            let config = jpegxs_core::types::DecoderConfig {
                strict_mode: false,
            };

            // Decode
            let decoded_image = jpegxs_core::decode_frame(&bitstream, &config)?;

            // Save raw YUV data
            std::fs::write(&output, &decoded_image.data)?;

            println!(
                "✅ Decoded successfully: {}x{} image saved to {}",
                decoded_image.width, decoded_image.height, output
            );
        }

        Commands::Info { input } => {
            info!("Getting info for {}", input);

            // Load JPEG XS bitstream
            let bitstream_data = std::fs::read(&input)?;

            // Parse headers using clean-room decoder
            let mut decoder = jpegxs_core_clean::JpegXsDecoder::new(bitstream_data.clone())
                .map_err(|e| anyhow::anyhow!("Decoder error: {}", e))?;
            decoder.parse_headers()
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