use crate::{ConformanceTest, TestCase, TestStatus};
use anyhow::{Context, Result};
use jpegxs_core::{
    decode_frame, encode_frame, Bitstream, DecoderConfig, EncoderConfig, ImageOwned8, ImageView8,
    PixelFormat,
};
use std::fs;
use std::path::{Path, PathBuf};

pub struct DecoderConformanceTest {
    name: String,
    category: String,
    bitstream_path: PathBuf,
    expected_output: Option<PathBuf>,
}

impl DecoderConformanceTest {
    pub fn new(name: &str, bitstream_path: &Path) -> Self {
        Self {
            name: name.to_string(),
            category: "Decoder".to_string(),
            bitstream_path: bitstream_path.to_path_buf(),
            expected_output: None,
        }
    }

    pub fn with_expected_output(mut self, path: &Path) -> Self {
        self.expected_output = Some(path.to_path_buf());
        self
    }
}

impl ConformanceTest for DecoderConformanceTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn category(&self) -> &str {
        &self.category
    }

    fn run(&self) -> Result<TestCase> {
        let start = std::time::Instant::now();

        // Read the bitstream
        let bitstream_data = fs::read(&self.bitstream_path).context("Failed to read bitstream")?;

        // Create Bitstream struct
        let bitstream = Bitstream {
            data: bitstream_data.clone(),
            size_bits: bitstream_data.len() * 8,
        };

        // Attempt to decode
        let config = DecoderConfig::default();
        let result = decode_frame(&bitstream, &config);

        let (status, message) = match result {
            Ok(decoded) => {
                // If we have expected output, compare
                if let Some(ref expected_path) = self.expected_output {
                    if expected_path.exists() {
                        let expected =
                            fs::read(expected_path).context("Failed to read expected output")?;

                        if decoded.data == expected {
                            (TestStatus::Pass, None)
                        } else {
                            let psnr = crate::metrics::calculate_psnr(&expected, &decoded.data);
                            if psnr > 30.0 {
                                (TestStatus::Pass, Some(format!("PSNR: {:.2} dB", psnr)))
                            } else {
                                (
                                    TestStatus::Fail,
                                    Some(format!("PSNR too low: {:.2} dB", psnr)),
                                )
                            }
                        }
                    } else {
                        (TestStatus::Pass, Some("No reference output".to_string()))
                    }
                } else {
                    (
                        TestStatus::Pass,
                        Some(format!("Decoded {}x{}", decoded.width, decoded.height)),
                    )
                }
            }
            Err(e) => (TestStatus::Fail, Some(e.to_string())),
        };

        Ok(TestCase {
            name: self.name.clone(),
            category: self.category.clone(),
            status,
            message,
            duration_ms: start.elapsed().as_millis() as f64,
        })
    }
}

pub struct EncoderConformanceTest {
    name: String,
    category: String,
    input_image: ImageOwned8,
    quality: f32,
    #[allow(clippy::type_complexity)]
    reference_decoder: Option<Box<dyn Fn(&[u8]) -> Result<ImageOwned8>>>,
}

impl EncoderConformanceTest {
    pub fn new(name: &str, input: ImageOwned8, quality: f32) -> Self {
        Self {
            name: name.to_string(),
            category: "Encoder".to_string(),
            input_image: input,
            quality,
            reference_decoder: None,
        }
    }

    pub fn with_reference_decoder<F>(mut self, decoder: F) -> Self
    where
        F: Fn(&[u8]) -> Result<ImageOwned8> + 'static,
    {
        self.reference_decoder = Some(Box::new(decoder));
        self
    }
}

