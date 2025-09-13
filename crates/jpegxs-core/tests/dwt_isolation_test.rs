// Test to isolate DWT implementation and verify fix without quantization/entropy
use jpegxs_core_clean::dwt;

#[test]
fn test_dwt_only_precision() {
    let width = 256u32;
    let height = 256u32;
    let size = (width * height) as usize;

    // Create gradient test pattern - same as pipeline test
    let mut input_data = vec![0.0f32; size];
    for y in 0..height {
        for x in 0..width {
            let pixel_idx = (y * width + x) as usize;
            let val = (x * 255 / width) as f32;
            input_data[pixel_idx] = val - 128.0; // Center around 0 like the pipeline
        }
    }

    println!("Input data statistics:");
    let min_val = input_data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_val = input_data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let mean_val = input_data.iter().sum::<f32>() / input_data.len() as f32;
    println!(
        "  Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        min_val, max_val, mean_val
    );

    // Apply forward DWT
    let mut dwt_coeffs = vec![0.0f32; size];
    dwt::dwt_53_forward_2d(&input_data, &mut dwt_coeffs, width, height)
        .expect("Forward DWT failed");

    println!("\nPost-DWT coefficients:");
    let dwt_min = dwt_coeffs.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let dwt_max = dwt_coeffs.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let dwt_mean = dwt_coeffs.iter().sum::<f32>() / dwt_coeffs.len() as f32;
    println!(
        "  Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        dwt_min, dwt_max, dwt_mean
    );

    // Apply inverse DWT
    let mut reconstructed = vec![0.0f32; size];
    dwt::dwt_53_inverse_2d(&dwt_coeffs, &mut reconstructed, width, height)
        .expect("Inverse DWT failed");

    println!("\nPost-Inverse-DWT reconstruction:");
    let recon_min = reconstructed.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let recon_max = reconstructed
        .iter()
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let recon_mean = reconstructed.iter().sum::<f32>() / reconstructed.len() as f32;
    println!(
        "  Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        recon_min, recon_max, recon_mean
    );

    // Calculate reconstruction error
    let mut max_error = 0.0f32;
    let mut mse = 0.0f64;

    for (orig, recon) in input_data.iter().zip(reconstructed.iter()) {
        let error = (orig - recon).abs();
        max_error = max_error.max(error);
        mse += (error as f64).powi(2);
    }

    mse /= size as f64;
    let psnr = if mse > 0.0 {
        10.0 * (127.0_f64.powi(2) / mse).log10() // Using 127 as max range since we center around 0
    } else {
        f64::INFINITY
    };

    println!("\nDWT-only reconstruction results:");
    println!("  Max error: {:.6}", max_error);
    println!("  MSE: {:.6}", mse);
    println!("  PSNR: {:.2} dB", psnr);

    if max_error < 1e-6 {
        println!("✅ DWT implementation is PERFECT (error < 1e-6)");
    } else if max_error < 0.1 {
        println!("✅ DWT implementation is GOOD (error < 0.1)");
    } else {
        println!(
            "❌ DWT implementation has ISSUES (error = {:.6})",
            max_error
        );
    }

    // Assert that DWT should be near-perfect for lossless compression
    assert!(
        max_error < 1e-6,
        "DWT should be perfectly reconstructed, but max error is {:.6}",
        max_error
    );
}
