# Hybrid Development Guide

## Quick Start: Which Code Can I Use?

### For Commercial Products 💰

**YES, You Can Use:**
```
commercial/          ✅ Full commercial rights (when available)
crates/jpegxs-cli/   ✅ Original CLI code
crates/jpegxs-io/    ✅ I/O utilities (except where marked)
crates/jpegxs-ffi/   ✅ FFI bindings
```

**NO, You Cannot Use (without RAND license):**
```
crates/jpegxs-core/src/dwt.rs  ❌ Derivative DWT implementation
reference/                      ❌ Reference implementation
```

### For Research/Education 📚

**You Can Use Everything:**
- All code for evaluation and testing
- Perfect for academic research
- Great for learning JPEG XS

## Development Workflow

### Working on Evaluation Track (Fast Iteration)

1. Use existing code in `crates/`
2. Test and validate algorithms
3. Develop new features
4. Mark derivative vs. original work

```bash
# Development in evaluation track
cd crates/jpegxs-core
cargo test
cargo bench
```

### Working on Commercial Track (Clean Room)

1. **NEVER** look at code in `reference/` or derivative parts
2. Study ISO/IEC 21122 specification
3. Implement in `commercial/` directory
4. Document in clean-room log

```bash
# Development in commercial track
cd commercial
# Implement from spec only
cargo test
```

## Migration Strategy

### Phase 1: Current State
```
crates/
  └── jpegxs-core/
      ├── dwt.rs         [DERIVATIVE - Need replacement]
      ├── quant.rs       [ORIGINAL - Can commercialize]
      ├── io.rs          [ORIGINAL - Can commercialize]
      └── ...
```

### Phase 2: Parallel Development
```
crates/                  commercial/
  └── jpegxs-core/         └── jpegxs-core-clean/
      ├── dwt.rs                ├── dwt.rs [CLEAN ROOM]
      └── ...                   └── ...
```

### Phase 3: Commercial Ready
```
commercial/
  └── jpegxs-core-clean/
      ├── dwt.rs         [ORIGINAL - Full rights]
      ├── quant.rs       [ORIGINAL - Full rights]
      ├── entropy.rs     [ORIGINAL - Full rights]
      └── ...           [ALL CLEAN]
```

## Testing Strategy

### Compliance Testing
```rust
#[test]
fn test_both_implementations() {
    let input = test_data();
    
    // Test evaluation version
    let eval_result = crates::jpegxs_core::encode(input);
    
    // Test commercial version
    let commercial_result = commercial::jpegxs_core::encode(input);
    
    // Both should produce valid JPEG XS
    assert!(validate_jpeg_xs(eval_result));
    assert!(validate_jpeg_xs(commercial_result));
}
```

## Legal Checkpoints

### Before Starting Commercial Track Work

- [ ] Have ISO/IEC 21122 standard documents
- [ ] Understand clean-room requirements
- [ ] Sign clean-room certification
- [ ] Set up separate development environment

### Before Commercial Release

- [ ] All algorithms from public specs
- [ ] Clean-room log complete
- [ ] No derivative code in commercial track
- [ ] Legal review completed
- [ ] License agreements prepared

## FAQ

### Q: Can I look at the evaluation code while writing commercial code?
**A:** YES, but only the parts marked as 🟢 ORIGINAL. Never look at 🔴 DERIVATIVE parts.

### Q: Can I reuse my own original code from evaluation track?
**A:** YES, if you wrote it and it's marked as original, you can move it to commercial track.

### Q: How do I know what's safe to commercialize?
**A:** Check `DERIVATIVE-TRACKING.md` - only 🟢 GREEN items are safe.

### Q: What if I accidentally saw derivative code?
**A:** Document it, wait 30 days, then implement from spec with fresh perspective.

### Q: Can I use AI tools (like GitHub Copilot)?
**A:** NO for commercial track - AI might suggest derivative code. YES for evaluation track.

## Support

- **Technical Questions**: Create GitHub issue
- **Legal Questions**: k1.ebrahimpour@gmail.com
- **Commercial Licensing**: k1.ebrahimpour@gmail.com

## Timeline

- **Now - Week 4**: Parallel development starts
- **Week 5-8**: Core algorithms clean room
- **Week 9-10**: Testing and validation
- **Week 11**: Commercial track ready
- **Week 12**: First commercial license available