use anyhow::Result;
use jpegxs_core::{ImageOwned8, PixelFormat};
use std::f32::consts::PI;

/// Comprehensive test vector generation for JPEG XS conformance testing
pub struct TestVectorGenerator {
    patterns: Vec<TestPattern>,
}

#[derive(Debug, Clone)]
pub struct TestPattern {
    pub name: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub expected_psnr_threshold: f64,
    pub generator: PatternType,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    SolidColor { r: u8, g: u8, b: u8 },
    HorizontalGradient,
    VerticalGradient,
    DiagonalGradient,
    CheckerBoard { size: u32 },
    RandomNoise { seed: u64 },
    SineWave { frequency: f32 },
    Impulse { position: (u32, u32) },
    RampPattern,
    ColorBars,
    Zone,
    NaturalImageProxy,
    HighFrequencyTest,
    LowFrequencyTest,
    EdgeTest,
    TextPattern,
}

impl TestVectorGenerator {
    pub fn new() -> Self {
        Self {
            patterns: Self::default_test_patterns(),
        }
    }

    fn default_test_patterns() -> Vec<TestPattern> {
        vec![
            // Basic patterns for fundamental testing
            TestPattern {
                name: "solid_red".to_string(),
                description: "Pure red image for basic codec validation".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 40.0,
                generator: PatternType::SolidColor { r: 255, g: 0, b: 0 },
            },
            TestPattern {
                name: "solid_green".to_string(),
                description: "Pure green image".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 40.0,
                generator: PatternType::SolidColor { r: 0, g: 255, b: 0 },
            },
            TestPattern {
                name: "solid_blue".to_string(),
                description: "Pure blue image".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 40.0,
                generator: PatternType::SolidColor { r: 0, g: 0, b: 255 },
            },
            TestPattern {
                name: "black_image".to_string(),
                description: "All zeros - tests minimum values".to_string(),
                width: 512,
                height: 512,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 50.0,
                generator: PatternType::SolidColor { r: 0, g: 0, b: 0 },
            },
            TestPattern {
                name: "white_image".to_string(),
                description: "All maximum values - tests clipping".to_string(),
                width: 512,
                height: 512,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 50.0,
                generator: PatternType::SolidColor { r: 255, g: 255, b: 255 },
            },

            // Gradient patterns for smooth transitions
            TestPattern {
                name: "horizontal_gradient".to_string(),
                description: "Horizontal gradient for smooth transition testing".to_string(),
                width: 512,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 35.0,
                generator: PatternType::HorizontalGradient,
            },
            TestPattern {
                name: "vertical_gradient".to_string(),
                description: "Vertical gradient".to_string(),
                width: 256,
                height: 512,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 35.0,
                generator: PatternType::VerticalGradient,
            },
            TestPattern {
                name: "diagonal_gradient".to_string(),
                description: "Diagonal gradient for edge case testing".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 30.0,
                generator: PatternType::DiagonalGradient,
            },

            // High frequency patterns
            TestPattern {
                name: "checker_8x8".to_string(),
                description: "8x8 checkerboard pattern - high frequency test".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 25.0,
                generator: PatternType::CheckerBoard { size: 8 },
            },
            TestPattern {
                name: "checker_4x4".to_string(),
                description: "4x4 checkerboard pattern - very high frequency".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 20.0,
                generator: PatternType::CheckerBoard { size: 4 },
            },
            TestPattern {
                name: "checker_2x2".to_string(),
                description: "2x2 checkerboard pattern - extreme high frequency".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 15.0,
                generator: PatternType::CheckerBoard { size: 2 },
            },

            // Frequency domain tests
            TestPattern {
                name: "sine_wave_low".to_string(),
                description: "Low frequency sine wave pattern".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 35.0,
                generator: PatternType::SineWave { frequency: 2.0 },
            },
            TestPattern {
                name: "sine_wave_high".to_string(),
                description: "High frequency sine wave pattern".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 25.0,
                generator: PatternType::SineWave { frequency: 16.0 },
            },

            // Impulse and edge cases
            TestPattern {
                name: "impulse_center".to_string(),
                description: "Single white pixel impulse at center".to_string(),
                width: 128,
                height: 128,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 30.0,
                generator: PatternType::Impulse { position: (64, 64) },
            },

            // Random noise
            TestPattern {
                name: "random_noise".to_string(),
                description: "Random noise pattern - worst case for compression".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 10.0,
                generator: PatternType::RandomNoise { seed: 42 },
            },

            // Standard test patterns
            TestPattern {
                name: "color_bars".to_string(),
                description: "Standard color bars pattern".to_string(),
                width: 320,
                height: 240,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 30.0,
                generator: PatternType::ColorBars,
            },
            TestPattern {
                name: "zone_pattern".to_string(),
                description: "Zone pattern with varying intensities".to_string(),
                width: 256,
                height: 256,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 25.0,
                generator: PatternType::Zone,
            },

            // Different resolutions
            TestPattern {
                name: "hd_gradient".to_string(),
                description: "HD resolution gradient test".to_string(),
                width: 1920,
                height: 1080,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 35.0,
                generator: PatternType::HorizontalGradient,
            },
            TestPattern {
                name: "4k_solid".to_string(),
                description: "4K resolution solid color test".to_string(),
                width: 3840,
                height: 2160,
                format: PixelFormat::Rgb8,
                expected_psnr_threshold: 50.0,
                generator: PatternType::SolidColor { r: 128, g: 128, b: 128 },
            },

            // YUV format tests
            TestPattern {
                name: "yuv444_gradient".to_string(),
                description: "YUV 4:4:4 format gradient test".to_string(),
                width: 512,
                height: 384,
                format: PixelFormat::Yuv444p8,
                expected_psnr_threshold: 35.0,
                generator: PatternType::HorizontalGradient,
            },
            TestPattern {
                name: "yuv422_bars".to_string(),
                description: "YUV 4:2:2 format color bars".to_string(),
                width: 640,
                height: 480,
                format: PixelFormat::Yuv422p8,
                expected_psnr_threshold: 30.0,
                generator: PatternType::ColorBars,
            },
            TestPattern {
                name: "yuv420_natural".to_string(),
                description: "YUV 4:2:0 format natural image proxy".to_string(),
                width: 352,
                height: 288,
                format: PixelFormat::Yuv420p8,
                expected_psnr_threshold: 25.0,
                generator: PatternType::NaturalImageProxy,
            },
        ]
    }

