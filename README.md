# Image-Resizer-Rust

Image-Resizer-Rust is a command-line tool for efficiently resizing images. It supports JPEG and PNG formats, with automatic format detection and conversion.

Please note that this command-line tool is intended for personal use and is not published on Rust's docs.rs or crates.io. Therefore, it must be installed manually.

## Features

- Resize images by specifying width, height, or both
- Maintain aspect ratio when resizing
- Support for JPEG and PNG formats
- Automatic format detection and conversion
- Efficient resizing using the `fast_image_resize` library

## Main Dependencies

- `clap`: For parsing command-line arguments
- `image`: For reading and writing various image formats
- `fast_image_resize`: For efficient image resizing operations

## Installation

To install, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/idr4n/image-resizer-rust.git
   cd image-resizer-rust
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The executable will be available in the `target/release` directory.

## Usage

To use Image-Resizer-Rust, run the following command:

```
image-resizer-rust <input_file> [OPTIONS]
```

### Options

- `-W, --width <WIDTH>`: New width of the image. Required if --height not provided.
- `-H, --height <HEIGHT>`: New height of the image. Required if --width not provided.
- `-F, --format <FORMAT>`: Specify the image format (jpeg or png)
- `-o, --output <OUTPUT>`: Absolute or relative path including new image name. If only a name is provided (e.g. output.jpg), then the directory of the input image will be used.

You must specify at least one of `--width` or `--height`. If only one dimension is provided, the other will be calculated to maintain the aspect ratio.

### Examples

1. Resize an image to a width of 800 pixels, maintaining the aspect ratio:
   ```
   image-resizer-rust input.jpg -W 800 -o resized.jpg
   ```

2. Resize an image to a height of 600 pixels and convert it to PNG:
   ```
   image-resizer-rust input.jpg -H 600 -F png -o resized.png
   ```

3. Resize an image to 1024x768 pixels:
   ```
   image-resizer-rust input.png -W 1024 -H 768 -o resized.png
   ```

## Error Handling

The application provides informative error messages for various scenarios, such as:

- Invalid input file format
- Nonexistent input or output directories
- Unsupported output formats

## Development

To run tests:

```
cargo test
```

To format the code:

```
cargo fmt
```

To run linter:

```
cargo clippy
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
