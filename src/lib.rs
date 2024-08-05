use fast_image_resize::{self as fr, images::Image, IntoImageView};
use std::path::Path;

use image::{DynamicImage, ImageBuffer, Rgba};

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
    format: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("The input given was: '{:?}'", input_path);

    println!(
        "Resizing image {:?} with new width {}, new height {}, and format {}.",
        input_path,
        width.unwrap_or(0),
        height.unwrap_or(0),
        format.unwrap()
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

    // After resizing...
    let resized_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        img.new_width,
        img.new_height,
        img.dst_image.buffer().to_vec(),
    )
    .unwrap();

    let output_path = match format {
        Some(f) => match f.to_lowercase().as_str() {
            "jpeg" => "output.jpg",
            "png" => "output.png",
            _ => "output.png", // Default to PNG for unknown formats
        },
        None => "output.png",
    };
    // resized_img.save(output_path)?;

    Ok(())
}
