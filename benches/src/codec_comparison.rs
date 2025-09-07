// Copyright (c) 2024 Keyvan Ebrahimpour. All rights reserved.

use anyhow::Result;
use clap::{Parser, ValueEnum};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tabled::{Table, Tabled};

#[derive(Parser)]
#[command(name = "codec_comparison")]
#[command(about = "Comprehensive codec performance comparison")]
struct Args {
    /// Input directory containing test images
    #[arg(short, long, default_value = "test_images")]
    input_dir: PathBuf,

    /// Output directory for results
    #[arg(short, long, default_value = "benchmark_results")]
    output_dir: PathBuf,

    /// Codecs to test
    #[arg(short, long, value_enum, default_values = ["jpegxs", "jpeg", "png", "webp"])]
    codecs: Vec<CodecType>,

    /// Quality levels to test (0.1 to 1.0)
    #[arg(short, long, default_values = ["0.5", "0.7", "0.9", "0.95"])]
    quality_levels: Vec<f32>,

    /// Number of iterations for timing
    #[arg(long, default_value = "5")]
    iterations: usize,

    /// Generate detailed report
    #[arg(long)]
    detailed_report: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CodecType {
    JpegXs,
    Jpeg,
    Png,
    WebP,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
struct BenchmarkResult {
    image_name: String,
    codec: String,
    quality: f32,
    original_size: u64,
    compressed_size: u64,
    compression_ratio: f32,
    encode_time_ms: f64,
    decode_time_ms: f64,
    psnr: f64,
    ssim: f64,
    encode_throughput_mbps: f64,
    decode_throughput_mbps: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComparisonReport {
    test_metadata: TestMetadata,
    results: Vec<BenchmarkResult>,
    summary: CodecSummary,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestMetadata {
    timestamp: String,
    test_images: Vec<String>,
    quality_levels: Vec<f32>,
    iterations: usize,
    system_info: SystemInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    os: String,
    cpu: String,
    memory_gb: f64,
    rust_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodecSummary {
    by_codec: HashMap<String, CodecStats>,
    by_quality: HashMap<String, QualityStats>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodecStats {
    avg_compression_ratio: f64,
    avg_encode_time_ms: f64,
    avg_decode_time_ms: f64,
    avg_psnr: f64,
    avg_ssim: f64,
    avg_encode_throughput_mbps: f64,
    avg_decode_throughput_mbps: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct QualityStats {
    compression_ratios: HashMap<String, f64>,
    encode_times: HashMap<String, f64>,
    decode_times: HashMap<String, f64>,
    psnr_values: HashMap<String, f64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 JPEG XS Codec Performance Comparison Suite");
    println!("=============================================");

    // Create output directory
    fs::create_dir_all(&args.output_dir)?;

    // Load test images
    let test_images = load_test_images(&args.input_dir)?;
    println!("📁 Loaded {} test images", test_images.len());

    // Initialize progress bar
    let total_tests = test_images.len() * args.codecs.len() * args.quality_levels.len();
    let pb = ProgressBar::new(total_tests as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Run benchmarks
    let mut results = Vec::new();
    for image_path in &test_images {
        for &codec in &args.codecs {
            for &quality in &args.quality_levels {
                pb.set_message(format!(
                    "Testing {} with {:?} @ {:.1}",
                    image_path.file_stem().unwrap().to_str().unwrap(),
                    codec,
                    quality
                ));

                let result = benchmark_codec(image_path, codec, quality, args.iterations)?;
                results.push(result);
                pb.inc(1);
            }
        }
    }
    pb.finish_with_message("✅ Benchmarking complete!");

    // Generate report
    let report = generate_report(results, &args, &test_images)?;

    // Save results
    save_results(&report, &args.output_dir)?;

    // Print summary
    print_summary(&report);

    println!("\n📊 Full results saved to: {}", args.output_dir.display());

    Ok(())
}

fn load_test_images(input_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    if !input_dir.exists() {
        println!("📁 Creating test image directory: {}", input_dir.display());
        fs::create_dir_all(input_dir)?;

        // Create some test images if none exist
        create_test_images(input_dir)?;
    }

    for entry in fs::read_dir(input_dir)? {
        let path = entry?.path();
        if let Some(ext) = path.extension() {
            if matches!(ext.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
                images.push(path);
            }
        }
    }

    if images.is_empty() {
        create_test_images(input_dir)?;
        // Re-scan after creating test images
        for entry in fs::read_dir(input_dir)? {
            let path = entry?.path();
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
                    images.push(path);
                }
            }
        }
    }

    Ok(images)
}

fn create_test_images(dir: &Path) -> Result<()> {
    println!("🎨 Creating synthetic test images...");

    // Create various test patterns
    let test_cases = [
        (
            "gradient_256x256.png",
            256,
            256,
            generate_gradient as fn(u32, u32) -> DynamicImage,
        ),
        (
            "noise_512x512.png",
            512,
            512,
            generate_noise as fn(u32, u32) -> DynamicImage,
        ),
        (
            "pattern_1024x768.png",
            1024,
            768,
            generate_pattern as fn(u32, u32) -> DynamicImage,
        ),
        (
            "photo_realistic_640x480.png",
            640,
            480,
            generate_photo_realistic as fn(u32, u32) -> DynamicImage,
        ),
    ];

    for (name, width, height, generator) in test_cases {
        let img = generator(width, height);
        let path = dir.join(name);
        img.save(&path)?;
        println!("   ✓ Created {}", name);
    }

    Ok(())
}

fn generate_gradient(width: u32, height: u32) -> DynamicImage {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let r = (x * 255 / width) as u8;
        let g = (y * 255 / height) as u8;
        let b = ((x + y) * 255 / (width + height)) as u8;
        Rgb([r, g, b])
    });
    DynamicImage::ImageRgb8(img)
}

fn generate_noise(width: u32, height: u32) -> DynamicImage {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let mut hasher = DefaultHasher::new();
        (x, y).hash(&mut hasher);
        let hash = hasher.finish();
        let r = (hash >> 16) as u8;
        let g = (hash >> 8) as u8;
        let b = hash as u8;
        Rgb([r, g, b])
    });
    DynamicImage::ImageRgb8(img)
}

fn generate_pattern(width: u32, height: u32) -> DynamicImage {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let checker = ((x / 32) + (y / 32)) % 2;
        let intensity = if checker == 0 { 64 } else { 192 };

        let r = intensity + (x % 32) as u8 * 2;
        let g = intensity + (y % 32) as u8 * 2;
        let b = intensity + ((x + y) % 32) as u8 * 2;

        Rgb([r, g, b])
    });
    DynamicImage::ImageRgb8(img)
}

