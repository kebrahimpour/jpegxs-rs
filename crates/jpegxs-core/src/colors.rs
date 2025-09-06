use anyhow::Result;

pub fn rgb_to_yuv(_rgb: &[u8], _yuv: &mut [u8], _width: u32, _height: u32) -> Result<()> {
    todo!("RGB to YUV conversion")
}

pub fn yuv_to_rgb(_yuv: &[u8], _rgb: &mut [u8], _width: u32, _height: u32) -> Result<()> {
    todo!("YUV to RGB conversion")
}

pub fn downsample_444_to_422(
    _y: &[u8],
    _u: &[u8],
    _v: &[u8],
    _width: u32,
    _height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    todo!("444 to 422 downsampling")
}

pub fn upsample_422_to_444(
    _y: &[u8],
    _u: &[u8],
    _v: &[u8],
    _width: u32,
    _height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    todo!("422 to 444 upsampling")
}
