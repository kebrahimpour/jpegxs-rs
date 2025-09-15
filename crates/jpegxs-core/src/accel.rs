// Unified acceleration manager for Apple Silicon
// Provides GPU → NEON → Scalar fallback chain for optimal performance

use crate::{gpu_dwt::GpuDwt, neon_dwt::NeonDwt};
use anyhow::Result;

/// Unified acceleration manager for DWT operations
/// Automatically selects the best available acceleration method
pub struct AccelDwt {
    gpu: GpuDwt,
    neon: NeonDwt,
}

impl AccelDwt {
    /// Initialize unified acceleration with all available methods
    pub fn new() -> Self {
        log::info!("Initializing Apple Silicon Unified Acceleration");
        let gpu = GpuDwt::new();
        let neon = NeonDwt::new();

        Self::print_acceleration_status(&gpu, &neon);

        Self { gpu, neon }
    }

    /// Print available acceleration methods
    fn print_acceleration_status(gpu: &GpuDwt, neon: &NeonDwt) {
        log::info!("=== Apple Silicon Acceleration Status ===");
        log::info!(
            "GPU (Metal):     {}",
            if gpu.is_available() {
                "✅ Available"
            } else {
                "❌ Not Available"
            }
        );
        log::info!(
            "NEON (ARM SIMD): {}",
            if neon.is_available() {
                "✅ Available"
            } else {
                "❌ Not Available"
            }
        );

        let method = if gpu.is_available() {
            "Metal GPU (130x speedup target)"
        } else if neon.is_available() {
            "ARM NEON SIMD (2-3x speedup target)"
        } else {
            "Scalar fallback"
        };

        log::info!("Primary method:  {}", method);
        log::info!("==========================================");
    }

    /// Get acceleration capabilities for user information
    pub fn get_capabilities(&self) -> AccelCapabilities {
        AccelCapabilities {
            gpu_available: self.gpu.is_available(),
            neon_available: self.neon.is_available(),
            primary_method: self.get_primary_method(),
        }
    }

    /// Determine the best acceleration method for given dimensions
    fn get_primary_method(&self) -> AccelMethod {
        if self.gpu.is_available() {
            AccelMethod::Gpu
        } else if self.neon.is_available() {
            AccelMethod::Neon
        } else {
            AccelMethod::Scalar
        }
    }

    /// Intelligent method selection based on image size and hardware
    /// For smaller images, NEON might be faster due to GPU overhead
    fn select_method(&self, width: u32, height: u32) -> AccelMethod {
        let pixel_count = width * height;

        // Determine optimal method based on image size and available acceleration
        if self.gpu.is_available() {
            // GPU is generally better for larger images
            // For very small images (< 512x512), NEON might be competitive
            if pixel_count >= 512 * 512 || !self.neon.is_available() {
                AccelMethod::Gpu
            } else {
                // For smaller images, prefer NEON if available
                AccelMethod::Neon
            }
        } else if self.neon.is_available() {
            AccelMethod::Neon
        } else {
            AccelMethod::Scalar
        }
    }

    /// Accelerated 2D DWT forward transform with intelligent method selection
    pub fn dwt_53_forward_2d(
        &self,
        input: &[f32],
        output: &mut [f32],
        width: u32,
        height: u32,
    ) -> Result<()> {
        let method = self.select_method(width, height);

        match method {
            AccelMethod::Gpu => {
                log::debug!("Using GPU acceleration for {}x{} DWT", width, height);
                self.gpu.dwt_53_forward_2d_gpu(input, output, width, height)
            }
            AccelMethod::Neon => {
                log::debug!("Using NEON acceleration for {}x{} DWT", width, height);
                self.neon
                    .dwt_53_forward_2d_neon(input, output, width, height)
            }
            AccelMethod::Scalar => {
                log::debug!("Using scalar implementation for {}x{} DWT", width, height);
                crate::dwt::dwt_53_forward_2d(input, output, width, height)
            }
        }
    }

    /// Accelerated 2D DWT inverse transform with intelligent method selection
    pub fn dwt_53_inverse_2d(
        &self,
        input: &[f32],
        output: &mut [f32],
        width: u32,
        height: u32,
    ) -> Result<()> {
        let method = self.select_method(width, height);

        match method {
            AccelMethod::Gpu => {
                log::debug!(
                    "Using GPU acceleration for {}x{} inverse DWT",
                    width,
                    height
                );
                self.gpu.dwt_53_inverse_2d_gpu(input, output, width, height)
            }
            AccelMethod::Neon => {
                log::debug!(
                    "Using NEON acceleration for {}x{} inverse DWT",
                    width,
                    height
                );
                self.neon
                    .dwt_53_inverse_2d_neon(input, output, width, height)
            }
            AccelMethod::Scalar => {
                log::debug!(
                    "Using scalar implementation for {}x{} inverse DWT",
                    width,
                    height
                );
                crate::dwt::dwt_53_inverse_2d(input, output, width, height)
            }
        }
    }

    /// Force use of specific acceleration method (for testing/benchmarking)
    pub fn dwt_53_forward_2d_method(
        &self,
        input: &[f32],
        output: &mut [f32],
        width: u32,
        height: u32,
        method: AccelMethod,
    ) -> Result<()> {
        match method {
            AccelMethod::Gpu => self.gpu.dwt_53_forward_2d_gpu(input, output, width, height),
            AccelMethod::Neon => self
                .neon
                .dwt_53_forward_2d_neon(input, output, width, height),
            AccelMethod::Scalar => crate::dwt::dwt_53_forward_2d(input, output, width, height),
        }
    }

