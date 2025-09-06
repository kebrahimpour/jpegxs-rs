# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with Rust workspace
- Docker development environment configuration
- GitHub Actions CI/CD pipeline with multi-OS support
- Pre-commit hooks for code quality (rustfmt, clippy, tests)
- Core library structure (`jpegxs-core`) with type definitions
- I/O utilities library (`jpegxs-io`) for bit operations and YUV handling
- Command-line interface (`jpegxs-cli`) with encode/decode/validate commands
- FFI library (`jpegxs-ffi`) for cross-implementation validation
- Reference implementation setup script for TangKii/jxs
- Comprehensive testing and benchmarking structure

### Infrastructure
- Dual licensing: MIT/Apache-2.0
- Support for YUV422p 8-bit format (YUV444p planned)
- Security audit integration in CI
- Artifact upload for built binaries
- Cross-platform build support (Linux, macOS, Windows)

### Documentation
- Project README with setup instructions
- CHANGELOG for tracking project evolution
- Code of conduct and contributing guidelines

## [0.1.0] - TBD

Initial release (planned)