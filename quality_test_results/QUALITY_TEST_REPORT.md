# JPEG XS Quality Parameter Test Report

**Generated**: 2025-09-07 14:36:25 UTC
**Quality Levels Tested**: 6

## Quality vs Compression Analysis

| Quality | QP | Original (KB) | Compressed (KB) | Ratio | Reduction % | Encode (ms) |
|---------|----|--------------:|----------------:|------:|------------:|------------:|
| 0.10 | 32 | 136.6 | 11.1 | 12.3:1 | 91.9% | 95.47 |
| 0.30 | 16 | 136.6 | 19.5 | 7.0:1 | 85.7% | 86.83 |
| 0.50 | 8 | 136.6 | 31.8 | 4.3:1 | 76.7% | 88.43 |
| 0.70 | 4 | 136.6 | 46.4 | 2.9:1 | 66.0% | 87.53 |
| 0.90 | 2 | 136.6 | 119.7 | 1.1:1 | 12.3% | 89.36 |
| 0.95 | 1 | 136.6 | 334.3 | 0.4:1 | -144.8% | 93.95 |

## Analysis

- **Best Compression**: Quality 0.10 achieved 12.3:1 ratio
- **Fastest Encoding**: Quality 0.30 at 86.83ms
- **Quality Mapping**: ⚠️ May need further tuning
