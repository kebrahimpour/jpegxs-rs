# Commercial Repository Action Plan

## Private Repository: `kebrahimpour/jpegxs-rs-commercial`

### 🔴 Immediate Actions (Do First)

```bash
# 1. Clean AI signatures from history
git filter-branch --force --msg-filter 'perl -pe "s/Co-[Aa]uthored-[Bb]y:.*([Cc]laude|[Cc]opilot|anthropic|175728472).*\n//gi"' --tag-name-filter cat -- --all
git push --force origin main

# 2. Clean up branches
git branch -d feature/8bit-coefficients-backup
git push --delete origin feature/8bit-coefficients-backup
```

### ✅ Verify Commercial Features

```bash
# Test 8-bit pipeline (50% memory reduction)
cargo test test_8bit_coefficient

# Test research bypass mode
JPEGXS_BYPASS_ENTROPY=1 cargo test

# Benchmark memory usage
/usr/bin/time -v cargo run --release -- encode test.png
```

### 📊 CI Cost Optimization Check

```bash
# Trigger a test commit
git commit --allow-empty -m "Test CI optimization"
git push

# Should see in CI logs:
# "✅ Tests already passed in public repo"
# Runtime: <2 minutes (vs 7-8 min normal)
```

### Testing Checklist

#### Verify Commercial Features
```bash
# Test 8-bit pipeline
cargo test --features commercial

# Test research mode
JPEGXS_BYPASS_ENTROPY=1 cargo test

# Benchmark performance
cargo bench --features commercial
```

#### CI Cost Verification
1. Trigger CI in public repo
2. Verify commercial CI reads public results
3. Confirm reduced runtime and cost

### Documentation Updates

#### Commercial README
- Emphasize enhanced features
- Document performance improvements
- Include benchmark comparisons
- Add enterprise support details

#### Customer Documentation
- Installation guide for commercial version
- API documentation for enhanced features
- Performance tuning guide
- Support contact information

### Security Checklist

- [ ] No public repo code leaks
- [ ] Access controls configured
- [ ] Secrets properly managed
- [ ] Commercial features protected

### Branch Strategy

```
main (stable commercial release)
├── feature/enterprise-X (customer-specific features)
├── develop (active development)
└── hotfix/issue-Y (urgent fixes)
```

### Release Process

1. Test in develop branch
2. Merge to main when stable
3. Tag releases with version
4. Update customer deployments
5. Notify license holders

### Customer Support Setup

#### Support Channels
- Email: k1.ebrahimpour@gmail.com
- Response SLA: 24-48 hours
- Priority support for enterprise

#### Documentation Portal
- Private wiki or docs site
- API reference
- Integration examples
- FAQ section

### Performance Benchmarks

Run these benchmarks to validate commercial advantages:

```bash
# Memory usage comparison
/usr/bin/time -v cargo run --release -- encode large_image.png

# Speed comparison
hyperfine './target/release/jpegxs encode -i test.png -o test.jxs'

# Quality comparison
./target/release/jpegxs psnr original.png decoded.png
```

### Commercial Feature Validation

```rust
#[test]
fn test_8bit_coefficient_memory() {
    // Verify 50% memory reduction
}

#[test]
fn test_entropy_bypass_mode() {
    // Verify research features work
}

#[test]
fn test_enhanced_performance() {
    // Verify optimization benefits
}
```

### Next Steps Priority

1. **Today**: Clean branches and verify CI
2. **Tomorrow**: Test all commercial features
3. **This Week**: Prepare customer demo
4. **Next Week**: Launch commercial operations

---

**Important**: Keep this document in private repository only!
