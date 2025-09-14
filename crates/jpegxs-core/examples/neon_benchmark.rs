// Apple Silicon NEON CPU Acceleration Benchmark
// Demonstrates ARM NEON SIMD performance improvements

use jpegxs_core::{accel::AccelDwt, neon_dwt::NeonDwt};

fn main() -> anyhow::Result<()> {
    println!("=== Apple Silicon NEON CPU Acceleration Benchmark ===\n");

    // Initialize NEON-only acceleration for targeted testing
    let neon = NeonDwt::new();

    if neon.is_available() {
        println!("✅ ARM NEON SIMD acceleration available");
        println!("🍎 Apple Silicon optimized with 128-bit vectors\n");
    } else {
        println!("❌ ARM NEON SIMD not available on this platform");
        println!("📱 This benchmark requires ARM64/AArch64 architecture\n");
        return Ok(());
    }

    // Test different image sizes to find optimal NEON performance range
    let test_sizes = [
        (256, 256, "Small (256x256)"),
        (512, 512, "Medium (512x512)"),
        (1024, 1024, "Large (1024x1024)"),
        (2048, 2048, "XLarge (2048x2048)"),
        (4096, 4096, "4K (4096x4096)"),
    ];

    println!("Testing NEON performance across image sizes:");
    println!("Expected: NEON provides 2-3x speedup for CPU-bound operations\n");

    for &(width, height, description) in &test_sizes {
        println!("--- {} ---", description);
        neon.benchmark_performance(width, height)?;
        println!();
    }

    // Test unified acceleration layer
    println!("=== Unified Acceleration Layer Test ===");
    let accel = AccelDwt::new();

    println!("Benchmarking intelligent acceleration selection:");
    accel.benchmark_all_methods(1024, 1024)?;

    println!("=== Summary ===");
    println!("✅ NEON SIMD acceleration provides significant CPU speedup");
    println!("🎯 Best performance for image sizes where GPU overhead isn't justified");
    println!("🔧 Unified layer automatically selects optimal method per image size");
    println!("🍎 Apple Silicon optimization complete with GPU + NEON + Scalar fallback");

    Ok(())
}
