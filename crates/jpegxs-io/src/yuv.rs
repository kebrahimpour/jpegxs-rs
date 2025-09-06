use anyhow::Result;
use std::path::Path;

pub fn load_yuv422p<P: AsRef<Path>>(
    _path: P,
    _width: u32,
    _height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    todo!("Load YUV 4:2:2 planar")
}

pub fn save_yuv422p<P: AsRef<Path>>(_path: P, _y: &[u8], _u: &[u8], _v: &[u8]) -> Result<()> {
    todo!("Save YUV 4:2:2 planar")
}

pub fn load_yuv444p<P: AsRef<Path>>(
    _path: P,
    _width: u32,
    _height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    todo!("Load YUV 4:4:4 planar")
}

pub fn save_yuv444p<P: AsRef<Path>>(_path: P, _y: &[u8], _u: &[u8], _v: &[u8]) -> Result<()> {
    todo!("Save YUV 4:4:4 planar")
}