    pub fn generate_pattern(&self, pattern: &TestPattern) -> Result<ImageOwned8> {
        match &pattern.generator {
            PatternType::SolidColor { r, g, b } => {
                self.generate_solid_color(pattern.width, pattern.height, pattern.format, *r, *g, *b)
            }
            PatternType::HorizontalGradient => {
                self.generate_horizontal_gradient(pattern.width, pattern.height, pattern.format)
            }
            PatternType::VerticalGradient => {
                self.generate_vertical_gradient(pattern.width, pattern.height, pattern.format)
            }
            PatternType::DiagonalGradient => {
                self.generate_diagonal_gradient(pattern.width, pattern.height, pattern.format)
            }
            PatternType::CheckerBoard { size } => {
                self.generate_checkerboard(pattern.width, pattern.height, pattern.format, *size)
            }
            PatternType::RandomNoise { seed } => {
                self.generate_random_noise(pattern.width, pattern.height, pattern.format, *seed)
            }
            PatternType::SineWave { frequency } => {
                self.generate_sine_wave(pattern.width, pattern.height, pattern.format, *frequency)
            }
            PatternType::Impulse { position } => {
                self.generate_impulse(pattern.width, pattern.height, pattern.format, *position)
            }
            PatternType::ColorBars => {
                self.generate_color_bars(pattern.width, pattern.height, pattern.format)
            }
            PatternType::Zone => {
                self.generate_zone_pattern(pattern.width, pattern.height, pattern.format)
            }
            PatternType::NaturalImageProxy => {
                self.generate_natural_image_proxy(pattern.width, pattern.height, pattern.format)
            }
            _ => {
                // Default to solid gray for unimplemented patterns
                self.generate_solid_color(pattern.width, pattern.height, pattern.format, 128, 128, 128)
            }
        }
    }

