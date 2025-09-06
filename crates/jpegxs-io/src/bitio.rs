use anyhow::Result;

pub struct BitReader<'a> {
    _data: &'a [u8],
    position: usize,
    _bit_position: u8,
}

pub struct BitWriter {
    data: Vec<u8>,
    _bit_position: u8,
    _current_byte: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            _data: data,
            position: 0,
            _bit_position: 0,
        }
    }

    pub fn read_bits(&mut self, num_bits: u32) -> Result<u32> {
        if num_bits == 0 || num_bits > 32 {
            return Err(anyhow::anyhow!("Invalid number of bits: {}", num_bits));
        }
        
        let mut result = 0u32;
        let mut bits_read = 0u32;
        
        while bits_read < num_bits {
            if self.position >= self._data.len() {
                return Err(anyhow::anyhow!("End of data reached"));
            }
            
            let current_byte = self._data[self.position];
            let bits_available = 8 - self._bit_position;
            let bits_needed = num_bits - bits_read;
            let bits_to_read = bits_available.min(bits_needed as u8);
            
            let mask = (1u8 << bits_to_read) - 1;
            let shifted_byte = current_byte >> (bits_available - bits_to_read);
            let bits = shifted_byte & mask;
            
            result = (result << bits_to_read) | (bits as u32);
            bits_read += bits_to_read as u32;
            self._bit_position += bits_to_read;
            
            if self._bit_position >= 8 {
                self._bit_position = 0;
                self.position += 1;
            }
        }
        
        Ok(result)
    }

    pub fn bytes_read(&self) -> usize {
        self.position
    }
}

impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _bit_position: 0,
            _current_byte: 0,
        }
    }

    pub fn write_bits(&mut self, value: u32, num_bits: u32) -> Result<()> {
        if num_bits == 0 || num_bits > 32 {
            return Err(anyhow::anyhow!("Invalid number of bits: {}", num_bits));
        }
        
        let mut remaining_bits = num_bits;
        let mut remaining_value = value;
        
        while remaining_bits > 0 {
            let bits_to_write = (8 - self._bit_position).min(remaining_bits as u8);
            let shift = remaining_bits - bits_to_write as u32;
            let bits = (remaining_value >> shift) as u8;
            let mask = (1u8 << bits_to_write) - 1;
            
            self._current_byte |= (bits & mask) << (8 - self._bit_position - bits_to_write);
            self._bit_position += bits_to_write;
            remaining_bits -= bits_to_write as u32;
            remaining_value &= (1u32 << shift) - 1;
            
            if self._bit_position >= 8 {
                self.data.push(self._current_byte);
                self._current_byte = 0;
                self._bit_position = 0;
            }
        }
        
        Ok(())
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self._bit_position > 0 {
            self.data.push(self._current_byte);
        }
        self.data
    }
}