fn generate_photo_realistic(width: u32, height: u32) -> DynamicImage {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let fx = x as f32 / width as f32;
        let fy = y as f32 / height as f32;

        // Create a natural-looking scene with sky gradient and ground texture
        let sky_blue = if fy < 0.3 {
            let intensity = (135.0 + (fy * 120.0)) as u8;
            Rgb([100, 149, intensity])
        } else {
            // Ground with texture
            let base_green = 34 + (fx * 40.0) as u8;
            let noise = ((x * 7 + y * 11) % 64) as u8;
            Rgb([base_green, 80 + noise / 4, base_green / 2])
        };

        sky_blue
    });
    DynamicImage::ImageRgb8(img)
}

fn benchmark_codec(
    image_path: &Path,
    codec: CodecType,
    quality: f32,
    iterations: usize,
) -> Result<BenchmarkResult> {
    // Load original image
    let original_img = image::open(image_path)?;
    let original_size = fs::metadata(image_path)?.len();

    let mut encode_times = Vec::new();
    let mut decode_times = Vec::new();
    let mut compressed_data = Vec::new();

    // Run multiple iterations for timing accuracy
    for _ in 0..iterations {
        let (encode_time, compressed) = time_encode(&original_img, codec, quality)?;
        let (decode_time, _decoded) = time_decode(&compressed, codec)?;

        encode_times.push(encode_time);
        decode_times.push(decode_time);
        compressed_data = compressed; // Keep last iteration's data
    }

    // Calculate averages
    let avg_encode_time =
        encode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;
    let avg_decode_time =
        decode_times.iter().sum::<Duration>().as_secs_f64() / iterations as f64 * 1000.0;

    let compressed_size = compressed_data.len() as u64;
    let compression_ratio = original_size as f32 / compressed_size as f32;

    // Calculate quality metrics (simplified)
    let psnr = calculate_psnr(&original_img, &time_decode(&compressed_data, codec)?.1)?;
    let ssim = calculate_ssim(&original_img, &time_decode(&compressed_data, codec)?.1)?;

    // Calculate throughput (MB/s)
    let original_mb = original_size as f64 / 1_048_576.0;
    let encode_throughput = original_mb / (avg_encode_time / 1000.0);
    let decode_throughput = original_mb / (avg_decode_time / 1000.0);

    Ok(BenchmarkResult {
        image_name: image_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        codec: format!("{:?}", codec),
        quality,
        original_size,
        compressed_size,
        compression_ratio,
        encode_time_ms: avg_encode_time,
        decode_time_ms: avg_decode_time,
        psnr,
        ssim,
        encode_throughput_mbps: encode_throughput,
        decode_throughput_mbps: decode_throughput,
    })
}

