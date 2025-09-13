#!/usr/bin/env python3
"""
JPEG XS Implementation Progress Tracker

Monitors progress metrics over time and identifies trends.
"""

import json
import glob
from pathlib import Path
import datetime


def load_all_results(results_dir):
    """Load all test results from the results directory."""
    results_files = glob.glob(str(Path(results_dir) / "test-results-*.json"))
    results_files.sort()

    results = []
    for file_path in results_files:
        try:
            with open(file_path, 'r') as f:
                data = json.load(f)
                data['file_path'] = file_path
                results.append(data)
        except Exception as e:
            print(f"Error loading {file_path}: {e}")

    return results


def analyze_progress(results):
    """Analyze progress trends across test runs."""
    if not results:
        print("No test results found.")
        return

    print("📈 JPEG XS IMPLEMENTATION PROGRESS ANALYSIS")
    print("=" * 60)

    # Get latest results
    latest = results[-1]
    metrics = latest.get('metrics', {})

    print(f"\n🕒 Latest Test: {latest.get('timestamp', 'Unknown')}")
    print("-" * 40)

    if 'metrics' in latest:
        m = latest['metrics']
        print(f"File Size Ratio:     {m.get('size_ratio', 'N/A'):.1f}x larger than reference")
        print(f"Compression Gap:     {m.get('compression_gap_kb', 'N/A'):.1f} KB")
        print(f"Format Compliance:   {'✅ Yes' if m.get('format_compliance', False) else '❌ No'}")
        print(f"JPEG XS Markers:     {'✅ Present' if m.get('rust_has_soc_marker', False) else '❌ Missing'}")

    # Progress over time
    if len(results) > 1:
        print(f"\n📊 PROGRESS OVER TIME ({len(results)} test runs)")
        print("-" * 40)

        # Size ratio trend
        size_ratios = []
        for result in results:
            if 'metrics' in result and 'size_ratio' in result['metrics']:
                size_ratios.append(result['metrics']['size_ratio'])

        if len(size_ratios) > 1:
            first_ratio = size_ratios[0]
            latest_ratio = size_ratios[-1]
            improvement = first_ratio - latest_ratio

            print(f"Size Ratio Trend:")
            print(f"  First:   {first_ratio:.1f}x")
            print(f"  Latest:  {latest_ratio:.1f}x")
            print(f"  Change:  {improvement:+.1f}x {'📈 Better' if improvement > 0 else '📉 Worse' if improvement < 0 else '➡️  Same'}")

        # Compliance tracking
        compliance_history = []
        for result in results:
            if 'metrics' in result:
                compliance_history.append(result['metrics'].get('format_compliance', False))

        compliant_count = sum(compliance_history)
        print(f"\nFormat Compliance: {compliant_count}/{len(compliance_history)} runs")

    # Current gaps analysis
    print(f"\n🎯 CURRENT IMPLEMENTATION GAPS")
    print("-" * 40)

    if metrics.get('rust_has_soc_marker', False):
        print("✅ JPEG XS format structure present")
    else:
        print("❌ CRITICAL: Missing JPEG XS format structure")
        print("   → Need to implement SOC marker (FF 10)")

    size_ratio = metrics.get('size_ratio', 1)
    if size_ratio > 10:
        print("❌ CRITICAL: File size 10x+ larger than reference")
        print("   → Missing entropy coding implementation")
        print("   → No compression occurring")
    elif size_ratio > 2:
        print("⚠️  WARNING: File size 2x+ larger than reference")
        print("   → Entropy coding may be incomplete")
    else:
        print("✅ File size within reasonable range")

    # Next priorities
    print(f"\n🚀 RECOMMENDED NEXT STEPS")
    print("-" * 40)

    if not metrics.get('format_compliance', False):
        print("1. 🔥 URGENT: Implement JPEG XS bitstream format")
        print("   - Add SOC (Start of Codestream) marker")
        print("   - Add SIZ (Image Size) marker")
        print("   - Add basic packet structure")

    if size_ratio > 5:
        print("2. 🔥 URGENT: Implement entropy coding")
        print("   - VLC (Variable Length Coding) tables")
        print("   - Significance propagation passes")
        print("   - Bit packing and stream generation")

    print("3. 📋 Create systematic test cases")
    print("   - Multiple image sizes")
    print("   - Different YUV formats")
    print("   - Edge cases and error conditions")

    print("\n" + "=" * 60)


def main():
    results_dir = Path("validation-results")
    if not results_dir.exists():
        print("No validation results found. Run test_runner.py first.")
        return

    results = load_all_results(results_dir)
    analyze_progress(results)


if __name__ == "__main__":
    main()
