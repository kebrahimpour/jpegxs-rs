# Contributing to JPEG XS Codec

Thank you for your interest in contributing to our JPEG XS codec implementation!

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- Git

### Setup
```bash
git clone https://github.com/kebrahimpour/jpegxs-rs.git
cd jpegxs-rs
cargo build --all-features
cargo test
```

## 🎯 Ways to Contribute

### Code Contributions
- **Performance optimizations** (SIMD, memory usage)
- **New features** (see FUTURE-IDEAS.md)
- **Bug fixes** and improvements
- **Platform support** (Windows, ARM, etc.)
- **Testing** and validation

### Non-Code Contributions
- **Documentation** improvements
- **Examples** and tutorials
- **Issue reporting** with reproducible cases
- **Performance benchmarks**

## 📋 Development Process

### Before Contributing
1. **Check existing issues** to avoid duplicate work
2. **Open an issue** to discuss major changes
3. **Review** FUTURE-IDEAS.md for planned features

### Code Guidelines
- Follow Rust conventions (`cargo fmt`, `cargo clippy`)
- Add tests for new functionality
- Update documentation as needed
- Keep commits focused and well-described

### Testing
```bash
# Run all tests
cargo test --all-features

# Check formatting
cargo fmt --all --check

# Run linter
cargo clippy --all-targets --all-features
```

## 🏗️ Project Structure

- `crates/jpegxs-core` - Main codec implementation
- `crates/jpegxs-cli` - Command-line interface
- `crates/jpegxs-io` - I/O utilities
- `commercial/jpegxs-core-clean` - Clean-room implementation
- `testing/` - All testing infrastructure (benchmarks, fixtures, integration tests)

## 💡 Good First Issues

Looking for easy ways to contribute? Try:
- Adding more test cases
- Improving error messages
- Writing examples
- Optimizing memory allocations
- Adding documentation comments

## 📄 License

By contributing, you agree that your contributions will be licensed under the same terms as the project. See LICENSE-COMMERCIAL.md for details.

## 📧 Questions?

- **Open an issue** for bugs or feature requests
- **Email**: k1.ebrahimpour@gmail.com for complex questions
- **Check**: README.md and DEVELOPMENT-SUMMARY.md for technical details

## 🎉 Recognition

Contributors will be acknowledged in:
- CHANGELOG updates
- Release notes
- Project documentation

We appreciate all contributions, no matter how small!
