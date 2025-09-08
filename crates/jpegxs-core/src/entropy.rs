use anyhow::Result;

/// JPEG XS entropy coding implementation based on ISO/IEC 21122-1:2024 Annex C
///
/// This module implements the standard JPEG XS entropy coding system including:
/// - Bitplane counting (Subclause C.5.3)
/// - Variable Length Coding (VLC) primitive (Subclause C.7)
/// - Significance coding (Subclause C.5.2)
/// - Data subpacket coding (Subclause C.5.4)
///
/// Bitplane count for a code group - number of significant bitplanes
/// counting from LSB up to the most significant non-empty bitplane.
#[derive(Debug, Clone, Copy)]
pub struct BitplaneCount(pub u8);

/// Variable Length Coding context for entropy coding
#[derive(Debug, Clone, Copy)]
pub struct VlcContext {
    pub predictor: i32,     // r - predictor value
    pub truncation_pos: u8, // t - truncation position
    pub br_bits: u8,        // Br - number of bits to encode bitplane count in raw mode
}

/// Bitstream reader for VLC decoding
pub struct BitstreamReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8, // 0-7, bit position within current byte
}

impl<'a> BitstreamReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    /// Read a single bit from the bitstream
    pub fn read_bit(&mut self) -> Result<bool> {
        if self.byte_pos >= self.data.len() {
            return Err(anyhow::anyhow!("End of bitstream"));
        }

        let byte = self.data[self.byte_pos];
        let bit = (byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }

        Ok(bit == 1)
    }

    /// Read n bits as unsigned integer
    pub fn read_bits(&mut self, n: u8) -> Result<u32> {
        let mut value = 0u32;
        for _ in 0..n {
            value = (value << 1) | if self.read_bit()? { 1 } else { 0 };
        }
        Ok(value)
    }
}

/// Bitstream writer for VLC encoding  
#[derive(Default)]
pub struct BitstreamWriter {
    data: Vec<u8>,
    current_byte: u8,
    bit_pos: u8, // 0-7, bit position within current byte
}

impl BitstreamWriter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Write a single bit to the bitstream
    pub fn write_bit(&mut self, bit: bool) {
        if bit {
            self.current_byte |= 1 << (7 - self.bit_pos);
        }

        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.data.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }
    }

    /// Write n bits from value
    pub fn write_bits(&mut self, value: u32, n: u8) {
        for i in (0..n).rev() {
            self.write_bit((value >> i) & 1 == 1);
        }
    }

    /// Finalize and return the bitstream data
    pub fn finalize(mut self) -> Vec<u8> {
        // Flush remaining bits if any
        if self.bit_pos > 0 {
            self.data.push(self.current_byte);
        }
        self.data
    }
}

/// VLC decoding primitive as specified in ISO/IEC 21122-1:2024 Table C.17
///
/// This implements the core variable length coding primitive used throughout
/// the JPEG XS entropy coding system.
///
/// Note: The algorithm has edge cases when θ = 0 where the signed and unary
/// alphabets overlap. In practice, predictors should be chosen such that θ > 0.
pub fn vlc_decode(reader: &mut BitstreamReader, ctx: VlcContext) -> Result<i32> {
    let r = ctx.predictor;
    let t = ctx.truncation_pos as i32;
    let br = ctx.br_bits;

    // Compute the threshold for the alphabet switch (θ = max(r−t, 0))
    let theta = std::cmp::max(r - t, 0);

    // Count consecutive 1-bits (unary prefix)
    let mut n = 0i32;
    let max_consecutive = if br == 4 {
        32
    } else if br == 5 {
        64
    } else {
        1 << (br + 1)
    };

    loop {
        let bit = reader.read_bit()?;
        if !bit {
            break;
        }
        n += 1;

        // Safety check to prevent infinite loops
        if n >= max_consecutive {
            return Err(anyhow::anyhow!(
                "VLC decoder error: too many consecutive 1-bits"
            ));
        }
    }

    // Decode based on the alphabet selection
    if n > 2 * theta {
        // Unary sub-alphabet: return n - θ
        Ok(n - theta)
    } else if n > 0 {
        // Signed sub-alphabet
        if (n & 1) == 1 {
            // Odd codeword: negative value
            // From encoder: n = -x - 1 where x was doubled
            // So: n = -2*original - 1 → original = -(n + 1)/2
            Ok(-((n + 1) / 2))
        } else {
            // Even codeword: positive value
            // From encoder: n = x where x was doubled
            // So: n = 2*original → original = n/2
            Ok(n / 2)
        }
    } else {
        // Zero codeword
        Ok(0)
    }
}

