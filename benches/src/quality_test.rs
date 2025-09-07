// Copyright (c) 2025 Keyvan Ebrahimpour. All rights reserved.

use anyhow::Result;
use clap::Parser;
use image::DynamicImage;
use jpegxs_benchmarks::{rgb_to_yuv422p, yuv422p_to_rgb};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "quality_test")]
#[command(about = "Test JPEG XS quality parameter mapping")]
struct Args {
    /// Input image to test
    #[arg(short, long, default_value = "test_images/gradient_512x512.png")]
    input: PathBuf,

    /// Output directory for results
    #[arg(short, long, default_value = "quality_test_results")]
    output_dir: PathBuf,

    /// Quality levels to test (space separated)
    #[arg(short, long, default_values = ["0.1", "0.3", "0.5", "0.7", "0.9", "0.95"])]
    quality_levels: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QualityTestResult {
    quality: f32,
    quantization_parameter: u8,
    original_size_kb: f64,
    compressed_size_kb: f64,
    compression_ratio: f64,
    encode_time_ms: f64,
    decode_time_ms: f64,
    file_size_reduction_percent: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🔧 JPEG XS Quality Parameter Testing");
    println!("====================================");

    // Create output directory
    fs::create_dir_all(&args.output_dir)?;

    // Load test image
    if !args.input.exists() {
        return Err(anyhow::anyhow!(
            "Input image not found: {}",
            args.input.display()
        ));
    }

    let original_img = image::open(&args.input)?;
    let original_size = fs::metadata(&args.input)?.len() as f64 / 1024.0;

    println!(
        "📁 Test image: {} ({:.1} KB)",
        args.input.display(),
        original_size
    );
    println!("📊 Testing {} quality levels", args.quality_levels.len());

    let mut results = Vec::new();

    for (i, &quality) in args.quality_levels.iter().enumerate() {
        println!(
            "\n🎯 Testing Quality {:.2} ({}/{})",
            quality,
            i + 1,
            args.quality_levels.len()
        );

        let result = test_quality_level(&original_img, quality, original_size)?;

        println!(
            "   QP: {} | Size: {:.1} KB | Ratio: {:.1}:1 | Time: {:.2}ms",
            result.quantization_parameter,
            result.compressed_size_kb,
            result.compression_ratio,
            result.encode_time_ms
        );

        results.push(result);
    }

    // Save results
    save_results(&results, &args.output_dir)?;

    // Print summary
    print_quality_analysis(&results);

    println!("\n📊 Results saved to: {}", args.output_dir.display());

    Ok(())
}

fn test_quality_level(
    img: &DynamicImage,
    quality: f32,
    original_size_kb: f64,
) -> Result<QualityTestResult> {
    // Get quantization parameter that would be used
    let qps = jpegxs_core::quant::compute_quantization_parameters(quality)?;
    let qp = qps[0];

    // Encode with this quality
    let encode_start = Instant::now();
    let compressed = encode_jpegxs(img, quality)?;
    let encode_time = encode_start.elapsed().as_secs_f64() * 1000.0;

    // Decode to verify roundtrip
    let decode_start = Instant::now();
    let _decoded = decode_jpegxs(&compressed)?;
    let decode_time = decode_start.elapsed().as_secs_f64() * 1000.0;

    let compressed_size_kb = compressed.len() as f64 / 1024.0;
    let compression_ratio = original_size_kb / compressed_size_kb;
    let size_reduction = ((original_size_kb - compressed_size_kb) / original_size_kb) * 100.0;

    Ok(QualityTestResult {
        quality,
        quantization_parameter: qp,
        original_size_kb,
        compressed_size_kb,
        compression_ratio,
        encode_time_ms: encode_time,
        decode_time_ms: decode_time,
        file_size_reduction_percent: size_reduction,
    })
}

fn encode_jpegxs(img: &DynamicImage, quality: f32) -> Result<Vec<u8>> {
    // Convert to RGB and then to YUV422p
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    // Convert RGB to YUV422p
    let yuv_data = rgb_to_yuv422p(rgb_img.as_raw(), width, height);

    let image_view = jpegxs_core::types::ImageView8 {
        data: &yuv_data,
        width,
        height,
        format: jpegxs_core::types::PixelFormat::Yuv422p8,
    };

    let config = jpegxs_core::types::EncoderConfig {
        quality,
        profile: jpegxs_core::types::Profile::Main,
        level: jpegxs_core::types::Level::Level1,
    };

    let bitstream = jpegxs_core::encode_frame(image_view, &config)?;
    Ok(bitstream.data)
}

fn decode_jpegxs(data: &[u8]) -> Result<DynamicImage> {
    let bitstream = jpegxs_core::types::Bitstream {
        data: data.to_vec(),
        size_bits: data.len() * 8,
    };

    let config = jpegxs_core::types::DecoderConfig { strict_mode: false };
    let decoded = jpegxs_core::decode_frame(&bitstream, &config)?;

    // Convert YUV back to RGB
    let rgb_data = yuv422p_to_rgb(&decoded.data, decoded.width, decoded.height);

    let img_buffer = image::ImageBuffer::from_raw(decoded.width, decoded.height, rgb_data)
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;

    Ok(DynamicImage::ImageRgb8(img_buffer))
}

fn save_results(results: &[QualityTestResult], output_dir: &Path) -> Result<()> {
    // Save JSON results
    let json_path = output_dir.join("quality_test_results.json");
    let json_content = serde_json::to_string_pretty(results)?;
    fs::write(&json_path, json_content)?;

    // Save CSV results
    let csv_path = output_dir.join("quality_test_results.csv");
    let mut wtr = csv::Writer::from_path(&csv_path)?;
    for result in results {
        wtr.serialize(result)?;
    }
    wtr.flush()?;

    // Save markdown report
    let md_path = output_dir.join("QUALITY_TEST_REPORT.md");
    let md_content = generate_markdown_report(results)?;
    fs::write(&md_path, md_content)?;

    println!("   ✓ Saved JSON: quality_test_results.json");
    println!("   ✓ Saved CSV: quality_test_results.csv");
    println!("   ✓ Saved Report: QUALITY_TEST_REPORT.md");

    Ok(())
}

fn generate_markdown_report(results: &[QualityTestResult]) -> Result<String> {
    let mut md = String::new();

    md.push_str("# JPEG XS Quality Parameter Test Report\n\n");
    md.push_str(&format!(
        "**Generated**: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Quality Levels Tested**: {}\n\n", results.len()));

    md.push_str("## Quality vs Compression Analysis\n\n");
    md.push_str(
        "| Quality | QP | Original (KB) | Compressed (KB) | Ratio | Reduction % | Encode (ms) |\n",
    );
    md.push_str(
        "|---------|----|--------------:|----------------:|------:|------------:|------------:|\n",
    );

    for result in results {
        md.push_str(&format!(
            "| {:.2} | {} | {:.1} | {:.1} | {:.1}:1 | {:.1}% | {:.2} |\n",
            result.quality,
            result.quantization_parameter,
            result.original_size_kb,
            result.compressed_size_kb,
            result.compression_ratio,
            result.file_size_reduction_percent,
            result.encode_time_ms
        ));
    }

    md.push_str("\n## Analysis\n\n");

    let best_compression = results
        .iter()
        .max_by(|a, b| {
            a.compression_ratio
                .partial_cmp(&b.compression_ratio)
                .unwrap()
        })
        .unwrap();

    let fastest_encode = results
        .iter()
        .min_by(|a, b| a.encode_time_ms.partial_cmp(&b.encode_time_ms).unwrap())
        .unwrap();

    md.push_str(&format!(
        "- **Best Compression**: Quality {:.2} achieved {:.1}:1 ratio\n",
        best_compression.quality, best_compression.compression_ratio
    ));
    md.push_str(&format!(
        "- **Fastest Encoding**: Quality {:.2} at {:.2}ms\n",
        fastest_encode.quality, fastest_encode.encode_time_ms
    ));

    // Check if compression improves with lower quality
    let improvements = results
        .windows(2)
        .filter(|pair| {
            pair[0].quality > pair[1].quality
                && pair[1].compression_ratio > pair[0].compression_ratio
        })
        .count();

    if improvements > 0 {
        md.push_str(
            "- **Quality Mapping**: ✅ Working correctly (lower quality = better compression)\n",
        );
    } else {
        md.push_str("- **Quality Mapping**: ⚠️ May need further tuning\n");
    }

    Ok(md)
}

fn print_quality_analysis(results: &[QualityTestResult]) {
    println!("\n📊 QUALITY PARAMETER ANALYSIS");
    println!("=============================");

    println!("\n🎯 Compression Performance:");
    for result in results {
        let status = if result.compression_ratio >= 2.0 {
            "✅ Good"
        } else if result.compression_ratio >= 1.5 {
            "⚠️ Moderate"
        } else {
            "🔴 Poor"
        };

        println!(
            "   Quality {:.2} (QP {}): {:.1}:1 ratio - {}",
            result.quality, result.quantization_parameter, result.compression_ratio, status
        );
    }

    // Check for proper quality scaling
    let mut compression_improves = true;
    for i in 1..results.len() {
        if results[i].quality < results[i - 1].quality
            && results[i].compression_ratio <= results[i - 1].compression_ratio
        {
            compression_improves = false;
            break;
        }
    }

    println!("\n🔍 Quality Mapping Assessment:");
    if compression_improves {
        println!("   ✅ Quality parameter mapping appears to be working correctly");
        println!("   ✅ Lower quality settings achieve better compression ratios");
    } else {
        println!("   ⚠️ Quality parameter mapping may need further adjustment");
        println!("   ⚠️ Some quality levels don't follow expected compression pattern");
    }

    let avg_ratio: f64 =
        results.iter().map(|r| r.compression_ratio).sum::<f64>() / results.len() as f64;
    println!("   📈 Average compression ratio: {:.1}:1", avg_ratio);

    if avg_ratio >= 3.0 {
        println!("   🎉 Overall compression performance: Excellent!");
    } else if avg_ratio >= 2.0 {
        println!("   👍 Overall compression performance: Good");
    } else {
        println!("   ⚠️ Overall compression performance: Needs improvement");
    }
}
