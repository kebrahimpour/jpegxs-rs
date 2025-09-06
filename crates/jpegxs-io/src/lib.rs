pub mod bitio;
pub mod bitstream;
pub mod yuv;

pub use bitio::{BitReader, BitWriter};
pub use yuv::{load_yuv422p, load_yuv444p, save_yuv422p, save_yuv444p};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_bitio_roundtrip() {
        let mut writer = BitWriter::new();
        
        // Write some test data
        writer.write_bits(0b1010, 4).expect("Write failed");
        writer.write_bits(0b110011, 6).expect("Write failed");
        writer.write_bits(0b11111111, 8).expect("Write failed");
        
        let data = writer.finish();
        assert!(!data.is_empty());
        
        // Read back the data
        let mut reader = BitReader::new(&data);
        
        assert_eq!(reader.read_bits(4).expect("Read failed"), 0b1010);
        assert_eq!(reader.read_bits(6).expect("Read failed"), 0b110011);
        assert_eq!(reader.read_bits(8).expect("Read failed"), 0b11111111);
    }
    
    #[test]
    fn test_yuv422p_io() -> anyhow::Result<()> {
        let width = 16;
        let height = 8;
        let y_size = width * height;
        let uv_size = y_size / 2;
        
        // Create test YUV data
        let y_plane: Vec<u8> = (0..y_size).map(|i| (i % 256) as u8).collect();
        let u_plane: Vec<u8> = (0..uv_size).map(|i| ((i * 2) % 256) as u8).collect();
        let v_plane: Vec<u8> = (0..uv_size).map(|i| ((i * 3) % 256) as u8).collect();
        
        // Create temporary file
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path();
        
        // Save YUV data
        save_yuv422p(path, &y_plane, &u_plane, &v_plane)?;
        
        // Load YUV data back
        let (loaded_y, loaded_u, loaded_v) = load_yuv422p(path, width, height)?;
        
        // Verify data integrity
        assert_eq!(y_plane, loaded_y);
        assert_eq!(u_plane, loaded_u);
        assert_eq!(v_plane, loaded_v);
        
        Ok(())
    }
}
