# JPEG XS Performance Benchmarking Suite

This benchmark suite provides comprehensive performance comparisons between our JPEG XS implementation and other popular image codecs.

## 🚀 Quick Start

```bash
# Run simple benchmark with default settings
cargo run --bin simple_benchmark

# Run with custom quality and iterations
cargo run --bin simple_benchmark -- --quality 0.8 --iterations 5

# Run comprehensive codec comparison
cargo run --bin codec_comparison -- --detailed-report
```

## 📊 Available Benchmarks

### Simple Benchmark (`simple_benchmark`)
Fast performance comparison focusing on JPEG XS vs JPEG vs PNG:
- **Focus**: Compression ratios, encoding/decoding speed
- **Duration**: ~30 seconds
- **Output**: Detailed report with summary statistics

### Codec Comparison (`codec_comparison`)  
Comprehensive comparison across multiple codecs and quality levels:
- **Focus**: Multi-codec analysis, quality metrics (PSNR, SSIM)
- **Duration**: ~5-10 minutes
- **Output**: Detailed analysis with visual charts

## 🎯 Benchmark Categories

### Compression Performance
- **Compression Ratio**: Original size / Compressed size
- **File Size Reduction**: Absolute size savings
- **Quality Comparison**: PSNR and SSIM metrics

### Speed Performance
- **Encoding Speed**: Time to compress (ms) + Throughput (MB/s)
- **Decoding Speed**: Time to decompress (ms) + Throughput (MB/s)
- **Memory Usage**: Peak memory consumption (future)

### Quality Metrics
- **PSNR**: Peak Signal-to-Noise Ratio (dB)
- **SSIM**: Structural Similarity Index (0-1)
- **Visual Quality**: Perceptual quality assessment (future)

## 📁 Test Images

The benchmark suite automatically creates synthetic test images if none are found:

- **gradient_512x512.png**: Color gradients (good for compression testing)
- **landscape_640x480.png**: Photo-realistic synthetic scene
- **Custom Images**: Add your own PNG/JPEG files to `test_images/` directory

### Adding Custom Test Images

```bash
mkdir test_images
cp your_image.png test_images/
cp your_photo.jpg test_images/
# Benchmark will automatically include these
```

## 📈 Understanding Results

### Current JPEG XS Performance Issues

**⚠️ Important Note**: Current benchmark results show our JPEG XS implementation has performance issues:

```
JPEG XS: 0.8:1 compression ratio (worse than uncompressed!)
JPEG:    4.4:1 compression ratio
```

This indicates potential issues in our implementation:
1. **Compression Algorithm**: May not be optimally configured
2. **Quality Settings**: Quality parameter might not be working correctly
3. **Color Space Conversion**: RGB↔YUV conversion overhead
4. **Encoder Configuration**: Profile/Level settings may be suboptimal

### Expected JPEG XS Performance

Based on the standard, JPEG XS should achieve:
- **Compression Ratios**: 2:1 to 10:1 (depending on quality)
- **Encoding Speed**: Faster than JPEG (optimized for low latency)
- **Quality**: Visually lossless at moderate compression

### Improvement Areas

1. **Algorithm Optimization**:
   - Review quantization parameters
   - Optimize DWT implementation
   - Improve entropy coding

2. **Configuration Tuning**:
   - Quality parameter mapping
   - Profile/Level selection
   - Rate control implementation

3. **Performance Optimization**:
   - SIMD instructions usage
   - Memory allocation optimization
   - Parallel processing

## 🔧 Configuration Options

### Simple Benchmark Options

```bash
cargo run --bin simple_benchmark -- --help

Options:
  -i, --input-dir <INPUT_DIR>      Input directory [default: test_images]
  -o, --output-dir <OUTPUT_DIR>    Output directory [default: benchmark_results]  
  -q, --quality <QUALITY>          Quality level 0.1-1.0 [default: 0.9]
      --iterations <ITERATIONS>    Number of iterations [default: 3]
```

### Codec Comparison Options

```bash
cargo run --bin codec_comparison -- --help

Options:
  -c, --codecs <CODECS>...         Codecs to test [default: jpegxs jpeg png webp]
  -q, --quality-levels <LEVELS>... Quality levels [default: 0.5 0.7 0.9 0.95]
      --iterations <ITERATIONS>    Timing iterations [default: 5]
      --detailed-report            Generate comprehensive analysis
```