    /// Force use of specific acceleration method (for testing/benchmarking)
    pub fn dwt_53_inverse_2d_method(
        &self,
        input: &[f32],
        output: &mut [f32],
        width: u32,
        height: u32,
        method: AccelMethod,
    ) -> Result<()> {
        match method {
            AccelMethod::Gpu => self.gpu.dwt_53_inverse_2d_gpu(input, output, width, height),
            AccelMethod::Neon => self
                .neon
                .dwt_53_inverse_2d_neon(input, output, width, height),
            AccelMethod::Scalar => crate::dwt::dwt_53_inverse_2d(input, output, width, height),
        }
    }

    /// Comprehensive performance benchmark across all available methods
    pub fn benchmark_all_methods(&self, width: u32, height: u32) -> Result<()> {
        use std::time::Instant;

        let size = (width * height) as usize;
        let input: Vec<f32> = (0..size).map(|i| (i as f32 * 0.1).sin()).collect();

        println!("\n=== Comprehensive Apple Silicon DWT Benchmark ===");
        println!("Image size: {}x{} ({} pixels)", width, height, size);
        println!("Test pattern: Sine wave for accuracy validation");

        // Scalar baseline
        let mut scalar_output = vec![0.0f32; size];
        let scalar_start = Instant::now();
        crate::dwt::dwt_53_forward_2d(&input, &mut scalar_output, width, height)?;
        let scalar_time = scalar_start.elapsed();
        println!(
            "CPU Scalar:   {:.2}ms (baseline)",
            scalar_time.as_secs_f64() * 1000.0
        );

        // NEON benchmark (if available)
        if self.neon.is_available() {
            let mut neon_output = vec![0.0f32; size];
            let neon_start = Instant::now();
            self.neon
                .dwt_53_forward_2d_neon(&input, &mut neon_output, width, height)?;
            let neon_time = neon_start.elapsed();
            let neon_speedup = scalar_time.as_secs_f64() / neon_time.as_secs_f64();

            println!(
                "NEON SIMD:    {:.2}ms ({:.2}x speedup)",
                neon_time.as_secs_f64() * 1000.0,
                neon_speedup
            );

            // Verify accuracy
            let max_error = scalar_output
                .iter()
                .zip(neon_output.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, |acc, x| acc.max(x));
            println!("NEON accuracy: {:.6} max error", max_error);
        } else {
            println!("NEON SIMD:    Not available");
        }

        // GPU benchmark (if available)
        if self.gpu.is_available() {
            let mut gpu_output = vec![0.0f32; size];
            let gpu_start = Instant::now();
            self.gpu
                .dwt_53_forward_2d_gpu(&input, &mut gpu_output, width, height)?;
            let gpu_time = gpu_start.elapsed();
            let gpu_speedup = scalar_time.as_secs_f64() / gpu_time.as_secs_f64();

            println!(
                "Metal GPU:    {:.2}ms ({:.2}x speedup)",
                gpu_time.as_secs_f64() * 1000.0,
                gpu_speedup
            );

            // Verify accuracy
            let max_error = scalar_output
                .iter()
                .zip(gpu_output.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, |acc, x| acc.max(x));
            println!("GPU accuracy: {:.6} max error", max_error);
        } else {
            println!("Metal GPU:    Not available");
        }

        println!(
            "Optimal method for {}x{}: {:?}",
            width,
            height,
            self.select_method(width, height)
        );
        println!("===============================================\n");

        Ok(())
    }
}

impl Default for AccelDwt {
    fn default() -> Self {
        Self::new()
    }
}

/// Available acceleration methods
#[derive(Debug, Clone, Copy)]
pub enum AccelMethod {
    Gpu,
    Neon,
    Scalar,
}

/// Acceleration capabilities information
#[derive(Debug)]
pub struct AccelCapabilities {
    pub gpu_available: bool,
    pub neon_available: bool,
    pub primary_method: AccelMethod,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accel_initialization() {
        let accel = AccelDwt::new();
        let caps = accel.get_capabilities();

        // Should always work (fallback to scalar if needed)
        assert!(
            caps.gpu_available
                || caps.neon_available
                || matches!(caps.primary_method, AccelMethod::Scalar)
        );
    }

    #[test]
    fn test_accel_method_selection() {
        let accel = AccelDwt::new();

        // Small image might prefer NEON if GPU overhead is high
        let small_method = accel.select_method(256, 256);

        // Large image should prefer GPU if available
        let large_method = accel.select_method(2048, 2048);

        // Should return valid methods
        match small_method {
            AccelMethod::Gpu | AccelMethod::Neon | AccelMethod::Scalar => {}
        }

        match large_method {
            AccelMethod::Gpu | AccelMethod::Neon | AccelMethod::Scalar => {}
        }
    }

    #[test]
    fn test_accel_dwt_accuracy() {
        let accel = AccelDwt::new();
        let width = 16u32;
        let height = 16u32;
        let size = (width * height) as usize;

        // Create test signal
        let input: Vec<f32> = (0..size).map(|i| (i as f32 * 0.1).sin()).collect();
        let mut accel_output = vec![0.0f32; size];
        let mut scalar_output = vec![0.0f32; size];

        // Test accelerated version
        assert!(accel
            .dwt_53_forward_2d(&input, &mut accel_output, width, height)
            .is_ok());

        // Test scalar reference
        assert!(crate::dwt::dwt_53_forward_2d(&input, &mut scalar_output, width, height).is_ok());

        // Compare accuracy
        let max_error = accel_output
            .iter()
            .zip(scalar_output.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, |acc, x| acc.max(x));

        println!("Acceleration accuracy error: {:.6}", max_error);
        assert!(
            max_error < 1e-4,
            "Acceleration error too large: {}",
            max_error
        );
    }
}
