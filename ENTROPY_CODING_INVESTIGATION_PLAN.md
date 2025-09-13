# Entropy Coding Investigation Plan

## Problem Statement

Despite fixing the DWT (perfect reconstruction achieved) and quantization parameter mapping (quality 0.95 → QP=1), PSNR remains at 8.3 dB instead of the target 30+ dB. Analysis points to the entropy coding system as the primary culprit.

## Root Cause Hypothesis

The `jpegxs_core_clean::JpegXsBitstream::add_entropy_coded_data()` function implements aggressive multi-level quantization that may be destroying coefficient precision:

```rust
// Current problematic implementation:
if abs_coeff <= 15 {
    let quantized = ((abs_coeff + 1) / 2).min(15) as u8;  // 2x quantization loss
} else if abs_coeff <= 127 {
    let quantized = (abs_coeff / 4).min(127) as u8;       // 4x quantization loss  
} else {
    let quantized = (abs_coeff / 16).min(63) as u8;       // 16x quantization loss
}
```

**Impact Assessment**: This multi-tier quantization can cause 2x-16x additional precision loss beyond the already applied DWT quantization (QP=1), explaining the poor quality.

## Investigation Phases

### Phase 1: Entropy Coding Impact Analysis ⚡ HIGH PRIORITY

#### 1.1 Coefficient Distribution Profiling
**Goal**: Understand actual DWT coefficient ranges and distributions

**Tasks**:
- [ ] Add logging to capture coefficient statistics before entropy coding
- [ ] Measure min/max/mean/std of Y, U, V coefficients for test patterns
- [ ] Identify which quantization tiers are being triggered most frequently
- [ ] Document coefficient range patterns for different image types

**Expected Output**: 
```
Coefficient Analysis Report:
- Y coefficients: range [-127, 89], mean=2.3, std=15.4
- U coefficients: range [-45, 67], mean=0.1, std=8.9  
- V coefficients: range [-52, 43], mean=-0.3, std=9.2
- Quantization tier usage: Tier1(2x): 65%, Tier2(4x): 30%, Tier3(16x): 5%
```

#### 1.2 Entropy Bypass Experiment  
**Goal**: Isolate entropy coding impact on quality

**Tasks**:
- [ ] Implement temporary bypass mode that skips entropy quantization
- [ ] Store coefficients with minimal loss (simple run-length encoding)
- [ ] Measure PSNR improvement with entropy bypass enabled
- [ ] Compare bitstream sizes (compression ratio impact)

**Success Criteria**: If PSNR jumps to 30+ dB with bypass, confirms entropy coding as root cause

#### 1.3 Precision Loss Tracking
**Goal**: Quantify where quality degradation occurs

**Tasks**:
- [ ] Add coefficient comparison logging at each pipeline stage:
  - Post-DWT coefficients
  - Post-quantization coefficients  
  - Post-entropy-quantization coefficients
  - Post-decode coefficients
- [ ] Calculate cumulative PSNR loss at each stage
- [ ] Identify the largest quality degradation point

### Phase 2: Alternative Entropy Coding Implementation ⚡ MEDIUM PRIORITY

#### 2.1 Lossless Entropy Coding
**Goal**: Implement truly lossless entropy coding for high-quality compression

**Tasks**:
- [ ] Research JPEG XS standard entropy coding requirements (ISO Annex C)
- [ ] Implement arithmetic coding or Huffman coding without quantization loss
- [ ] Create quality-aware entropy coding (less aggressive for high quality settings)
- [ ] Test PSNR improvement with lossless entropy implementation

#### 2.2 Quality-Adaptive Entropy Coding
**Goal**: Make entropy coding respect quality settings

**Tasks**:
- [ ] Modify entropy coding to use quality parameter
- [ ] For quality > 0.9: Use minimal/no entropy quantization
- [ ] For quality 0.5-0.9: Use moderate entropy quantization  
- [ ] For quality < 0.5: Use current aggressive quantization
- [ ] Validate PSNR scaling with quality settings

### Phase 3: Color Space and Pipeline Validation 🔍 LOW PRIORITY

#### 3.1 Color Conversion Accuracy
**Goal**: Ensure RGB↔YUV conversions are not introducing errors

**Tasks**:  
- [ ] Validate ITU-R BT.601 color matrix implementation
- [ ] Test round-trip RGB→YUV→RGB accuracy
- [ ] Measure color conversion precision loss
- [ ] Compare against reference color conversion implementations

#### 3.2 Chroma Processing Validation
**Goal**: Verify 422/420 upsampling/downsampling accuracy

**Tasks**:
- [ ] Test YUV422→444 upsampling quality
- [ ] Test YUV420→444 upsampling quality  
- [ ] Measure chroma interpolation accuracy
- [ ] Compare against reference upsampling algorithms

## Expected Timeline

### Week 1: Entropy Coding Analysis
- **Day 1-2**: Coefficient profiling and entropy bypass implementation
- **Day 3**: PSNR testing and impact quantification
- **Day 4-5**: Alternative entropy coding implementation

### Week 2: Validation and Integration  
- **Day 1-2**: Quality-adaptive entropy coding
- **Day 3**: Color space validation (if needed)
- **Day 4-5**: Integration testing and conformance validation

## Success Metrics

### Primary Goals
- [ ] **PSNR Target**: Achieve 30+ dB for quality 0.95 (currently 8.3 dB)
- [ ] **Quality Scaling**: PSNR should scale linearly with quality setting
- [ ] **Compression Efficiency**: Maintain reasonable compression ratios

### Secondary Goals
- [ ] **Conformance**: Maintain 100% decoder success rate
- [ ] **Performance**: Encoding speed > 40 Mbps
- [ ] **ISO Compliance**: Follow JPEG XS entropy coding standards

## Risk Assessment

### High Risk
- **Entropy coding changes breaking decoder compatibility**: Mitigation via extensive testing
- **Quality vs compression ratio trade-offs**: Careful balancing required

### Medium Risk  
- **Performance regression from new entropy coding**: Profile and optimize
- **ISO compliance issues**: Validate against standard requirements

### Low Risk
- **Color space issues**: Well-established conversion algorithms
- **Chroma processing**: Standard interpolation methods

## Deliverables

1. **Entropy Coding Analysis Report**: Detailed coefficient analysis and impact assessment
2. **Fixed Entropy Implementation**: Quality-aware entropy coding system  
3. **Updated Conformance Results**: New PSNR measurements and success rates
4. **Performance Benchmarks**: Speed and compression ratio analysis
5. **Integration Guide**: How to configure quality vs compression trade-offs

This systematic approach should resolve the final quality bottleneck and achieve the target 30+ dB PSNR performance.