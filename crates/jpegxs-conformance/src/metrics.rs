use std::time::Instant;
use memory_stats::memory_stats;

pub struct MemoryProfiler {
    baseline: Option<usize>,
    peak: usize,
    samples: Vec<usize>,
}

impl MemoryProfiler {
    pub fn new() -> Self {
        Self {
            baseline: None,
            peak: 0,
            samples: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        if let Some(usage) = memory_stats() {
            self.baseline = Some(usage.physical_mem);
            self.peak = usage.physical_mem;
        }
    }

    pub fn sample(&mut self) {
        if let Some(usage) = memory_stats() {
            let current = usage.physical_mem;
            self.samples.push(current);
            if current > self.peak {
                self.peak = current;
            }
        }
    }

    pub fn stop(&mut self) -> MemoryReport {
        let baseline = self.baseline.unwrap_or(0);
        let peak_usage = self.peak.saturating_sub(baseline);
        let avg_usage = if !self.samples.is_empty() {
            let sum: usize = self.samples.iter()
                .map(|&s| s.saturating_sub(baseline))
                .sum();
            sum / self.samples.len()
        } else {
            0
        };

        MemoryReport {
            peak_bytes: peak_usage,
            average_bytes: avg_usage,
            samples: self.samples.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub peak_bytes: usize,
    pub average_bytes: usize,
    pub samples: usize,
}

impl MemoryReport {
    pub fn peak_mb(&self) -> f64 {
        self.peak_bytes as f64 / (1024.0 * 1024.0)
    }

    pub fn average_mb(&self) -> f64 {
        self.average_bytes as f64 / (1024.0 * 1024.0)
    }
}

pub struct SpeedProfiler {
    start_time: Option<Instant>,
    operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub duration_ns: u128,
    pub bytes_processed: usize,
}

impl SpeedProfiler {
    pub fn new() -> Self {
        Self {
            start_time: None,
            operations: Vec::new(),
        }
    }

    pub fn start_operation(&mut self, name: &str) -> OperationTimer {
        OperationTimer {
            name: name.to_string(),
            start: Instant::now(),
            bytes: 0,
        }
    }

    pub fn record(&mut self, timer: OperationTimer) {
        let duration = timer.start.elapsed().as_nanos();
        self.operations.push(Operation {
            name: timer.name,
            duration_ns: duration,
            bytes_processed: timer.bytes,
        });
    }

    pub fn report(&self) -> SpeedReport {
        let total_duration: u128 = self.operations.iter()
            .map(|op| op.duration_ns)
            .sum();
        
        let total_bytes: usize = self.operations.iter()
            .map(|op| op.bytes_processed)
            .sum();

        let throughput_mbps = if total_duration > 0 {
            (total_bytes as f64 * 8.0 * 1_000_000_000.0) / 
            (total_duration as f64 * 1_000_000.0)
        } else {
            0.0
        };

        SpeedReport {
            total_operations: self.operations.len(),
            total_duration_ms: total_duration as f64 / 1_000_000.0,
            total_bytes,
            throughput_mbps,
            operations: self.operations.clone(),
        }
    }
}

pub struct OperationTimer {
    name: String,
    start: Instant,
    bytes: usize,
}

impl OperationTimer {
    pub fn set_bytes(&mut self, bytes: usize) {
        self.bytes = bytes;
    }
}

#[derive(Debug, Clone)]
pub struct SpeedReport {
    pub total_operations: usize,
    pub total_duration_ms: f64,
    pub total_bytes: usize,
    pub throughput_mbps: f64,
    pub operations: Vec<Operation>,
}

pub fn calculate_psnr(original: &[u8], compressed: &[u8]) -> f64 {
    if original.len() != compressed.len() {
        return 0.0;
    }

    let mse: f64 = original.iter()
        .zip(compressed.iter())
        .map(|(&o, &c)| {
            let diff = o as f64 - c as f64;
            diff * diff
        })
        .sum::<f64>() / original.len() as f64;

    if mse == 0.0 {
        f64::INFINITY
    } else {
        20.0 * (255.0_f64).log10() - 10.0 * mse.log10()
    }
}

pub fn calculate_ssim(original: &[u8], compressed: &[u8], width: usize, height: usize) -> f64 {
    // Simplified SSIM calculation
    // In production, use a proper SSIM library
    
    const C1: f64 = 6.5025;  // (0.01 * 255)^2
    const C2: f64 = 58.5225; // (0.03 * 255)^2
    
    if original.len() != compressed.len() || original.len() != width * height {
        return 0.0;
    }

    let n = original.len() as f64;
    
    let mean_x: f64 = original.iter().map(|&x| x as f64).sum::<f64>() / n;
    let mean_y: f64 = compressed.iter().map(|&y| y as f64).sum::<f64>() / n;
    
    let var_x: f64 = original.iter()
        .map(|&x| {
            let diff = x as f64 - mean_x;
            diff * diff
        })
        .sum::<f64>() / n;
    
    let var_y: f64 = compressed.iter()
        .map(|&y| {
            let diff = y as f64 - mean_y;
            diff * diff
        })
        .sum::<f64>() / n;
    
    let cov_xy: f64 = original.iter()
        .zip(compressed.iter())
        .map(|(&x, &y)| {
            (x as f64 - mean_x) * (y as f64 - mean_y)
        })
        .sum::<f64>() / n;
    
    let numerator = (2.0 * mean_x * mean_y + C1) * (2.0 * cov_xy + C2);
    let denominator = (mean_x * mean_x + mean_y * mean_y + C1) * (var_x + var_y + C2);
    
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_profiler() {
        let mut profiler = MemoryProfiler::new();
        profiler.start();
        
        // Allocate some memory
        let _data: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
        profiler.sample();
        
        let report = profiler.stop();
        assert!(report.samples > 0);
    }

    #[test]
    fn test_speed_profiler() {
        let mut profiler = SpeedProfiler::new();
        
        let mut timer = profiler.start_operation("test");
        timer.set_bytes(1024);
        std::thread::sleep(std::time::Duration::from_millis(10));
        profiler.record(timer);
        
        let report = profiler.report();
        assert_eq!(report.total_operations, 1);
        assert!(report.total_duration_ms >= 10.0);
    }

    #[test]
    fn test_psnr_calculation() {
        let original = vec![100; 100];
        let compressed = vec![100; 100];
        assert!(calculate_psnr(&original, &compressed).is_infinite());
        
        let compressed_lossy = vec![101; 100];
        let psnr = calculate_psnr(&original, &compressed_lossy);
        assert!(psnr > 40.0 && psnr < 50.0);
    }
}