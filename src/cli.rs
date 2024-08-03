use clap::{error::ErrorKind, Arg, Command, Error};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("Image Resizer")
        .version("1.0")
        .about("Resizes images based on provided dimensions")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Path to the input image")
                .required(true)
                .value_parser(value_parser_for_path),
        )
        .arg(
            Arg::new("width")
                .short('W')
                .long("width")
                .help("New width of the image")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .help("New height of the image")
                .value_parser(clap::value_parser!(u32)),
        )
}

fn value_parser_for_path(p: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(p);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        // Clap custom error
        Err(Error::raw(
            ErrorKind::InvalidValue,
            format!("The path '{}' does not exist or is not a file.", p),
        ))
    }
}
