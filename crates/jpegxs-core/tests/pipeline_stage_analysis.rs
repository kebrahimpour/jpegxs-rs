// Comprehensive pipeline stage analysis to isolate quality loss
use jpegxs_core_clean::dwt;

#[test]
fn test_pipeline_stage_by_stage() {
    let width = 256u32;
    let height = 256u32;
    let size = (width * height) as usize;

    // Create gradient test pattern (same as our main test)
    let mut test_data = vec![0u8; size * 3];
    for y in 0..height {
        for x in 0..width {
            let pixel_idx = (y * width + x) as usize;
            let val = (x * 255 / width) as u8;
            test_data[pixel_idx] = val; // Y channel - gradient
            test_data[pixel_idx + size] = 128; // U channel - neutral
            test_data[pixel_idx + 2 * size] = 128; // V channel - neutral
        }
    }

    println!("=== PIPELINE STAGE-BY-STAGE ANALYSIS ===");
    println!("Input: {}x{} YUV444p8 gradient pattern", width, height);

    // Stage 1: RGB/YUV conversion precision
    println!("\n1. INPUT DATA ANALYSIS:");
    let input_y = &test_data[0..size];
    let input_min = *input_y.iter().min().unwrap();
    let input_max = *input_y.iter().max().unwrap();
    let input_mean = input_y.iter().map(|&x| x as f64).sum::<f64>() / size as f64;
    println!(
        "  Y channel - Min: {}, Max: {}, Mean: {:.2}",
        input_min, input_max, input_mean
    );

    // Stage 2: Convert to floating point and center around 0
    let mut y_plane = vec![0.0f32; size];
    for (i, &val) in input_y.iter().enumerate() {
        y_plane[i] = val as f32 - 128.0;
    }

    println!("\n2. AFTER FLOAT CONVERSION (-128 centering):");
    let float_min = y_plane.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let float_max = y_plane.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let float_mean = y_plane.iter().sum::<f32>() / y_plane.len() as f32;
    println!(
        "  Y plane - Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        float_min, float_max, float_mean
    );

    // Stage 3: Apply DWT
    let mut y_dwt = vec![0.0f32; size];
    dwt::dwt_53_forward_2d(&y_plane, &mut y_dwt, width, height).expect("Forward DWT failed");

    println!("\n3. AFTER DWT:");
    let dwt_min = y_dwt.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let dwt_max = y_dwt.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let dwt_mean = y_dwt.iter().sum::<f32>() / y_dwt.len() as f32;
    println!(
        "  Y DWT coeffs - Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        dwt_min, dwt_max, dwt_mean
    );

    // Stage 4: Apply quantization (QP=1 for quality 0.9)
    let qp = 1u8;
    let mut y_quantized = Vec::with_capacity(size);
    for &coeff in &y_dwt {
        let quantized = (coeff / qp as f32).round() as i32;
        y_quantized.push(quantized);
    }

    println!("\n4. AFTER QUANTIZATION (QP={}):", qp);
    let quant_min = y_quantized.iter().min().unwrap();
    let quant_max = y_quantized.iter().max().unwrap();
    let quant_mean = y_quantized.iter().sum::<i32>() as f32 / y_quantized.len() as f32;
    println!(
        "  Y quantized - Min: {}, Max: {}, Mean: {:.3}",
        quant_min, quant_max, quant_mean
    );

    // Stage 5: Apply dequantization
    let mut y_dequantized = Vec::with_capacity(size);
    for &quant_coeff in &y_quantized {
        let dequantized = quant_coeff as f32 * qp as f32;
        y_dequantized.push(dequantized);
    }

    println!("\n5. AFTER DEQUANTIZATION:");
    let dequant_min = y_dequantized.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let dequant_max = y_dequantized
        .iter()
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let dequant_mean = y_dequantized.iter().sum::<f32>() / y_dequantized.len() as f32;
    println!(
        "  Y dequantized - Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        dequant_min, dequant_max, dequant_mean
    );

    // Stage 6: Apply inverse DWT
    let mut y_reconstructed = vec![0.0f32; size];
    dwt::dwt_53_inverse_2d(&y_dequantized, &mut y_reconstructed, width, height)
        .expect("Inverse DWT failed");

    println!("\n6. AFTER INVERSE DWT:");
    let recon_min = y_reconstructed.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let recon_max = y_reconstructed
        .iter()
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let recon_mean = y_reconstructed.iter().sum::<f32>() / y_reconstructed.len() as f32;
    println!(
        "  Y reconstructed - Min: {:.3}, Max: {:.3}, Mean: {:.3}",
        recon_min, recon_max, recon_mean
    );

    // Stage 7: Convert back to 8-bit
    let mut y_final = Vec::with_capacity(size);
    for &sample in &y_reconstructed {
        y_final.push((sample + 128.0).clamp(0.0, 255.0) as u8);
    }

    println!("\n7. AFTER 8-BIT CONVERSION (+128, clamp):");
    let final_min = *y_final.iter().min().unwrap();
    let final_max = *y_final.iter().max().unwrap();
    let final_mean = y_final.iter().map(|&x| x as f64).sum::<f64>() / size as f64;
    println!(
        "  Y final - Min: {}, Max: {}, Mean: {:.2}",
        final_min, final_max, final_mean
    );

    // Calculate cumulative errors at each stage
    println!("\n=== CUMULATIVE ERROR ANALYSIS ===");

    // Error after quantization roundtrip (DWT -> Quant -> Dequant -> Inverse DWT)
    let mut quant_error = 0.0f64;
    let mut quant_max_error = 0.0f32;
    for (orig, recon) in y_plane.iter().zip(y_reconstructed.iter()) {
        let error = (orig - recon).abs();
        quant_max_error = quant_max_error.max(error);
        quant_error += (error as f64).powi(2);
    }
    quant_error /= size as f64;
    let quant_psnr = if quant_error > 0.0 {
        10.0 * (127.0_f64.powi(2) / quant_error).log10()
    } else {
        f64::INFINITY
    };
    println!(
        "Quantization roundtrip - Max error: {:.6}, PSNR: {:.2} dB",
        quant_max_error, quant_psnr
    );

    // Error after full pipeline (including 8-bit conversion)
    let mut total_error = 0.0f64;
    let mut total_max_error = 0i32;
    for (orig, recon) in input_y.iter().zip(y_final.iter()) {
        let error = (*orig as i32 - *recon as i32).abs();
        total_max_error = total_max_error.max(error);
        total_error += (error as f64).powi(2);
    }
    total_error /= size as f64;
    let total_psnr = if total_error > 0.0 {
        10.0 * (255.0_f64.powi(2) / total_error).log10()
    } else {
        f64::INFINITY
    };
    println!(
        "Full pipeline - Max error: {}, PSNR: {:.2} dB",
        total_max_error, total_psnr
    );

    // Expected vs Actual quality
    println!("\n=== QUALITY ASSESSMENT ===");
    if total_psnr > 40.0 {
        println!("✅ EXCELLENT quality (>40 dB)");
    } else if total_psnr > 30.0 {
        println!("✅ GOOD quality (30-40 dB)");
    } else if total_psnr > 20.0 {
        println!("⚠️  MODERATE quality (20-30 dB)");
    } else {
        println!("❌ POOR quality (<20 dB) - Major issue detected");
    }

    println!("Expected for QP=1: >30 dB, Actual: {:.2} dB", total_psnr);

    if total_psnr < 30.0 {
        println!("\n🔍 DIAGNOSIS:");
        println!("Quality is too low for QP=1. Possible issues:");
        println!("- Integer precision loss in quantization");
        println!("- Coefficient clamping during processing");
        println!("- Additional unidentified quantization step");
        println!("- Bitstream encoding/decoding precision loss");
    }
}
