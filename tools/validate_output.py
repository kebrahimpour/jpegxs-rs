#!/usr/bin/env python3
"""
JPEG XS Output Validation Tool

Analyzes and compares JPEG XS bitstream outputs between reference and Rust implementations.
Identifies specific gaps in format compliance and compression efficiency.
"""

import sys
import os
from pathlib import Path


def analyze_jpeg_xs_markers(filepath):
    """Analyze JPEG XS markers in a file."""
    markers = {
        0xFF10: "SOC (Start of Codestream)",
        0xFF50: "SIZ (Image and tile size)",
        0xFF51: "CAP (Extended capabilities)",
        0xFF52: "COD (Coding style default)",
        0xFF12: "PIH (Packet information header)",
        0xFF13: "PIV (Packet information variable)",
        0xFF14: "EPH (End of packet header)",
        0xFF90: "SOT (Start of tile)",
        0xFF93: "SOD (Start of data)",
        0xFFD9: "EOC (End of codestream)"
    }

    found_markers = []

    try:
        with open(filepath, 'rb') as f:
            data = f.read()
            i = 0
            while i < len(data) - 1:
                if data[i] == 0xFF:
                    marker = (data[i] << 8) | data[i + 1]
                    if marker in markers:
                        found_markers.append({
                            'offset': hex(i),
                            'marker': hex(marker),
                            'name': markers[marker]
                        })
                        i += 2
                    else:
                        i += 1
                else:
                    i += 1
    except Exception as e:
        return f"Error reading file: {e}"

    return found_markers


def analyze_file_structure(filepath):
    """Analyze basic file structure and properties."""
    try:
        stat = os.stat(filepath)
        size = stat.st_size

        with open(filepath, 'rb') as f:
            first_16 = f.read(16)
            f.seek(-16, 2)  # Seek to 16 bytes from end
            last_16 = f.read(16)

        return {
            'size_bytes': size,
            'size_kb': round(size / 1024, 1),
            'first_16_hex': first_16.hex(),
            'last_16_hex': last_16.hex(),
            'is_jpeg_xs': first_16.startswith(b'\xff\x10')
        }
    except Exception as e:
        return f"Error analyzing file: {e}"


def compare_outputs(ref_file, rust_file):
    """Compare reference and Rust implementation outputs."""
    print("=" * 60)
    print("JPEG XS OUTPUT VALIDATION REPORT")
    print("=" * 60)

    # File structure analysis
    print("\n1. FILE STRUCTURE ANALYSIS")
    print("-" * 30)

    ref_info = analyze_file_structure(ref_file)
    rust_info = analyze_file_structure(rust_file)

    print(f"Reference (C):    {ref_info['size_kb']} KB")
    print(f"Rust:             {rust_info['size_kb']} KB")
    print(f"Size ratio:       {rust_info['size_bytes'] / ref_info['size_bytes']:.1f}x larger")

    print(f"\nReference starts: {ref_info['first_16_hex']}")
    print(f"Rust starts:      {rust_info['first_16_hex']}")

    print(f"\nReference JPEG XS compliant: {ref_info['is_jpeg_xs']}")
    print(f"Rust JPEG XS compliant:      {rust_info['is_jpeg_xs']}")

    # Marker analysis
    print("\n2. JPEG XS MARKER ANALYSIS")
    print("-" * 30)

    ref_markers = analyze_jpeg_xs_markers(ref_file)
    rust_markers = analyze_jpeg_xs_markers(rust_file)

    print(f"\nReference markers found: {len(ref_markers)}")
    for marker in ref_markers:
        print(f"  {marker['offset']}: {marker['marker']} - {marker['name']}")

    print(f"\nRust markers found: {len(rust_markers)}")
    if rust_markers:
        for marker in rust_markers:
            print(f"  {marker['offset']}: {marker['marker']} - {marker['name']}")
    else:
        print("  None - Raw data output")

    # Gap analysis
    print("\n3. IMPLEMENTATION GAPS")
    print("-" * 30)

    if not rust_info['is_jpeg_xs']:
        print("❌ CRITICAL: No JPEG XS format compliance")
        print("   - Missing SOC (Start of Codestream) marker")
        print("   - Output appears to be raw coefficient data")

    if len(rust_markers) == 0:
        print("❌ CRITICAL: No bitstream structure")
        print("   - No packet headers")
        print("   - No entropy coding")
        print("   - No compression")

    compression_ratio_ref = 128 / ref_info['size_kb']  # Assuming YUV input ~128KB
    compression_ratio_rust = 128 / rust_info['size_kb'] if rust_info['size_kb'] > 128 else 1

    print(f"\nCompression Analysis:")
    print(f"Reference compression: {compression_ratio_ref:.1f}:1")
    print(f"Rust compression:      {compression_ratio_rust:.1f}:1")
    print(f"Missing compression:   {(rust_info['size_bytes'] - ref_info['size_bytes']) / 1024:.1f} KB")

    # Next steps
    print("\n4. PRIORITY FIXES NEEDED")
    print("-" * 30)
    print("1. Implement JPEG XS bitstream format:")
    print("   - SOC marker (FF 10)")
    print("   - SIZ marker (FF 50) with image parameters")
    print("   - COD marker (FF 52) with coding parameters")
    print("2. Add entropy coding:")
    print("   - VLC tables")
    print("   - Significance propagation passes")
    print("3. Proper packet structure")

    print("\n" + "=" * 60)


def main():
    if len(sys.argv) != 3:
        print("Usage: python validate_output.py <reference.jxs> <rust.jxs>")
        sys.exit(1)

    ref_file = sys.argv[1]
    rust_file = sys.argv[2]

    if not os.path.exists(ref_file):
        print(f"Error: Reference file not found: {ref_file}")
        sys.exit(1)

    if not os.path.exists(rust_file):
        print(f"Error: Rust file not found: {rust_file}")
        sys.exit(1)

    compare_outputs(ref_file, rust_file)


if __name__ == "__main__":
    main()
