// Copyright (c) 2025 Keyvan Ebrahimpour. All rights reserved.

use anyhow::Result;
use clap::Parser;
use image::{DynamicImage, ImageFormat};
use jpegxs_benchmarks::{rgb_to_yuv422p, yuv422p_to_rgb};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "simple_benchmark")]
#[command(about = "Simple JPEG XS performance benchmark")]
struct Args {
    /// Input directory containing test images
    #[arg(short, long, default_value = "testing/fixtures")]
    input_dir: PathBuf,

    /// Output directory for results
    #[arg(short, long, default_value = "benchmark_results")]
    output_dir: PathBuf,

    /// Quality level to test (0.1 to 1.0)
    #[arg(short, long, default_value = "0.9")]
    quality: f32,

    /// Number of iterations for timing
    #[arg(long, default_value = "3")]
    iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchmarkResult {
    image_name: String,
    original_size_kb: f64,

    // JPEG XS results
    jpegxs_compressed_size_kb: f64,
    jpegxs_compression_ratio: f64,
    jpegxs_encode_time_ms: f64,
    jpegxs_decode_time_ms: f64,
    jpegxs_encode_throughput_mbps: f64,
    jpegxs_decode_throughput_mbps: f64,

    // JPEG results (for comparison)
    jpeg_compressed_size_kb: f64,
    jpeg_compression_ratio: f64,
    jpeg_encode_time_ms: f64,
    jpeg_decode_time_ms: f64,

    // PNG results (for comparison)
    png_compressed_size_kb: f64,
    png_compression_ratio: f64,
    png_encode_time_ms: f64,
    png_decode_time_ms: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 JPEG XS Simple Performance Benchmark");
    println!("=====================================");
    println!("Quality level: {}", args.quality);
    println!("Iterations: {}", args.iterations);

    // Create output directory
    fs::create_dir_all(&args.output_dir)?;

    // Load test images
    let test_images = load_test_images(&args.input_dir)?;
    println!("📁 Found {} test images", test_images.len());

    let test_images = if test_images.is_empty() {
        println!("❌ No test images found. Creating sample images...");
        create_sample_images(&args.input_dir)?;
        load_test_images(&args.input_dir)? // Load again after creating
    } else {
        test_images
    };

    let mut results = Vec::new();

    for (i, image_path) in test_images.iter().enumerate() {
        println!(
            "\n🖼️  Testing {}/{}: {}",
            i + 1,
            test_images.len(),
            image_path.file_name().unwrap().to_str().unwrap()
        );

        let result = benchmark_image(image_path, args.quality, args.iterations)?;
        results.push(result);

        // Print immediate results
        let r = &results[results.len() - 1];
        println!(
            "   JPEG XS: {:.1}:1 compression, {:.2}ms encode, {:.1} MB/s",
            r.jpegxs_compression_ratio, r.jpegxs_encode_time_ms, r.jpegxs_encode_throughput_mbps
        );
        println!(
            "   JPEG:    {:.1}:1 compression, {:.2}ms encode",
            r.jpeg_compression_ratio, r.jpeg_encode_time_ms
        );
    }

    // Generate summary
    print_summary(&results);

    // Save results
    save_results(&results, &args.output_dir)?;

    println!("\n📊 Results saved to: {}", args.output_dir.display());

    Ok(())
}

fn load_test_images(input_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    if !input_dir.exists() {
        return Ok(images);
    }

    for entry in fs::read_dir(input_dir)? {
        let path = entry?.path();
        if let Some(ext) = path.extension() {
            if matches!(ext.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
                images.push(path);
            }
        }
    }

    Ok(images)
}

fn create_sample_images(dir: &Path) -> Result<()> {
    use image::{ImageBuffer, Rgb};

    println!("🎨 Creating sample test images...");
    fs::create_dir_all(dir)?;

    // Create a gradient image
    let gradient = ImageBuffer::from_fn(512, 512, |x, y| {
        let r = (x * 255 / 512) as u8;
        let g = (y * 255 / 512) as u8;
        let b = ((x + y) * 255 / 1024) as u8;
        Rgb([r, g, b])
    });

    let gradient_img = DynamicImage::ImageRgb8(gradient);
    gradient_img.save(dir.join("gradient_512x512.png"))?;

    // Create a photo-realistic pattern
    let photo = ImageBuffer::from_fn(640, 480, |x, y| {
        let fx = x as f32 / 640.0;
        let fy = y as f32 / 480.0;

        if fy < 0.4 {
            // Sky
            let blue_intensity = (180.0 + fy * 75.0) as u8;
            Rgb([100, 149, blue_intensity])
        } else {
            // Ground with texture
            let green_base = 40 + (fx * 60.0) as u8;
            let noise = ((x * 7 + y * 11) % 32) as u8;
            Rgb([green_base, 90 + noise, green_base / 2])
        }
    });

    let photo_img = DynamicImage::ImageRgb8(photo);
    photo_img.save(dir.join("landscape_640x480.png"))?;

    println!("   ✓ Created gradient_512x512.png");
    println!("   ✓ Created landscape_640x480.png");

    Ok(())
}

fn benchmark_image(image_path: &Path, quality: f32, iterations: usize) -> Result<BenchmarkResult> {
    // Load original image
    let original_img = image::open(image_path)?;
    let original_size = fs::metadata(image_path)?.len() as f64 / 1024.0; // KB

    // Benchmark JPEG XS
    println!("   🔧 Testing JPEG XS...");
    let jpegxs_results = benchmark_jpegxs(&original_img, quality, iterations)?;

    // Benchmark JPEG
    println!("   🔧 Testing JPEG...");
    let jpeg_results = benchmark_jpeg(&original_img, quality, iterations)?;

    // Benchmark PNG
    println!("   🔧 Testing PNG...");
    let png_results = benchmark_png(&original_img, iterations)?;

    Ok(BenchmarkResult {
        image_name: image_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        original_size_kb: original_size,

        jpegxs_compressed_size_kb: jpegxs_results.0,
        jpegxs_compression_ratio: original_size / jpegxs_results.0,
        jpegxs_encode_time_ms: jpegxs_results.1,
        jpegxs_decode_time_ms: jpegxs_results.2,
        jpegxs_encode_throughput_mbps: (original_size / 1024.0) / (jpegxs_results.1 / 1000.0),
        jpegxs_decode_throughput_mbps: (original_size / 1024.0) / (jpegxs_results.2 / 1000.0),

        jpeg_compressed_size_kb: jpeg_results.0,
        jpeg_compression_ratio: original_size / jpeg_results.0,
        jpeg_encode_time_ms: jpeg_results.1,
        jpeg_decode_time_ms: jpeg_results.2,

        png_compressed_size_kb: png_results.0,
        png_compression_ratio: original_size / png_results.0,
        png_encode_time_ms: png_results.1,
        png_decode_time_ms: png_results.2,
    })
}

fn benchmark_jpegxs(
    img: &DynamicImage,
    quality: f32,
    iterations: usize,
) -> Result<(f64, f64, f64)> {
    let mut encode_times = Vec::new();
    let mut decode_times = Vec::new();
    let mut compressed_size = 0;

    for _ in 0..iterations {
        // Encode
        let encode_start = Instant::now();
        let compressed = encode_jpegxs(img, quality)?;
        let encode_time = encode_start.elapsed();
        encode_times.push(encode_time);
        compressed_size = compressed.len();

        // Decode
        let decode_start = Instant::now();
        let _decoded = decode_jpegxs(&compressed)?;
        let decode_time = decode_start.elapsed();
        decode_times.push(decode_time);
    }

    let avg_encode_ms =
        encode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let avg_decode_ms =
        decode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let size_kb = compressed_size as f64 / 1024.0;

    Ok((size_kb, avg_encode_ms, avg_decode_ms))
}

fn benchmark_jpeg(img: &DynamicImage, quality: f32, iterations: usize) -> Result<(f64, f64, f64)> {
    let mut encode_times = Vec::new();
    let mut decode_times = Vec::new();
    let mut compressed_size = 0;

    for _ in 0..iterations {
        // Encode
        let encode_start = Instant::now();
        let compressed = encode_jpeg(img, quality)?;
        let encode_time = encode_start.elapsed();
        encode_times.push(encode_time);
        compressed_size = compressed.len();

        // Decode
        let decode_start = Instant::now();
        let _decoded = decode_jpeg(&compressed)?;
        let decode_time = decode_start.elapsed();
        decode_times.push(decode_time);
    }

    let avg_encode_ms =
        encode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let avg_decode_ms =
        decode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let size_kb = compressed_size as f64 / 1024.0;

    Ok((size_kb, avg_encode_ms, avg_decode_ms))
}

fn benchmark_png(img: &DynamicImage, iterations: usize) -> Result<(f64, f64, f64)> {
    let mut encode_times = Vec::new();
    let mut decode_times = Vec::new();
    let mut compressed_size = 0;

    for _ in 0..iterations {
        // Encode
        let encode_start = Instant::now();
        let compressed = encode_png(img)?;
        let encode_time = encode_start.elapsed();
        encode_times.push(encode_time);
        compressed_size = compressed.len();

        // Decode
        let decode_start = Instant::now();
        let _decoded = decode_png(&compressed)?;
        let decode_time = decode_start.elapsed();
        decode_times.push(decode_time);
    }

    let avg_encode_ms =
        encode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let avg_decode_ms =
        decode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let size_kb = compressed_size as f64 / 1024.0;

    Ok((size_kb, avg_encode_ms, avg_decode_ms))
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

fn encode_jpeg(img: &DynamicImage, quality: f32) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let rgb_img = img.to_rgb8();

    // Use the standard JPEG encoder
    let mut encoder =
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, (quality * 100.0) as u8);
    encoder.encode_image(&rgb_img)?;

