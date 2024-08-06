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
    use tempfile::TempDir;

    fn create_temp_dir() -> TempDir {
        TempDir::new().expect("Failed to create temp directory")
    }

    mod determine_output_path_tests {
        use super::*;
        use std::process::Command;

        #[test]
        fn with_output() {
            let input = PathBuf::from("/path/to/input.jpg");
            let output = Some(String::from("output.png"));
            let result = determine_output_path(&input, output).unwrap();
            assert_eq!(result, Path::new("/path/to/output.png"));
        }

        #[test]
        fn without_output() {
            let input = PathBuf::from("/path/to/input.jpg");
            let result = determine_output_path(&input, None).unwrap();
            assert_eq!(result, Path::new("/path/to/input_resized.jpg"));
        }

        #[test]
        fn with_absolute_output() {
            let temp_dir = create_temp_dir();
            let input = temp_dir.path().join("input.jpg");
            let output = temp_dir.path().join("output.png");

            let result =
                determine_output_path(&input, Some(output.to_string_lossy().to_string())).unwrap();
            assert_eq!(result, output);
            assert!(result.is_absolute());
            assert_eq!(result.extension().unwrap(), "png");
            assert_ne!(result, input);
        }

        #[test]
        fn with_current_dir() {
            let input = PathBuf::from("/path/to/input.jpg");
            let output = Some(String::from("./output.png"));
            let result = determine_output_path(&input, output).unwrap();
            assert_eq!(result, Path::new("/path/to/output.png"));
        }

        #[test]
        fn from_shell_relative() {
            let input = Path::new("~/Downloads/shell_output.jpeg");
            let output = Some(String::from("./output"));

            let cmd_output = Command::new("echo")
                .arg(input.to_string_lossy().to_string())
                .output()
                .expect("Failed to execute command");

            let shell_path = String::from_utf8(cmd_output.stdout)
                .unwrap()
                .trim()
                .to_string();

            let result = determine_output_path(Path::new(&shell_path), output);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Path::new("~/Downloads/output"));
        }
    }

    mod validate_output_path_tests {
        use super::*;
        use std::process::Command;

        #[test]
        fn valid_path() {
            let temp_dir = create_temp_dir();
            let path = temp_dir.path().join("output.jpeg");
            let result = validate_output_path(&path.to_string_lossy().to_string());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), path.to_string_lossy().to_string());
        }

        #[test]
        fn invalid_extension() {
            let path = String::from("/tmp/output.gif");
            let result = validate_output_path(&path);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "You need to specify a valid extension, either jpeg, png or no extension."
            );
        }

        #[test]
        fn nonexistent_directory() {
            let path = String::from("/nonexistent/directory/output.jpeg");
            let result = validate_output_path(&path);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be found"));
        }

        #[test]
        fn from_shell() {
            let temp_dir = create_temp_dir();
            let path = temp_dir.path().join("shell_output.jpeg");

            let output = Command::new("echo")
                .arg(path.to_string_lossy().to_string())
                .output()
                .expect("Failed to execute command");

            let shell_path = String::from_utf8(output.stdout).unwrap().trim().to_string();

            let result = validate_output_path(&shell_path);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), path.to_string_lossy().to_string());
        }
    }

    mod value_parser_for_path_test {
        use super::*;
        use std::fs::File;
        use std::process::Command;

        #[test]
        fn from_shell() {
            let temp_dir = create_temp_dir();
            let path = temp_dir.path().join("shell_output.jpeg");

            // Create the file
            File::create(&path).expect("Failed to create test file");

            let output = Command::new("echo")
                .arg(path.to_string_lossy().to_string())
                .output()
                .expect("Failed to execute command");

            let shell_path = String::from_utf8(output.stdout).unwrap().trim().to_string();

            let result = value_parser_for_path(&shell_path);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), path);
        }

        #[test]
        fn nonexistent_file() {
            let temp_dir = create_temp_dir();
            let nonexistent_path = temp_dir.path().join("nonexistent.jpeg");

            let result = value_parser_for_path(nonexistent_path.to_str().unwrap());
            assert!(result.is_err());

            if let Err(err) = result {
                assert_eq!(err.kind(), ErrorKind::InvalidValue);
                assert!(err.to_string().contains("does not exist or is not a file"));
            } else {
                panic!("Expected an error, but got Ok");
            }
        }
    }
}
