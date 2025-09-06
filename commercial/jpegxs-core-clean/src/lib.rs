// Clean-room JPEG XS implementation from ISO/IEC 21122-1:2024
// Source: Table A.2 and A.3 for marker definitions

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

        // Lcap: Size of capabilities marker (4 bytes minimum: 2 for length + 0 capabilities)
        let lcap: u16 = 4;
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

    /// Add entropy coded data (basic implementation)
    /// Per ISO Annex C: Quantized coefficients are entropy coded for compression
    pub fn add_entropy_coded_data(&mut self, coefficients: &[i32]) {
        // Basic entropy coding: use simple run-length encoding for zero coefficients
        // and variable-length encoding for non-zero values
        let mut encoded_data = Vec::new();

        let mut i = 0;
        while i < coefficients.len() {
            let coeff = coefficients[i];

            if coeff == 0 {
                // Count consecutive zeros for run-length encoding
                let mut zero_count = 0;
                while i + zero_count < coefficients.len() && coefficients[i + zero_count] == 0 {
                    zero_count += 1;
                    if zero_count >= 255 {
                        // Limit run length to 255
                        break;
                    }
                }

                // Encode zero run: 0x00 followed by count
                encoded_data.push(0x00);
                encoded_data.push(zero_count as u8);
                i += zero_count;
            } else {
                // Quantize non-zero coefficient to 8-bit signed value
                let quantized = coeff.clamp(-127, 127) as i8;
                if quantized == 0 {
                    // Handle edge case where clamp results in zero
                    encoded_data.push(0x01); // Encode as minimal non-zero
                } else {
                    encoded_data.push(quantized as u8);
                }
                i += 1;
            }
        }

        // Add encoded data to bitstream
        self.data.extend_from_slice(&encoded_data);
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

        // Lcap field (4 bytes for minimal CAP marker)
        assert_eq!(data[4], 0x00);
        assert_eq!(data[5], 0x04);
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
}