    Ok(output)
}

fn decode_jpeg(data: &[u8]) -> Result<DynamicImage> {
    Ok(image::load_from_memory_with_format(
        data,
        ImageFormat::Jpeg,
    )?)
}

fn encode_png(img: &DynamicImage) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut output), ImageFormat::Png)?;
    Ok(output)
}

fn decode_png(data: &[u8]) -> Result<DynamicImage> {
    Ok(image::load_from_memory_with_format(data, ImageFormat::Png)?)
}

fn print_summary(results: &[BenchmarkResult]) {
    println!("\n📊 PERFORMANCE SUMMARY");
    println!("=====================");

    let total_original_size: f64 = results.iter().map(|r| r.original_size_kb).sum();
    let total_jpegxs_size: f64 = results.iter().map(|r| r.jpegxs_compressed_size_kb).sum();
    let total_jpeg_size: f64 = results.iter().map(|r| r.jpeg_compressed_size_kb).sum();
    let total_png_size: f64 = results.iter().map(|r| r.png_compressed_size_kb).sum();

    let avg_jpegxs_encode: f64 =
        results.iter().map(|r| r.jpegxs_encode_time_ms).sum::<f64>() / results.len() as f64;
    let avg_jpeg_encode: f64 =
        results.iter().map(|r| r.jpeg_encode_time_ms).sum::<f64>() / results.len() as f64;
    let avg_png_encode: f64 =
        results.iter().map(|r| r.png_encode_time_ms).sum::<f64>() / results.len() as f64;

    let avg_jpegxs_throughput: f64 = results
        .iter()
        .map(|r| r.jpegxs_encode_throughput_mbps)
        .sum::<f64>()
        / results.len() as f64;

    println!("\n🗜️  COMPRESSION COMPARISON:");
    println!(
        "   JPEG XS:  {:.1}:1 ratio ({:.1} KB total)",
        total_original_size / total_jpegxs_size,
        total_jpegxs_size
    );
    println!(
        "   JPEG:     {:.1}:1 ratio ({:.1} KB total)",
        total_original_size / total_jpeg_size,
        total_jpeg_size
    );
    println!(
        "   PNG:      {:.1}:1 ratio ({:.1} KB total)",
        total_original_size / total_png_size,
        total_png_size
    );

    println!("\n⚡ SPEED COMPARISON:");
    println!(
        "   JPEG XS:  {:.2} ms avg encode ({:.1} MB/s)",
        avg_jpegxs_encode, avg_jpegxs_throughput
    );
    println!("   JPEG:     {:.2} ms avg encode", avg_jpeg_encode);
    println!("   PNG:      {:.2} ms avg encode", avg_png_encode);

    // Determine winner categories
    let best_compression =
        if total_original_size / total_jpegxs_size > total_original_size / total_jpeg_size {
            "JPEG XS"
        } else {
            "JPEG"
        };

    let fastest_encode =
        if avg_jpegxs_encode < avg_jpeg_encode && avg_jpegxs_encode < avg_png_encode {
            "JPEG XS"
        } else if avg_jpeg_encode < avg_png_encode {
            "JPEG"
        } else {
            "PNG"
        };

    println!("\n🏆 RESULTS:");
    println!("   Best Compression: {}", best_compression);
    println!("   Fastest Encoding: {}", fastest_encode);

    if best_compression == "JPEG XS" {
        let improvement = ((total_original_size / total_jpegxs_size)
            / (total_original_size / total_jpeg_size)
            - 1.0)
            * 100.0;
        println!(
            "   🎉 JPEG XS achieves {:.1}% better compression than JPEG!",
            improvement
        );
    }
}

