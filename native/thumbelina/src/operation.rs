use std::io;
use std::io::ErrorKind::Unsupported;
use image::{imageops::FilterType::Nearest, DynamicImage, ImageFormat, ImageError};
use rustler::NifUnitEnum;
use crate::image::Direction;

/// Enum representing the different image operations that can be performed.
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
///
/// # Arguments
///
/// * `extension` - A string slice that holds the file extension of the image.
/// * `buffer` - A byte slice that holds the image data.
/// * `nwidth` - The width of the thumbnail.
/// * `nheight` - The height of the thumbnail.
///
/// # Returns
///
/// A `Result` containing a tuple of the thumbnail image and its format, or an `ImageError` if the operation fails.
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
///
/// # Arguments
///
/// * `extension` - A string slice that holds the file extension of the image.
/// * `buffer` - A byte slice that holds the image data.
/// * `direction` - The direction in which to flip the image.
///
/// # Returns
///
/// A `Result` containing a tuple of the flipped image and its format, or an `ImageError` if the operation fails.
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

/// Tries to rotate the given image by the specified angle.
///
/// # Arguments
///
/// * `extension` - A string slice that holds the file extension of the image.
/// * `buffer` - A byte slice that holds the image data.
/// * `angle` - The angle by which to rotate the image.
///
/// # Returns
///
/// A `Result` containing a tuple of the rotated image and its format, or an `ImageError` if the operation fails.
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
///
/// # Arguments
///
/// * `extension` - A string slice that holds the file extension of the image.
/// * `buffer` - A byte slice that holds the image data.
/// * `sigma` - The sigma value to use for the blur operation.
///
/// # Returns
///
/// A `Result` containing a tuple of the blurred image and its format, or an `ImageError` if the operation fails.
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
///
/// # Arguments
///
/// * `extension` - A string slice that holds the file extension of the image.
/// * `buffer` - A byte slice that holds the image data.
/// * `value` - The value to use for the brighten operation.
///
/// # Returns
///
/// A `Result` containing a tuple of the brightened image and its format, or an `ImageError` if the operation fails.
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
///
/// # Arguments 
///
/// * `extension` - A string slice that holds the file extension of the image.
/// * `buffer` - A byte slice that holds the image data.
///
/// # Returns
///
/// A `Result` containing a tuple of the greyscaled image and its format, or an `ImageError` if the operation fails.
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
    use image::GenericImageView;

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