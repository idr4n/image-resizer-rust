//! # Image Resizing Library
//!
//! This library provides functionality for resizing images and saving them in various formats.
//! It uses the `fast_image_resize` crate for efficient image resizing operations and the `image`
//! crate for image I/O and format handling.
//!
//! Key features:
//! - Resize images while maintaining aspect ratio
//! - Support for JPEG and PNG formats
//! - Automatic format detection and conversion
//! - Efficient resizing using the `fast_image_resize` library
//!
//! The main functions provided are:
//! - `resize_image`: Resizes an image file to specified dimensions
//! - `save_image`: Saves a resized image buffer to a file
//!
//! This library is designed to be easy to use while providing robust error handling and
//! flexibility in image processing tasks.

use fast_image_resize::{self as fr, images::Image};
use image::{buffer::ConvertBuffer, guess_format, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

/// A container for holding source and destination images during the resizing process.
///
/// This struct encapsulates the data needed for image resizing operations, including
/// the dimensions of the new image and the source and destination image buffers.
struct ImageContainer {
    /// The width of the resized image in pixels.
    new_width: u32,
    /// The height of the resized image in pixels.
    new_height: u32,
    /// The source image buffer, containing the original image data.
    src_image: Image<'static>,
    /// The destination image buffer, where the resized image data will be stored.
    dst_image: Image<'static>,
}

impl ImageContainer {
    /// Creates a new `ImageContainer` from a `DynamicImage` and optional new dimensions.
    ///
    /// # Arguments
    ///
    /// * `img` - The input image as a `DynamicImage`.
    /// * `width` - An optional new width for the image. If None, it will be calculated based on the height.
    /// * `height` - An optional new height for the image. If None, it will be calculated based on the width.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `ImageContainer` if successful, or an error if the operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The new dimensions cannot be determined.
    /// - The image buffers cannot be created.
    fn new(
        img: DynamicImage,
        width: Option<&u32>,
        height: Option<&u32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (new_width, new_height) = determine_new_dimensions(&img, width, height)?;

        // Create a fast_image_resize::Image from the opened image
        let src_width = std::num::NonZeroU32::new(img.width()).unwrap();
        let src_height = std::num::NonZeroU32::new(img.height()).unwrap();
        let src_image = fr::images::Image::from_vec_u8(
            src_width.get(),
            src_height.get(),
            img.into_rgba8().into_raw(),
            fr::PixelType::U8x4,
        )?;

        // Create destination image
        let dst_width = std::num::NonZeroU32::new(new_width).unwrap();
        let dst_height = std::num::NonZeroU32::new(new_height).unwrap();
        let dst_image =
            fr::images::Image::new(dst_width.get(), dst_height.get(), src_image.pixel_type());

        Ok(Self {
            new_width,
            new_height,
            src_image,
            dst_image,
        })
    }
}

/// Represents information about an image.
pub struct ImageInfo {
    /// The width of the image in pixels.
    pub width: u32,
    /// The height of the image in pixels.
    pub height: u32,
    /// The format of the image (e.g., PNG, JPEG).
    pub format: ImageFormat,
    /// The file path of the image.
    pub path: PathBuf,
}

/// Determines the new dimensions for an image based on the provided width and height options.
///
/// # Arguments
///
/// * `img` - The original image.
/// * `width` - An optional new width for the image.
/// * `height` - An optional new height for the image.
///
/// # Returns
///
/// A tuple containing the new width and height, or an error if neither width nor height is specified.
fn determine_new_dimensions(
    img: &DynamicImage,
    width: Option<&u32>,
    height: Option<&u32>,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let (new_width, new_height) = match (width, height) {
        (Some(w), Some(h)) => (*w, *h),
        (Some(w), None) => {
            let aspect_ratio = img.height() as f32 / img.width() as f32;
            // println!("src_width {}, src_height {}", img.width(), img.height());
            (*w, (*w as f32 * aspect_ratio) as u32)
        }
        (None, Some(h)) => {
            let aspect_ratio = img.width() as f32 / img.height() as f32;
            // println!("src_width {}, src_height {}", img.width(), img.height());
            ((*h as f32 * aspect_ratio) as u32, *h)
        }
        (None, None) => {
            return Err("Error: At least one of width or height must be specified".into())
        }
    };

    Ok((new_width, new_height))
}

/// Resizes an image to the specified dimensions.
///
/// # Arguments
///
/// * `input` - The input image as a `DynamicImage`.
/// * `width` - An optional new width for the image. If None, it will be calculated based on the height.
/// * `height` - An optional new height for the image. If None, it will be calculated based on the width.
///
/// # Returns
///
/// A `Result` containing the resized image as an `ImageBuffer<Rgba<u8>, Vec<u8>>`, or an error if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Neither width nor height is specified.
/// - The resizing operation fails.
pub fn resize_image(
    input: DynamicImage,
    width: Option<&u32>,
    height: Option<&u32>,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    // Create Image instance from a DynamicImage input
    let mut img = ImageContainer::new(input, width, height)?;

    println!(
        "New image dimensions: width {} x height {}",
        img.new_width, img.new_height
    );

    // Create Resizer instance and resize
    let mut resizer = fr::Resizer::new();
    let resize_options = fr::ResizeOptions::default();
    resizer.resize(&img.src_image, &mut img.dst_image, &resize_options)?;

    // After resizing, create image buffer
    let resized_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
        img.new_width,
        img.new_height,
        img.dst_image.buffer().to_vec(),
    )
    .unwrap();

    Ok(resized_img)
}

