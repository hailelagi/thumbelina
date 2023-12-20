use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use rustler::{Error, NifStruct};
use std::io::{BufReader, Cursor};

// A freshly allocated Image struct
// built from the operations on the src slice in the BEAM
#[derive(Debug, NifStruct)]
#[module = "Thumbelina.Image"]
pub struct Image {
    pub extension: String,
    pub height: u32,
    pub width: u32,
    pub bytes: Vec<u8>,
    pub compressed: bool,
}

impl<'a> Image {
    fn new(extension: &'a str, image: &DynamicImage, bytes: Vec<u8>) -> Self {
        return Image {
            extension: String::from(extension),
            height: image.height(),
            width: image.width(),
            bytes,
            compressed: false,
        };
    }

    pub fn new_raw(bytes: Vec<u8>) -> Self {
        return Image {
            extension: String::from(".gz"),
            height: 0,
            width: 0,
            bytes,
            compressed: true,
        };
    }

    pub fn from_compressed(bytes: Vec<u8>) -> Result<Image, image::ImageError> {
        let reader = BufReader::new(Cursor::new(&bytes));
        let image = ImageReader::new(reader).with_guessed_format()?.decode()?;

        return Ok(Image {
            extension: String::from(".gz"),
            height: image.height(),
            width: image.width(),
            bytes,
            compressed: false,
        });
    }

    pub fn build(
        image: DynamicImage,
        extension: &'a str,
        format: ImageFormat,
    ) -> Result<Image, Error> {
        let mut result = Cursor::new(Vec::new());

        match image.write_to(&mut result, format) {
            Ok(_) => Ok(Image::new(extension, &image, result.get_ref().to_owned())),
            Err(_) => Err(Error::BadArg),
        }
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn test_build_image() {
        let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(100, 100);
        let dynamic_image = DynamicImage::ImageRgba8(rgba_image);
        let image = Image::build(dynamic_image, "png", ImageFormat::Png);
        assert!(image.is_ok());
    }

    #[test]
    fn test_build_image_with_empty_bytes() {
        let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(100, 100);
        let dynamic_image = DynamicImage::ImageRgba8(rgba_image);
        let bytes = vec![];
        let image = Image::new("png", &dynamic_image, bytes);
        assert_eq!(image.bytes.len(), 0);
    }

    #[test]
    fn test_build_image_with_large_bytes() {
        let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(100, 100);
        let dynamic_image = DynamicImage::ImageRgba8(rgba_image);
        let bytes = vec![0; 1000000];
        let image = Image::new("png", &dynamic_image, bytes);
        assert_eq!(image.bytes.len(), 1000000);
    }

    #[test]
    fn test_build_image_with_dimensions() {
        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let image = DynamicImage::ImageRgba8(buffer);
        let extension = "png";
        let format = ImageFormat::Png;

        let result = Image::build(image, extension, format).unwrap();

        assert_eq!(result.extension, extension);
        assert_eq!(result.height, 100);
        assert_eq!(result.width, 100);
        assert_eq!(result.bytes.len(), 265);
        assert_eq!(result.bytes[0], 137);
    }
}
