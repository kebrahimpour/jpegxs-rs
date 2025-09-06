# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Core JPEG XS Implementation**
  - 5/3 lifting-based DWT (Discrete Wavelet Transform) forward/inverse
  - Basic quantization and dequantization algorithms
  - Working encode/decode pipeline with roundtrip capability
  - YUV 422p/444p pixel format support with file I/O
  - Bit-level I/O operations for precise bitstream handling
  
- **Comprehensive Test Suite**
  - Encode/decode roundtrip tests
  - DWT forward/inverse transform validation
  - Quantization/dequantization accuracy tests
  - YUV file I/O integrity verification
  - Bit-level I/O operations testing
  - All 5 unit tests passing with full coverage

- **Project Infrastructure** 
  - Initial project structure with Rust workspace (4 crates)
  - Docker development environment configuration
  - GitHub Actions CI/CD pipeline with multi-OS support
  - Pre-commit hooks for code quality (rustfmt, clippy, tests)
  - Reference implementation (TangKii/jxs) as git submodule
  - Security audit integration and dependency checking

- **Libraries and Tools**
  - `jpegxs-core`: Core algorithms with working codec implementation
  - `jpegxs-io`: I/O utilities with YUV and bitstream handling
  - `jpegxs-cli`: Command-line interface with all commands functional
  - `jpegxs-ffi`: FFI library prepared for cross-implementation validation

### Technical Implementation
- Modular architecture supporting configurable quality parameters
- IEEE 754 floating-point precision in transform pipeline  
- Proper error handling with `anyhow::Result` throughout
- Memory-efficient buffer management for large images
- Cross-platform compatibility (Linux, macOS, Windows)

### Infrastructure
- Dual licensing: MIT/Apache-2.0
- Support for YUV422p 8-bit format (YUV444p implemented)
- Artifact upload for built binaries (fixed deprecated actions)
- Branch protection and code review workflows prepared

### Documentation
- Comprehensive README with development setup
- CHANGELOG tracking all implemented features
- Code documentation for all public APIs

## [0.1.0] - TBD

Initial release (planned)