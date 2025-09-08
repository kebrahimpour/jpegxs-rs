use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ImageView8<'a> {
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
}

#[derive(Debug, Clone)]
pub struct ImageOwned8 {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PixelFormat {
    Yuv422p8,
    Yuv444p8,
    Rgb8,
    Bgr8,
    Rgb8Planar,
    Yuv420p8,
}

#[derive(Debug, Clone)]
pub struct Bitstream {
    pub data: Vec<u8>,
    pub size_bits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncoderConfig {
    pub quality: f32,
    pub profile: Profile,
    pub level: Level,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecoderConfig {
    #[serde(default)]
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Profile {
    Main,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Level {
    Level1,
    Level2,
    Level3,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            quality: 0.9,
            profile: Profile::Main,
            level: Level::Level1,
        }
    }
}
