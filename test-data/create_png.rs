//! Simple script to create test PNG using the image crate
use image::{ImageBuffer, Rgb, DynamicImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple test pattern
    let width = 256;
    let height = 256;
    
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        // Create a simple pattern with red/green/blue gradients
        let r = (255.0 * x as f32 / width as f32) as u8;
        let g = (255.0 * y as f32 / height as f32) as u8;
        let b = if (x / 32 + y / 32) % 2 == 0 { 255 } else { 0 };
        Rgb([r, g, b])
    });
    
    let dynamic_img = DynamicImage::ImageRgb8(img);
    dynamic_img.save("gradient_test.png")?;
    println!("Created gradient_test.png");
    
    // Create another pattern
    let img2: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let val = if ((x / 16) + (y / 16)) % 2 == 0 { 255 } else { 128 };
        Rgb([val, val, val])
    });
    
    let dynamic_img2 = DynamicImage::ImageRgb8(img2);
    dynamic_img2.save("checkerboard_test.png")?;
    println!("Created checkerboard_test.png");
    
    Ok(())
}
