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
}