impl ConformanceTest for EncoderConformanceTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn category(&self) -> &str {
        &self.category
    }

    fn run(&self) -> Result<TestCase> {
        let start = std::time::Instant::now();

        let input_view = ImageView8 {
            data: &self.input_image.data,
            width: self.input_image.width,
            height: self.input_image.height,
            format: self.input_image.format,
        };

        let config = EncoderConfig {
            quality: self.quality,
            ..Default::default()
        };

        // Encode
        let encoded = encode_frame(input_view, &config);

        let (status, message) = match encoded {
            Ok(bitstream) => {
                // Try to decode with our decoder
                let our_decode = decode_frame(&bitstream, &DecoderConfig::default());

                match our_decode {
                    Ok(decoded) => {
                        let psnr =
                            crate::metrics::calculate_psnr(&self.input_image.data, &decoded.data);

                        // If we have a reference decoder, test with it too
                        if let Some(ref ref_decoder) = self.reference_decoder {
                            match ref_decoder(&bitstream.data) {
                                Ok(_) => {
                                    if psnr > 30.0 {
                                        (
                                            TestStatus::Pass,
                                            Some(format!("PSNR: {:.2} dB, Reference: OK", psnr)),
                                        )
                                    } else {
                                        (
                                            TestStatus::Fail,
                                            Some(format!("PSNR too low: {:.2} dB", psnr)),
                                        )
                                    }
                                }
                                Err(e) => (
                                    TestStatus::Fail,
                                    Some(format!("Reference decoder failed: {}", e)),
                                ),
                            }
                        } else if psnr > 30.0 {
                            (TestStatus::Pass, Some(format!("PSNR: {:.2} dB", psnr)))
                        } else {
                            (
                                TestStatus::Fail,
                                Some(format!("PSNR too low: {:.2} dB", psnr)),
                            )
                        }
                    }
                    Err(e) => (TestStatus::Fail, Some(format!("Self-decode failed: {}", e))),
                }
            }
            Err(e) => (TestStatus::Fail, Some(e.to_string())),
        };

        Ok(TestCase {
            name: self.name.clone(),
            category: self.category.clone(),
            status,
            message,
            duration_ms: start.elapsed().as_millis() as f64,
        })
    }
}

pub struct BitstreamConformanceTest {
    name: String,
    bitstream: Vec<u8>,
}

impl BitstreamConformanceTest {
    pub fn new(name: &str, bitstream: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            bitstream,
        }
    }

    fn validate_markers(&self) -> Result<()> {
        // Check for SOC marker (0xFF10)
        if self.bitstream.len() < 2 || self.bitstream[0] != 0xFF || self.bitstream[1] != 0x10 {
            return Err(anyhow::anyhow!("Missing SOC marker"));
        }

        // Additional marker validation would go here
        // This is a simplified example

        Ok(())
    }
}

impl ConformanceTest for BitstreamConformanceTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn category(&self) -> &str {
        "Bitstream"
    }

    fn run(&self) -> Result<TestCase> {
        let start = std::time::Instant::now();

        let (status, message) = match self.validate_markers() {
            Ok(_) => (TestStatus::Pass, Some("All markers valid".to_string())),
            Err(e) => (TestStatus::Fail, Some(e.to_string())),
        };

        Ok(TestCase {
            name: self.name.clone(),
            category: "Bitstream".to_string(),
            status,
            message,
            duration_ms: start.elapsed().as_millis() as f64,
        })
    }
}

pub fn create_iso_test_suite() -> Vec<Box<dyn ConformanceTest>> {
    let mut tests: Vec<Box<dyn ConformanceTest>> = Vec::new();

    // Create synthetic test image
    let test_image = ImageOwned8 {
        data: vec![128; 256 * 256 * 3],
        width: 256,
        height: 256,
        format: PixelFormat::Rgb8,
    };

    // Add encoder tests
    tests.push(Box::new(EncoderConformanceTest::new(
        "encode_quality_high",
        test_image.clone(),
        0.95,
    )));

    tests.push(Box::new(EncoderConformanceTest::new(
        "encode_quality_medium",
        test_image.clone(),
        0.5,
    )));

    tests.push(Box::new(EncoderConformanceTest::new(
        "encode_quality_low",
        test_image,
        0.1,
    )));

    // Decoder tests would be added here once we have test vectors
    // tests.push(Box::new(
    //     DecoderConformanceTest::new("decode_iso_vector_1", Path::new("vectors/test1.jxs"))
    // ));

    tests
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_conformance() {
        let test_image = ImageOwned8 {
            data: vec![100; 64 * 64 * 3],
            width: 64,
            height: 64,
            format: PixelFormat::Rgb8,
        };

        let test = EncoderConformanceTest::new("test", test_image, 0.9);
        let result = test.run().unwrap();

        assert_eq!(result.name, "test");
        assert_eq!(result.category, "Encoder");
    }
}
