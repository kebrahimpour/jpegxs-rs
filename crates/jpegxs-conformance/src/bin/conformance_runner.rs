use anyhow::Result;
use clap::{Arg, Command};
use jpegxs_conformance::test_runner::ConformanceTestRunner;
use jpegxs_core::{EncoderConfig, DecoderConfig};

fn main() -> Result<()> {
    let matches = Command::new("JPEG XS Conformance Test Runner")
        .version("0.1.0")
        .author("JPEG XS Codec Team")
        .about("Runs comprehensive conformance tests for JPEG XS implementation")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file for test report (JSON format)")
                .default_value("conformance_report.json")
        )
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .value_name("FLOAT")
                .help("Encoder quality setting (0.0-1.0)")
                .default_value("0.95")
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("MS")
                .help("Maximum test timeout in milliseconds")
                .default_value("30000")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose output")
        )
        .arg(
            Arg::new("quick")
                .long("quick")
                .action(clap::ArgAction::SetTrue)
                .help("Run quick test suite (subset of tests)")
        )
        .get_matches();

    let output_file = matches.get_one::<String>("output").unwrap();
    let quality: f32 = matches.get_one::<String>("quality").unwrap()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid quality value"))?;
    let timeout: u64 = matches.get_one::<String>("timeout").unwrap()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid timeout value"))?;
    let verbose = matches.get_flag("verbose");
    let quick = matches.get_flag("quick");

    if !(0.0..=1.0).contains(&quality) {
        return Err(anyhow::anyhow!("Quality must be between 0.0 and 1.0"));
    }

    // Set up encoder config
    let encoder_config = EncoderConfig {
        quality,
        ..Default::default()
    };

    let decoder_config = DecoderConfig {
        strict_mode: true,
    };

    // Create test runner
    let runner = ConformanceTestRunner::new()
        .with_encoder_config(encoder_config)
        .with_decoder_config(decoder_config)
        .with_timeout(timeout);

    if verbose {
        println!("🔧 Configuration:");
        println!("   Quality: {:.2}", quality);
        println!("   Timeout: {}ms", timeout);
        println!("   Output: {}", output_file);
        println!("   Mode: {}", if quick { "Quick" } else { "Full" });
        println!();
    }

    // Run the test suite
    let report = if quick {
        run_quick_tests(&runner)?
    } else {
        runner.run_full_conformance_suite()?
    };

    // Save the report
    runner.save_report(&report, output_file)?;

    // Print summary
    print_summary(&report);

    // Exit with appropriate code
    let exit_code = if report.conformance.compliance_percentage >= 80.0 {
        0 // Success
    } else if report.conformance.compliance_percentage >= 60.0 {
        1 // Warning
    } else {
        2 // Failure
    };

    std::process::exit(exit_code);
}

fn run_quick_tests(runner: &ConformanceTestRunner) -> Result<jpegxs_conformance::TestReport> {
    println!("🚀 Running Quick Conformance Test Suite");
    println!("═══════════════════════════════════════");
    println!("Note: This is a subset of the full test suite for faster feedback.");
    println!();

    // For now, just run the full suite
    // In the future, we could implement a reduced test set
    runner.run_full_conformance_suite()
}

fn print_summary(report: &jpegxs_conformance::TestReport) {
    let separator = "═".repeat(60);
    println!("\n{}", separator);
    println!("📋 CONFORMANCE TEST SUMMARY");
    println!("{}", separator);

    println!("\n🎯 Overall Results:");
    println!("   Compliance Level: {:.1}%", report.conformance.compliance_percentage);

    let status_icon = if report.conformance.compliance_percentage >= 80.0 {
        "✅"
    } else if report.conformance.compliance_percentage >= 60.0 {
        "⚠️"
    } else {
        "❌"
    };

    println!("   Status: {} {}", status_icon, get_compliance_status(report.conformance.compliance_percentage));

    println!("\n📊 Test Categories:");
    println!("   Encoder Tests:   {}/{} passed ({:.1}%)",
        report.conformance.encoder_tests.passed,
        report.conformance.encoder_tests.total,
        (report.conformance.encoder_tests.passed as f64 / report.conformance.encoder_tests.total as f64) * 100.0
    );
    println!("   Decoder Tests:   {}/{} passed ({:.1}%)",
        report.conformance.decoder_tests.passed,
        report.conformance.decoder_tests.total,
        (report.conformance.decoder_tests.passed as f64 / report.conformance.decoder_tests.total as f64) * 100.0
    );
    println!("   Bitstream Tests: {}/{} passed ({:.1}%)",
        report.conformance.bitstream_tests.passed,
        report.conformance.bitstream_tests.total,
        (report.conformance.bitstream_tests.passed as f64 / report.conformance.bitstream_tests.total as f64) * 100.0
    );

    println!("\n⚡ Performance Metrics:");
    println!("   Encoding Speed:  {:.1} Mbps", report.performance.speed.encode_mbps);
    println!("   Decoding Speed:  {:.1} Mbps", report.performance.speed.decode_mbps);
    println!("   Memory Usage:    {:.1} MB peak", report.performance.memory.peak_heap_mb);
    println!("   Avg PSNR:        {:.1} dB", report.performance.compression.avg_psnr_db);
    println!("   Compression:     {:.1}:1", report.performance.compression.avg_ratio);

    if report.conformance.encoder_tests.failed > 0 ||
       report.conformance.decoder_tests.failed > 0 ||
       report.conformance.bitstream_tests.failed > 0 {

        println!("\n❌ Failed Tests:");
        print_failed_tests(&report.conformance.encoder_tests, "Encoder");
        print_failed_tests(&report.conformance.decoder_tests, "Decoder");
        print_failed_tests(&report.conformance.bitstream_tests, "Bitstream");
    }

    println!("\n📄 Detailed report saved as JSON");
    println!("{}", "═".repeat(60));
}

fn get_compliance_status(percentage: f64) -> &'static str {
    if percentage >= 95.0 {
        "Excellent"
    } else if percentage >= 80.0 {
        "Good"
    } else if percentage >= 60.0 {
        "Moderate"
    } else if percentage >= 40.0 {
        "Poor"
    } else {
        "Critical"
    }
}

fn print_failed_tests(test_suite: &jpegxs_conformance::TestSuite, category: &str) {
    let failed_tests: Vec<_> = test_suite.details.iter()
        .filter(|t| matches!(t.status, jpegxs_conformance::TestStatus::Fail))
        .collect();

    if !failed_tests.is_empty() {
        println!("   {} Failures:", category);
        for test in failed_tests {
            println!("     • {} - {}",
                test.name,
                test.message.as_deref().unwrap_or("No details")
            );
        }
    }
}
