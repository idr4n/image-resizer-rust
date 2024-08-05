mod cli;

use std::path::PathBuf;

use image_resizer_rust::{resize_image, save_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap();
    let output = matches.get_one::<PathBuf>("output").cloned();
    let width = matches.get_one::<u32>("width").copied();
    let height = matches.get_one::<u32>("height").copied();

    // this is also checked by resize_image(), so not really needed here.
    if width.is_none() && height.is_none() {
        return Err("At least one of width or height must be specified".into());
    }

    let format = matches.get_one::<String>("format");

    let resized_img = resize_image(input, width, height)?;

    // save_image(resized_img, format)?;

    let output_path = cli::determine_output_path(input, output);

    println!("output path: {:?}", output_path);

    Ok(())
}
