# Public vs Commercial Architecture Design

## Overview

This document outlines the architecture for maintaining both public open-source and commercial versions of the JPEG XS codec, maximizing code reuse while protecting commercial value.

## Repository Structure

### Public Repository (GitHub)
```
github.com/yourusername/jpeg-xs-rs/
├── crates/
│   ├── jpegxs-core/          # Core codec (trait-based)
│   ├── jpegxs-cli/           # Basic CLI
│   ├── jpegxs-ffi/           # C API bindings
│   └── jpegxs-conformance/   # Basic tests
├── docs/                      # Public documentation
├── examples/                  # Usage examples
├── LICENSE                    # MIT/Apache-2.0
└── README.md                  # Public facing docs
```

### Commercial Repository (Private)
```
private-git/jpeg-xs-pro/
├── crates/
│   ├── jpegxs-pro-core/      # Optimized implementations
│   ├── jpegxs-pro-simd/      # SIMD optimizations
│   ├── jpegxs-pro-gpu/       # GPU acceleration
│   ├── jpegxs-enterprise/    # Enterprise features
│   └── jpegxs-conformance-full/ # Full ISO test suite
├── LICENSE                    # Commercial license
└── PATENT_LICENSE            # Patent indemnification
```

## Architecture Design

### Core Trait System

The public repository defines traits that commercial implementations fulfill:

```rust
// Public: crates/jpegxs-core/src/traits.rs
pub trait DwtTransform {
    fn forward(&self, data: &mut [i32], width: usize, height: usize);
    fn inverse(&self, data: &mut [i32], width: usize, height: usize);
}

pub trait EntropyEncoder {
    fn encode(&self, coefficients: &[i32]) -> Vec<u8>;
    fn decode(&self, bitstream: &[u8]) -> Vec<i32>;
}

pub trait Optimizer {
    fn optimize(&self, transform: &dyn DwtTransform) -> Box<dyn DwtTransform>;
}
```

### Public Implementation

```rust
// Public: crates/jpegxs-core/src/dwt.rs
pub struct BasicDwt;

impl DwtTransform for BasicDwt {
    fn forward(&self, data: &mut [i32], width: usize, height: usize) {
        // Basic, unoptimized implementation
        dwt_53_2d_forward_basic(data, width, height);
    }
    
    fn inverse(&self, data: &mut [i32], width: usize, height: usize) {
        dwt_53_2d_inverse_basic(data, width, height);
    }
}
```

### Commercial Implementation

```rust
// Commercial: crates/jpegxs-pro-simd/src/dwt.rs
use jpegxs_core::DwtTransform;

pub struct SimdDwt {
    use_avx2: bool,
    use_neon: bool,
}

impl DwtTransform for SimdDwt {
    fn forward(&self, data: &mut [i32], width: usize, height: usize) {
        if self.use_avx2 {
            unsafe { dwt_53_2d_forward_avx2(data, width, height) }
        } else if self.use_neon {
            unsafe { dwt_53_2d_forward_neon(data, width, height) }
        } else {
            dwt_53_2d_forward_optimized(data, width, height)
        }
    }
    
    fn inverse(&self, data: &mut [i32], width: usize, height: usize) {
        // Similar optimized paths
    }
}
```

## Feature Comparison

| Feature | Public | Commercial | Implementation Strategy |
|---------|--------|------------|------------------------|
| **Core Codec** | ✓ | ✓ | Shared trait interface |
| **Basic CLI** | ✓ | ✓ | Extended in commercial |
| **C API** | ✓ | ✓ | Same interface, different backend |
| **SIMD Optimization** | ✗ | ✓ | Trait implementation |
| **GPU Acceleration** | ✗ | ✓ | Optional plugin |
| **Multi-threading** | Basic | Advanced | Thread pool vs work stealing |
| **Memory Pool** | ✗ | ✓ | Custom allocator |
| **Rate Control** | Basic | Advanced | Strategy pattern |
| **ISO Test Suite** | Partial | Full | Extended test crate |
| **Support** | Community | SLA | Separate infrastructure |

## Build System

### Public Build

```toml
# Cargo.toml (public)
[dependencies]
jpegxs-core = { version = "0.3" }

[features]
default = ["basic"]
basic = []
```

### Commercial Build

```toml
# Cargo.toml (commercial)
[dependencies]
jpegxs-core = { version = "0.3" }
jpegxs-pro-core = { version = "1.0" }
jpegxs-pro-simd = { version = "1.0" }

[features]
default = ["pro", "simd", "gpu"]
pro = ["jpegxs-pro-core"]
simd = ["jpegxs-pro-simd"]
gpu = ["jpegxs-pro-gpu"]
enterprise = ["all-features"]
```

## Runtime Selection

### Factory Pattern for Implementation Selection

```rust
// Public API
pub struct CodecBuilder {
    optimizer: Option<Box<dyn Optimizer>>,
}

impl CodecBuilder {
    pub fn new() -> Self {
        Self { optimizer: None }
    }
    
    pub fn with_optimizer(mut self, opt: Box<dyn Optimizer>) -> Self {
        self.optimizer = Some(opt);
        self
    }
    
    pub fn build(self) -> Codec {
        let dwt: Box<dyn DwtTransform> = if let Some(opt) = self.optimizer {
            opt.optimize(&BasicDwt)
        } else {
            Box::new(BasicDwt)
        };
        
        Codec { dwt }
    }
}

// Commercial usage
let codec = CodecBuilder::new()
    .with_optimizer(Box::new(SimdOptimizer::new()))
    .build();
```

