use anyhow::Result;

pub fn quantize(_coeffs: &[f32], _qp: u8) -> Result<Vec<i32>> {
    todo!("Quantization")
}

pub fn dequantize(_coeffs: &[i32], _qp: u8) -> Result<Vec<f32>> {
    todo!("Dequantization")
}

pub fn compute_quantization_parameters(_target_bpp: f32) -> Result<Vec<u8>> {
    todo!("Compute quantization parameters")
}
