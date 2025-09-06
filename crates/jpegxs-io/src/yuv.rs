use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn load_yuv422p<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let file = File::open(&path)
        .with_context(|| format!("Failed to open YUV file: {:?}", path.as_ref()))?;
    let mut reader = BufReader::new(file);

    let y_size = (width * height) as usize;
    let uv_size = (width * height / 2) as usize; // 4:2:2 subsampling

    let mut y_plane = vec![0u8; y_size];
    let mut u_plane = vec![0u8; uv_size];
    let mut v_plane = vec![0u8; uv_size];

    reader
        .read_exact(&mut y_plane)
        .context("Failed to read Y plane")?;
    reader
        .read_exact(&mut u_plane)
        .context("Failed to read U plane")?;
    reader
        .read_exact(&mut v_plane)
        .context("Failed to read V plane")?;

    Ok((y_plane, u_plane, v_plane))
}

pub fn save_yuv422p<P: AsRef<Path>>(path: P, y: &[u8], u: &[u8], v: &[u8]) -> Result<()> {
    use std::io::Write;

    let mut file = File::create(&path)
        .with_context(|| format!("Failed to create YUV file: {:?}", path.as_ref()))?;

    file.write_all(y).context("Failed to write Y plane")?;
    file.write_all(u).context("Failed to write U plane")?;
    file.write_all(v).context("Failed to write V plane")?;

    Ok(())
}

pub fn load_yuv444p<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let file = File::open(&path)
        .with_context(|| format!("Failed to open YUV file: {:?}", path.as_ref()))?;
    let mut reader = BufReader::new(file);

    let plane_size = (width * height) as usize;

    let mut y_plane = vec![0u8; plane_size];
    let mut u_plane = vec![0u8; plane_size];
    let mut v_plane = vec![0u8; plane_size];

    reader
        .read_exact(&mut y_plane)
        .context("Failed to read Y plane")?;
    reader
        .read_exact(&mut u_plane)
        .context("Failed to read U plane")?;
    reader
        .read_exact(&mut v_plane)
        .context("Failed to read V plane")?;

    Ok((y_plane, u_plane, v_plane))
}

pub fn save_yuv444p<P: AsRef<Path>>(path: P, y: &[u8], u: &[u8], v: &[u8]) -> Result<()> {
    use std::io::Write;

    let mut file = File::create(&path)
        .with_context(|| format!("Failed to create YUV file: {:?}", path.as_ref()))?;

    file.write_all(y).context("Failed to write Y plane")?;
    file.write_all(u).context("Failed to write U plane")?;
    file.write_all(v).context("Failed to write V plane")?;

    Ok(())
}