/// Saves an image buffer to a file.
///
/// # Arguments
///
/// * `image` - The `ImageBuffer` to save.
/// * `output_path` - The path where the image should be saved.
/// * `save_format` - The `ImageFormat` specifying the desired output format (e.g., `ImageFormat::Jpeg` or `ImageFormat::Png`).
///
/// # Returns
///
/// A `Result` containing an `ImageInfo` struct with metadata about the saved image, or an error if the save operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The image buffer is empty.
/// - The file extension doesn't match the specified save format.
/// - The output path has no file extension.
/// - The image cannot be saved to the specified path.
pub fn save_image(
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    output_path: &Path,
    save_format: ImageFormat,
) -> Result<ImageInfo, Box<dyn std::error::Error>> {
    let width = image.width();
    let height = image.height();

    if width == 0 || height == 0 {
        return Err("Failed to save image: Empty image buffer".into());
    }

    // Check if the file extension matches the save format
    if let Some(extension) = output_path.extension().and_then(|ext| ext.to_str()) {
        let ext_format = string_to_image_format(extension)?;
        if ext_format != save_format {
            return Err(format!(
                "Output file extension is not compatible with the specified format. Expected: {:?}, got: {:?}",
                save_format, ext_format
            ).into());
        }
    } else {
        return Err("Output path has no file extension".into());
    }

    println!("Saving image to: {:?}", output_path);
    println!("Using format: {:?}", save_format);

    let image_to_save = match save_format {
        ImageFormat::Jpeg => DynamicImage::ImageRgb8(image.convert()),
        _ => DynamicImage::ImageRgba8(image),
    };

    image_to_save
        .save_with_format(output_path, save_format)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(ImageInfo {
        width,
        height,
        format: save_format,
        path: output_path.to_path_buf(),
    })
}

/// Determines the save format and output path for an image.
///
/// This function takes an image buffer, an output path, and an optional output format,
/// and determines the appropriate save format and final output path for the image.
///
/// # Arguments
///
/// * `image` - A reference to the `ImageBuffer` containing the image data.
/// * `output_path` - A reference to the `Path` where the image should be saved.
/// * `output_format` - An optional reference to a `String` specifying the desired output format.
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// - The determined `ImageFormat` for saving the image.
/// - A `PathBuf` representing the final output path (which may differ from the input if the extension changes).
///
/// # Errors
///
/// This function will return an error if:
/// - The provided output format (if any) is invalid or unsupported.
/// - The inferred format (when no output format is provided) is unsupported.
///
/// # Examples
///
/// ```
/// use image::{ImageBuffer, Rgba};
/// use std::path::Path;
/// use image_resizer_rust::determine_save_format_and_path;
///
/// let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(100, 100);
/// let output_path = Path::new("output.jpg");
/// let output_format = Some(String::from("png"));
///
/// let (format, path) = determine_save_format_and_path(&image, output_path, output_format.as_ref()).unwrap();
/// assert_eq!(format, image::ImageFormat::Png);
/// assert_eq!(path, Path::new("output.png"));
/// ```
pub fn determine_save_format_and_path(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    output_path: &Path,
    output_format: Option<&String>,
) -> Result<(ImageFormat, PathBuf), Box<dyn std::error::Error>> {
    let save_format = match output_format {
        Some(f) => string_to_image_format(f),
        None => validate_new_image_format(infer_format(image, Some(output_path))),
    }?;

    let new_extension = determine_extension(output_path, save_format);
    let new_output = output_path.with_extension(new_extension);

    Ok((save_format, new_output))
}

