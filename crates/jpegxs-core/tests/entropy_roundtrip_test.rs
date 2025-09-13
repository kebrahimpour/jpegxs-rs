// Test entropy coding roundtrip accuracy
use jpegxs_core_clean::{JpegXsBitstream, JpegXsDecoder};

#[test]
fn test_entropy_roundtrip_precision() {
    println!("=== ENTROPY CODING ROUNDTRIP ANALYSIS ===");

    // Create representative coefficient patterns found in DWT
    let test_coefficients = vec![
        // Small coefficients (should be encoded perfectly)
        0, 1, -1, 2, -2, 3, -3, // Medium coefficients (4-127 range)
        4, -4, 15, -15, 32, -32, 63, -63, 127, -127, // Large coefficients (>127)
        128, -128, 200, -200, 255, -255, // Zero runs
        0, 0, 0, 0, 0, // Mixed pattern
        5, 0, -12, 0, 0, 23, -45, 0, 67,
    ];

    println!("Original coefficients: {:?}", test_coefficients);

    // Create minimal bitstream with entropy data
    let mut bitstream = JpegXsBitstream::new();
    bitstream.write_cap_marker();
    bitstream.write_pih_marker(16, 16, 1); // Small test image
    bitstream.write_cdt_marker(1);
    bitstream.write_wgt_marker(Some(&[1])); // QP=1
    bitstream.add_entropy_coded_data(&test_coefficients);
    bitstream.finalize();

    let encoded_data = bitstream.into_bytes();
    println!("Bitstream size: {} bytes", encoded_data.len());

    // Decode and compare
    let mut decoder = JpegXsDecoder::new(encoded_data).expect("Decoder creation failed");
    decoder.parse_headers().expect("Header parsing failed");
    let decoded_coefficients = decoder
        .decode_entropy_data()
        .expect("Entropy decoding failed");

    println!("Decoded coefficients:  {:?}", decoded_coefficients);

    // Analyze precision loss
    let mut max_error = 0i32;
    let mut total_error = 0i64;
    let mut error_count = 0;

    println!("\n=== COEFFICIENT-BY-COEFFICIENT ANALYSIS ===");
    for (i, (&orig, &decoded)) in test_coefficients
        .iter()
        .zip(decoded_coefficients.iter())
        .enumerate()
    {
        let error = (orig - decoded).abs();
        if error > 0 {
            println!("Coeff[{}]: {} → {} (error: {})", i, orig, decoded, error);
            error_count += 1;
        }
        max_error = max_error.max(error);
        total_error += error as i64;
    }

    let avg_error = if !test_coefficients.is_empty() {
        total_error as f64 / test_coefficients.len() as f64
    } else {
        0.0
    };

    println!("\n=== ENTROPY ROUNDTRIP QUALITY ===");
    println!("Total coefficients: {}", test_coefficients.len());
    println!("Coefficients with error: {}", error_count);
    println!("Max error: {}", max_error);
    println!("Average error: {:.3}", avg_error);
    println!(
        "Error rate: {:.1}%",
        (error_count as f64 / test_coefficients.len() as f64) * 100.0
    );

    if max_error == 0 {
        println!("✅ PERFECT entropy roundtrip");
    } else if max_error <= 1 {
        println!("✅ EXCELLENT entropy roundtrip (max error ≤ 1)");
    } else if max_error <= 5 {
        println!("⚠️  MODERATE entropy roundtrip (max error ≤ 5)");
    } else {
        println!("❌ POOR entropy roundtrip (max error > 5)");
    }

    // Check for systematic patterns in errors
    println!("\n=== ERROR PATTERN ANALYSIS ===");
    let mut small_coeff_errors = 0;
    let mut medium_coeff_errors = 0;
    let mut large_coeff_errors = 0;

    for (&orig, &decoded) in test_coefficients.iter().zip(decoded_coefficients.iter()) {
        if (orig - decoded).abs() > 0 {
            if orig.abs() <= 3 {
                small_coeff_errors += 1;
            } else if orig.abs() <= 127 {
                medium_coeff_errors += 1;
            } else {
                large_coeff_errors += 1;
            }
        }
    }

    println!(
        "Small coefficient errors (|coeff| ≤ 3): {}",
        small_coeff_errors
    );
    println!(
        "Medium coefficient errors (4 ≤ |coeff| ≤ 127): {}",
        medium_coeff_errors
    );
    println!(
        "Large coefficient errors (|coeff| > 127): {}",
        large_coeff_errors
    );

    if large_coeff_errors > 0 {
        println!("🔍 Large coefficients show precision loss - check range clamping");
    }
    if medium_coeff_errors > 0 {
        println!("🔍 Medium coefficients show precision loss - check encoding logic");
    }
    if small_coeff_errors > 0 {
        println!("🔍 Small coefficients show precision loss - check basic encoding");
    }

    // Test expectation: For a lossless entropy coding system, max_error should be 0
    // For practical purposes, max_error ≤ 1 is acceptable
    if max_error > 1 {
        println!("\n❌ ENTROPY CODING PRECISION ISSUE DETECTED");
        println!("Expected: max_error ≤ 1, Actual: max_error = {}", max_error);
        println!("This precision loss explains the poor pipeline quality!");
    }
}
