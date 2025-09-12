use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;
use jpegxs_core::{ImageOwned8, PixelFormat};

/// Interface for comparing with reference implementations
pub trait ReferenceImplementation {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn encode(&self, input: &ImageOwned8, quality: f32) -> Result<Vec<u8>>;
    fn decode(&self, bitstream: &[u8]) -> Result<ImageOwned8>;
    fn is_available(&self) -> bool;
}

/// ISO Reference Software wrapper
pub struct IsoReference {
    executable_path: String,
    version: String,
}

impl IsoReference {
    pub fn new(path: &str) -> Self {
        Self {
            executable_path: path.to_string(),
            version: "ISO/IEC 21122-5:2024".to_string(),
        }
    }
    
    pub fn detect() -> Option<Self> {
        // Try common locations
        let possible_paths = vec![
            "/usr/local/bin/jpegxs_reference",
            "/opt/jpegxs/bin/jpegxs",
            "./reference/jpegxs",
        ];
        
        for path in possible_paths {
            if Path::new(path).exists() {
                return Some(Self::new(path));
            }
        }
        
        None
    }
}

impl ReferenceImplementation for IsoReference {
    fn name(&self) -> &str {
        "ISO Reference"
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn encode(&self, input: &ImageOwned8, quality: f32) -> Result<Vec<u8>> {
        // Save input to temp file
        let temp_dir = tempfile::tempdir()?;
        let input_path = temp_dir.path().join("input.raw");
        let output_path = temp_dir.path().join("output.jxs");
        
        std::fs::write(&input_path, &input.data)?;
        
        // Run reference encoder
        let output = Command::new(&self.executable_path)
            .arg("encode")
            .arg("-i").arg(&input_path)
            .arg("-o").arg(&output_path)
            .arg("-w").arg(input.width.to_string())
            .arg("-h").arg(input.height.to_string())
            .arg("-q").arg(quality.to_string())
            .output()
            .context("Failed to run reference encoder")?;
        
        if !output.status.success() {
            return Err(anyhow::anyhow!("Reference encoder failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        std::fs::read(&output_path)
            .context("Failed to read encoded output")
    }
    
    fn decode(&self, bitstream: &[u8]) -> Result<ImageOwned8> {
        let temp_dir = tempfile::tempdir()?;
        let input_path = temp_dir.path().join("input.jxs");
        let output_path = temp_dir.path().join("output.raw");
        
        std::fs::write(&input_path, bitstream)?;
        
        // Run reference decoder
        let output = Command::new(&self.executable_path)
            .arg("decode")
            .arg("-i").arg(&input_path)
            .arg("-o").arg(&output_path)
            .output()
            .context("Failed to run reference decoder")?;
        
        if !output.status.success() {
            return Err(anyhow::anyhow!("Reference decoder failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        let data = std::fs::read(&output_path)
            .context("Failed to read decoded output")?;
        
        // Parse metadata from reference output to get dimensions
        // This would need actual implementation based on reference format
        Ok(ImageOwned8 {
            data,
            width: 0, // Would be parsed from reference
            height: 0, // Would be parsed from reference
            format: PixelFormat::Rgb8,
        })
    }
    
    fn is_available(&self) -> bool {
        Path::new(&self.executable_path).exists()
    }
}

/// Comparison results between our implementation and reference
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub reference_name: String,
    pub test_name: String,
    pub our_size: usize,
    pub ref_size: usize,
    pub size_ratio: f64, // our/ref (lower is better)
    pub our_psnr: f64,
    pub ref_psnr: f64,
    pub psnr_diff: f64, // our - ref (higher is better)
    pub encode_time_ratio: f64, // our/ref (lower is faster)
    pub decode_time_ratio: f64, // our/ref (lower is faster)
}

pub fn compare_with_reference(
    reference: &dyn ReferenceImplementation,
    test_image: &ImageOwned8,
    quality: f32,
) -> Result<ComparisonResult> {
    use std::time::Instant;
    use crate::metrics::calculate_psnr;
    
    // Encode with our implementation
    let our_start = Instant::now();
    let our_encoded = jpegxs_core::encode_frame(
        jpegxs_core::ImageView8 {
            data: &test_image.data,
            width: test_image.width,
            height: test_image.height,
            format: test_image.format,
        },
        &jpegxs_core::EncoderConfig {
            quality,
            ..Default::default()
        },
    )?;
    let our_encode_time = our_start.elapsed();
    
    // Encode with reference
    let ref_start = Instant::now();
    let ref_encoded = reference.encode(test_image, quality)?;
    let ref_encode_time = ref_start.elapsed();
    
    // Decode both
    let our_start = Instant::now();
    let our_decoded = jpegxs_core::decode_frame(
        &our_encoded,
        &jpegxs_core::DecoderConfig::default(),
    )?;
    let our_decode_time = our_start.elapsed();
    
    let ref_start = Instant::now();
    let ref_decoded = reference.decode(&ref_encoded)?;
    let ref_decode_time = ref_start.elapsed();
    
    // Calculate quality metrics
    let our_psnr = calculate_psnr(&test_image.data, &our_decoded.data);
    let ref_psnr = calculate_psnr(&test_image.data, &ref_decoded.data);
    
    Ok(ComparisonResult {
        reference_name: reference.name().to_string(),
        test_name: format!("{}x{}_q{}", test_image.width, test_image.height, quality),
        our_size: our_encoded.data.len(),
        ref_size: ref_encoded.len(),
        size_ratio: our_encoded.data.len() as f64 / ref_encoded.len() as f64,
        our_psnr,
        ref_psnr,
        psnr_diff: our_psnr - ref_psnr,
        encode_time_ratio: our_encode_time.as_secs_f64() / ref_encode_time.as_secs_f64(),
        decode_time_ratio: our_decode_time.as_secs_f64() / ref_decode_time.as_secs_f64(),
    })
}

pub fn generate_comparison_report(results: &[ComparisonResult]) -> String {
    let mut report = String::new();
    
    report.push_str("# JPEG XS Implementation Comparison Report\n\n");
    report.push_str("## Summary\n\n");
    
    // Calculate averages
    let avg_size_ratio = results.iter().map(|r| r.size_ratio).sum::<f64>() / results.len() as f64;
    let avg_psnr_diff = results.iter().map(|r| r.psnr_diff).sum::<f64>() / results.len() as f64;
    let avg_encode_ratio = results.iter().map(|r| r.encode_time_ratio).sum::<f64>() / results.len() as f64;
    let avg_decode_ratio = results.iter().map(|r| r.decode_time_ratio).sum::<f64>() / results.len() as f64;
    
    report.push_str(&format!("- **Average Size Ratio**: {:.2}x ({})\n", 
        avg_size_ratio,
        if avg_size_ratio < 1.0 { "better" } else { "worse" }
    ));
    report.push_str(&format!("- **Average PSNR Difference**: {:+.2} dB\n", avg_psnr_diff));
    report.push_str(&format!("- **Average Encode Speed**: {:.2}x {}\n", 
        avg_encode_ratio,
        if avg_encode_ratio < 1.0 { "faster" } else { "slower" }
    ));
    report.push_str(&format!("- **Average Decode Speed**: {:.2}x {}\n", 
        avg_decode_ratio,
        if avg_decode_ratio < 1.0 { "faster" } else { "slower" }
    ));
    
    report.push_str("\n## Detailed Results\n\n");
    report.push_str("| Test | Our Size | Ref Size | Size Ratio | Our PSNR | Ref PSNR | PSNR Diff | Encode Speed | Decode Speed |\n");
    report.push_str("|------|----------|----------|------------|----------|----------|-----------|--------------|-------------|\n");
    
    for result in results {
        report.push_str(&format!(
            "| {} | {} | {} | {:.2}x | {:.2} dB | {:.2} dB | {:+.2} dB | {:.2}x | {:.2}x |\n",
            result.test_name,
            result.our_size,
            result.ref_size,
            result.size_ratio,
            result.our_psnr,
            result.ref_psnr,
            result.psnr_diff,
            result.encode_time_ratio,
            result.decode_time_ratio,
        ));
    }
    
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_reference_detection() {
        let iso_ref = IsoReference::detect();
        // This will be None in CI, which is fine
        if let Some(ref_impl) = iso_ref {
            assert!(ref_impl.is_available());
        }
    }
    
    #[test]
    fn test_comparison_report_generation() {
        let results = vec![
            ComparisonResult {
                reference_name: "ISO Reference".to_string(),
                test_name: "256x256_q0.5".to_string(),
                our_size: 1000,
                ref_size: 1100,
                size_ratio: 0.91,
                our_psnr: 35.0,
                ref_psnr: 34.5,
                psnr_diff: 0.5,
                encode_time_ratio: 0.8,
                decode_time_ratio: 0.9,
            },
        ];
        
        let report = generate_comparison_report(&results);
        assert!(report.contains("Comparison Report"));
        assert!(report.contains("0.91"));
    }
}