/// Checks if a given path exists and prompts the user for confirmation if it does.
///
/// # Arguments
///
/// * `path` - A reference to the `PathBuf` to check.
///
/// # Returns
///
/// A `Result` containing `()` if the path doesn't exist or the user confirms replacement,
/// or an error if the path exists and the user declines replacement or if there's an error checking the path.
///
/// # Errors
///
/// This function will return an error if:
/// - The path exists and the user chooses not to replace it.
/// - There's an error while checking if the path exists.
///
/// # Side Effects
///
/// This function prints to stdout and reads from stdin if the path exists.
pub fn check_if_path_exists(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    match path.try_exists() {
        Ok(true) => {
            print!(
                "\n{:?} already exists. Do you want to replace it? (y/n): ",
                path
            );
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "y" {
                return Err(format!("{:?} already exists. Operation cancelled!", path).into());
            }
        }
        Ok(false) => (),
        Err(e) => return Err(format!("Error checking path {:?}: {}", path, e).into()),
    }
    Ok(())
}

/// Converts a string representation of an image format to the corresponding `ImageFormat`.
///
/// # Arguments
///
/// * `format` - A string representing the image format ("jpeg", "jpg", or "png").
///
/// # Returns
///
/// The corresponding `ImageFormat`, or an error if the format is unsupported.
fn string_to_image_format(format: &str) -> Result<ImageFormat, Box<dyn std::error::Error>> {
    match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
        "png" => Ok(ImageFormat::Png),
        _ => Err(format!("Unsoported image format {}", format).into()),
    }
}

/// Validates that the given image format is supported by this library.
///
/// # Arguments
///
/// * `format` - The `ImageFormat` to validate.
///
/// # Returns
///
/// The validated `ImageFormat` if it's supported, or an error if it's not.
fn validate_new_image_format(
    format: ImageFormat,
) -> Result<ImageFormat, Box<dyn std::error::Error>> {
    match format {
        ImageFormat::Png | ImageFormat::Jpeg => Ok(format),
        _ => Err(format!(
            "Unsoported conversion to image format '{:?}'. Specify a valid format with --format.",
            format
        )
        .into()),
    }
}

/// Determines the appropriate file extension based on the image format and original path.
///
/// # Arguments
///
/// * `path` - The original file path.
/// * `format` - The `ImageFormat` to use for determining the extension.
///
/// # Returns
///
/// A string representing the appropriate file extension.
fn determine_extension(path: &Path, format: ImageFormat) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .filter(|&ext| (ext == "jpg" || ext == "jpeg") && format == ImageFormat::Jpeg)
        .map(|ext| ext.to_string())
        .unwrap_or_else(|| format.extensions_str()[0].to_string())
}

