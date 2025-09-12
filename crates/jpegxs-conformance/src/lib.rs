pub mod metrics;
pub mod conformance;
pub mod benchmarks;
pub mod reference;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub timestamp: String,
    pub version: String,
    pub conformance: ConformanceResults,
    pub performance: PerformanceResults,
    pub comparison: ComparisonResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformanceResults {
    pub decoder_tests: TestSuite,
    pub encoder_tests: TestSuite,
    pub bitstream_tests: TestSuite,
    pub compliance_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub details: Vec<TestCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub category: String,
    pub status: TestStatus,
    pub message: Option<String>,
    pub duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestStatus {
    Pass,
    Fail,
    Skip,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResults {
    pub memory: MemoryMetrics,
    pub speed: SpeedMetrics,
    pub compression: CompressionMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub peak_heap_mb: f64,
    pub peak_stack_kb: f64,
    pub allocations: usize,
    pub working_set_4k_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedMetrics {
    pub encode_mbps: f64,
    pub decode_mbps: f64,
    pub latency_ms: f64,
    pub throughput_4k_fps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetrics {
    pub avg_ratio: f64,
    pub avg_bpp: f64,
    pub avg_psnr_db: f64,
    pub avg_ssim: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResults {
    pub reference_implementation: String,
    pub compression_delta: f64,  // Percentage better/worse
    pub speed_delta: f64,        // Percentage faster/slower
    pub memory_delta: f64,        // Percentage more/less
    pub quality_delta: f64,       // PSNR difference in dB
}

pub trait ConformanceTest {
    fn name(&self) -> &str;
    fn category(&self) -> &str;
    fn run(&self) -> Result<TestCase>;
}

pub trait Benchmark {
    fn name(&self) -> &str;
    fn setup(&mut self) -> Result<()>;
    fn run(&mut self) -> Result<Vec<f64>>;
    fn teardown(&mut self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_serialization() {
        let report = TestReport {
            timestamp: "2025-09-12T10:00:00Z".to_string(),
            version: "0.2.0".to_string(),
            conformance: ConformanceResults {
                decoder_tests: TestSuite {
                    total: 10,
                    passed: 9,
                    failed: 1,
                    skipped: 0,
                    details: vec![],
                },
                encoder_tests: TestSuite {
                    total: 10,
                    passed: 10,
                    failed: 0,
                    skipped: 0,
                    details: vec![],
                },
                bitstream_tests: TestSuite {
                    total: 5,
                    passed: 5,
                    failed: 0,
                    skipped: 0,
                    details: vec![],
                },
                compliance_percentage: 96.0,
            },
            performance: PerformanceResults {
                memory: MemoryMetrics {
                    peak_heap_mb: 85.5,
                    peak_stack_kb: 128.0,
                    allocations: 1500,
                    working_set_4k_mb: 92.0,
                },
                speed: SpeedMetrics {
                    encode_mbps: 28.0,
                    decode_mbps: 45.0,
                    latency_ms: 12.5,
                    throughput_4k_fps: 15.0,
                },
                compression: CompressionMetrics {
                    avg_ratio: 2.2,
                    avg_bpp: 3.64,
                    avg_psnr_db: 31.15,
                    avg_ssim: 0.92,
                },
            },
            comparison: ComparisonResults {
                reference_implementation: "ISO Reference v1.0".to_string(),
                compression_delta: 4.8,
                speed_delta: -15.0,
                memory_delta: -8.0,
                quality_delta: 0.5,
            },
        };

        let json = serde_json::to_string_pretty(&report).unwrap();
        let _deserialized: TestReport = serde_json::from_str(&json).unwrap();
    }
}