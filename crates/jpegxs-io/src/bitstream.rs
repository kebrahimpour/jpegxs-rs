use anyhow::Result;

#[derive(Debug, Clone)]
pub struct BitstreamContainer {
    pub data: Vec<u8>,
    pub metadata: BitstreamMetadata,
}

#[derive(Debug, Clone)]
pub struct BitstreamMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub bitrate: Option<u32>,
}

impl BitstreamContainer {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            metadata: BitstreamMetadata {
                width,
                height,
                format: "jpegxs".to_string(),
                bitrate: None,
            },
        }
    }

    pub fn from_file(_path: &str) -> Result<Self> {
        todo!("Load bitstream from file")
    }

    pub fn to_file(&self, _path: &str) -> Result<()> {
        todo!("Save bitstream to file")
    }
}
