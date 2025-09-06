pub mod colors;
pub mod dwt;
pub mod entropy;
pub mod packet;
pub mod quant;
pub mod types;

use anyhow::Result;
use types::{Bitstream, DecoderConfig, EncoderConfig, ImageOwned8, ImageView8};

pub fn encode_frame(_input: ImageView8, _config: &EncoderConfig) -> Result<Bitstream> {
    todo!("Encoder implementation pending")
}

pub fn decode_frame(_bitstream: &Bitstream, _config: &DecoderConfig) -> Result<ImageOwned8> {
    todo!("Decoder implementation pending")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test - will be replaced with actual tests
    }
}
