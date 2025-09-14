use image::{ImageBuffer, Rgb, DynamicImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple gradient test image
    let width = 256;
    let height = 256;

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let r = (255.0 * x as f32 / width as f32) as u8;
        let g = (255.0 * y as f32 / height as f32) as u8;
        let b = (255.0 * (x + y) as f32 / (width + height) as f32) as u8;
        Rgb([r, g, b])
    });

    let dynamic_img = DynamicImage::ImageRgb8(img);
    dynamic_img.save("testing/fixtures/test_256x256.png")?;
    println!("✅ Created test PNG: testing/fixtures/test_256x256.png");

    // Also create a JPEG version
    dynamic_img.save("testing/fixtures/test_256x256.jpg")?;
    println!("✅ Created test JPEG: testing/fixtures/test_256x256.jpg");

    Ok(())
}
