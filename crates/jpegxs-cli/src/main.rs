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
    Encode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,

        #[arg(short = 'W', long)]
        width: u32,

        #[arg(short = 'H', long)]
        height: u32,

        #[arg(short, long, default_value = "yuv422p")]
        format: String,

        #[arg(short, long, default_value = "0.9")]
        quality: f32,
    },

    Decode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },

    Validate {
        #[arg(short, long)]
        input: String,

        #[arg(long, default_value = "jxs")]
        reference: String,
    },

    Info {
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

            // Load input image
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
            match jpegxs_core::encode_frame(image, &config) {
                Ok(bitstream) => {
                    std::fs::write(&output, &bitstream.data)?;
                    println!("Encoded successfully: {} bytes", bitstream.data.len());
                }
                Err(e) => {
                    eprintln!("Encoding failed: {}", e);
                    return Err(e);
                }
            }
        }

        Commands::Decode { input, output } => {
            info!("Decoding {} to {}", input, output);
            println!("Decoding not yet implemented");
        }

        Commands::Validate { input, reference } => {
            info!("Validating {} against {}", input, reference);
            println!("Validation not yet implemented");
        }

        Commands::Info { input } => {
            info!("Getting info for {}", input);
            println!("Info not yet implemented");
        }
    }

    Ok(())
}
