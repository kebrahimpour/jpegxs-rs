// Apple Silicon Performance Benchmark
// Tests CPU vs GPU DWT performance on M1 Max

use jpegxs_core::gpu_dwt::GpuDwt;

fn main() -> anyhow::Result<()> {
    println!("🚀 Apple Silicon M1 Max JPEG XS Performance Benchmark");
    println!("===================================================");

    // Initialize GPU acceleration
    let gpu_dwt = GpuDwt::new();

    if gpu_dwt.is_available() {
        println!("✅ GPU Acceleration: Available (32-core Apple M1 Max GPU)");
    } else {
        println!("❌ GPU Acceleration: Not Available");
    }

    // Test different image sizes
    let test_sizes = vec![(512, 512), (1024, 1024), (2048, 2048), (4096, 4096)];

    for (width, height) in test_sizes {
        println!(
            "\n🔄 Testing {}x{} image ({}MB)",
            width,
            height,
            (width * height * 4) as f64 / 1_000_000.0
        );

        // Run benchmark
        gpu_dwt.benchmark_performance(width, height)?;
    }

    println!("\n🎯 Apple Silicon Optimization Opportunities:");
    println!("• Unified Memory: 64GB shared between CPU/GPU");
    println!("• ARM NEON SIMD: 128-bit vector processing");
    println!("• Metal Performance Shaders: GPU compute optimization");
    println!("• P/E Core Threading: 8P + 2E cores for workload distribution");

    Ok(())
}
