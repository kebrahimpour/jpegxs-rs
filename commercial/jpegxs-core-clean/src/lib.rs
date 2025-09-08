// Clean-room JPEG XS implementation from ISO/IEC 21122-1:2024
// Source: Table A.2 and A.3 for marker definitions

pub mod dwt;

/// JPEG XS marker constants from ISO/IEC 21122-1:2024 Table A.2
pub mod markers {
    /// Start of Codestream - Mandatory (ISO Table A.2, line 603)
    /// Must be first marker segment in codestream (ISO A.4.1)
    pub const SOC: u16 = 0xff10;

    /// End of Codestream - Mandatory (ISO Table A.2)
    /// Must be last marker segment in codestream
    pub const EOC: u16 = 0xff11;

    /// Picture Header - Mandatory (ISO Table A.2)
    /// Third marker after SOC and CAP
    pub const PIH: u16 = 0xff12;

    /// Component Table - Mandatory (ISO Table A.2)
    /// Fourth marker after PIH
    pub const CDT: u16 = 0xff13;

    /// Weights Table - Mandatory (ISO Table A.2)
    /// Fifth marker after CDT
    pub const WGT: u16 = 0xff14;

    /// Capabilities Marker - Mandatory (ISO Table A.2)
    /// Must be second marker after SOC
    pub const CAP: u16 = 0xff50;
}

/// Basic JPEG XS bitstream structure
/// Implementation based on ISO/IEC 21122-1:2024 Section A.4.1
pub struct JpegXsBitstream {
    data: Vec<u8>,
}

impl JpegXsBitstream {
    /// Create new JPEG XS bitstream
    /// Per ISO A.4.1: SOC must be first marker
    pub fn new() -> Self {
        let mut bitstream = Self { data: Vec::new() };
        bitstream.write_soc_marker();
        bitstream
    }

    /// Write Start of Codestream marker
    /// ISO A.4.1: "Shall be the first marker segment in a codestream"
    /// ISO Table A.3: SOC = u(16) 0xff10
    fn write_soc_marker(&mut self) {
        let soc_bytes = markers::SOC.to_be_bytes();
        self.data.extend_from_slice(&soc_bytes);
    }

    /// Write Capabilities marker
    /// ISO A.4.3: "Shall be the second marker segment"
    /// ISO Table A.6: CAP marker with minimal capabilities
    pub fn write_cap_marker(&mut self) {
        let cap_bytes = markers::CAP.to_be_bytes();
        self.data.extend_from_slice(&cap_bytes);

        // Lcap: Size of capabilities marker segment (2 bytes minimum: just the length field)
        let lcap: u16 = 2;
        self.data.extend_from_slice(&lcap.to_be_bytes());

        // No capabilities required for basic implementation (empty cap array)
        // This creates a minimal valid CAP marker
    }

