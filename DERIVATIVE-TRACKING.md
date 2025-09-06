# Derivative Work Tracking

## Purpose
This document tracks which parts of the codebase are:
- **DERIVATIVE**: Ported/inspired from reference implementation (restricted license)
- **ORIGINAL**: Clean Rust implementation (full commercial rights)
- **PLANNED**: To be replaced with clean-room implementation

## Status Legend
- 🔴 **Derivative** - Cannot commercialize without RAND license
- 🟡 **Partially Original** - Mixed implementation
- 🟢 **Original** - Full commercial rights
- 🔵 **Planned** - Scheduled for clean-room rewrite

---

## Component Status

### Core Library (`jpegxs-core`)

#### DWT Module (`src/dwt.rs`)
- **Status**: 🟡 Partially Original
- **5/3 DWT Algorithm**: 🔴 Derivative (lifting scheme from reference)
- **Rust Implementation**: 🟢 Original (memory management, API design)
- **TODO**: Reimplement from ISO/IEC 21122 specification

#### Quantization Module (`src/quant.rs`)
- **Status**: 🟢 Original
- **Implementation**: Basic quantization written from scratch
- **Commercial**: ✅ Can be commercialized

#### Entropy Module (`src/entropy.rs`)
- **Status**: 🟢 Original
- **Implementation**: Placeholder, not yet implemented
- **Commercial**: ✅ Will be original when implemented

#### Color Module (`src/colors.rs`)
- **Status**: 🟢 Original  
- **Implementation**: Standard YUV conversions
- **Commercial**: ✅ Can be commercialized

#### Packet Module (`src/packet.rs`)
- **Status**: 🟢 Original
- **Implementation**: Placeholder structure
- **Commercial**: ✅ Will be original when implemented

### I/O Library (`jpegxs-io`)

#### Bit I/O (`src/bitio.rs`)
- **Status**: 🟢 Original
- **Implementation**: Written from scratch in Rust
- **Commercial**: ✅ Can be commercialized

#### YUV I/O (`src/yuv.rs`)
- **Status**: 🟢 Original
- **Implementation**: Standard file I/O operations
- **Commercial**: ✅ Can be commercialized

### CLI (`jpegxs-cli`)
- **Status**: 🟢 Original
- **Implementation**: Fully original Rust code
- **Commercial**: ✅ Can be commercialized

### FFI (`jpegxs-ffi`)
- **Status**: 🟢 Original
- **Implementation**: Rust FFI bindings
- **Commercial**: ✅ Can be commercialized

---

## Clean-Room Implementation Plan

### Phase 1 - High Priority (Blocks Commercialization)
1. **DWT 5/3 Transform** 
   - Source: ISO/IEC 21122-1:2019 Section 7.3
   - Timeline: 2 weeks
   - Approach: Implement from mathematical definition

2. **DWT 9/7 Transform**
   - Source: ISO/IEC 21122-1:2019 Section 7.3
   - Timeline: 2 weeks
   - Approach: Implement from mathematical definition

### Phase 2 - Medium Priority
3. **Entropy Coding**
   - Source: ISO/IEC 21122-1:2019 Section 8
   - Timeline: 3 weeks
   - Approach: Implement VLC tables from spec

4. **Packet Structure**
   - Source: ISO/IEC 21122-1:2019 Section 9
   - Timeline: 2 weeks
   - Approach: Implement bitstream format from spec

### Phase 3 - Optimization
5. **SIMD Optimizations**
   - Timeline: 4 weeks
   - Approach: Original optimizations

---

## Legal Safe Components for Immediate Commercialization

The following can be commercially licensed immediately:
- CLI interface and argument parsing
- Rust workspace structure and build system
- Test framework and benchmarks
- I/O utilities (file handling, bit operations)
- Basic quantization
- YUV file format handling
- Documentation and tooling

---

## How to Contribute Clean-Room Code

1. **DO NOT** look at reference implementation
2. **DO** read ISO/IEC 21122 standard documents
3. **DO** read academic papers about JPEG XS
4. **DO** implement from mathematical descriptions
5. **DO** mark new implementations as 🟢 Original

---

## Tracking Updates

Last Updated: 2024-12-06
Next Review: 2024-12-13

When replacing derivative code:
1. Create new implementation in separate file
2. Test against same test cases
3. Verify compliance with standard
4. Replace derivative version
5. Update this tracking document