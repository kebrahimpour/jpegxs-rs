# 🚀 Next Session Quick Start Guide

**Date Created**: September 13, 2025
**Session Ready**: ✅ **READY TO START IMMEDIATELY**
**Primary Mission**: 🔍 **Entropy Coding Quality Investigation**

## ⚡ Quick Status Check

Run these commands to verify environment:

```bash
# 1. Verify branch status
git status
git branch -a

# 2. Build and test current state
cargo build --release
cargo test --all-features

# 3. Run current conformance baseline
./target/release/conformance_runner --output conformance_baseline.json

# 4. Verify current PSNR (should be ~8.3 dB)
grep -i "psnr" conformance_baseline.json
```

**Expected Results**:
- ✅ On main branch with clean working directory
- ✅ All tests passing
- ✅ Overall compliance: 54.2%
- ✅ Average PSNR: 8.3 dB (the problem we're solving)

## 🎯 **IMMEDIATE NEXT TASKS - START HERE**

### Task 1: Coefficient Logging Setup (30 mins)

**Objective**: Add logging to entropy coding pipeline to understand coefficient distributions

**Files to modify**:
- `commercial/jpegxs-core-clean/src/entropy.rs`
- `commercial/jpegxs-core-clean/src/bitstream.rs`

**Add logging before quantization**:
```rust
// In add_entropy_coded_data() function, add before quantization:
log::info!("Coefficient analysis - component: {}, abs_coeff: {}, tier: {}",
           component, abs_coeff,
           if abs_coeff <= 15 { "15" }
           else if abs_coeff <= 127 { "127" }
           else { "63" });
```

### Task 2: Run Coefficient Analysis (15 mins)

```bash
# Run with logging to capture coefficient stats
RUST_LOG=info ./target/release/jpegxs encode -i test-data/lenna.png -o test_analysis.jxs --quality 0.9 2> coefficient_analysis.log

# Analyze the results
grep "Coefficient analysis" coefficient_analysis.log | head -100
```

### Task 3: Entropy Bypass Experiment (45 mins)

**Objective**: Create temporary bypass to test hypothesis that entropy coding is the quality bottleneck

**Add bypass flag to encoder**:
```rust
// In entropy coding function, add bypass mode
if bypass_entropy_quantization {
    // Store coefficient with minimal loss
    let stored_coeff = abs_coeff.min(255) as u8;  // Simple clamp instead of quantization
    // ... rest of encoding without quantization tiers
}
```

**Test bypass mode**:
```bash
# Encode with bypass (modify code to enable)
./target/release/jpegxs encode -i test-data/lenna.png -o test_bypass.jxs --quality 0.9

# Decode and measure PSNR
./target/release/jpegxs decode -i test_bypass.jxs -o test_bypass_decoded.png
./target/release/jpegxs psnr -r test-data/lenna.png -t test_bypass_decoded.png
```

**Success Criteria**: If PSNR jumps to >30 dB, confirms entropy coding as root cause.

## 🔍 **INVESTIGATION HYPOTHESIS**

**Current Theory**: Entropy coding aggressive quantization (2x-16x loss) is the primary quality bottleneck.

**Evidence to collect**:
1. **Coefficient Distribution**: Which quantization tiers are used most frequently
2. **Stage-by-Stage Loss**: Where in the pipeline is the most quality lost
3. **Bypass Validation**: Does removing entropy quantization achieve target PSNR

## 📊 **Expected Session 1 Results**

By end of investigation session, you should have:

- [ ] **Coefficient Statistics**: Clear data on tier usage (15, 127, 63 thresholds)
- [ ] **Bypass Test Results**: PSNR measurement with entropy quantization disabled
- [ ] **Quality Loss Location**: Quantified precision loss at each pipeline stage
- [ ] **Root Cause Confirmation**: Data-driven proof entropy coding is bottleneck
- [ ] **Implementation Plan**: Clear approach for quality-adaptive entropy coding

## 🛠️ **Environment Status**

### ✅ Ready Components
- **DWT System**: Perfect reconstruction (< 1e-6 error)
- **Quantization Pipeline**: Quality→QP mapping working correctly
- **Decoder**: 100% success rate (22/22 tests)
- **Conformance Framework**: Comprehensive test suite operational
- **CI/CD**: Pre-commit hooks preventing regressions

### 🔧 Investigation Target
- **Entropy Coding**: Multi-tier quantization causing 2x-16x precision loss

### 📈 Current Baseline Metrics
- **Overall Compliance**: 54.2%
- **Encoder Tests**: 0/22 passing (quality issue)
- **Decoder Tests**: 22/22 passing ✅
- **Average PSNR**: 8.3 dB (target: 30+ dB)
- **Performance**: 698.6 Mbps encoding (exceeding targets)

## 🎯 **Session Success Definition**

**Primary Goal**: Prove entropy coding is quality bottleneck and demonstrate path to 30+ dB PSNR

**Measurements Needed**:
1. Current coefficient distribution analysis
2. PSNR with entropy quantization bypassed
3. Stage-by-stage precision loss quantification

**Decision Point**: If bypass achieves >30 dB PSNR → Implement quality-adaptive entropy coding
**Timeline**: 1-2 days investigation, 2-3 days implementation

## 📚 **Key Reference Documents**

1. **`ENTROPY_CODING_INVESTIGATION_PLAN.md`** - Detailed investigation roadmap
2. **`SESSION_SUMMARY_AND_NEXT_STEPS.md`** - Complete context and progress
3. **`QUALITY_IMPROVEMENTS_ANALYSIS.md`** - Previous fixes and lessons learned
4. **`conformance_report.json`** - Current test results and baseline metrics

---

**🚀 START COMMAND**: Open `commercial/jpegxs-core-clean/src/entropy.rs` and add coefficient logging

**Expected First Session Duration**: 2-3 hours for complete investigation phase

**Ready to Go**: ✅ **ALL SYSTEMS READY - START IMMEDIATELY**
