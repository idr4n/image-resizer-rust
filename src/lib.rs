use fast_image_resize::{self as fr, images::Image};
use image::{guess_format, DynamicImage, ImageBuffer, ImageFormat, ImageResult, Rgba};
use std::path::Path;

struct ImageContainer {
    new_width: u32,
    new_height: u32,
    src_image: Image<'static>,
    dst_image: Image<'static>,
}

impl ImageContainer {
    fn new(
        path: &Path,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;

        // Create a fast_image_resize::Image from the opened image
        let src_width = std::num::NonZeroU32::new(img.width()).unwrap();
        let src_height = std::num::NonZeroU32::new(img.height()).unwrap();
        let src_image = fr::images::Image::from_vec_u8(
            src_width.get(),
            src_height.get(),
            img.to_rgba8().into_raw(),
            fr::PixelType::U8x4,
        )?;

        let (new_width, new_height) = determine_new_dimensions(img, width, height)?;

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

fn determine_new_dimensions(
    img: DynamicImage,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let (new_width, new_height) = match (width, height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let aspect_ratio = img.height() as f32 / img.width() as f32;
            // println!("src_width {}, src_height {}", img.width(), img.height());
            (w, (w as f32 * aspect_ratio) as u32)
        }
        (None, Some(h)) => {
            let aspect_ratio = img.width() as f32 / img.height() as f32;
            // println!("src_width {}, src_height {}", img.width(), img.height());
            ((h as f32 * aspect_ratio) as u32, h)
        }
        (None, None) => return Err("At least one of width or height must be specified".into()),
    };

    Ok((new_width, new_height))
}

pub fn resize_image(
    input_path: &Path,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    println!("The input given was: '{:?}'", input_path);

    println!(
        "Resizing image {:?} with new width {} and new height {}.",
        input_path,
        width.unwrap_or(0),
        height.unwrap_or(0),
    );

    // Create Image instance from image path
    let mut img = ImageContainer::new(input_path, width, height)?;

    println!(
        "New image dimensions: width {} x height {}",
        img.new_width, img.new_height
    );

    // Create Resizer instance and resize
    let mut resizer = fr::Resizer::new();
    let resize_options = fr::ResizeOptions::default();
    resizer.resize(&img.src_image, &mut img.dst_image, &resize_options)?;

    // After resizing, create image buffer
    let resized_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        img.new_width,
        img.new_height,
        img.dst_image.buffer().to_vec(),
    )
    .unwrap();

    Ok(resized_img)
}

pub fn string_to_image_format(format: &str) -> Result<ImageFormat, Box<dyn std::error::Error>> {
    match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
        "png" => Ok(ImageFormat::Png),
        _ => Err(format!("Unsoported image format {}", format).into()),
    }
}

pub fn save_image(
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    output: &Path,
    format: Option<ImageFormat>,
) -> Result<ImageResult<()>, Box<dyn std::error::Error>> {
    let extension = output.extension();

    let save_format = match extension {
        Some(ext) => {
            string_to_image_format(ext.to_str().expect("Invalid Unicode in file extension"))?
        }
        None => format.unwrap_or_else(|| infer_format(&image)),
    };

    Ok(image.save_with_format(output, save_format))
}

fn infer_format(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageFormat {
    // Convert the image buffer to a byte slice
    let bytes = image.as_raw();

    // Use guess_format to infer the image format
    // guess_format(bytes).unwrap_or(ImageFormat::Jpeg)
    guess_format(bytes).expect("Could not guess image format.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

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
            assert_eq!(infer_format(&image), ImageFormat::Jpeg);
        }

        #[test]
        fn test_infer_format_png() {
            let image = create_mock_png();
            assert_eq!(infer_format(&image), ImageFormat::Png);
        }

        #[test]
        #[should_panic(expected = "Could not guess image format.")]
        fn test_infer_format_unknown() {
            let image = create_mock_unknown();
            infer_format(&image);
        }
    }
}
