# JPEG XS Rust Implementation

A Rust implementation of the JPEG XS (ISO/IEC 21122) codec for low-latency, high-quality image and video compression.

## Project Structure

```
jpegxs-rs/
├── crates/
│   ├── jpegxs-core/    # Core encoding/decoding algorithms
│   ├── jpegxs-io/      # I/O utilities and bit operations
│   ├── jpegxs-cli/     # Command-line interface
│   └── jpegxs-ffi/     # FFI bindings for cross-validation
├── reference/          # Reference C implementation (TangKii/jxs)
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
└── tools/              # Utility scripts
```

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Docker and Docker Compose
- Git

### Quick Start

1. Clone the repository:
```bash
git clone https://github.com/kebrahimpour/jpegxs-rs.git
cd jpegxs-rs
```

2. Setup reference implementation:
```bash
chmod +x setup-fork.sh
./setup-fork.sh
```

3. Start development environment:
```bash
docker-compose up -d
docker-compose exec dev bash
```

4. Build the project:
```bash
cargo build --all-features
```

5. Run tests:
```bash
cargo test --all-features
```

## Usage

### Encoding
```bash
jpegxs encode -i input.yuv -o output.jxs -w 1920 -h 1080 --format yuv422p --quality 0.9
```

### Decoding
```bash
jpegxs decode -i input.jxs -o output.yuv
```

### Validation
```bash
jpegxs validate -i test.jxs --reference jxs
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Linting**: rustfmt and clippy checks
- **Testing**: Multi-OS testing (Linux, macOS, Windows)
- **Building**: Debug and release builds
- **Security**: Dependency audit
- **Artifacts**: Binary uploads for each platform

## Pre-commit Hooks

Install pre-commit hooks:
```bash
pre-commit install
```

This will run:
- Format checking (rustfmt)
- Linting (clippy)
- Tests
- Build verification

## License

**PROPRIETARY SOFTWARE - COMMERCIAL USE PROHIBITED**

This software is licensed under a proprietary license that restricts commercial use.

### Non-Commercial Use
This software is free for:
- Personal use
- Educational purposes  
- Academic research
- Open source projects (non-commercial)

### Commercial Use
**Commercial use requires a paid license.** This includes:
- Using in products or services that generate revenue
- Deployment in business environments
- Integration into proprietary/commercial software
- Providing services to third parties

### Obtaining a Commercial License
For commercial licensing options, contact:
- **Email**: k1.ebrahimpour@gmail.com
- **Available licenses**: Single Application, Enterprise, OEM, Source Code with redistribution

### License Terms
See the [LICENSE](LICENSE) file for complete terms and conditions.

⚠️ **WARNING**: Violation of license terms will result in immediate termination of rights and potential legal action.

## Author

Keyvan Ebrahimpour

## Contributing

Contributions are welcome! Please ensure:
1. All tests pass
2. Code follows Rust formatting guidelines
3. Pre-commit hooks pass
4. Changes are documented in CHANGELOG.md