    fn generate_solid_color(&self, width: u32, height: u32, format: PixelFormat, r: u8, g: u8, b: u8) -> Result<ImageOwned8> {
        let data = match format {
            PixelFormat::Rgb8 => {
                let mut data = Vec::with_capacity((width * height * 3) as usize);
                for _ in 0..width * height {
                    data.extend_from_slice(&[r, g, b]);
                }
                data
            }
            PixelFormat::Bgr8 => {
                let mut data = Vec::with_capacity((width * height * 3) as usize);
                for _ in 0..width * height {
                    data.extend_from_slice(&[b, g, r]);
                }
                data
            }
            PixelFormat::Yuv444p8 => {
                // Convert RGB to YUV
                let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                let u = (128.0 - 0.168736 * r as f32 - 0.331264 * g as f32 + 0.5 * b as f32) as u8;
                let v = (128.0 + 0.5 * r as f32 - 0.418688 * g as f32 - 0.081312 * b as f32) as u8;

                let mut data = Vec::with_capacity((width * height * 3) as usize);

                // Y plane
                for _ in 0..width * height {
                    data.push(y);
                }
                // U plane
                for _ in 0..width * height {
                    data.push(u);
                }
                // V plane
                for _ in 0..width * height {
                    data.push(v);
                }
                data
            }
            _ => {
                // Default RGB for other formats
                let mut data = Vec::with_capacity((width * height * 3) as usize);
                for _ in 0..width * height {
                    data.extend_from_slice(&[r, g, b]);
                }
                data
            }
        };

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_horizontal_gradient(&self, width: u32, height: u32, format: PixelFormat) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for _y in 0..height {
            for x in 0..width {
                let value = (x * 255 / (width - 1)) as u8;
                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_vertical_gradient(&self, width: u32, height: u32, format: PixelFormat) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            let value = (y * 255 / (height - 1)) as u8;
            for _x in 0..width {
                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_diagonal_gradient(&self, width: u32, height: u32, format: PixelFormat) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            for x in 0..width {
                let value = ((x + y) * 255 / (width + height - 2)) as u8;
                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_checkerboard(&self, width: u32, height: u32, format: PixelFormat, size: u32) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            for x in 0..width {
                let is_white = ((x / size) + (y / size)) % 2 == 0;
                let value = if is_white { 255 } else { 0 };

                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_random_noise(&self, width: u32, height: u32, format: PixelFormat, seed: u64) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        // Simple linear congruential generator
        let mut rng = seed;

        for _ in 0..width * height {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let r = (rng >> 16) as u8;

            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let g = (rng >> 16) as u8;

            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            let b = (rng >> 16) as u8;

            match format {
                PixelFormat::Rgb8 => {
                    data.extend_from_slice(&[r, g, b]);
                }
                PixelFormat::Bgr8 => {
                    data.extend_from_slice(&[b, g, r]);
                }
                _ => {
                    data.extend_from_slice(&[r, g, b]);
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_sine_wave(&self, width: u32, height: u32, format: PixelFormat, frequency: f32) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32 / width as f32;
                let fy = y as f32 / height as f32;
                let wave = ((fx * frequency * 2.0 * PI).sin() + (fy * frequency * 2.0 * PI).sin()) / 2.0;
                let value = ((wave + 1.0) * 127.5) as u8;

                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_impulse(&self, width: u32, height: u32, format: PixelFormat, position: (u32, u32)) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            for x in 0..width {
                let is_impulse = x == position.0 && y == position.1;
                let value = if is_impulse { 255 } else { 0 };

                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_color_bars(&self, width: u32, height: u32, format: PixelFormat) -> Result<ImageOwned8> {
        let colors = [
            (255, 255, 255), // White
            (255, 255, 0),   // Yellow
            (0, 255, 255),   // Cyan
            (0, 255, 0),     // Green
            (255, 0, 255),   // Magenta
            (255, 0, 0),     // Red
            (0, 0, 255),     // Blue
            (0, 0, 0),       // Black
        ];

        let mut data = Vec::with_capacity((width * height * 3) as usize);
        let bar_width = width / colors.len() as u32;

        for _y in 0..height {
            for x in 0..width {
                let color_index = (x / bar_width).min(colors.len() as u32 - 1);
                let (r, g, b) = colors[color_index as usize];

                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[r, g, b]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[b, g, r]);
                    }
                    _ => {
                        data.extend_from_slice(&[r, g, b]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_zone_pattern(&self, width: u32, height: u32, format: PixelFormat) -> Result<ImageOwned8> {
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            for x in 0..width {
                let cx = width as f32 / 2.0;
                let cy = height as f32 / 2.0;
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let distance = (dx * dx + dy * dy).sqrt();
                let max_distance = (cx * cx + cy * cy).sqrt();
                let value = ((1.0 - distance / max_distance) * 255.0) as u8;

                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    fn generate_natural_image_proxy(&self, width: u32, height: u32, format: PixelFormat) -> Result<ImageOwned8> {
        // Generate a proxy for natural image characteristics
        // Combines low frequency background with some texture
        let mut data = Vec::with_capacity((width * height * 3) as usize);

        for y in 0..height {
            for x in 0..width {
                // Background gradient
                let bg = (x * 255 / width + y * 255 / height) / 2;

                // Add some texture
                let texture = ((x as f32 * 0.1).sin() * (y as f32 * 0.1).cos() * 32.0) as i32;
                let value = (bg as i32 + texture).clamp(0, 255) as u8;

                match format {
                    PixelFormat::Rgb8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    PixelFormat::Bgr8 => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                    _ => {
                        data.extend_from_slice(&[value, value, value]);
                    }
                }
            }
        }

        Ok(ImageOwned8 {
            data,
            width,
            height,
            format,
        })
    }

    pub fn get_all_patterns(&self) -> &[TestPattern] {
        &self.patterns
    }

    pub fn get_pattern(&self, name: &str) -> Option<&TestPattern> {
        self.patterns.iter().find(|p| p.name == name)
    }

    pub fn generate_all(&self) -> Result<Vec<(TestPattern, ImageOwned8)>> {
        let mut results = Vec::new();

        for pattern in &self.patterns {
            let image = self.generate_pattern(pattern)?;
            results.push((pattern.clone(), image));
        }

        Ok(results)
    }
}

impl Default for TestVectorGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_generation() {
        let generator = TestVectorGenerator::new();
        let patterns = generator.get_all_patterns();

        assert!(!patterns.is_empty());
        assert!(patterns.len() > 10);

        // Test a few specific patterns
        let solid_red = generator.get_pattern("solid_red").unwrap();
        let image = generator.generate_pattern(solid_red).unwrap();
        assert_eq!(image.width, 256);
        assert_eq!(image.height, 256);
        assert_eq!(image.format, PixelFormat::Rgb8);
        assert_eq!(image.data.len(), 256 * 256 * 3);

        // Check that it's actually red
        assert_eq!(image.data[0], 255); // R
        assert_eq!(image.data[1], 0);   // G
        assert_eq!(image.data[2], 0);   // B
    }

    #[test]
    fn test_gradient_pattern() {
        let generator = TestVectorGenerator::new();
        let pattern = generator.get_pattern("horizontal_gradient").unwrap();
        let image = generator.generate_pattern(pattern).unwrap();

        assert_eq!(image.width, 512);
        assert_eq!(image.height, 256);

        // Check gradient properties
        let first_pixel = image.data[0];
        let last_pixel = image.data[image.data.len() - 3];
        assert!(last_pixel > first_pixel); // Should increase from left to right
    }

    #[test]
    fn test_checkerboard_pattern() {
        let generator = TestVectorGenerator::new();
        let pattern = generator.get_pattern("checker_8x8").unwrap();
        let image = generator.generate_pattern(pattern).unwrap();

        // Check that we have alternating values
        let pixel1 = image.data[0];
        let pixel2 = image.data[24]; // 8 pixels over
        assert_ne!(pixel1, pixel2);
    }
}
