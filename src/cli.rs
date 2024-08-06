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
                .help("Absolute or relative path including new image name.\nIf only a name is provide (e.g. output.), then the directory of the input image will be used.")
                .required(false)
                .value_parser(value_parser!(String)),
        )
}

pub fn determine_output_path(
    input: &Path,
    output: Option<String>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let parent = input.parent().unwrap_or(Path::new(""));
    let stem = input.file_stem().unwrap_or(OsStr::new("output"));
    let extension = input.extension().unwrap_or(OsStr::new("jpeg"));

    match output {
        Some(output_path) => {
            let validated_output = validate_output_path(&output_path)?;
            let path_new = Path::new(&validated_output);
            if path_new.is_absolute() {
                // println!("Path is absolute");
                Ok(path_new.to_path_buf())
            } else {
                Ok(parent.join(output_path))
            }
        }
        None => {
            let new_stem = format!("{}_resized", stem.to_string_lossy());
            Ok(parent
                .join(PathBuf::from(new_stem))
                .with_extension(extension))
        }
    }
}

fn validate_output_path(path: &String) -> Result<String, Box<dyn std::error::Error>> {
    let parent = Path::new(&path).parent().unwrap_or(Path::new(""));

    if !parent.is_dir() && parent != Path::new("") {
        return Err(format!("The given output directory {:?} cannot be found.", parent).into());
    };

    let stem = Path::new(&path).file_stem().unwrap_or(OsStr::new("output"));
    let extension = Path::new(&path).extension().unwrap_or(OsStr::new(""));

    match extension.to_str() {
        Some("jpeg") | Some("jpg") | Some("png") | Some("") => {
            let validated_path = parent.join(stem).with_extension(extension);
            Ok(validated_path.to_string_lossy().to_string())
        }
        _ => Err("You need to specify a valid extension, either jpeg, png or no extension.".into()),
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
