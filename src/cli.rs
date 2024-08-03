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
                // .value_parser(clap::value_parser!(PathBuf)),
                .value_parser(value_parser_for_path),
        )
}

// fn value_parser_for_path(p: &str) -> Result<PathBuf, io::Error> {
fn value_parser_for_path(p: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(p);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        // io Error
        // Err(io::Error::new(
        //     ErrorKind::NotFound,
        //     "The provided path does not exist or is not a file.",
        // ))

        // Clap error
        Err(Error::raw(
            ErrorKind::InvalidValue,
            format!("The path '{}' does not exist or is not a file.", p),
        ))
    }
}