fn time_encode(img: &DynamicImage, codec: CodecType, quality: f32) -> Result<(Duration, Vec<u8>)> {
    let start = Instant::now();
    let compressed = match codec {
        CodecType::JpegXs => encode_jpegxs(img, quality)?,
        CodecType::Jpeg => encode_jpeg(img, quality)?,
        CodecType::Png => encode_png(img)?,
        CodecType::WebP => encode_webp(img, quality)?,
    };
    let duration = start.elapsed();
    Ok((duration, compressed))
}

fn time_decode(data: &[u8], codec: CodecType) -> Result<(Duration, DynamicImage)> {
    let start = Instant::now();
    let decoded = match codec {
        CodecType::JpegXs => decode_jpegxs(data)?,
        CodecType::Jpeg => decode_jpeg(data)?,
        CodecType::Png => decode_png(data)?,
        CodecType::WebP => decode_webp(data)?,
    };
    let duration = start.elapsed();
    Ok((duration, decoded))
}

// Codec-specific encoding/decoding functions
fn encode_jpegxs(img: &DynamicImage, quality: f32) -> Result<Vec<u8>> {
    // Convert to RGB and then to YUV422p
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    // Convert RGB to YUV422p (simplified)
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
    let _quality_u8 = (quality * 100.0) as u8;
    img.write_to(&mut std::io::Cursor::new(&mut output), ImageFormat::Jpeg)?;
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

fn encode_webp(img: &DynamicImage, quality: f32) -> Result<Vec<u8>> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let encoder = webp::Encoder::from_rgb(rgb_img.as_raw(), width, height);
    let encoded = encoder.encode(quality * 100.0);
    Ok(encoded.to_vec())
}

fn decode_webp(data: &[u8]) -> Result<DynamicImage> {
    let decoder = webp::Decoder::new(data);
    let decoded = decoder
        .decode()
        .ok_or_else(|| anyhow::anyhow!("WebP decode failed"))?;

    let img_buffer =
        image::ImageBuffer::from_raw(decoded.width(), decoded.height(), decoded.to_vec())
            .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;

    Ok(DynamicImage::ImageRgb8(img_buffer))
}

