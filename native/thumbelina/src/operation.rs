use std::io;
use std::io::ErrorKind::Unsupported;
use std::sync::Mutex;
use image::{imageops::FilterType::Nearest, DynamicImage, ImageFormat, ImageError};
use rustler::{NifUnitEnum, LocalPid};
use crate::image::Direction;
// use log::trace;

/// Public enum representing the different image operations that can be performed.
#[derive(NifUnitEnum)]
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

/// Tries to resize the given image with the specified dimensions.
/// https://docs.rs/image/latest/image/enum.DynamicImage.html#method.resize
pub fn try_resize<'a>(
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
pub fn try_thumbnail<'a>(
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
pub fn try_flip<'a>(
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
pub fn try_rotate<'a>(
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
pub fn try_blur<'a>(
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
pub fn try_brighten<'a>(
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
pub fn try_greyscale<'a>(
    extension: &'a str,
    buffer: &'a [u8],
) -> Result<(DynamicImage, ImageFormat), ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.grayscale();

    Ok((img, format))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_try_rotate() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();
        let (img, format) = try_rotate("png", &buffer, 90).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_try_blur() {
        let buffer = fs::read("../../example/abra.png").unwrap();
        let (img, format) = try_blur("png", &buffer, 1.0).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_try_brighten() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();
        let (img, format) = try_brighten("png", &buffer, 10).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }

    #[test]
    fn test_try_greyscale() {
        let buffer = fs::read("../../example/abra.png").unwrap().clone();
        let (img, format) = try_greyscale("png", &buffer).unwrap();
        assert_eq!(format, ImageFormat::Png);
        assert_ne!(img.as_bytes(), buffer);
    }
    
}