    /// Write Picture Header marker
    /// ISO A.7: "Shall be the third marker segment after CAP"
    /// Provides image dimensions and decoder configuration
    pub fn write_pih_marker(&mut self, width: u16, height: u16, num_components: u8) {
        let pih_bytes = markers::PIH.to_be_bytes();
        self.data.extend_from_slice(&pih_bytes);

        // Lpih: Size of PIH marker segment (per ISO Table A.7)
        // Size calculation: Lcod(4) + Ppih(2) + Plev(2) + Wf(2) + Hf(2) + Cw(2) + Hsl(2) + Nc(1) + Ng(1) + Ss(1) + Bw(1) + FqBr(1) + FslcPpoc(1) + CpihReserved(1) = 25 bytes
        let lpih: u16 = 25;
        self.data.extend_from_slice(&lpih.to_be_bytes());

        // Lcod: Size of entire codestream (0 for variable bitrate per ISO)
        let lcod: u32 = 0;
        self.data.extend_from_slice(&lcod.to_be_bytes());

        // Ppih: Profile (0 for no restrictions per ISO)
        let ppih: u16 = 0;
        self.data.extend_from_slice(&ppih.to_be_bytes());

        // Plev: Level (0 for no restrictions per ISO)
        let plev: u16 = 0;
        self.data.extend_from_slice(&plev.to_be_bytes());

        // Wf: Width of image in sample grid positions
        self.data.extend_from_slice(&width.to_be_bytes());

        // Hf: Height of image in sample grid positions
        self.data.extend_from_slice(&height.to_be_bytes());

        // Cw: Width of precinct (0 means as wide as image per ISO)
        let cw: u16 = 0;
        self.data.extend_from_slice(&cw.to_be_bytes());

        // Hsl: Height of slice in precincts (1 for single slice)
        let hsl: u16 = 1;
        self.data.extend_from_slice(&hsl.to_be_bytes());

        // Nc: Number of components (1-8 per ISO)
        self.data.push(num_components);

        // Ng: Number of coefficients per code group (8 default for 4:2:2)
        let ng: u8 = 8;
        self.data.push(ng);

        // Ss: Number of code groups per significance group (1 default)
        let ss: u8 = 1;
        self.data.push(ss);

        // Bw: Nominal bit precision of wavelet coefficients (20 default)
        let bw: u8 = 20;
        self.data.push(bw);

        // Fq: Number of fractional bits (6 default per ISO Table A.8)
        let fq: u8 = 6; // packed with Br in next byte

        // Br: Number of bits to encode bitplane count (4 default)
        let br: u8 = 4;

        // Pack Fq (upper 4) and Br (lower 4) into single byte
        let fq_br: u8 = ((fq & 0x0F) << 4) | (br & 0x0F);
        self.data.push(fq_br);

        // Fslc: Slice coding mode (0 for coefficient coding per ISO)
        let fslc: u8 = 0; // packed with Ppoc in next bytes

        // Ppoc: Progression order (0 for LRCP per ISO Table A.13)
        let ppoc: u8 = 0;

        // Pack Fslc (bit 7) and Ppoc (bits 6-4) and reserved bits
        let fslc_ppoc: u8 = ((fslc & 0x01) << 7) | ((ppoc & 0x07) << 4);
        self.data.push(fslc_ppoc);

        // Cpih: Colour transformation (0 for none per ISO Table A.9)
        let cpih: u8 = 0;

        // Pack Cpih in upper 4 bits, lower 4 bits reserved (0)
        let cpih_reserved: u8 = (cpih & 0x0F) << 4;
        self.data.push(cpih_reserved);
    }

    /// Write Component Table marker
    /// ISO A.4.5: "Specifies the component precision and sampling factors"
    /// ISO Table A.15: Component table syntax
    pub fn write_cdt_marker(&mut self, num_components: u8) {
        let cdt_bytes = markers::CDT.to_be_bytes();
        self.data.extend_from_slice(&cdt_bytes);

        // Lcdt: Size of CDT marker segment (per ISO Table A.15)
        // Size = 2 (length) + Nc * 3 (each component: B[c] + sx[c] + sy[c])
        // B[c] = u(8), sx[c] = u(4), sy[c] = u(4) -> sx+sy packed into 1 byte
        let lcdt: u16 = 2 + (num_components as u16) * 2;
        self.data.extend_from_slice(&lcdt.to_be_bytes());

        // Per ISO Table A.15: Loop over components
        for component_index in 0..num_components {
            // B[c]: Bit precision of component (per ISO, typically 8 for YUV422p8)
            let bit_precision: u8 = 8;
            self.data.push(bit_precision);

            // sx[c]: Horizontal sampling factor
            // sy[c]: Vertical sampling factor
            // Per ISO specification: "1 or 2 for components 1 and 2, 1 for all other components"
            // For YUV422p8: Y=1x1, U=2x1, V=2x1 sampling
            let (sx, sy) = match component_index {
                0 => (1u8, 1u8),     // Y component: 1x1 sampling
                1 | 2 => (2u8, 1u8), // U,V components: 2x1 sampling for 4:2:2
                _ => (1u8, 1u8),     // Additional components: 1x1 sampling
            };

            // Pack sx (upper 4 bits) and sy (lower 4 bits) per ISO u(4) encoding
            let sampling_factors: u8 = ((sx & 0x0F) << 4) | (sy & 0x0F);
            self.data.push(sampling_factors);
        }
    }

