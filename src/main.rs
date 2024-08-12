//! Image Resizer
//!
//! This program resizes images based on command-line arguments.
//! It uses the `image_resizer_rust` library for image processing operations
//! (which in turn uses `fast_image_resize` for efficient resizing),
//! and `clap` for parsing command-line arguments.

mod cli;

use clap::error::ErrorKind;
use image_resizer_rust::{
    check_if_path_exists, determine_save_format_and_path, resize_image, save_image,
};
use std::path::PathBuf;

/// The main entry point of the image resizer program.
///
/// This function calls the `run` function and handles any errors that occur.
/// If an error is encountered, it uses `clap`'s error handling mechanism to
/// display the error message and exit the program with a non-zero status code.
fn main() {
    if let Err(e) = run() {
        let mut cmd = cli::cli();
        cmd.error(clap::error::ErrorKind::InvalidValue, e).exit();
    }
}

/// The core logic of the image resizer program.
///
/// This function orchestrates the image resizing process by:
/// 1. Parsing command-line arguments
/// 2. Validating input parameters
/// 3. Determining the output path
/// 4. Loading and resizing the input image
/// 5. Determining the save format and final output path
/// 6. Checking if the output path already exists
/// 7. Saving the resized image
/// 8. Printing information about the saved image
///
/// It supports resizing images while maintaining aspect ratio and
/// allows specifying output format (JPEG or PNG).
///
/// # Errors
///
/// Returns an error if:
/// - Neither width nor height is specified
/// - The input file cannot be read or is not a valid image
/// - The output path cannot be determined
/// - The resizing operation fails
/// - The output format cannot be determined
/// - The output path already exists (to prevent accidental overwriting)
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
fn run() -> Result<(), Box<dyn std::error::Error>> {
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

    let output_path = cli::determine_output_path(input, output)?;
    let new_format = matches.get_one::<String>("format");

    let img = image::ImageReader::open(input)?.decode()?;
    let resized_img = resize_image(img, width, height)?;

    let (save_format, new_output) =
        determine_save_format_and_path(&resized_img, &output_path, new_format)?;
    check_if_path_exists(&new_output)?;

    let save_info = save_image(resized_img, &new_output, save_format)?;

    println!("Image resized and saved!");
    println!("New dimensions: {}x{}", save_info.width, save_info.height);
    println!("Format: {:?}", save_info.format);
    println!("Output path: {:?}", save_info.path);

    Ok(())
}
