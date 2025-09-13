# JPEG XS Codec - Licensing Guide

## 🎯 Quick Summary

This project offers a **dual-license model**:
- **Free** for non-commercial use (education, research, personal projects)
- **Paid license** required for commercial use (businesses, products, services)

## 📁 Project Structure & Licensing

```
jpeg-xs/
├── commercial/jpegxs-core-clean/  ← 100% Clean-room implementation (our IP)
├── crates/                        ← Mixed (some derivative from reference)
├── reference/jxs/                 ← Reference implementation (3rd party license)
└── LICENSE                        ← Main license file
```

### Three Distinct Components

1. **Clean-Room Implementation** (`commercial/jpegxs-core-clean/`)
   - ✅ 100% original code from ISO specification
   - ✅ No derivative work restrictions
   - ✅ Full commercial licensing available
   - ✅ We own all IP rights

2. **Main Codebase** (`crates/`)
   - ⚠️ Contains some derivative work from reference
   - ⚠️ Being replaced with clean-room code
   - ✅ Free for non-commercial use
   - ⚠️ Commercial use requires checking with original authors

3. **Reference Implementation** (`reference/jxs/`)
   - ❌ Not our code - from intoPIX SA, Fraunhofer IIS, Canon Inc
   - ❌ Only for evaluation and testing
   - ❌ Commercial use requires RAND license from them

## 💰 Commercial Licensing

### What We Can License
✅ **Clean-room implementation** - Full commercial rights
✅ **Original Rust code** - Our tooling, CLI, optimizations
✅ **Professional services** - Support, custom development, optimization

### What Requires Third-Party License
⚠️ Parts derived from reference implementation (being phased out)

### Commercial Options

#### 1. Standard Commercial License
- Use our clean-room implementation in your products
- Includes 1 year of updates and support
- Deploy on unlimited servers
- Starting at $X,XXX/year

#### 2. Enterprise License
- Everything in Standard
- Priority support
- Custom feature development
- Training and consulting
- Contact for pricing

#### 3. OEM/Embedded License
- Ship in hardware/embedded systems
- White-label options
- Royalty or one-time fee models
- Contact for pricing

#### 4. Professional Services
- **GPU/SIMD Optimization**: Custom optimizations for your hardware
- **FPGA/ASIC Implementation**: Hardware codec implementations
- **Custom Features**: Specific profiles, bit depths, color spaces
- **Performance Tuning**: Optimize for your specific use case
- **Training & Support**: On-site or remote training

## 🆓 Non-Commercial Use

### Allowed (Free)
✅ Personal projects
✅ Academic research
✅ Educational use
✅ Open source projects (non-commercial)
✅ Evaluation and testing

### Not Allowed (Requires License)
❌ Use in any business
❌ SaaS/cloud services
❌ Embedded in products for sale
❌ Proprietary software
❌ Providing paid services

## 📋 FAQ

### Q: Can I use this in my startup?
**A:** No, startups are commercial entities. You need a commercial license.

### Q: Can I use this for my university research?
**A:** Yes, academic research is free under non-commercial license.

### Q: Can I modify the code?
**A:** Yes, but modifications must be shared under the same license for non-commercial use.

### Q: Why multiple license files?
**A:**
- `LICENSE` - Main license (non-commercial terms)
- `LICENSE-COMMERCIAL.md` - Commercial licensing information
- `reference/jxs/LICENSE.md` - Reference implementation (not ours)

### Q: What's the clean-room implementation?
**A:** Code written solely from the ISO specification without looking at any existing implementation. This ensures no copyright issues and full commercial licensing rights.

### Q: Can I get support?
**A:**
- Non-commercial: Community support via GitHub issues
- Commercial: Priority support with SLA included

## 📧 Contact

For commercial licensing, custom development, or questions:

**Keyvan Ebrahimpour**
Email: k1.ebrahimpour@gmail.com
GitHub: [@kebrahimpour](https://github.com/kebrahimpour)

## ⚖️ Legal Notice

This project contains multiple components with different licensing:
1. Review the specific license for each component
2. When in doubt, contact us for clarification
3. Commercial use always requires explicit permission
