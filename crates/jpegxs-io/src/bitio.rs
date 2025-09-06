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

    pub fn read_bits(&mut self, _num_bits: u32) -> Result<u32> {
        todo!("Read bits implementation")
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

    pub fn write_bits(&mut self, _value: u32, _num_bits: u32) -> Result<()> {
        todo!("Write bits implementation")
    }

    pub fn finish(self) -> Vec<u8> {
        self.data
    }
}