fn save_results(results: &[BenchmarkResult], output_dir: &Path) -> Result<()> {
    // Save JSON results
    let json_path = output_dir.join("benchmark_results.json");
    let json_content = serde_json::to_string_pretty(results)?;
    fs::write(&json_path, json_content)?;

    // Save CSV results
    let csv_path = output_dir.join("benchmark_results.csv");
    let mut wtr = csv::Writer::from_path(&csv_path)?;
    for result in results {
        wtr.serialize(result)?;
    }
    wtr.flush()?;

    // Save markdown report
    let md_path = output_dir.join("BENCHMARK_REPORT.md");
    let md_content = generate_markdown_report(results)?;
    fs::write(&md_path, md_content)?;

    println!("   ✓ Saved JSON: benchmark_results.json");
    println!("   ✓ Saved CSV: benchmark_results.csv");
    println!("   ✓ Saved Report: BENCHMARK_REPORT.md");

    Ok(())
}

fn generate_markdown_report(results: &[BenchmarkResult]) -> Result<String> {
    let mut md = String::new();

    md.push_str("# JPEG XS Performance Benchmark Report\n\n");
    md.push_str(&format!(
        "**Generated**: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Images Tested**: {}\n\n", results.len()));

    md.push_str("## Summary\n\n");

    let total_original_size: f64 = results.iter().map(|r| r.original_size_kb).sum();
    let total_jpegxs_size: f64 = results.iter().map(|r| r.jpegxs_compressed_size_kb).sum();
    let total_jpeg_size: f64 = results.iter().map(|r| r.jpeg_compressed_size_kb).sum();

    md.push_str(&format!(
        "- **JPEG XS Compression**: {:.1}:1 ratio\n",
        total_original_size / total_jpegxs_size
    ));
    md.push_str(&format!(
        "- **JPEG Compression**: {:.1}:1 ratio\n",
        total_original_size / total_jpeg_size
    ));

    let improvement =
        ((total_original_size / total_jpegxs_size) / (total_original_size / total_jpeg_size) - 1.0)
            * 100.0;
    if improvement > 0.0 {
        md.push_str(&format!(
            "- **JPEG XS vs JPEG**: {:.1}% better compression\n",
            improvement
        ));
    }

    md.push_str("\n## Detailed Results\n\n");
    md.push_str("| Image | Original (KB) | JPEG XS (KB) | JPEG XS Ratio | JPEG (KB) | JPEG Ratio | JPEG XS Encode (ms) | JPEG Encode (ms) |\n");
    md.push_str("|-------|---------------|---------------|---------------|-----------|------------|---------------------|------------------|\n");

    for result in results {
        md.push_str(&format!(
            "| {} | {:.1} | {:.1} | {:.1}:1 | {:.1} | {:.1}:1 | {:.2} | {:.2} |\n",
            result.image_name,
            result.original_size_kb,
            result.jpegxs_compressed_size_kb,
            result.jpegxs_compression_ratio,
            result.jpeg_compressed_size_kb,
            result.jpeg_compression_ratio,
            result.jpegxs_encode_time_ms,
            result.jpeg_encode_time_ms
        ));
    }

    Ok(md)
}
