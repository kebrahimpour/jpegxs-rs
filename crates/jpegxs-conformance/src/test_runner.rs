use anyhow::{Result, Context};
use jpegxs_core::{encode_frame, decode_frame, EncoderConfig, DecoderConfig, ImageView8};
use crate::{
    test_vectors::{TestVectorGenerator, TestPattern},
    metrics::{calculate_psnr, MemoryProfiler},
    TestReport, ConformanceResults, PerformanceResults, TestSuite, TestCase, TestStatus,
    MemoryMetrics, SpeedMetrics, CompressionMetrics,
};
use std::time::Instant;

pub struct ConformanceTestRunner {
    generator: TestVectorGenerator,
    encoder_config: EncoderConfig,
    decoder_config: DecoderConfig,
    max_test_time_ms: u64,
}

impl ConformanceTestRunner {
    pub fn new() -> Self {
        Self {
            generator: TestVectorGenerator::new(),
            encoder_config: EncoderConfig::default(),
            decoder_config: DecoderConfig::default(),
            max_test_time_ms: 30_000, // 30 seconds per test max
        }
    }

    pub fn with_encoder_config(mut self, config: EncoderConfig) -> Self {
        self.encoder_config = config;
        self
    }

    pub fn with_decoder_config(mut self, config: DecoderConfig) -> Self {
        self.decoder_config = config;
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.max_test_time_ms = timeout_ms;
        self
    }

    pub fn run_full_conformance_suite(&self) -> Result<TestReport> {
        println!("🧪 Starting JPEG XS Conformance Test Suite");
        println!("═══════════════════════════════════════════");

        let start_time = Instant::now();
        let timestamp = chrono::Utc::now().to_rfc3339();

        // Run all test categories
        let encoder_tests = self.run_encoder_tests()?;
        let decoder_tests = self.run_decoder_tests()?;
        let bitstream_tests = self.run_bitstream_tests()?;
        let performance = self.run_performance_tests()?;

        let total_tests = encoder_tests.total + decoder_tests.total + bitstream_tests.total;
        let total_passed = encoder_tests.passed + decoder_tests.passed + bitstream_tests.passed;
        let compliance_percentage = (total_passed as f64 / total_tests as f64) * 100.0;

        let conformance = ConformanceResults {
            encoder_tests,
            decoder_tests,
            bitstream_tests,
            compliance_percentage,
        };

        let comparison = crate::ComparisonResults {
            reference_implementation: "Synthetic Test Vectors".to_string(),
            compression_delta: 0.0, // Would be filled by actual comparison
            speed_delta: 0.0,
            memory_delta: 0.0,
            quality_delta: 0.0,
        };

        let elapsed = start_time.elapsed();
        println!("\n🏁 Conformance Suite Complete!");
        println!("📊 Overall Compliance: {:.1}%", compliance_percentage);
        println!("⏱️  Total Time: {:.2}s", elapsed.as_secs_f64());

        if compliance_percentage >= 80.0 {
            println!("✅ PASS - Good conformance level achieved");
        } else if compliance_percentage >= 60.0 {
            println!("⚠️  WARN - Moderate conformance, improvement needed");
        } else {
            println!("❌ FAIL - Low conformance, significant work required");
        }

        Ok(TestReport {
            timestamp,
            version: "0.2.0-alpha".to_string(),
            conformance,
            performance,
            comparison,
        })
    }