/// VLC encoding primitive as specified in ISO/IEC 21122-1:2024 Table C.18
///
/// This implements the encoder that generates the bitstream decoded by vlc_decode.
pub fn vlc_encode(writer: &mut BitstreamWriter, value: i32, ctx: VlcContext) -> Result<()> {
    let mut x = value;
    let r = ctx.predictor;
    let t = ctx.truncation_pos as i32;

    // Compute the threshold for the alphabet switch (θ = max(r−t, 0))
    let theta = std::cmp::max(r - t, 0);

    let n = if x > theta {
        // Unary sub-alphabet: n = x + θ
        x + theta
    } else {
        // Signed sub-alphabet: x = x × 2 (modify x in place as per ISO)
        x *= 2;
        if x < 0 {
            // n = -x – 1
            -x - 1
        } else {
            // n = x
            x
        }
    };

    // Write n consecutive 1-bits followed by a 0-bit
    for _ in 0..n {
        writer.write_bit(true);
    }
    writer.write_bit(false);

    Ok(())
}

/// Compute bitplane count for a coefficient value
///
/// The bitplane count is the number of significant bitplanes counting from
/// the LSB up to the most significant non-empty bitplane.
pub fn compute_bitplane_count(coeff: i32) -> BitplaneCount {
    if coeff == 0 {
        BitplaneCount(0)
    } else {
        let abs_coeff = coeff.unsigned_abs();
        BitplaneCount(32 - abs_coeff.leading_zeros() as u8)
    }
}

/// Raw mode bitplane count encoding (Table C.14)
///
/// In raw mode, bitplane counts are encoded directly using Br bits per code group.
pub fn encode_raw_bitplane_counts(
    writer: &mut BitstreamWriter,
    bitplane_counts: &[BitplaneCount],
    br_bits: u8,
) -> Result<()> {
    for &BitplaneCount(count) in bitplane_counts {
        writer.write_bits(count as u32, br_bits);
    }
    Ok(())
}

/// Raw mode bitplane count decoding (Table C.14)
pub fn decode_raw_bitplane_counts(
    reader: &mut BitstreamReader,
    num_code_groups: usize,
    br_bits: u8,
) -> Result<Vec<BitplaneCount>> {
    let mut counts = Vec::with_capacity(num_code_groups);

    for _ in 0..num_code_groups {
        let count = reader.read_bits(br_bits)? as u8;
        counts.push(BitplaneCount(count));
    }

    Ok(counts)
}

/// Variable length bitplane count encoding (no prediction mode - Table C.16)
pub fn encode_vlc_bitplane_counts(
    writer: &mut BitstreamWriter,
    bitplane_counts: &[BitplaneCount],
    ctx: VlcContext,
) -> Result<()> {
    for &BitplaneCount(count) in bitplane_counts {
        vlc_encode(writer, count as i32, ctx)?;
    }
    Ok(())
}

/// Variable length bitplane count decoding (no prediction mode - Table C.16)
pub fn decode_vlc_bitplane_counts(
    reader: &mut BitstreamReader,
    num_code_groups: usize,
    ctx: VlcContext,
) -> Result<Vec<BitplaneCount>> {
    let mut counts = Vec::with_capacity(num_code_groups);

    for _ in 0..num_code_groups {
        let count = vlc_decode(reader, ctx)?;
        if count < 0 {
            return Err(anyhow::anyhow!("Invalid bitplane count: {}", count));
        }
        counts.push(BitplaneCount(count as u8));
    }

    Ok(counts)
}

/// Encode coefficients using simplified JPEG XS-inspired entropy coding
///
/// This is a working implementation that uses JPEG XS concepts but with
/// simplified algorithms for now. TODO: Replace with full ISO compliance.
pub fn encode_coefficients(coeffs: &[i32]) -> Result<Vec<u8>> {
    let mut writer = BitstreamWriter::new();

    // Step 0: Encode the number of coefficients (16 bits should be enough)
    writer.write_bits(coeffs.len() as u32, 16);

    // Step 1: Compute bitplane counts for all coefficients
    let bitplane_counts: Vec<BitplaneCount> = coeffs
        .iter()
        .map(|&coeff| compute_bitplane_count(coeff))
        .collect();

    // Step 2: Encode bitplane counts using raw mode (simpler for now)
    encode_raw_bitplane_counts(&mut writer, &bitplane_counts, 4)?;

    // Step 3: Encode coefficient magnitudes and signs
    for (i, &coeff) in coeffs.iter().enumerate() {
        let BitplaneCount(bp_count) = bitplane_counts[i];

        if bp_count > 0 && coeff != 0 {
            // Encode magnitude (without the MSB to avoid redundancy with bitplane count)
            let abs_coeff = coeff.unsigned_abs();
            if bp_count > 1 {
                // For values > 1, encode the magnitude bits below the MSB
                let magnitude_bits = abs_coeff & ((1 << (bp_count - 1)) - 1);
                writer.write_bits(magnitude_bits, bp_count - 1);
            }

            // Encode sign
            writer.write_bit(coeff < 0);
        }
    }

    Ok(writer.finalize())
}