## License Strategy

### Public Repository
```
SPDX-License-Identifier: MIT OR Apache-2.0
```

### Commercial Repository
```
SPDX-License-Identifier: LicenseRef-Commercial

Commercial License Agreement
Copyright (c) 2025 Your Company

This software requires a commercial license for use.
Contact: licensing@yourcompany.com
```

## API Compatibility

### Ensuring Drop-in Replacement

```rust
// Public crate
pub fn encode(image: &[u8], config: Config) -> Result<Vec<u8>> {
    let codec = get_default_codec();
    codec.encode(image, config)
}

// Commercial crate (same API)
pub fn encode(image: &[u8], config: Config) -> Result<Vec<u8>> {
    let codec = get_optimized_codec(); // Different implementation
    codec.encode(image, config)
}
```

## Performance Targets

### Public Version
- Encoding: 20-30 Mbps
- Memory: 150-200 MB for 4K
- Quality: 30-35 dB PSNR
- Compliance: 60-70%

### Commercial Version
- Encoding: 100+ Mbps (3-5x faster)
- Memory: <75 MB for 4K (50% less)
- Quality: 45+ dB PSNR
- Compliance: 100% ISO certified

## Development Workflow

### Dual Development Process

1. **Core Features**: Develop in public first
2. **Optimizations**: Implement in commercial
3. **Bug Fixes**: Apply to both
4. **Tests**: Basic public, comprehensive commercial

### Git Strategy

```bash
# Public repository (origin)
git remote add origin git@github.com:user/jpeg-xs-rs.git

# Commercial repository (commercial)
git remote add commercial git@private:jpeg-xs-pro.git

# Sync core changes
git checkout main
git pull origin main
git checkout commercial-main
git merge main --no-ff
git push commercial commercial-main
```

## Security Considerations

### Code Protection

1. **No optimizations in public**: Keep SIMD/GPU private
2. **Obfuscation**: Consider for critical paths
3. **License checks**: Runtime validation
4. **Watermarking**: Embed customer info

### Patent Protection

```rust
// Commercial only
#[cfg(feature = "commercial")]
fn validate_license() -> Result<()> {
    let license = read_license_file()?;
    let machine_id = get_machine_id()?;
    verify_license(&license, &machine_id)?;
    check_patent_coverage(&license)?;
    Ok(())
}
```

## Migration Path

### For Public Users

```rust
// Easy upgrade path
// Before (public):
use jpegxs_core::Codec;
let codec = Codec::new();

// After (commercial):
use jpegxs_pro::Codec; // Drop-in replacement
let codec = Codec::new(); // Same API, faster implementation
```

## Commercial Features

### 1. Enterprise API

```rust
pub struct EnterpriseCodec {
    thread_pool: ThreadPool,
    memory_pool: MemoryPool,
    gpu_context: Option<GpuContext>,
    metrics: MetricsCollector,
}

impl EnterpriseCodec {
    pub fn encode_batch(&self, images: &[Image]) -> Vec<Result<Vec<u8>>>
    pub fn encode_stream(&self, stream: impl Stream<Item=Image>) -> impl Stream<Item=Vec<u8>>
    pub fn get_metrics(&self) -> PerformanceMetrics
}
```

### 2. Cloud Integration

```rust
#[cfg(feature = "cloud")]
pub struct CloudCodec {
    pub fn encode_s3(&self, bucket: &str, key: &str) -> Result<()>
    pub fn encode_distributed(&self, cluster: &ClusterConfig) -> Result<()>
}
```

## Release Strategy

### Version Alignment

| Public Version | Commercial Version | Notes |
|---------------|-------------------|-------|
| 0.3.0 | 1.0.0 | Initial commercial release |
| 0.4.0 | 1.1.0 | Feature parity + optimizations |
| 1.0.0 | 2.0.0 | Stable public API |

### Release Process

1. **Public Release**: Monthly updates
2. **Commercial Release**: Quarterly with SLA
3. **Security Updates**: Both immediately
4. **Breaking Changes**: Major version only

## Support Model

### Public Support
- GitHub Issues
- Community Discord
- Stack Overflow
- Best effort response

### Commercial Support
- Dedicated support portal
- 24h response SLA
- Phone/video support
- Custom development
- Training available

## Success Metrics

### Public Repository
- GitHub stars > 1000
- Monthly downloads > 10K
- Active contributors > 10
- Issue response < 1 week

### Commercial Product
- Paying customers > 20
- ARR > $500K
- Customer satisfaction > 90%
- Performance lead > 3x

## Implementation Timeline

### Phase 1: Foundation (Weeks 1-2)
- Set up private repository
- Implement trait system
- Basic commercial features

### Phase 2: Optimization (Weeks 3-4)
- SIMD implementations
- Memory optimizations
- Benchmark suite

### Phase 3: Differentiation (Weeks 5-6)
- GPU acceleration
- Enterprise features
- Full conformance

### Phase 4: Launch (Week 7-8)
- Documentation
- Sales materials
- Support infrastructure
- Initial customers

---

**Document Version**: 1.0  
**Created**: 2025-09-12  
**Next Review**: 2025-09-26