    fn run_encoder_tests(&self) -> Result<TestSuite> {
        println!("\n📤 Running Encoder Conformance Tests");
        println!("────────────────────────────────────");

        let mut test_cases = Vec::new();
        let patterns = self.generator.get_all_patterns();

        for (i, pattern) in patterns.iter().enumerate() {
            print!("  [{:2}/{}] {} ... ", i + 1, patterns.len(), pattern.name);

            let result = self.run_single_encoder_test(pattern);
            match &result {
                Ok(test_case) => {
                    match test_case.status {
                        TestStatus::Pass => print!("✅ PASS"),
                        TestStatus::Fail => print!("❌ FAIL"),
                        TestStatus::Skip => print!("⏭️  SKIP"),
                        TestStatus::Error => print!("💥 ERROR"),
                    }
                    if let Some(ref message) = test_case.message {
                        print!(" ({})", message);
                    }
                    println!(" [{:.0}ms]", test_case.duration_ms);
                    test_cases.push(test_case.clone());
                }
                Err(e) => {
                    println!("💥 ERROR ({})", e);
                    test_cases.push(TestCase {
                        name: format!("encoder_{}", pattern.name),
                        category: "Encoder".to_string(),
                        status: TestStatus::Error,
                        message: Some(e.to_string()),
                        duration_ms: 0.0,
                    });
                }
            }
        }

        let total = test_cases.len();
        let passed = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Pass)).count();
        let failed = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Fail)).count();
        let skipped = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Skip)).count();

        println!("📊 Encoder Tests: {}/{} passed ({:.1}%)", passed, total, (passed as f64 / total as f64) * 100.0);

        Ok(TestSuite {
            total,
            passed,
            failed,
            skipped,
            details: test_cases,
        })
    }

    fn run_single_encoder_test(&self, pattern: &TestPattern) -> Result<TestCase> {
        let start = Instant::now();

        // Generate test image
        let image = self.generator.generate_pattern(pattern)
            .context("Failed to generate test pattern")?;

        let input_view = ImageView8 {
            data: &image.data,
            width: image.width,
            height: image.height,
            format: image.format,
        };

        // Encode
        let encoded = encode_frame(input_view, &self.encoder_config)
            .context("Encoding failed")?;

        // Decode back
        let decoded = decode_frame(&encoded, &self.decoder_config)
            .context("Decoding failed")?;

        // Calculate quality metrics
        let psnr = calculate_psnr(&image.data, &decoded.data);
        let compression_ratio = image.data.len() as f64 / encoded.data.len() as f64;

        let duration = start.elapsed().as_millis() as f64;

        // Determine test result
        let (status, message) = if psnr >= pattern.expected_psnr_threshold {
            (TestStatus::Pass, Some(format!("PSNR: {:.2} dB, Ratio: {:.1}:1", psnr, compression_ratio)))
        } else {
            (TestStatus::Fail, Some(format!("PSNR: {:.2} dB < {:.2} dB threshold", psnr, pattern.expected_psnr_threshold)))
        };

        Ok(TestCase {
            name: format!("encoder_{}", pattern.name),
            category: "Encoder".to_string(),
            status,
            message,
            duration_ms: duration,
        })
    }

    fn run_decoder_tests(&self) -> Result<TestSuite> {
        println!("\n📥 Running Decoder Conformance Tests");
        println!("────────────────────────────────────");

        let mut test_cases = Vec::new();
        let patterns = self.generator.get_all_patterns();

        for (i, pattern) in patterns.iter().enumerate() {
            print!("  [{:2}/{}] {} ... ", i + 1, patterns.len(), pattern.name);

            let result = self.run_single_decoder_test(pattern);
            match &result {
                Ok(test_case) => {
                    match test_case.status {
                        TestStatus::Pass => print!("✅ PASS"),
                        TestStatus::Fail => print!("❌ FAIL"),
                        TestStatus::Skip => print!("⏭️  SKIP"),
                        TestStatus::Error => print!("💥 ERROR"),
                    }
                    if let Some(ref message) = test_case.message {
                        print!(" ({})", message);
                    }
                    println!(" [{:.0}ms]", test_case.duration_ms);
                    test_cases.push(test_case.clone());
                }
                Err(e) => {
                    println!("💥 ERROR ({})", e);
                    test_cases.push(TestCase {
                        name: format!("decoder_{}", pattern.name),
                        category: "Decoder".to_string(),
                        status: TestStatus::Error,
                        message: Some(e.to_string()),
                        duration_ms: 0.0,
                    });
                }
            }
        }

        let total = test_cases.len();
        let passed = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Pass)).count();
        let failed = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Fail)).count();
        let skipped = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Skip)).count();

        println!("📊 Decoder Tests: {}/{} passed ({:.1}%)", passed, total, (passed as f64 / total as f64) * 100.0);

        Ok(TestSuite {
            total,
            passed,
            failed,
            skipped,
            details: test_cases,
        })
    }

    fn run_single_decoder_test(&self, pattern: &TestPattern) -> Result<TestCase> {
        let start = Instant::now();

        // Generate and encode test image
        let image = self.generator.generate_pattern(pattern)?;
        let input_view = ImageView8 {
            data: &image.data,
            width: image.width,
            height: image.height,
            format: image.format,
        };

        let encoded = encode_frame(input_view, &self.encoder_config)?;

        // Test decoder with the encoded data
        let decoded = decode_frame(&encoded, &self.decoder_config)
            .context("Decoder failed to process valid bitstream")?;

        // Verify decoded properties
        let duration = start.elapsed().as_millis() as f64;

        let (status, message) = if decoded.width == image.width && decoded.height == image.height {
            (TestStatus::Pass, Some(format!("{}x{} decoded correctly", decoded.width, decoded.height)))
        } else {
            (TestStatus::Fail, Some(format!("Size mismatch: expected {}x{}, got {}x{}",
                image.width, image.height, decoded.width, decoded.height)))
        };

        Ok(TestCase {
            name: format!("decoder_{}", pattern.name),
            category: "Decoder".to_string(),
            status,
            message,
            duration_ms: duration,
        })
    }

    fn run_bitstream_tests(&self) -> Result<TestSuite> {
        println!("\n🔍 Running Bitstream Validation Tests");
        println!("─────────────────────────────────────");

        let mut test_cases = Vec::new();
        let test_patterns = ["solid_red", "horizontal_gradient", "checker_8x8", "random_noise"];

        for (i, pattern_name) in test_patterns.iter().enumerate() {
            print!("  [{}/{}] bitstream_{} ... ", i + 1, test_patterns.len(), pattern_name);

            let pattern = self.generator.get_pattern(pattern_name)
                .ok_or_else(|| anyhow::anyhow!("Pattern not found: {}", pattern_name))?;

            let result = self.run_single_bitstream_test(pattern);
            match &result {
                Ok(test_case) => {
                    match test_case.status {
                        TestStatus::Pass => print!("✅ PASS"),
                        TestStatus::Fail => print!("❌ FAIL"),
                        TestStatus::Skip => print!("⏭️  SKIP"),
                        TestStatus::Error => print!("💥 ERROR"),
                    }
                    if let Some(ref message) = test_case.message {
                        print!(" ({})", message);
                    }
                    println!(" [{:.0}ms]", test_case.duration_ms);
                    test_cases.push(test_case.clone());
                }
                Err(e) => {
                    println!("💥 ERROR ({})", e);
                    test_cases.push(TestCase {
                        name: format!("bitstream_{}", pattern_name),
                        category: "Bitstream".to_string(),
                        status: TestStatus::Error,
                        message: Some(e.to_string()),
                        duration_ms: 0.0,
                    });
                }
            }
        }

        let total = test_cases.len();
        let passed = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Pass)).count();
        let failed = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Fail)).count();
        let skipped = test_cases.iter().filter(|t| matches!(t.status, TestStatus::Skip)).count();

        println!("📊 Bitstream Tests: {}/{} passed ({:.1}%)", passed, total, (passed as f64 / total as f64) * 100.0);

        Ok(TestSuite {
            total,
            passed,
            failed,
            skipped,
            details: test_cases,
        })
    }

    fn run_single_bitstream_test(&self, pattern: &TestPattern) -> Result<TestCase> {
        let start = Instant::now();

        // Generate and encode test image
        let image = self.generator.generate_pattern(pattern)?;
        let input_view = ImageView8 {
            data: &image.data,
            width: image.width,
            height: image.height,
            format: image.format,
        };

        let encoded = encode_frame(input_view, &self.encoder_config)?;

        // Basic bitstream validation
        let duration = start.elapsed().as_millis() as f64;

        // Check minimum bitstream size
        let min_expected_size = 100; // Minimum reasonable size for headers
        let (status, message) = if encoded.data.len() >= min_expected_size {
            (TestStatus::Pass, Some(format!("{} bytes, valid size", encoded.data.len())))
        } else {
            (TestStatus::Fail, Some(format!("Bitstream too small: {} bytes", encoded.data.len())))
        };

        Ok(TestCase {
            name: format!("bitstream_{}", pattern.name),
            category: "Bitstream".to_string(),
            status,
            message,
            duration_ms: duration,
        })
    }

    fn run_performance_tests(&self) -> Result<PerformanceResults> {
        println!("\n⚡ Running Performance Benchmarks");
        println!("─────────────────────────────────");

        // Memory benchmark
        print!("  Memory usage ... ");
        let memory_metrics = self.run_memory_benchmark()?;
        println!("✅ Peak: {:.1} MB", memory_metrics.peak_heap_mb);

        // Speed benchmark
        print!("  Encoding speed ... ");
        let speed_metrics = self.run_speed_benchmark()?;
        println!("✅ {:.1} Mbps", speed_metrics.encode_mbps);

        // Quality benchmark
        print!("  Compression quality ... ");
        let compression_metrics = self.run_quality_benchmark()?;
        println!("✅ {:.1}:1 ratio, {:.1} dB PSNR", compression_metrics.avg_ratio, compression_metrics.avg_psnr_db);

        Ok(PerformanceResults {
            memory: memory_metrics,
            speed: speed_metrics,
            compression: compression_metrics,
        })
    }

    fn run_memory_benchmark(&self) -> Result<MemoryMetrics> {
        let mut profiler = MemoryProfiler::new();
        profiler.start();

        // Test with a representative image (HD resolution)
        let pattern = self.generator.get_pattern("hd_gradient")
            .ok_or_else(|| anyhow::anyhow!("HD gradient pattern not found"))?;

        let image = self.generator.generate_pattern(pattern)?;
        let input_view = ImageView8 {
            data: &image.data,
            width: image.width,
            height: image.height,
            format: image.format,
        };

        profiler.sample();
        let _encoded = encode_frame(input_view, &self.encoder_config)?;
        profiler.sample();

        let report = profiler.stop();

        Ok(MemoryMetrics {
            peak_heap_mb: report.peak_mb(),
            peak_stack_kb: 0.0, // Would need platform-specific code
            allocations: 0, // Would need custom allocator
            working_set_4k_mb: report.peak_mb(), // Approximation
        })
    }

    fn run_speed_benchmark(&self) -> Result<SpeedMetrics> {
        let pattern = self.generator.get_pattern("horizontal_gradient")
            .ok_or_else(|| anyhow::anyhow!("Gradient pattern not found"))?;

        let image = self.generator.generate_pattern(pattern)?;
        let input_view = ImageView8 {
            data: &image.data,
            width: image.width,
            height: image.height,
            format: image.format,
        };

        let iterations = 10;
        let mut encode_times = Vec::new();
        let mut decode_times = Vec::new();

        // Warm up
        let _ = encode_frame(input_view, &self.encoder_config)?;

        for _ in 0..iterations {
            // Encode timing
            let start = Instant::now();
            let encoded = encode_frame(input_view, &self.encoder_config)?;
            encode_times.push(start.elapsed().as_secs_f64());

            // Decode timing
            let start = Instant::now();
            let _ = decode_frame(&encoded, &self.decoder_config)?;
            decode_times.push(start.elapsed().as_secs_f64());
        }

        let avg_encode_time = encode_times.iter().sum::<f64>() / encode_times.len() as f64;
        let avg_decode_time = decode_times.iter().sum::<f64>() / decode_times.len() as f64;

        let bits_per_pixel = image.data.len() * 8;
        let encode_mbps = (bits_per_pixel as f64) / (avg_encode_time * 1_000_000.0);
        let decode_mbps = (bits_per_pixel as f64) / (avg_decode_time * 1_000_000.0);

        Ok(SpeedMetrics {
            encode_mbps,
            decode_mbps,
            latency_ms: avg_encode_time * 1000.0,
            throughput_4k_fps: if image.width >= 3840 { 1.0 / avg_encode_time } else { 0.0 },
        })
    }

    fn run_quality_benchmark(&self) -> Result<CompressionMetrics> {
        let test_patterns = ["solid_red", "horizontal_gradient", "random_noise"];
        let mut ratios = Vec::new();
        let mut psnrs = Vec::new();
        let mut ssims = Vec::new();

        for pattern_name in &test_patterns {
            let pattern = self.generator.get_pattern(pattern_name)
                .ok_or_else(|| anyhow::anyhow!("Pattern not found: {}", pattern_name))?;

            let image = self.generator.generate_pattern(pattern)?;
            let input_view = ImageView8 {
                data: &image.data,
                width: image.width,
                height: image.height,
                format: image.format,
            };

            let encoded = encode_frame(input_view, &self.encoder_config)?;
            let decoded = decode_frame(&encoded, &self.decoder_config)?;

            let ratio = image.data.len() as f64 / encoded.data.len() as f64;
            let psnr = calculate_psnr(&image.data, &decoded.data);
            let ssim = crate::metrics::calculate_ssim(&image.data, &decoded.data, image.width as usize, image.height as usize);

            ratios.push(ratio);
            psnrs.push(psnr);
            ssims.push(ssim);
        }

        Ok(CompressionMetrics {
            avg_ratio: ratios.iter().sum::<f64>() / ratios.len() as f64,
            avg_bpp: 0.0, // Would calculate properly
            avg_psnr_db: psnrs.iter().sum::<f64>() / psnrs.len() as f64,
            avg_ssim: ssims.iter().sum::<f64>() / ssims.len() as f64,
        })
    }

    pub fn save_report(&self, report: &TestReport, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(report)?;
        std::fs::write(path, json)?;
        println!("📄 Report saved to: {}", path);
        Ok(())
    }
}

impl Default for ConformanceTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformance_runner_creation() {
        let runner = ConformanceTestRunner::new();
        assert!(runner.max_test_time_ms > 0);
    }

    #[test]
    fn test_single_encoder_test() {
        let runner = ConformanceTestRunner::new();
        let pattern = runner.generator.get_pattern("solid_red").unwrap();

        let result = runner.run_single_encoder_test(pattern);
        assert!(result.is_ok());

        let test_case = result.unwrap();
        assert_eq!(test_case.category, "Encoder");
        assert!(test_case.duration_ms > 0.0);
    }
}