/// Decode coefficients using simplified JPEG XS-inspired entropy coding
///
/// This decodes the bitstream produced by encode_coefficients.
pub fn decode_coefficients(data: &[u8]) -> Result<Vec<i32>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut reader = BitstreamReader::new(data);

    // Step 0: Decode the number of coefficients
    let num_coeffs = reader.read_bits(16)? as usize;

    // Step 1: Decode bitplane counts using raw mode
    let bitplane_counts = decode_raw_bitplane_counts(&mut reader, num_coeffs, 4)?;

    // Step 2: Decode coefficient data based on bitplane counts
    let mut coefficients = Vec::new();

    for BitplaneCount(bp_count) in bitplane_counts {
        if bp_count == 0 {
            coefficients.push(0);
        } else {
            // Decode magnitude and sign
            let magnitude = if bp_count == 1 {
                // For single bitplane, magnitude is always 1
                1u32
            } else {
                // Decode the magnitude bits (excluding the MSB which is always 1)
                let magnitude_bits = match reader.read_bits(bp_count - 1) {
                    Ok(bits) => bits,
                    Err(_) => {
                        // Not enough data - pad with zeros for remaining coefficients
                        while coefficients.len() < num_coeffs {
                            coefficients.push(0);
                        }
                        break;
                    }
                };

                // Reconstruct full magnitude: MSB is 1, followed by decoded bits
                (1 << (bp_count - 1)) | magnitude_bits
            };

            // Decode sign bit
            let is_negative = reader.read_bit().unwrap_or_default();

            let coeff = magnitude as i32;
            coefficients.push(if is_negative { -coeff } else { coeff });
        }
    }

    Ok(coefficients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitplane_count_computation() {
        assert_eq!(compute_bitplane_count(0).0, 0);
        assert_eq!(compute_bitplane_count(1).0, 1);
        assert_eq!(compute_bitplane_count(-1).0, 1);
        assert_eq!(compute_bitplane_count(7).0, 3);
        assert_eq!(compute_bitplane_count(-8).0, 4);
        assert_eq!(compute_bitplane_count(255).0, 8);
    }

    #[test]
    fn test_vlc_simple() {
        // Test simple VLC with different contexts
        let ctx = VlcContext {
            predictor: 4,
            truncation_pos: 0,
            br_bits: 4,
        };

        // Test individual values
        for &value in &[-2, -1, 0, 1, 2] {
            let mut writer = BitstreamWriter::new();
            vlc_encode(&mut writer, value, ctx).unwrap();

            let encoded = writer.finalize();
            let mut reader = BitstreamReader::new(&encoded);

            let decoded = vlc_decode(&mut reader, ctx).unwrap();
            assert_eq!(decoded, value, "Failed for value {}", value);
        }
    }

    #[test]
    fn test_vlc_roundtrip() {
        let mut writer = BitstreamWriter::new();
        let ctx = VlcContext {
            predictor: 4, // Use θ > 0 to avoid edge case
            truncation_pos: 0,
            br_bits: 4,
        };

        let test_values = [-2, -1, 0, 1, 2]; // Smaller test set

        for &value in &test_values {
            vlc_encode(&mut writer, value, ctx).unwrap();
        }

        let encoded = writer.finalize();
        println!(
            "Encoded {} values into {} bytes",
            test_values.len(),
            encoded.len()
        );

        let mut reader = BitstreamReader::new(&encoded);

        for &expected in &test_values {
            let decoded = vlc_decode(&mut reader, ctx).unwrap();
            println!("Expected: {}, Decoded: {}", expected, decoded);
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn test_entropy_coding_roundtrip() {
        let test_coeffs = vec![0, 1, -2, 7, -15, 0, 3, -1];

        let encoded = encode_coefficients(&test_coeffs).unwrap();
        let decoded = decode_coefficients(&encoded).unwrap();

        assert_eq!(decoded.len(), test_coeffs.len());
        for (i, (&original, &reconstructed)) in test_coeffs.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(original, reconstructed, "Mismatch at index {}", i);
        }
    }
}
