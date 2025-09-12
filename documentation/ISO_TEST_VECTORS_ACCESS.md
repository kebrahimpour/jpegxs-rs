# JPEG XS ISO Test Vectors and Reference Implementations

## Executive Summary

ISO test vectors for JPEG XS are **not freely available** - they must be purchased as part of the ISO/IEC 21122 standard series. However, there are several reference implementations and alternative test materials available for conformance testing.

## Official ISO Standards (Paid)

### ISO/IEC 21122-4 - Conformance Testing
- **Current Version**: ISO/IEC 21122-4:2022
- **Contains**: Official test vectors, reference bitstreams, decoded images
- **Price**: ~$200-300 USD from ISO store
- **Includes**:
  - Codestreams for decoder conformance testing
  - Reference decoded images
  - Error metrics and validation procedures
  - Tests for profiles/levels compliance

### ISO/IEC 21122-5 - Reference Software  
- **Current Version**: ISO/IEC 21122-5:2022
- **Contains**: Official reference encoder/decoder
- **Price**: ~$200-300 USD from ISO store
- **Purpose**: Implementation guideline and conformance reference

## Alternative Test Resources (Free)

### 1. TangKii/jxs Reference Implementation
**Repository**: https://github.com/TangKii/jxs
- ✅ **Conformance tested** against ISO/IEC 21122 standards
- ✅ **Validation anchor** for other implementations
- ✅ **Full encoder/decoder** implementation
- ✅ **Test bitstreams** included in repository
- ✅ **Configuration examples** from reference codestreams

**Key Features**:
- Parameters are one-to-one representations of ISO 21122-1/21122-2
- Supports profiles, levels, and sublevels from ISO/IEC 21122-2
- Can decode reference codestreams from ISO/IEC 21122-4
- Provides configuration dumping for learning XS syntax

### 2. SVT-JPEG-XS (Intel/Open Visual Cloud)
**Repository**: https://github.com/OpenVisualCloud/SVT-JPEG-XS
- ✅ **Professional implementation** of ISO/IEC 21122
- ✅ **Optimized for performance** (SIMD, multi-threading)
- ✅ **Industry-grade** encoder/decoder
- ✅ **Active development** and maintenance

**Key Features**:
- High-performance implementation
- Production-ready quality
- Supports multiple profiles and levels
- Comprehensive build system

### 3. Academic/Research Test Materials
- **Various research papers** include test bitstreams
- **University implementations** may have test vectors
- **Conference proceedings** often include sample data

## Practical Approach for Your Project

### Phase 1: Use Available Resources (Immediate)
1. **Download TangKii/jxs** reference implementation
2. **Extract test bitstreams** from the repository
3. **Use as validation anchor** for your decoder
4. **Compare encoding results** with reference encoder

### Phase 2: Generate Custom Test Vectors
1. **Create synthetic test images** (gradients, patterns, noise)
2. **Encode with TangKii/jxs** reference encoder
3. **Use resulting bitstreams** for decoder testing
4. **Validate roundtrip accuracy** (pixel-perfect reconstruction)

### Phase 3: Community Collaboration
1. **Connect with JPEG XS community** (JPEG committee, research groups)
2. **Share test results** and validation data
3. **Participate in interoperability testing**
4. **Contribute to open-source ecosystem**

### Phase 4: Commercial Certification (If Needed)
1. **Purchase ISO/IEC 21122-4** for official test vectors
2. **Run full conformance suite** against your implementation
3. **Document compliance results** for customers/partners
4. **Obtain ISO certification** if required for market

## Implementation Strategy

### Cross-Validation Approach
```rust
// Test against multiple reference implementations
pub fn validate_implementation() -> Result<ValidationReport> {
    let test_images = generate_test_suite();
    let mut report = ValidationReport::new();
    
    for image in test_images {
        // Test against TangKii/jxs reference
        let tangkii_encoded = encode_with_tangkii(image)?;
        let our_decoded = decode_frame(&tangkii_encoded)?;
        report.add_cross_decode_test("TangKii", calculate_psnr(&image, &our_decoded));
        
        // Test our encoder against reference decoder
        let our_encoded = encode_frame(image)?;
        let tangkii_decoded = decode_with_tangkii(&our_encoded)?;
        report.add_cross_encode_test("TangKii", calculate_psnr(&image, &tangkii_decoded));
        
        // Roundtrip test
        let roundtrip = decode_frame(&our_encoded)?;
        report.add_roundtrip_test(calculate_psnr(&image, &roundtrip));
    }
    
    Ok(report)
}
```

### Test Vector Generation
```rust
pub fn generate_conformance_test_vectors() -> Vec<TestVector> {
    vec![
        // Basic patterns
        create_solid_color_test(255, 0, 0),      // Red
        create_gradient_test(256, 256),          // Smooth gradient  
        create_checker_pattern(64),              // High frequency
        
        // Edge cases
        create_black_image(1920, 1080),          // All zeros
        create_white_image(1920, 1080),          // All max values
        create_random_noise(512, 512),           // Random data
        
        // Real-world scenarios
        create_natural_image_proxy(),            // Natural image characteristics
        create_graphics_content(),               // Sharp edges, text
        create_video_frame_sequence(10),         // Temporal consistency
        
        // Bit depth variations
        create_10bit_test_pattern(),
        create_12bit_test_pattern(),
        create_16bit_test_pattern(),
        
        // Format variations
        create_yuv422_test(),
        create_yuv420_test(),
        create_rgb_planar_test(),
    ]
}
```

## Cost-Benefit Analysis

### Free Approach
- **Cost**: $0
- **Coverage**: ~80-90% of real-world conformance
- **Risk**: May miss edge cases in official test suite
- **Timeline**: Immediate start

### Official ISO Approach  
- **Cost**: ~$400-600 USD (both standards)
- **Coverage**: 100% official conformance
- **Risk**: Low - official reference
- **Timeline**: 2-3 weeks for procurement

## Recommendations

### For Open Source/Public Version
1. **Use TangKii/jxs** as primary reference
2. **Generate comprehensive test suite** from multiple sources
3. **Focus on interoperability** with existing implementations
4. **Document compliance level** achieved (e.g., "90% conformant")

### For Commercial Version
1. **Purchase ISO standards** for complete test vectors
2. **Run official conformance suite** for certification
3. **Achieve 100% compliance** for market credibility
4. **Include compliance certificate** in commercial offering

## Next Steps

1. **This week**: Clone and analyze TangKii/jxs repository
2. **Next week**: Extract test bitstreams and run against your decoder  
3. **Week 3**: Generate custom test vectors for missing coverage
4. **Week 4**: Implement cross-validation with SVT-JPEG-XS
5. **Month 2**: Consider ISO standard purchase for full certification

## Conclusion

While official ISO test vectors require payment, there are sufficient free resources available to achieve high conformance confidence. The TangKii/jxs reference implementation provides an excellent validation anchor that covers most real-world scenarios.

For a public repository, using free resources is appropriate and provides good conformance coverage. For commercial deployment, investing in official ISO test vectors may be worthwhile for complete certification confidence.

---

**Document Version**: 1.0  
**Created**: 2025-09-12  
**Next Review**: When ISO test vectors are acquired