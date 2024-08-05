use clap::{error::ErrorKind, value_parser, Arg, Command, Error};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

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
        .arg(
            Arg::new("format")
                .short('F')
                .long("format")
                .help("Specify the image format")
                .value_parser(["jpeg", "png"])
                .default_value("jpeg"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Absolute path or relative to the input image.\nIf extension is not provided, it will be inferred from the image or determined by the --format flag")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
}

pub fn determine_output_path(input: &Path, output: Option<PathBuf>) -> PathBuf {
    let parent = input.parent().unwrap_or(Path::new(""));
    let stem = input.file_stem().unwrap_or(OsStr::new("output"));
    let extension = input.extension().unwrap_or(OsStr::new(""));

    match output {
        Some(path) => {
            if path.is_absolute() {
                println!("Path is absolute");
                path
            } else {
                parent.join(path)
            }
        }
        None => {
            let new_stem = format!("{}_resized", stem.to_string_lossy());
            parent
                .join(PathBuf::from(new_stem))
                .with_extension(extension)
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_determine_output_path_with_output() {
        let input = PathBuf::from("/path/to/input.jpg");
        let output = Some(PathBuf::from("output.png"));
        let result = determine_output_path(&input, output);
        assert_eq!(result, Path::new("/path/to/output.png"));
    }

    #[test]
    fn test_determine_output_path_without_output() {
        let input = PathBuf::from("/path/to/input.jpg");
        let result = determine_output_path(&input, None);
        assert_eq!(result, Path::new("/path/to/input_resized.jpg"));
    }

    #[test]
    fn test_determine_output_path_with_absolute_output() {
        let input = PathBuf::from("/path/to/input.jpg");
        let output = Some(PathBuf::from("/absolute/path/output.png"));
        let result = determine_output_path(&input, output);
        assert_eq!(result, Path::new("/absolute/path/output.png"));
    }

    #[test]
    fn test_determine_output_path_with_current_dir() {
        let input = PathBuf::from("/path/to/input.jpg");
        let output = Some(PathBuf::from("./output.png"));
        let result = determine_output_path(&input, output);
        assert_eq!(result, Path::new("/path/to/output.png"));
    }
}
