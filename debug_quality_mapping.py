#!/usr/bin/env python3

"""
Quality Parameter Analysis for JPEG XS
Shows the current problematic mapping and what it should be
"""

def current_quality_mapping(quality):
    """Current broken implementation"""
    target_bpp = quality * 8.0  # This is the problem!
    
    if target_bpp >= 8.0:
        return 1  # Almost no compression
    elif target_bpp >= 4.0:
        return 2  # Very little compression  
    elif target_bpp >= 2.0:
        return 4  # Some compression
    else:
        return 8  # More compression

def expected_quality_mapping(quality):
    """What the mapping should be"""
    # Quality should inversely map to quantization step size
    # Lower quality -> higher QP -> more compression
    if quality >= 0.9:
        return 2   # High quality, minimal compression loss
    elif quality >= 0.7:
        return 4   # Good quality, moderate compression
    elif quality >= 0.5: 
        return 8   # Medium quality, significant compression
    elif quality >= 0.3:
        return 16  # Lower quality, high compression
    else:
        return 32  # Low quality, maximum compression

def analyze_current_problem():
    print("🔍 JPEG XS Quality Parameter Analysis")
    print("="*50)
    
    test_qualities = [0.1, 0.3, 0.5, 0.7, 0.9, 0.95]
    
    print("\nCURRENT BROKEN IMPLEMENTATION:")
    print("Quality | Target BPP | QP  | Effect")
    print("--------|------------|-----|--------")
    for q in test_qualities:
        bpp = q * 8.0
        qp = current_quality_mapping(q)
        print(f"{q:7.2f} | {bpp:10.1f} | {qp:3d} | {'Terrible compression' if qp <= 2 else 'Still bad'}")
    
    print("\nEXPECTED CORRECT IMPLEMENTATION:")
    print("Quality | QP  | Expected Compression | Expected Ratio")
    print("--------|-----|---------------------|---------------")
    for q in test_qualities:
        qp = expected_quality_mapping(q)
        ratio = qp * 0.5  # Rough estimate
        print(f"{q:7.2f} | {qp:3d} | {'Excellent' if qp >= 16 else 'Good' if qp >= 8 else 'Moderate':<19} | {ratio:4.1f}:1")
    
    print("\n🚨 PROBLEM IDENTIFIED:")
    print("1. Quality 0.9 -> BPP 7.2 -> QP 1 (almost lossless!)")
    print("2. Even quality 0.1 -> BPP 0.8 -> QP 8 (still conservative)")
    print("3. Should be: Quality 0.9 -> QP 2, Quality 0.1 -> QP 32")
    
    print("\n🔧 SOLUTION:")
    print("Replace quality*8.0 mapping with proper inverse relationship")
    print("Higher quality -> Lower QP -> Less compression loss")
    print("Lower quality -> Higher QP -> More compression gain")

if __name__ == "__main__":
    analyze_current_problem()