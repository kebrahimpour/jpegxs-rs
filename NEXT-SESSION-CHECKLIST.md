# Next Session Quick Start Checklist

## 🚀 Quick Commands to Resume

```bash
# 1. Navigate to project
cd /Users/keyvan/Work/Projects/sandbox/jpeg-xs

# 2. Check git status
git status
git pull

# 3. Run tests to ensure everything works
source "$HOME/.cargo/env"
cargo test --all-features

# 4. Review current state
cat SESSION-SUMMARY-2024-12-06.md
cat DERIVATIVE-TRACKING.md

# 5. Continue development
cd commercial/  # For clean-room work
# OR
cd crates/      # For evaluation work
```

## ✅ Pre-Session Checklist

### Documents to Have Ready
- [ ] ISO/IEC 21122-1:2019 (Core coding system)
- [ ] ISO/IEC 21122-2:2019 (Profiles) 
- [ ] This checklist
- [ ] SESSION-SUMMARY-2024-12-06.md

### Environment Check
- [ ] Rust installed (`rustc --version`)
- [ ] Git configured (`git config --list`)
- [ ] GitHub CLI working (`gh auth status`)
- [ ] Docker available (optional)

### Mental Context
- [ ] Remember: DON'T look at reference code for commercial track
- [ ] Remember: We're doing dual-track development
- [ ] Remember: DWT needs clean-room replacement first

## 📋 TODO Priority List

### 🔴 Critical (Do First)
1. **Get ISO/IEC 21122 specification**
   - Required for clean-room implementation
   - Cannot proceed without this

2. **Start DWT clean-room implementation**
   ```bash
   cd commercial/
   # Create new DWT from spec only
   ```

### 🟡 Important (Do Soon)
3. **Update DERIVATIVE-TRACKING.md**
   - Mark completed clean-room components
   - Update commercial status

4. **Test both implementations**
   - Ensure compatibility
   - Benchmark performance

### 🟢 Nice to Have
5. **Documentation updates**
6. **Performance optimizations**
7. **Additional tests**

## 🎯 Today's Goals (Next Session)

### Goal 1: Clean-Room DWT Setup
```bash
# Create structure
cd commercial/
cargo new --lib jpegxs-core-clean
cd jpegxs-core-clean/src/

# Create module (DO NOT COPY)
echo "// Clean-room from ISO/IEC 21122-1:2019 Section 7.3" > dwt.rs
```

### Goal 2: Implement 5/3 Transform
- Read ISO spec Section 7.3.1
- Implement math formulas
- Test against known values
- Document in clean-room log

### Goal 3: Update Tracking
- Update DERIVATIVE-TRACKING.md
- Commit progress
- Push to repository

## 🔧 Useful Commands

### Testing
```bash
# Test everything
cargo test --all-features

# Test specific module
cargo test -p jpegxs-core

# Run with output
cargo test -- --nocapture

# Benchmark
cargo bench
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check
```

### Git Operations
```bash
# See what changed
git status
git diff

# Commit with message
git add -A
git commit -m "message"
git push

# Check CI status
gh run list --limit 5
```

## 📊 Current Status Summary

### What Works
- ✅ Full encode/decode pipeline
- ✅ YUV I/O (can commercialize)
- ✅ CLI tools (can commercialize)
- ✅ Tests passing

### What Needs Work
- ⚠️ DWT is derivative (needs clean-room)
- ⚠️ Some codec elements need replacement
- 🚧 Commercial track just started

### Timeline
- Week 1-2: DWT clean-room ← **WE ARE HERE**
- Week 3-4: Entropy coding
- Week 5-6: Bitstream structure
- Week 7-8: Profiles
- Week 9-10: Final commercial release

## 💡 Key Decisions to Remember

1. **Hybrid Licensing**: Two tracks (evaluation + commercial)
2. **Clean-Room Required**: For full commercial rights
3. **Immediate Revenue**: Can license CLI and I/O now
4. **10-Week Timeline**: To full commercial implementation

## 🚨 Important Warnings

### DO NOT
- ❌ Look at reference code when working in `commercial/`
- ❌ Copy any code between tracks without checking license
- ❌ Use AI code completion in commercial track
- ❌ Forget to document clean-room work

### DO
- ✅ Document all resources used
- ✅ Test both implementations
- ✅ Keep tracks separate
- ✅ Update tracking documents

## 📞 Help & Support

- **Technical Issues**: Review SESSION-SUMMARY-2024-12-06.md
- **Legal Questions**: Check LICENSE-HYBRID
- **Implementation Guide**: See HYBRID-DEVELOPMENT-GUIDE.md
- **What's Original vs Derivative**: Check DERIVATIVE-TRACKING.md

## 🎯 Success Criteria for Next Session

- [ ] Clean-room DWT module created
- [ ] At least one transform implemented from spec
- [ ] Tests comparing both implementations
- [ ] Documentation updated
- [ ] Clean-room log entry added
- [ ] Progress committed and pushed

---

**Ready to continue?** Start with step 1 in Quick Commands above! 🚀