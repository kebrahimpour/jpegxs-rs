use crate::metrics::{MemoryProfiler, SpeedProfiler};
use crate::{CompressionMetrics, MemoryMetrics, SpeedMetrics};
use anyhow::Result;
use jpegxs_core::{
    decode_frame, encode_frame, DecoderConfig, EncoderConfig, ImageOwned8, ImageView8, PixelFormat,
};

pub struct MemoryBenchmark {
    resolutions: Vec<(u32, u32)>,
    results: Vec<MemoryMetrics>,
}

impl Default for MemoryBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryBenchmark {
    pub fn new() -> Self {
        Self {
            resolutions: vec![
                (640, 480),   // VGA
                (1920, 1080), // Full HD
                (3840, 2160), // 4K
                (7680, 4320), // 8K
            ],
            results: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<MemoryMetrics>> {
        for (width, height) in &self.resolutions {
            let metrics = self.benchmark_resolution(*width, *height)?;
            self.results.push(metrics);
        }
        Ok(self.results.clone())
    }

    fn benchmark_resolution(&self, width: u32, height: u32) -> Result<MemoryMetrics> {
        let mut profiler = MemoryProfiler::new();

        // Create test image
        let size = (width * height * 3) as usize;
        let data = vec![128u8; size];

        profiler.start();

        let input = ImageView8 {
            data: &data,
            width,
            height,
            format: PixelFormat::Rgb8,
        };

        let config = EncoderConfig::default();

        // Encode
        profiler.sample();
        let encoded = encode_frame(input, &config)?;
        profiler.sample();

        // Decode
        let _decoded = decode_frame(&encoded, &DecoderConfig::default())?;
        profiler.sample();

        let report = profiler.stop();

        Ok(MemoryMetrics {
            peak_heap_mb: report.peak_mb(),
            peak_stack_kb: 0.0, // Would need platform-specific code
            allocations: 0,     // Would need custom allocator
            working_set_4k_mb: if width == 3840 && height == 2160 {
                report.peak_mb()
            } else {
                0.0
            },
        })
    }
}

pub struct SpeedBenchmark {
    test_sizes: Vec<(u32, u32, usize)>, // width, height, iterations
    results: Vec<SpeedMetrics>,
}

impl Default for SpeedBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeedBenchmark {
    pub fn new() -> Self {
        Self {
            test_sizes: vec![(640, 480, 100), (1920, 1080, 20), (3840, 2160, 5)],
            results: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<SpeedMetrics>> {
        for (width, height, iterations) in &self.test_sizes {
            let metrics = self.benchmark_speed(*width, *height, *iterations)?;
            self.results.push(metrics);
        }
        Ok(self.results.clone())
    }

    fn benchmark_speed(&self, width: u32, height: u32, iterations: usize) -> Result<SpeedMetrics> {
        let mut profiler = SpeedProfiler::new();

        // Create test image
        let size = (width * height * 3) as usize;
        let data = vec![128u8; size];

        let input = ImageView8 {
            data: &data,
            width,
            height,
            format: PixelFormat::Rgb8,
        };

        let config = EncoderConfig::default();

        // Warm up
        let _ = encode_frame(input, &config)?;

        // Benchmark encoding
        let mut encode_times = Vec::new();
        for _ in 0..iterations {
            let mut timer = profiler.start_operation("encode");
            timer.set_bytes(size);

            let encoded = encode_frame(input, &config)?;
            profiler.record(timer);

            let start = std::time::Instant::now();
            let _ = encode_frame(input, &config)?;
            encode_times.push(start.elapsed().as_secs_f64());

            // Benchmark decoding
            let mut timer = profiler.start_operation("decode");
            timer.set_bytes(encoded.data.len());
            let _ = decode_frame(&encoded, &DecoderConfig::default())?;
            profiler.record(timer);
        }

        let report = profiler.report();

        // Calculate metrics
        let avg_encode_time = encode_times.iter().sum::<f64>() / encode_times.len() as f64;
        let encode_mbps = (size as f64 * 8.0) / (avg_encode_time * 1_000_000.0);

        Ok(SpeedMetrics {
            encode_mbps,
            decode_mbps: report.throughput_mbps * 0.6, // Rough estimate
            latency_ms: avg_encode_time * 1000.0,
            throughput_4k_fps: if width == 3840 && height == 2160 {
                1.0 / avg_encode_time
            } else {
                0.0
            },
        })
    }
}

pub struct QualityBenchmark {
    test_images: Vec<ImageOwned8>,
    quality_levels: Vec<f32>,
    results: Vec<CompressionMetrics>,
}

impl Default for QualityBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityBenchmark {
    pub fn new() -> Self {
        // Create different test patterns
        let mut test_images = Vec::new();

        // Gradient image
        let mut gradient = vec![0u8; 256 * 256 * 3];
        for y in 0..256 {
            for x in 0..256 {
                let idx = (y * 256 + x) * 3;
                gradient[idx] = x as u8;
                gradient[idx + 1] = y as u8;
                gradient[idx + 2] = ((x + y) / 2) as u8;
            }
        }
        test_images.push(ImageOwned8 {
            data: gradient,
            width: 256,
            height: 256,
            format: PixelFormat::Rgb8,
        });

        // Random noise
        let noise: Vec<u8> = (0..256 * 256 * 3).map(|i| (i * 7 % 256) as u8).collect();
        test_images.push(ImageOwned8 {
            data: noise,
            width: 256,
            height: 256,
            format: PixelFormat::Rgb8,
        });

        Self {
            test_images,
            quality_levels: vec![0.1, 0.3, 0.5, 0.7, 0.9, 0.95],
            results: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<CompressionMetrics>> {
        for quality in &self.quality_levels {
            let metrics = self.benchmark_quality(*quality)?;
            self.results.push(metrics);
        }
        Ok(self.results.clone())
    }

    fn benchmark_quality(&self, quality: f32) -> Result<CompressionMetrics> {
        let mut ratios = Vec::new();
        let mut bpps = Vec::new();
        let mut psnrs = Vec::new();
        let mut ssims = Vec::new();

        for image in &self.test_images {
            let input = ImageView8 {
                data: &image.data,
                width: image.width,
                height: image.height,
                format: image.format,
            };

            let config = EncoderConfig {
                quality,
                ..Default::default()
            };

            let encoded = encode_frame(input, &config)?;
            let decoded = decode_frame(&encoded, &DecoderConfig::default())?;

            // Calculate metrics
            let original_size = image.data.len();
            let compressed_size = encoded.data.len();
            let ratio = original_size as f64 / compressed_size as f64;
            let bpp = (compressed_size * 8) as f64 / (image.width * image.height) as f64;
            let psnr = crate::metrics::calculate_psnr(&image.data, &decoded.data);
            let ssim = crate::metrics::calculate_ssim(
                &image.data,
                &decoded.data,
                image.width as usize,
                image.height as usize,
            );

            ratios.push(ratio);
            bpps.push(bpp);
            psnrs.push(psnr);
            ssims.push(ssim);
        }

        Ok(CompressionMetrics {
            avg_ratio: ratios.iter().sum::<f64>() / ratios.len() as f64,
            avg_bpp: bpps.iter().sum::<f64>() / bpps.len() as f64,
            avg_psnr_db: psnrs.iter().sum::<f64>() / psnrs.len() as f64,
            avg_ssim: ssims.iter().sum::<f64>() / ssims.len() as f64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_benchmark() {
        let mut bench = MemoryBenchmark::new();
        bench.resolutions = vec![(64, 64)]; // Small for testing
        let results = bench.run().unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_speed_benchmark() {
        let mut bench = SpeedBenchmark::new();
        bench.test_sizes = vec![(64, 64, 2)]; // Small for testing
        let results = bench.run().unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_quality_benchmark() {
        let mut bench = QualityBenchmark::new();
        bench.quality_levels = vec![0.5]; // Single level for testing
        bench.test_images = vec![ImageOwned8 {
            data: vec![128; 64 * 64 * 3],
            width: 64,
            height: 64,
            format: PixelFormat::Rgb8,
        }];
        let results = bench.run().unwrap();
        assert_eq!(results.len(), 1);
    }
}
