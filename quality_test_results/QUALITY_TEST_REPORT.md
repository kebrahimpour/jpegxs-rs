# JPEG XS Quality Parameter Test Report

**Generated**: 2025-09-07 20:12:42 UTC
**Quality Levels Tested**: 6

## Quality vs Compression Analysis

| Quality | QP | Original (KB) | Compressed (KB) | Ratio | Reduction % | Encode (ms) |
|---------|----|--------------:|----------------:|------:|------------:|------------:|
| 0.10 | 32 | 136.6 | 11.1 | 12.3:1 | 91.9% | 7.39 |
| 0.30 | 16 | 136.6 | 19.5 | 7.0:1 | 85.7% | 6.41 |
| 0.50 | 8 | 136.6 | 31.8 | 4.3:1 | 76.7% | 5.95 |
| 0.70 | 4 | 136.6 | 46.4 | 2.9:1 | 66.0% | 6.03 |
| 0.90 | 2 | 136.6 | 119.7 | 1.1:1 | 12.3% | 6.04 |
| 0.95 | 1 | 136.6 | 334.3 | 0.4:1 | -144.8% | 7.36 |

## Analysis

- **Best Compression**: Quality 0.10 achieved 12.3:1 ratio
- **Fastest Encoding**: Quality 0.50 at 5.95ms
- **Quality Mapping**: ⚠️ May need further tuning