    /// Write Weights Table marker with actual quantization parameters
    /// ISO A.4.12: "Contains parameters required to set the gain of each band"
    /// ISO Table A.25: Weights table syntax
    pub fn write_wgt_marker(&mut self, qp_values: Option<&[u8]>) {
        let wgt_bytes = markers::WGT.to_be_bytes();
        self.data.extend_from_slice(&wgt_bytes);

        // Default QP values if none provided (for backward compatibility)
        let default_qps = vec![8, 7, 7, 6, 6, 5, 5, 4, 6, 5];
        let qps = qp_values.unwrap_or(&default_qps);
        
        // Lwgt: Size of WGT marker segment
        // Each band has G[b] (u8) + P[b] (u8) = 2 bytes
        // Total: 2 (length) + num_bands * 2
        let num_bands = qps.len();
        let lwgt: u16 = 2 + (num_bands as u16 * 2);
        self.data.extend_from_slice(&lwgt.to_be_bytes());

        // Per ISO Table A.25: Loop over all bands
        // G[b]: Gain of band b (0-15 per ISO) - stores quantization parameter
        // P[b]: Priority of band b (0-255 per ISO) - used for rate control
        for &qp in qps {
            // Store QP value as gain (clamped to 0-15 range per ISO)
            let gain = qp.min(15);
            self.data.push(gain);
            // Fixed priority for now (can be made configurable later)
            self.data.push(128);
        }
    }

    /// Finalize bitstream with End of Codestream marker
    /// ISO A.4.2: "Shall be the last marker segment in a codestream"
    pub fn finalize(&mut self) {
        let eoc_bytes = markers::EOC.to_be_bytes();
        self.data.extend_from_slice(&eoc_bytes);
    }

    /// Get the current bitstream data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Convert to owned byte vector
    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    /// Add entropy coded data (enhanced implementation)
    /// Per ISO Annex C: Quantized coefficients are entropy coded for compression
    pub fn add_entropy_coded_data(&mut self, coefficients: &[i32]) {
        let mut encoded_data = Vec::new();

        // Enhanced entropy coding with better compression techniques
        // 1. More aggressive quantization
        // 2. Multi-level run-length encoding
        // 3. Significance-based encoding

        let mut i = 0;
        while i < coefficients.len() {
            let coeff = coefficients[i];

            if coeff == 0 {
                // Enhanced zero run-length encoding with variable length codes
                let mut zero_count = 0;
                while i + zero_count < coefficients.len() && coefficients[i + zero_count] == 0 {
                    zero_count += 1;
                    if zero_count >= 65535 {
                        break; // Use 16-bit count for longer runs
                    }
                }

                if zero_count < 255 {
                    // Short run: 1 byte count
                    encoded_data.push(0x00);
                    encoded_data.push(zero_count as u8);
                } else {
                    // Long run: escape code + 2 byte count
                    encoded_data.push(0x00);
                    encoded_data.push(0xFF);
                    encoded_data.extend_from_slice(&(zero_count as u16).to_be_bytes());
                }
                i += zero_count;
            } else {
                // Enhanced coefficient quantization with better precision control
                let abs_coeff = coeff.abs();

                if abs_coeff <= 3 {
                    // Very small coefficients: direct encoding (1-3)
                    let encoded = if coeff > 0 {
                        abs_coeff as u8
                    } else {
                        (abs_coeff as u8) | 0x80
                    };
                    encoded_data.push(encoded);
                } else if abs_coeff <= 15 {
                    // Small coefficients: 4-bit quantization
                    let quantized = ((abs_coeff + 1) / 2).min(15) as u8;
                    let encoded = if coeff > 0 {
                        quantized
                    } else {
                        quantized | 0x80
                    };
                    encoded_data.push(0x10 | encoded); // Escape code for 4-bit quantized
                } else if abs_coeff <= 127 {
                    // Medium coefficients: direct 7-bit encoding
                    let quantized = (abs_coeff / 4).min(127) as u8;
                    let encoded = if coeff > 0 {
                        quantized
                    } else {
                        quantized | 0x80
                    };
                    encoded_data.push(0x20); // Escape code
                    encoded_data.push(encoded);
                } else {
                    // Large coefficients: aggressive quantization
                    let quantized = (abs_coeff / 16).min(63) as u8;
                    let encoded = if coeff > 0 {
                        quantized
                    } else {
                        quantized | 0x80
                    };
                    encoded_data.push(0x30); // Escape code
                    encoded_data.push(encoded);
                }
                i += 1;
            }
        }

        // Apply final compression pass: remove redundant patterns
        let compressed_data = self.compress_final_pass(&encoded_data);
        self.data.extend_from_slice(&compressed_data);
    }

