use std::io;
use std::io::ErrorKind::Unsupported;
// use std::sync::Mutex;
use crate::image::{Direction, Image};
use image::{imageops::FilterType::Nearest, DynamicImage, ImageError, ImageFormat};
use rustler::{error, NifUnitEnum};

// use log::trace;

/// Public enum/atom representing the different image operations that can be performed.
#[allow(clippy::non_snake_case)]
#[derive(NifUnitEnum, Debug, Copy, Clone)]
pub enum Operation {
    Blur,
    Brighten,
    FlipHorizontal,
    FlipVertical,
    Greyscale,
    Resize,
    Thumbnail,
    Rotate,
}

pub fn perform(
    operation: Operation,
    width: f32,
    height: f32,
    extension: String,
    buffer: &[u8],
) -> Result<Image, error::Error> {
    let transform = match operation {
        Operation::Resize => resize(&extension, buffer, width as u32, height as u32),
        Operation::Thumbnail => thumbnail(&extension, buffer, width as u32, height as u32),
        Operation::FlipHorizontal => flip(&extension, buffer, Direction::Horizontal),
        Operation::FlipVertical => flip(&extension, buffer, Direction::Vertical),
        Operation::Rotate => rotate(&extension, buffer, width as i32),
        Operation::Blur => blur(&extension, buffer, width as f32),
        Operation::Brighten => brighten(&extension, buffer, width as i32),
        Operation::Greyscale => greyscale(&extension, buffer),
    };

    match transform {
        Ok((image, format)) => Image::build(image, &extension, format),
        Err(_err) => Err(rustler::Error::BadArg),
    }
}

/// Tries to resize the given image with the specified dimensions.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.resize
pub fn resize<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    width: u32,
    height: u32,
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.resize_to_fill(width, height, Nearest);

    Ok((img, format))
}

/// Tries to create a thumbnail of the given image with the specified dimensions.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.thumbnail
pub fn thumbnail<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    nwidth: u32,
    nheight: u32,
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.thumbnail(nwidth, nheight);

    Ok((img, format))
}

/// Tries to flip the given image in the specified direction.
/// either veritcal or horizontal.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.flipv
pub fn flip<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    direction: Direction,
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = match direction {
        Direction::Vertical => img.flipv(),
        Direction::Horizontal => img.fliph(),
    };

    Ok((img, format))
}

/// Tries to rotate the given image by the specified angle, 90, 180, or 270.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.rotate90
pub fn rotate<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    angle: i32,
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = match angle {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img.huerotate(angle),
    };

    Ok((img, format))
}

/// Tries to blur the given image with the specified sigma value.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.blur
pub fn blur<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    sigma: f32,
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.blur(sigma);

    Ok((img, format))
}

/// Tries to brighten the given image with the specified value.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.brighten
pub fn brighten<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    value: i32,
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.brighten(value);

    Ok((img, format))
}

/// Tries to convert the given image to greyscale.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.grayscale
pub fn greyscale<'a>(
    extension: &'a str,
    buffer: &'a [u8],
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.grayscale();

    Ok((img, format))
}

// Compress an entire known block of bytes into a compressed .gz format
// NOTE: This requires reading the entire input into memory.
pub fn block_compress<'a>(buffer: &'a [u8]) -> Result<Vec<u8>, snap::Error> {
    snap::raw::Encoder::new().compress_vec(&buffer)
}

// Decompress an entire known block of bytes back into its native format
// NOTE: This requires reading the entire input into memory.
pub fn block_decompress<'a>(buffer: &'a [u8]) -> Result<Vec<u8>, snap::Error> {
    snap::raw::Decoder::new().decompress_vec(&buffer)
}

// TODO: Streaming api with network file handles
// pub fn stream_compress<'a>(buffer: &'a [u8]) {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_rotate() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();
        let (img, format) = rotate("png", &buffer, 90).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_blur() {
        let buffer = fs::read("../../example/abra.png").unwrap();
        let (img, format) = blur("png", &buffer, 1.0).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_brighten() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();
        let (img, format) = brighten("png", &buffer, 10).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_greyscale() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();
        let (img, format) = greyscale("png", &buffer).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_block_compress_and_decompress() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();

        let compressed = block_compress(&buffer).expect("it compresses an image bytes");
        assert_ne!(buffer, compressed);

        let decompressed = block_decompress(&compressed).expect("it decompresses an image bytes");
        assert_eq!(buffer, decompressed);
    }
}