// Helper functions for color conversion (simplified versions)
fn rgb_to_yuv422p(rgb_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let pixels = width as usize * height as usize;
    let mut yuv_data = Vec::with_capacity(pixels * 2);

    // Y plane
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

fn calculate_psnr(img1: &DynamicImage, img2: &DynamicImage) -> Result<f64> {
    let rgb1 = img1.to_rgb8();
    let rgb2 = img2.to_rgb8();

    if rgb1.dimensions() != rgb2.dimensions() {
        return Ok(0.0);
    }

    let mut mse = 0.0;
    let pixels = (rgb1.width() * rgb1.height() * 3) as f64;

    for (p1, p2) in rgb1.as_raw().iter().zip(rgb2.as_raw().iter()) {
        let diff = *p1 as f64 - *p2 as f64;
        mse += diff * diff;
    }

    mse /= pixels;

    if mse == 0.0 {
        Ok(f64::INFINITY)
    } else {
        Ok(20.0 * (255.0 / mse.sqrt()).log10())
    }
}

fn calculate_ssim(img1: &DynamicImage, img2: &DynamicImage) -> Result<f64> {
    // Simplified SSIM calculation (placeholder)
    let psnr = calculate_psnr(img1, img2)?;

    // Convert PSNR to approximate SSIM (simplified)
    if psnr.is_infinite() {
        Ok(1.0)
    } else {
        Ok((psnr / 100.0).min(1.0).max(0.0))
    }
}

fn generate_report(
    results: Vec<BenchmarkResult>,
    args: &Args,
    test_images: &[PathBuf],
) -> Result<ComparisonReport> {
    let metadata = TestMetadata {
        timestamp: chrono::Utc::now().to_rfc3339(),
        test_images: test_images
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect(),
        quality_levels: args.quality_levels.clone(),
        iterations: args.iterations,
        system_info: SystemInfo {
            os: std::env::consts::OS.to_string(),
            cpu: "Unknown".to_string(), // Would need additional crate for CPU detection
            memory_gb: 8.0,             // Placeholder
            rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
        },
    };

    let summary = generate_summary(&results)?;

    Ok(ComparisonReport {
        test_metadata: metadata,
        results,
        summary,
    })
}

fn generate_summary(results: &[BenchmarkResult]) -> Result<CodecSummary> {
    let mut by_codec: HashMap<String, Vec<&BenchmarkResult>> = HashMap::new();
    let mut by_quality: HashMap<String, Vec<&BenchmarkResult>> = HashMap::new();

    for result in results {
        by_codec
            .entry(result.codec.clone())
            .or_default()
            .push(result);
        by_quality
            .entry(result.quality.to_string())
            .or_default()
            .push(result);
    }

    let codec_stats = by_codec
        .into_iter()
        .map(|(codec, results)| {
            let stats = CodecStats {
                avg_compression_ratio: results
                    .iter()
                    .map(|r| r.compression_ratio as f64)
                    .sum::<f64>()
                    / results.len() as f64,
                avg_encode_time_ms: results.iter().map(|r| r.encode_time_ms).sum::<f64>()
                    / results.len() as f64,
                avg_decode_time_ms: results.iter().map(|r| r.decode_time_ms).sum::<f64>()
                    / results.len() as f64,
                avg_psnr: results.iter().map(|r| r.psnr).sum::<f64>() / results.len() as f64,
                avg_ssim: results.iter().map(|r| r.ssim).sum::<f64>() / results.len() as f64,
                avg_encode_throughput_mbps: results
                    .iter()
                    .map(|r| r.encode_throughput_mbps)
                    .sum::<f64>()
                    / results.len() as f64,
                avg_decode_throughput_mbps: results
                    .iter()
                    .map(|r| r.decode_throughput_mbps)
                    .sum::<f64>()
                    / results.len() as f64,
            };
            (codec, stats)
        })
        .collect();

    let quality_stats = by_quality
        .into_iter()
        .map(|(quality, results)| {
            let compression_ratios = results.iter().fold(HashMap::new(), |mut acc, r| {
                *acc.entry(r.codec.clone()).or_insert(0.0) += r.compression_ratio as f64;
                acc
            });

            let stats = QualityStats {
                compression_ratios,
                encode_times: HashMap::new(), // Simplified
                decode_times: HashMap::new(), // Simplified
                psnr_values: HashMap::new(),  // Simplified
            };
            (quality, stats)
        })
        .collect();

    Ok(CodecSummary {
        by_codec: codec_stats,
        by_quality: quality_stats,
    })
}

fn save_results(report: &ComparisonReport, output_dir: &Path) -> Result<()> {
    // Save JSON report
    let json_path = output_dir.join("benchmark_report.json");
    let json_content = serde_json::to_string_pretty(report)?;
    fs::write(&json_path, json_content)?;

    // Save CSV results
    let csv_path = output_dir.join("benchmark_results.csv");
    let mut wtr = csv::Writer::from_path(&csv_path)?;
    for result in &report.results {
        wtr.serialize(result)?;
    }
    wtr.flush()?;

    // Save markdown summary
    let md_path = output_dir.join("BENCHMARK_SUMMARY.md");
    let md_content = generate_markdown_report(report)?;
    fs::write(&md_path, md_content)?;

    Ok(())
}

fn generate_markdown_report(report: &ComparisonReport) -> Result<String> {
    let mut md = String::new();

    md.push_str("# JPEG XS Codec Performance Comparison\n\n");
    md.push_str(&format!(
        "**Generated**: {}\n",
        report.test_metadata.timestamp
    ));
    md.push_str(&format!(
        "**Test Images**: {}\n",
        report.test_metadata.test_images.len()
    ));
    md.push_str(&format!(
        "**Quality Levels**: {:?}\n",
        report.test_metadata.quality_levels
    ));
    md.push_str(&format!(
        "**Iterations**: {}\n\n",
        report.test_metadata.iterations
    ));

    md.push_str("## Codec Performance Summary\n\n");

    for (codec, stats) in &report.summary.by_codec {
        md.push_str(&format!("### {}\n", codec));
        md.push_str(&format!(
            "- **Average Compression Ratio**: {:.2}:1\n",
            stats.avg_compression_ratio
        ));
        md.push_str(&format!(
            "- **Average Encode Time**: {:.2} ms\n",
            stats.avg_encode_time_ms
        ));
        md.push_str(&format!(
            "- **Average Decode Time**: {:.2} ms\n",
            stats.avg_decode_time_ms
        ));
        md.push_str(&format!("- **Average PSNR**: {:.2} dB\n", stats.avg_psnr));
        md.push_str(&format!("- **Average SSIM**: {:.3}\n", stats.avg_ssim));
        md.push_str(&format!(
            "- **Encode Throughput**: {:.2} MB/s\n",
            stats.avg_encode_throughput_mbps
        ));
        md.push_str(&format!(
            "- **Decode Throughput**: {:.2} MB/s\n\n",
            stats.avg_decode_throughput_mbps
        ));
    }

    md.push_str("## Detailed Results\n\n");
    md.push_str(&Table::new(&report.results).to_string());

    Ok(md)
}

fn print_summary(report: &ComparisonReport) {
    println!("\n📊 BENCHMARK SUMMARY");
    println!("===================");

    for (codec, stats) in &report.summary.by_codec {
        println!("\n🔧 {} Performance:", codec);
        println!("   Compression Ratio: {:.2}:1", stats.avg_compression_ratio);
        println!(
            "   Encode Speed: {:.2} MB/s",
            stats.avg_encode_throughput_mbps
        );
        println!(
            "   Decode Speed: {:.2} MB/s",
            stats.avg_decode_throughput_mbps
        );
        println!("   Image Quality (PSNR): {:.2} dB", stats.avg_psnr);
    }

    // Find best performers
    let best_compression = report.summary.by_codec.iter().max_by(|a, b| {
        a.1.avg_compression_ratio
            .partial_cmp(&b.1.avg_compression_ratio)
            .unwrap()
    });

    let best_encode_speed = report.summary.by_codec.iter().max_by(|a, b| {
        a.1.avg_encode_throughput_mbps
            .partial_cmp(&b.1.avg_encode_throughput_mbps)
            .unwrap()
    });

    if let Some((codec, _)) = best_compression {
        println!("\n🏆 Best Compression: {}", codec);
    }

    if let Some((codec, _)) = best_encode_speed {
        println!("🚀 Fastest Encoding: {}", codec);
    }
}