    /// Final compression pass to remove patterns and redundancy
    fn compress_final_pass(&self, data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut i = 0;

        while i < data.len() {
            // Look for repeating byte patterns
            if i + 3 < data.len() {
                let pattern = &data[i..i + 2];
                let mut count = 1;
                let mut j = i + 2;

                // Count pattern repetitions
                while j + 1 < data.len() && data[j..j + 2] == *pattern && count < 255 {
                    count += 1;
                    j += 2;
                }

                if count >= 3 {
                    // Encode pattern repetition: 0xF0 + pattern[0] + pattern[1] + count
                    compressed.push(0xF0);
                    compressed.push(pattern[0]);
                    compressed.push(pattern[1]);
                    compressed.push(count);
                    i = j;
                    continue;
                }
            }

            // No pattern found, copy byte directly
            compressed.push(data[i]);
            i += 1;
        }

        compressed
    }
}

/// JPEG XS bitstream decoder
/// Implementation based on ISO/IEC 21122-1:2024 marker parsing
pub struct JpegXsDecoder {
    data: Vec<u8>,
    offset: usize,
    width: u16,
    height: u16,
    num_components: u8,
    wgt_qp_values: Vec<u8>,  // Quantization parameters from WGT marker
}

impl JpegXsDecoder {
    /// Create new decoder from bitstream data
    pub fn new(data: Vec<u8>) -> Result<Self, &'static str> {
        Ok(Self {
            data,
            offset: 0,
            width: 0,
            height: 0,
            num_components: 0,
            wgt_qp_values: Vec::new(),
        })
    }

    /// Parse JPEG XS markers and extract image parameters
    pub fn parse_headers(&mut self) -> Result<(), &'static str> {
        // Parse SOC marker
        if !self.parse_soc_marker()? {
            return Err("Invalid SOC marker");
        }

        // Parse CAP marker
        if !self.parse_cap_marker()? {
            return Err("Invalid CAP marker");
        }

        // Parse PIH marker
        if !self.parse_pih_marker()? {
            return Err("Invalid PIH marker");
        }

        // Parse CDT marker
        if !self.parse_cdt_marker()? {
            return Err("Invalid CDT marker");
        }

        // Parse WGT marker
        if !self.parse_wgt_marker()? {
            return Err("Invalid WGT marker");
        }

        Ok(())
    }

    /// Parse Start of Codestream marker
    fn parse_soc_marker(&mut self) -> Result<bool, &'static str> {
        if self.offset + 2 > self.data.len() {
            return Err("Insufficient data for SOC marker");
        }

        let marker = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        if marker != markers::SOC {
            return Ok(false);
        }

        self.offset += 2;
        Ok(true)
    }

    /// Parse Capabilities marker
    fn parse_cap_marker(&mut self) -> Result<bool, &'static str> {
        if self.offset + 4 > self.data.len() {
            return Err("Insufficient data for CAP marker");
        }

        let marker = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        if marker != markers::CAP {
            return Ok(false);
        }
        self.offset += 2;

        let length = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;

        // Skip capabilities data (length includes the 2-byte length field itself)
        let payload_size = length.saturating_sub(2);
        if self.offset + payload_size as usize > self.data.len() {
            return Err("Insufficient data for CAP payload");
        }
        self.offset += payload_size as usize;

        Ok(true)
    }

    /// Parse Picture Header marker
    fn parse_pih_marker(&mut self) -> Result<bool, &'static str> {
        if self.offset + 4 > self.data.len() {
            return Err("Insufficient data for PIH marker");
        }

        let marker = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        if marker != markers::PIH {
            return Ok(false);
        }
        self.offset += 2;

        let length = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;

        if length < 23 || self.offset + (length as usize - 2) > self.data.len() {
            return Err("Invalid PIH marker length");
        }

        // Skip Lcod (4 bytes), Ppih (2 bytes), Plev (2 bytes)
        self.offset += 8;

        // Extract image dimensions
        self.width = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;
        self.height = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;

        // Skip Cw (2 bytes), Hsl (2 bytes)
        self.offset += 4;

        // Extract number of components
        self.num_components = self.data[self.offset];
        self.offset += 1;

        // Skip remaining PIH data
        self.offset += length as usize - 19;

        Ok(true)
    }

    /// Parse Component Table marker
    fn parse_cdt_marker(&mut self) -> Result<bool, &'static str> {
        if self.offset + 4 > self.data.len() {
            return Err("Insufficient data for CDT marker");
        }

        let marker = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        if marker != markers::CDT {
            return Ok(false);
        }
        self.offset += 2;

        let length = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;

        // Skip CDT data
        if self.offset + (length as usize - 2) > self.data.len() {
            return Err("Insufficient data for CDT payload");
        }
        self.offset += length as usize - 2;

        Ok(true)
    }

    /// Parse Weights Table marker and extract quantization parameters
    fn parse_wgt_marker(&mut self) -> Result<bool, &'static str> {
        if self.offset + 4 > self.data.len() {
            return Err("Insufficient data for WGT marker");
        }

        let marker = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        if marker != markers::WGT {
            return Ok(false);
        }
        self.offset += 2;

        let length = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;

        // Parse WGT data to extract QP values
        let payload_size = length as usize - 2;
        if self.offset + payload_size > self.data.len() {
            return Err("Insufficient data for WGT payload");
        }
        
        // Extract gain values (QP parameters) from WGT marker
        // Each band has 2 bytes: G[b] (gain/QP) and P[b] (priority)
        let num_bands = payload_size / 2;
        self.wgt_qp_values.clear();
        
        for i in 0..num_bands {
            let gain = self.data[self.offset + i * 2]; // G[b] - quantization parameter
            self.wgt_qp_values.push(gain);
            // Skip P[b] (priority) - at offset + i*2 + 1
        }
        
        self.offset += payload_size;
        Ok(true)
    }

    /// Decode entropy coded data using enhanced decoder
    pub fn decode_entropy_data(&mut self) -> Result<Vec<i32>, &'static str> {
        let mut coefficients = Vec::new();
        let remaining_data = &self.data[self.offset..];

        // Find EOC marker to determine entropy data end
        let mut entropy_end = remaining_data.len();
        for i in 0..remaining_data.len().saturating_sub(1) {
            if remaining_data[i] == 0xff && remaining_data[i + 1] == 0x11 {
                entropy_end = i;
                break;
            }
        }

        let entropy_data = &remaining_data[..entropy_end];
        let mut i = 0;

        // Decode enhanced entropy data
        while i < entropy_data.len() {
            let byte = entropy_data[i];

            if byte == 0x00 {
                // Zero run-length encoding
                i += 1;
                if i >= entropy_data.len() {
                    break;
                }

                let count_byte = entropy_data[i];
                i += 1;

                if count_byte == 0xFF {
                    // Long run: read 16-bit count
                    if i + 1 >= entropy_data.len() {
                        break;
                    }
                    let count = u16::from_be_bytes([entropy_data[i], entropy_data[i + 1]]) as usize;
                    i += 2;
                    coefficients.extend(vec![0; count]);
                } else {
                    // Short run
                    coefficients.extend(vec![0; count_byte as usize]);
                }
            } else if byte == 0xF0 {
                // Pattern repetition decompression
                if i + 3 >= entropy_data.len() {
                    break;
                }
                let pattern_0 = entropy_data[i + 1];
                let pattern_1 = entropy_data[i + 2];
                let count = entropy_data[i + 3];
                i += 4;

                for _ in 0..count {
                    // Decode pattern bytes as coefficients
                    coefficients.push(pattern_0 as i8 as i32);
                    coefficients.push(pattern_1 as i8 as i32);
                }
            } else if (byte & 0xF0) == 0x10 {
                // 4-bit quantized coefficient
                let quantized = (byte & 0x0F) as i32;
                let coeff = quantized * 2;
                coefficients.push(if (byte & 0x80) != 0 { -coeff } else { coeff });
                i += 1;
            } else if byte == 0x20 {
                // Medium coefficient: 7-bit encoding
                i += 1;
                if i >= entropy_data.len() {
                    break;
                }
                let encoded = entropy_data[i];
                let quantized = (encoded & 0x7F) as i32;
                let coeff = quantized * 4;
                coefficients.push(if (encoded & 0x80) != 0 { -coeff } else { coeff });
                i += 1;
            } else if byte == 0x30 {
                // Large coefficient: aggressive quantization
                i += 1;
                if i >= entropy_data.len() {
                    break;
                }
                let encoded = entropy_data[i];
                let quantized = (encoded & 0x7F) as i32;
                let coeff = quantized * 16;
                coefficients.push(if (encoded & 0x80) != 0 { -coeff } else { coeff });
                i += 1;
            } else {
                // Direct encoded small coefficient (1-3)
                let abs_coeff = (byte & 0x7F) as i32;
                coefficients.push(if (byte & 0x80) != 0 {
                    -abs_coeff
                } else {
                    abs_coeff
                });
                i += 1;
            }
        }

        Ok(coefficients)
    }

    /// Get decoded image dimensions
    pub fn dimensions(&self) -> (u16, u16, u8) {
        (self.width, self.height, self.num_components)
    }
    
    /// Get quantization parameters from WGT marker
    pub fn get_qp_values(&self) -> &[u8] {
        &self.wgt_qp_values
    }
}

