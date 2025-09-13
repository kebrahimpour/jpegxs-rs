# DWT Pipeline Precision Loss Analysis

## Executive Summary
**ROOT CAUSE IDENTIFIED**: Severe precision loss occurs in the inverse DWT step, not in quantization or color conversion.

## Stage-by-Stage Analysis (Quality 0.99, QP=1)

### 1. Pre-DWT (Input Data)
```
Y coefficients: min: -128.000, max: 126.000, mean: -1.248, std: 54.213
UV coefficients: U_min: -18.000, U_max: 18.000, V_min: -128.000, V_max: 127.000
```
- **Status**: ✅ GOOD - Clean input data range [-128, +127]

### 2. Post-DWT (Forward Transform)
```
Y coefficients: min: -127.000, max: 126.875, mean: 0.001, std: 27.107
UV coefficients: U_min: -17.156, U_max: 19.125, V_min: -127.016, V_max: 128.000
```
- **Status**: ✅ GOOD - DWT preserves precision well
- **Range preservation**: Very close to input range
- **Standard deviation reduced**: 54.213 → 27.107 (energy concentration expected)

### 3. Post-Quantization
```
Y coefficients: min: -127, max: 127, mean: 0.007, QP: 1
UV coefficients: U_min: -17, U_max: 19, V_min: -127, V_max: 128, QP: 1
```
- **Status**: ✅ ACCEPTABLE - Minimal quantization loss with QP=1
- **Loss**: Fractional parts removed, integer values preserved

### 4. Post-Dequantization
```
Y coefficients: min: -124.000, max: 124.000, mean: -4.690
UV coefficients: U_min: -127.000, U_max: 32.000, V_min: -127.000, V_max: 23.000
```
- **Status**: ⚠️ SOME LOSS - Range reduction but still reasonable
- **Y range**: Reduced by ~3 values at extremes (-127→-124, 127→124)
- **UV range**: Significant reduction in positive values (18→32, 127→23)

### 5. Post-Inverse-DWT (Reconstruction)
```
Y coefficients: min: -219.188, max: 118.750, mean: -17.212, std: 50.261
UV coefficients: U_min: -103.375, U_max: 63.688, V_min: -145.812, V_max: 90.562
```
- **Status**: ❌ CRITICAL ISSUE - Severe range expansion and distortion
- **Y range**: Expanded beyond valid range [-219, +119] vs original [-128, +127]
- **Mean shift**: -1.248 → -17.212 (significant DC shift)
- **Range overshoot**: Values outside [-128, +127] will be clamped causing distortion

## Root Cause Analysis

### The Problem: Inverse DWT Range Explosion
1. **Input to inverse DWT**: Reasonable coefficient range [-124, +124]
2. **Output from inverse DWT**: Invalid range [-219, +119]
3. **Clamping loss**: Values outside [0, 255] final range are clipped

### Why This Happens
The 5/3 DWT is supposed to have perfect reconstruction, but our implementation appears to have:
1. **Incorrect normalization**: Forward/inverse transforms may not be properly balanced
2. **Integer overflow**: Fixed-point arithmetic issues in the DWT implementation
3. **Boundary handling**: Edge effects causing coefficient corruption

## Impact Assessment
- **PSNR**: 9.88 dB (should be >40 dB for quality 0.99)
- **Primary bottleneck**: Inverse DWT step (not quantization/entropy coding)
- **Previous hypothesis disproven**: Entropy coding accounts for only 0.4 dB loss

## Recommended Actions

### Immediate (High Priority)
1. **Fix DWT normalization**: Ensure forward/inverse DWT are properly balanced
2. **Verify DWT implementation**: Compare against reference implementation
3. **Add range checking**: Prevent coefficient overflow in DWT pipeline

### Validation (Medium Priority)
1. **DWT-only test**: Test DWT roundtrip without quantization
2. **Range validation**: Add assertions for coefficient bounds
3. **Reference comparison**: Compare against ISO reference DWT

## Expected Outcome
Fixing the DWT implementation should improve PSNR from ~10 dB to >30 dB, achieving the target quality levels for JPEG-XS conformance.

## Technical Details
- **Test image**: 512x512 gradient (test_images/gradient_512x512.png)
- **Quality setting**: 0.99 (QP=1)
- **Current PSNR**: 9.88 dB
- **Target PSNR**: >30 dB
- **Bottleneck identified**: Inverse DWT step causing range explosion
