use image::{DynamicImage, ImageFormat};
use rustler::{Error, NifStruct};
use std::io::Cursor;

#[derive(NifStruct)]
#[module = "Thumbelina.Image"]
pub struct Image {
    pub extension: String,
    pub height: u32,
    pub width: u32,
    pub bytes: Vec<u8>,
}

impl<'a> Image {
    fn new(extension: &'a str, image: &DynamicImage, bytes: Vec<u8>) -> Self {
        return Image {
            extension: String::from(extension),
            height: image.height(),
            width: image.width(),
            bytes,
        };
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

// pub struct ImageMetadata {
//     path: String,
//     source: Source,
//     // fn write to dest

//     // fn modify_inplace(extension, path, source, height, width) -> Self {
//     //     thumbelina::Image{e}
//     // }
// }

// pub enum Source {
//     Disk,
//     InMemory,
// }
