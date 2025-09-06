#!/usr/bin/env python3
"""
JPEG XS Test Runner

Automated test framework to validate Rust implementation against reference.
Tracks progress over development iterations.
"""

import subprocess
import os
import json
import datetime
from pathlib import Path


class JPEGXSTestRunner:
    def __init__(self, project_root):
        self.project_root = Path(project_root)
        self.test_data_dir = self.project_root / "test-data"
        self.results_dir = self.project_root / "validation-results"
        self.results_dir.mkdir(exist_ok=True)
        
    def run_rust_encoder(self, input_yuv, output_jxs):
        """Run the Rust encoder on test input."""
        try:
            cmd = [
                "cargo", "run", "--bin", "jpegxs", "--",
                "encode", 
                "--input", str(input_yuv),
                "--output", str(output_jxs),
                "--width", "256",
                "--height", "256",
                "--format", "yuv422p"
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, cwd=self.project_root)
            
            return {
                "success": result.returncode == 0,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "returncode": result.returncode
            }
        except Exception as e:
            return {
                "success": False,
                "error": str(e)
            }
    
    def run_reference_encoder(self, input_yuv, output_jxs):
        """Run the reference C encoder on test input."""
        try:
            ref_encoder = self.project_root / "reference/jxs/build/bin/jxs_encoder"
            if not ref_encoder.exists():
                return {"success": False, "error": "Reference encoder not found"}
            
            cmd = [
                str(ref_encoder),
                "-w", "256",
                "-h", "256", 
                "-c", "profile=Main422.10;rate=3.0",
                str(input_yuv),
                str(output_jxs)
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            return {
                "success": result.returncode == 0,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "returncode": result.returncode
            }
        except Exception as e:
            return {
                "success": False,
                "error": str(e)
            }
    
    def validate_output(self, ref_file, rust_file):
        """Run validation analysis on outputs."""
        try:
            validator = self.project_root / "tools/validate_output.py"
            cmd = ["python3", str(validator), str(ref_file), str(rust_file)]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            return {
                "success": result.returncode == 0,
                "output": result.stdout,
                "stderr": result.stderr
            }
        except Exception as e:
            return {
                "success": False,
                "error": str(e)
            }
    
    def extract_metrics(self, ref_file, rust_file):
        """Extract key metrics for tracking progress."""
        metrics = {}
        
        try:
            ref_stat = os.stat(ref_file)
            rust_stat = os.stat(rust_file)
            
            metrics["ref_size_bytes"] = ref_stat.st_size
            metrics["rust_size_bytes"] = rust_stat.st_size
            metrics["size_ratio"] = rust_stat.st_size / ref_stat.st_size
            metrics["compression_gap_kb"] = (rust_stat.st_size - ref_stat.st_size) / 1024
            
            # Check for JPEG XS compliance
            with open(ref_file, 'rb') as f:
                ref_start = f.read(4)
            with open(rust_file, 'rb') as f:
                rust_start = f.read(4)
            
            metrics["ref_has_soc_marker"] = ref_start.startswith(b'\xff\x10')
            metrics["rust_has_soc_marker"] = rust_start.startswith(b'\xff\x10')
            metrics["format_compliance"] = metrics["rust_has_soc_marker"]
            
        except Exception as e:
            metrics["error"] = str(e)
        
        return metrics
    
    def run_full_test_suite(self):
        """Run complete validation test suite."""
        timestamp = datetime.datetime.now().isoformat()
        
        print("🧪 JPEG XS Validation Test Suite")
        print("=" * 50)
        
        # Test files
        input_yuv = self.test_data_dir / "test_256x256.yuv"
        ref_output = self.test_data_dir / "test_256x256_ref.jxs"  
        rust_output = self.test_data_dir / "test_256x256_rust.jxs"
        
        results = {
            "timestamp": timestamp,
            "test_input": str(input_yuv),
            "reference_output": str(ref_output),
            "rust_output": str(rust_output)
        }
        
        # 1. Build Rust implementation
        print("📦 Building Rust implementation...")
        build_result = subprocess.run(
            ["cargo", "build", "--release"], 
            capture_output=True, text=True, cwd=self.project_root
        )
        
        results["build"] = {
            "success": build_result.returncode == 0,
            "stderr": build_result.stderr
        }
        
        if not results["build"]["success"]:
            print("❌ Build failed!")
            print(build_result.stderr)
            return results
        
        print("✅ Build successful")
        
        # 2. Generate/verify test input
        if not input_yuv.exists():
            print("📝 Generating test input...")
            gen_script = self.test_data_dir / "generate_test_image.py"
            if gen_script.exists():
                subprocess.run(["python3", str(gen_script)], cwd=self.test_data_dir)
        
        # 3. Run Rust encoder
        print("🦀 Running Rust encoder...")
        rust_encode = self.run_rust_encoder(input_yuv, rust_output)
        results["rust_encode"] = rust_encode
        
        if not rust_encode["success"]:
            print(f"❌ Rust encoding failed: {rust_encode.get('stderr', 'Unknown error')}")
        else:
            print("✅ Rust encoding completed")
        
        # 4. Run reference encoder (if available and input changed)
        if not ref_output.exists():
            print("🏛️  Running reference encoder...")
            ref_encode = self.run_reference_encoder(input_yuv, ref_output)
            results["ref_encode"] = ref_encode
            
            if not ref_encode["success"]:
                print(f"❌ Reference encoding failed: {ref_encode.get('stderr', 'Unknown error')}")
            else:
                print("✅ Reference encoding completed")
        
        # 5. Compare outputs
        if ref_output.exists() and rust_output.exists():
            print("🔍 Analyzing outputs...")
            
            validation = self.validate_output(ref_output, rust_output)
            results["validation"] = validation
            
            metrics = self.extract_metrics(ref_output, rust_output) 
            results["metrics"] = metrics
            
            # Print key metrics
            print(f"📊 File Size: {metrics.get('ref_size_bytes', 0) / 1024:.1f} KB (ref) vs {metrics.get('rust_size_bytes', 0) / 1024:.1f} KB (rust)")
            print(f"📊 Size Ratio: {metrics.get('size_ratio', 0):.1f}x larger")
            print(f"📊 Format Compliance: {'✅' if metrics.get('format_compliance', False) else '❌'}")
            
            # Print validation output
            if validation["success"]:
                print("\n" + validation["output"])
        
        # 6. Save results
        results_file = self.results_dir / f"test-results-{timestamp.replace(':', '-')}.json"
        with open(results_file, 'w') as f:
            json.dump(results, f, indent=2)
        
        print(f"\n💾 Results saved to: {results_file}")
        
        return results


def main():
    runner = JPEGXSTestRunner(".")
    runner.run_full_test_suite()


if __name__ == "__main__":
    main()