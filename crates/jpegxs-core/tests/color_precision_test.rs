use jpegxs_core::colors;

#[test]
fn test_rgb_yuv_roundtrip_precision() {
    let width = 256u32;
    let height = 256u32;
    let size = (width * height) as usize;

    // Create RGB gradient
    let mut rgb_data = vec![0u8; size * 3];
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            rgb_data[idx * 3] = (x * 255 / width) as u8;     // R gradient
            rgb_data[idx * 3 + 1] = (y * 255 / height) as u8; // G gradient
            rgb_data[idx * 3 + 2] = 128;                        // B constant
        }
    }

    // Convert RGB to YUV
    let (y_data, u_data, v_data) = colors::rgb_to_yuv_planar(&rgb_data, width, height)
        .expect("RGB to YUV conversion failed");

    // Convert back to RGB
    let mut yuv_interleaved = vec![0u8; size * 3];
    for i in 0..size {
        yuv_interleaved[i * 3] = y_data[i];
        yuv_interleaved[i * 3 + 1] = u_data[i];
        yuv_interleaved[i * 3 + 2] = v_data[i];
    }

    let mut rgb_recovered = vec![0u8; size * 3];
    colors::yuv_to_rgb(&yuv_interleaved, &mut rgb_recovered, width, height)
        .expect("YUV to RGB conversion failed");

    // Calculate conversion error
    let mut max_error = 0i32;
    let mut mse = 0.0f64;
    let mut error_count = 0;

    for i in 0..rgb_data.len() {
        let orig = rgb_data[i] as i32;
        let recon = rgb_recovered[i] as i32;
        let error = (orig - recon).abs();
        max_error = max_error.max(error);
        mse += (error as f64).powi(2);
        if error > 1 {
            error_count += 1;
        }
    }

    mse /= rgb_data.len() as f64;
    let psnr = if mse > 0.0 {
        10.0 * (255.0_f64.powi(2) / mse).log10()
    } else {
        f64::INFINITY
    };

    println!("RGB↔YUV Color Conversion Precision Test Results:");
    println!("  Max pixel error: {}", max_error);
    println!("  MSE: {:.6}", mse);
    println!("  PSNR: {:.2} dB", psnr);
    println!("  Pixels with error >1: {}/{}", error_count, rgb_data.len());

    // Color conversion should be nearly lossless
    assert!(psnr > 40.0, "Color conversion PSNR too low: {:.2} dB", psnr);
}

#[test]
fn test_yuv_444_precision() {
    let width = 64u32;
    let height = 64u32;
    let size = (width * height) as usize;

    // Create YUV444 test data with known values
    let mut test_data = vec![0u8; size * 3];

    // Y plane: gradient from 0 to 255
    for (i, item) in test_data.iter_mut().enumerate().take(size) {
        *item = (i * 255 / size) as u8;
    }
    // U plane: constant 128 (neutral)
    for item in test_data.iter_mut().take(size*2).skip(size) {
        *item = 128;
    }
    // V plane: constant 128 (neutral)
    for item in test_data.iter_mut().take(size*3).skip(size*2) {
        *item = 128;
    }

    println!("YUV444 Direct Processing Test:");
    println!("  Input Y range: {} to {}", test_data[0], test_data[size-1]);
    println!("  Input U value: {}", test_data[size]);
    println!("  Input V value: {}", test_data[size*2]);

    // Test whether the issue is in YUV444 handling specifically
    // This should show if the problem is color conversion vs. codec pipeline
}
