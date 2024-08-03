mod cli;

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();

    let input = matches.get_one::<PathBuf>("input").unwrap();
    println!("The input given was: '{:?}'", input);

    let width = matches.get_one::<u32>("width");
    let height = matches.get_one::<u32>("height");

    if width.is_none() && height.is_none() {
        return Err("At least one of width or height must be specified".into());
    }

    let format = matches.get_one::<String>("format");

    println!(
        "Resizing image {:?} with new width {:?}, new height {:?}, and format {}.",
        input,
        width.unwrap_or(&0),
        height.unwrap_or(&0),
        format.unwrap()
    );

    Ok(())
}