/// Infers the image format from the image buffer or file path.
///
/// # Arguments
///
/// * `image` - The `ImageBuffer` to analyze.
/// * `path` - An optional file path to use for format inference if the buffer analysis fails.
///
/// # Returns
///
/// The inferred `ImageFormat`, defaulting to JPEG if the format cannot be determined.
fn infer_format(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, path: Option<&Path>) -> ImageFormat {
    // Convert the image buffer to a byte slice
    let bytes = image.as_raw();

    // Use guess_format to infer the image format
    match guess_format(bytes) {
        Ok(format) => format,
        Err(_) => {
            // Try to infer format from path if available
            if let Some(file_path) = path {
                match ImageFormat::from_path(file_path) {
                    Ok(format) => format,
                    Err(_) => {
                        eprintln!("Warning: Could not guess image format. Defaulting to JPEG.");
                        ImageFormat::Jpeg
                    }
                }
            } else {
                eprintln!("Warning: Could not guess image format. Defaulting to JPEG.");
                ImageFormat::Jpeg
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};
    use std::path::PathBuf;

    fn create_mock_jpeg() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut buffer = vec![0; 400]; // 10x10 RGBA image = 400 bytes
        buffer[0] = 0xFF;
        buffer[1] = 0xD8;
        buffer[2] = 0xFF;
        buffer[3] = 0xE0;
        buffer[6] = 0x4A;
        buffer[7] = 0x46;
        buffer[8] = 0x49;
        buffer[9] = 0x46;
        buffer[10] = 0x00;
        ImageBuffer::from_raw(10, 10, buffer).expect("Failed to create mock JPEG")
    }

    fn create_mock_png() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut buffer = vec![0; 400]; // 10x10 RGBA image = 400 bytes
        buffer[0] = 0x89;
        buffer[1] = 0x50;
        buffer[2] = 0x4E;
        buffer[3] = 0x47;
        buffer[4] = 0x0D;
        buffer[5] = 0x0A;
        buffer[6] = 0x1A;
        buffer[7] = 0x0A;
        ImageBuffer::from_raw(10, 10, buffer).expect("Failed to create mock PNG")
    }

    fn create_mock_unknown() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        ImageBuffer::from_raw(10, 10, vec![0; 400]).expect("Failed to create mock unknown")
    }

    mod infer_format_test {
        use super::*;

        #[test]
        fn test_infer_format_jpeg() {
            let image = create_mock_jpeg();
            assert_eq!(infer_format(&image, None), ImageFormat::Jpeg);
        }

        #[test]
        fn test_infer_format_png() {
            let image = create_mock_png();
            assert_eq!(infer_format(&image, None), ImageFormat::Png);
        }

        #[test]
        fn test_infer_format_from_path() {
            let image = create_mock_unknown();
            let path = PathBuf::from("test_image.png");
            assert_eq!(infer_format(&image, Some(&path)), ImageFormat::Png);
        }

        #[test]
        fn test_infer_format_fallback_to_jpeg() {
            let image = create_mock_unknown();
            let path = PathBuf::from("test_image.unknown");
            assert_eq!(infer_format(&image, Some(&path)), ImageFormat::Jpeg);
        }
    }

    mod save_image_test {
        use super::*;
        use tempfile::TempDir;

        #[test]
        fn test_save_image_jpg() {
            let dir = TempDir::new().expect("Failed to create a temp dir");
            let image = create_mock_jpeg();
            let width = image.width();
            let height = image.height();
            let output_path = dir.path().join("output.jpg");
            let format = ImageFormat::Jpeg;

            let result = save_image(image, output_path.as_path(), format).unwrap();

            assert_eq!(result.path, output_path);
            assert_eq!(result.format, ImageFormat::Jpeg);
            assert_eq!(result.width, width);
            assert_eq!(result.height, height);
        }

        #[test]
        fn test_save_image_different_format() {
            let dir = TempDir::new().expect("Failed to create a temp dir");
            let image = create_mock_jpeg();
            let output_path = dir.path().join("output.jpg");
            let format = ImageFormat::Png;

            let result = save_image(image, output_path.as_path(), format);

            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Output file extension is not compatible with the specified format"));
                assert!(e.to_string().contains("Expected: Png, got: Jpeg"));
            } else {
                panic!("Expected an error, but got Ok");
            }
        }

        #[test]
        fn test_save_image_failure() {
            let image = create_mock_jpeg();
            let non_existent_dir = PathBuf::from("/non/existent/directory");
            let output_path = non_existent_dir.join("output.jpg");
            let format = ImageFormat::Jpeg;

            let result = save_image(image, output_path.as_path(), format);

            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e.to_string().contains("Failed to save image"));
            } else {
                panic!("Expected an error, but got Ok");
            }
        }

        #[test]
        fn test_save_image_empty_buffer() {
            let dir = TempDir::new().expect("Failed to create a temp dir");
            let empty_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(0, 0);
            let output_path = dir.path().join("empty_output.jpg");
            let format = ImageFormat::Jpeg;

            let result = save_image(empty_image, output_path.as_path(), format);

            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to save image: Empty image buffer"));
            } else {
                panic!("Expected an error, but got Ok");
            }
        }
    }
}