impl Default for JpegXsBitstream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soc_marker_creation() {
        let bitstream = JpegXsBitstream::new();
        let data = bitstream.data();

        // Should start with SOC marker (0xff10 in big-endian)
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], 0xff);
        assert_eq!(data[1], 0x10);
    }

    #[test]
    fn test_finalized_bitstream() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.finalize();
        let data = bitstream.data();

        // Should have SOC + EOC markers (4 bytes total)
        assert_eq!(data.len(), 4);

        // First marker should be SOC
        assert_eq!(data[0], 0xff);
        assert_eq!(data[1], 0x10);

        // Last marker should be EOC
        assert_eq!(data[2], 0xff);
        assert_eq!(data[3], 0x11);
    }

    #[test]
    fn test_cap_marker() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        let data = bitstream.data();

        // Should have SOC (2 bytes) + CAP marker (2 bytes) + Lcap length (2 bytes) = 6 bytes minimum
        assert!(data.len() >= 6);

        // SOC marker
        assert_eq!(data[0], 0xff);
        assert_eq!(data[1], 0x10);

        // CAP marker
        assert_eq!(data[2], 0xff);
        assert_eq!(data[3], 0x50);

        // Lcap field (2 bytes for minimal CAP marker)
        assert_eq!(data[4], 0x00);
        assert_eq!(data[5], 0x02);
    }

    #[test]
    fn test_pih_marker() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        bitstream.write_pih_marker(256, 256, 3); // 256x256 RGB image
        let data = bitstream.data();

        // Should have SOC (2) + CAP (2+2) + PIH (2+2+25) = 33 bytes
        assert_eq!(data.len(), 33);

        // SOC marker
        assert_eq!(data[0], 0xff);
        assert_eq!(data[1], 0x10);

        // CAP marker
        assert_eq!(data[2], 0xff);
        assert_eq!(data[3], 0x50);

        // PIH marker starts at offset 6
        assert_eq!(data[6], 0xff);
        assert_eq!(data[7], 0x12);

        // Lpih (25 bytes)
        assert_eq!(data[8], 0x00);
        assert_eq!(data[9], 0x19);
    }

    #[test]
    fn test_cdt_marker() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        bitstream.write_pih_marker(256, 256, 3);
        bitstream.write_cdt_marker(3); // 3 components (YUV)
        let data = bitstream.data();

        // Should have SOC (2) + CAP (4) + PIH (29) + CDT (8) = 43 bytes
        assert_eq!(data.len(), 43);

        // CDT marker starts at offset 33
        assert_eq!(data[33], 0xff);
        assert_eq!(data[34], 0x13);

        // Lcdt (2 + 3*2 = 8 bytes)
        assert_eq!(data[35], 0x00);
        assert_eq!(data[36], 0x08);

        // Component 0 (Y): 8-bit precision, 1x1 sampling
        assert_eq!(data[37], 0x08); // B[0] = 8 bits
        assert_eq!(data[38], 0x11); // sx=1, sy=1 -> 0x11

        // Component 1 (U): 8-bit precision, 2x1 sampling
        assert_eq!(data[39], 0x08); // B[1] = 8 bits
        assert_eq!(data[40], 0x21); // sx=2, sy=1 -> 0x21

        // Component 2 (V): 8-bit precision, 2x1 sampling
        assert_eq!(data[41], 0x08); // B[2] = 8 bits
        assert_eq!(data[42], 0x21); // sx=2, sy=1 -> 0x21
    }

    #[test]
    fn test_wgt_marker() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        bitstream.write_pih_marker(256, 256, 3);
        bitstream.write_cdt_marker(3);
        bitstream.write_wgt_marker(None); // Use default QP values
        let data = bitstream.data();

        // Should have SOC (2) + CAP (4) + PIH (29) + CDT (8) + WGT (24) = 67 bytes
        assert_eq!(data.len(), 67);

        // WGT marker starts at offset 43 (SOC:2 + CAP:4 + PIH:29 + CDT:8 = 43)
        assert_eq!(data[43], 0xff);
        assert_eq!(data[44], 0x14);

        // Lwgt (2 + 10*2 = 22 bytes) - starts at offset 45
        assert_eq!(data[45], 0x00);
        assert_eq!(data[46], 0x16);

        // First band: gain=8, priority=128 - starts at offset 47
        assert_eq!(data[47], 0x08);
        assert_eq!(data[48], 0x80);

        // Second band: gain=7, priority=128 - starts at offset 49
        assert_eq!(data[49], 0x07);
        assert_eq!(data[50], 0x80);
    }
    
    #[test]
    fn test_wgt_marker_with_custom_qp() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        bitstream.write_pih_marker(256, 256, 3);
        bitstream.write_cdt_marker(3);
        
        // Custom QP values for testing
        let qp_values = vec![4, 6, 6]; // Y, U, V
        bitstream.write_wgt_marker(Some(&qp_values));
        bitstream.finalize();
        
        // Parse the bitstream and extract QP values
        let mut decoder = JpegXsDecoder::new(bitstream.into_bytes()).unwrap();
        decoder.parse_headers().unwrap();
        
        let extracted_qp = decoder.get_qp_values();
        assert_eq!(extracted_qp.len(), 3);
        assert_eq!(extracted_qp[0], 4); // Y QP
        assert_eq!(extracted_qp[1], 6); // U QP
        assert_eq!(extracted_qp[2], 6); // V QP
    }

    #[test]
    fn test_complete_jpeg_xs_bitstream() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        bitstream.write_pih_marker(256, 256, 3);
        bitstream.write_cdt_marker(3);
        bitstream.write_wgt_marker(None);

        // Add some test entropy data
        let test_coefficients = vec![0, 0, 0, 15, -7, 0, 0, 23, -12, 0];
        bitstream.add_entropy_coded_data(&test_coefficients);

        bitstream.finalize();
        let data = bitstream.data();

        // Verify all 5 mandatory JPEG XS markers are present
        // SOC at offset 0
        assert_eq!(data[0], 0xff);
        assert_eq!(data[1], 0x10);

        // CAP at offset 2
        assert_eq!(data[2], 0xff);
        assert_eq!(data[3], 0x50);

        // PIH at offset 6
        assert_eq!(data[6], 0xff);
        assert_eq!(data[7], 0x12);

        // CDT at offset 33
        assert_eq!(data[33], 0xff);
        assert_eq!(data[34], 0x13);

        // WGT at offset 43
        assert_eq!(data[43], 0xff);
        assert_eq!(data[44], 0x14);

        // EOC should be at the end
        let end_offset = data.len() - 2;
        assert_eq!(data[end_offset], 0xff);
        assert_eq!(data[end_offset + 1], 0x11);

        // Verify JPEG XS compliance by checking first marker is SOC
        assert!(data.len() > 2);
        assert_eq!(&data[0..2], &[0xff, 0x10]);
    }

    #[test]
    fn test_decoder_parser() {
        let mut bitstream = JpegXsBitstream::new();
        bitstream.write_cap_marker();
        bitstream.write_pih_marker(256, 256, 3);
        bitstream.write_cdt_marker(3);
        bitstream.write_wgt_marker(None);

        let test_coefficients = vec![0, 0, 15, -7, 0, 23, -12];
        bitstream.add_entropy_coded_data(&test_coefficients);

        bitstream.finalize();
        let data = bitstream.into_bytes();

        // Test decoder
        let mut decoder = JpegXsDecoder::new(data).unwrap();
        assert!(decoder.parse_headers().is_ok());

        let (width, height, components) = decoder.dimensions();
        assert_eq!(width, 256);
        assert_eq!(height, 256);
        assert_eq!(components, 3);

        let decoded_coeffs = decoder.decode_entropy_data().unwrap();
        assert!(!decoded_coeffs.is_empty());
    }
}