## 📊 Output Formats

### Generated Files

```
benchmark_results/
├── benchmark_results.json     # Machine-readable detailed results
├── benchmark_results.csv      # Spreadsheet-compatible data
├── BENCHMARK_REPORT.md        # Human-readable summary
└── charts/                    # Visual comparisons (codec_comparison only)
    ├── compression_ratios.svg
    ├── encoding_speeds.svg
    └── quality_metrics.svg
```

### JSON Structure

```json
{
  "image_name": "gradient_512x512",
  "original_size_kb": 136.6,
  "jpegxs_compressed_size_kb": 119.7,
  "jpegxs_compression_ratio": 1.1,
  "jpegxs_encode_time_ms": 89.54,
  "jpegxs_decode_time_ms": 45.32,
  "jpeg_compressed_size_kb": 19.0,
  "jpeg_compression_ratio": 7.2,
  "psnr": 42.5,
  "ssim": 0.95
}
```

## 🧪 Development Benchmarking

### Continuous Performance Monitoring

```bash
# Run benchmarks as part of development workflow
./scripts/run_benchmarks.sh

# Compare before/after performance
git checkout baseline_branch
cargo run --bin simple_benchmark -- --output-dir results_before
git checkout feature_branch  
cargo run --bin simple_benchmark -- --output-dir results_after
python scripts/compare_results.py results_before results_after
```

### Regression Testing

```bash
# Set performance baseline
cargo run --bin simple_benchmark -- --output-dir baseline_results

# Check for regressions
cargo run --bin simple_benchmark -- --output-dir current_results
# Results should not be significantly worse than baseline
```

## 📚 Research Applications

### Academic Benchmarking

The benchmark suite is suitable for:
- **Codec Comparison Studies**: Systematic performance evaluation
- **Algorithm Development**: Testing new compression techniques  
- **Standards Validation**: Verifying ISO/IEC 21122-1:2024 compliance
- **Performance Analysis**: Identifying optimization opportunities

### Industry Applications

- **Product Development**: Evaluating codec integration
- **Quality Assurance**: Ensuring consistent performance
- **Competitive Analysis**: Comparing against alternatives
- **System Optimization**: Finding performance bottlenecks

## 🔬 Advanced Analysis

### Performance Profiling

```bash
# Profile with perf (Linux)
perf record cargo run --release --bin simple_benchmark
perf report

# Profile with instruments (macOS)  
cargo install cargo-instruments
cargo instruments --release --bin simple_benchmark --bench
```

### Memory Analysis

```bash
# Memory usage profiling
cargo install cargo-valgrind
cargo valgrind run --bin simple_benchmark

# Heap profiling
cargo install cargo-profdata
cargo run --release --bin simple_benchmark
```

## 🎯 Next Steps

### Immediate Improvements Needed

1. **Fix JPEG XS Compression**: Current 0.8:1 ratio is worse than uncompressed
2. **Optimize Encoding Speed**: Currently slower than JPEG
3. **Add Quality Metrics**: Implement proper PSNR/SSIM calculation
4. **Expand Test Suite**: More diverse test images

### Future Enhancements

1. **Real Image Dataset**: Professional photography test suite
2. **Video Benchmarking**: Frame-by-frame video compression
3. **Parallel Processing**: Multi-threaded encoding benchmarks
4. **GPU Acceleration**: CUDA/OpenCL performance testing
5. **Network Simulation**: Low-latency streaming scenarios

## 📝 Contributing

To add new benchmarks or improve existing ones:

1. **Add Test Cases**: Place images in `test_images/`
2. **Extend Metrics**: Modify benchmark structs in `src/`
3. **Add Codecs**: Implement new codec backends
4. **Improve Reports**: Enhance markdown/JSON output

## 📞 Support

For questions about benchmarking results or methodology:
- **Technical Issues**: Create GitHub issue with benchmark output
- **Performance Questions**: Include system specifications and timing results  
- **Codec Comparisons**: Share test images and configuration used

---

*This benchmarking suite is part of the JPEG XS Rust implementation project. Results help identify performance improvements and validate our implementation against industry standards.*