//! Image Resizer
//!
//! This program resizes images based on command-line arguments.
//! It uses the `image_resizer_rust` library for image processing operations
//! (which in turn uses `fast_image_resize` for efficient resizing),
//! and `clap` for parsing command-line arguments.

mod cli;

use clap::error::ErrorKind;
use image_resizer_rust::{resize_image, save_image};
use std::path::PathBuf;

/// The main function of the image resizer program.
///
/// This function orchestrates the image resizing process by:
/// 1. Parsing command-line arguments using the `cli` module
/// 2. Resizing the input image using the `resize_image` function
/// 3. Determining the output path
/// 4. Saving the resized image using the `save_image` function
///
/// It supports resizing images while maintaining aspect ratio and
/// allows specifying output format (JPEG or PNG).
///
/// # Errors
///
/// Returns an error if:
/// - Neither width nor height is specified
/// - The input file cannot be read or is not a valid image
/// - The resizing operation fails
/// - The output path cannot be determined
/// - The resized image cannot be saved
///
/// # Example
///
/// ```
/// image-resizer-rust input.jpg -w 800 -o resized.png
/// ```
///
/// This example resizes 'input.jpg' to a width of 800 pixels (maintaining aspect ratio)
/// and saves it as 'resized.png'.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap();
    let output = matches.get_one::<String>("output").cloned();
    let width = matches.get_one::<u32>("width").copied();
    let height = matches.get_one::<u32>("height").copied();

    if width.is_none() && height.is_none() {
        let err = cli::cli().error(
            ErrorKind::InvalidValue,
            "At least one of --width or --height must be specified.",
        );
        err.exit();
    }

    let format = matches.get_one::<String>("format");

    let (resized_img, _) = resize_image(input, width, height)?;

    let output_path = cli::determine_output_path(input, output)?;

    let save_info = save_image(resized_img, &output_path, format)?;

    println!("Image resized and saved!");
    println!("New dimensions: {}x{}", save_info.width, save_info.height);
    println!("Format: {:?}", save_info.format);
    println!("Output path: {:?}", save_info.path);

    Ok(())
}
