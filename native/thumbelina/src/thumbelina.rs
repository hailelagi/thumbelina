use rustler::{Atom, Binary, Error, NifResult, NifStruct};
//use rayon::prelude::*;
use image::{imageops::FilterType::Nearest, DynamicImage, ImageFormat};
use io::Cursor;
use std::io;

mod atoms {
    rustler::atoms! {ok, error, png, jpeg, svg}
}

#[derive(NifStruct)]
#[module = "Thumbelina.Image"]
pub struct Image {
    pub extension: String,
    pub height: u32,
    pub width: u32,
}

impl<'a> Image {
    fn new(extension: &'a str, image: &DynamicImage) -> Self {
        return Image {
            extension: String::from(extension),
            height: image.height(),
            width: image.width(),
        };
    }
}

// pub struct ImageMetadata {
//     path: String,
//     source: Source,
//     // fn write to dest

//     // fn modify_inplace(extension, path, source, height, width) -> Self {
//     //     thumbelina::Image{e}
//     // }
// }

// enum Source {
//     Disk,
//     InMemory,
// }

#[rustler::nif]
pub fn resize<'a>(
    extension: &'a str,
    bin: Binary<'a>,
    width: u32,
    height: u32,
) -> NifResult<(Atom, (Image, Vec<u8>))> {
    let buffer = bin.as_slice();
    match try_resize(extension, buffer, width, height) {
        Ok((image, format)) => {
            let mut result = Cursor::new(Vec::new());
            let thumbelina_image = Image::new(extension, &image);

            match image.write_to(&mut result, format) {
                Ok(_) => Ok((atoms::ok(), (thumbelina_image, result.get_ref().to_owned()))),
                Err(_) => Err(Error::BadArg),
            }
        }
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

fn try_resize<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    width: u32,
    height: u32,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension).ok_or(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "invalid format provided",
    ))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.resize_to_fill(width, height, Nearest);

    Ok((img, format))
}

// #[rustler::nif]
// pub fn resize_all<'a>(
//     images: Vec<thumbelina::Image>,
//     width: u32,
//     height: u32,
// ) -> NifResult<(Atom, (thumbelina::Image, Vec<u8>))> {
//     let images: Vec<_> = images
//         .par_iter()
//         .map(|image| alter_image(image.path, 300, 500))
//         .filter_map(|x| x.err())
//         .collect();
// }

// fn alter_dimensions(img: &mut DynamicImage, format: ImageFormat) -> Result<()> {
//     let img_buffer = bin.as_slice();

//     let img = image::load_from_memory_with_format(img_buffer, format).unwrap();
//     let img = img.resize_to_fill(width, height, Nearest);

//     let mut result = Cursor::new(Vec::new());

//     match img.write_to(&mut result, format) {
//         Ok(_) => Ok((atoms::ok(), (image, result.get_ref().to_owned()))),
//         Err(_) => Err(Error::BadArg),
//     }
// }

// #[rustler::nif(schedule = "DirtyCpu")]
// pub fn serialize_dirty<'a, 's>(
//     env: Env<'a>,
//     extension: &'a str,
//     path: String,
//     bin: Binary<'a>,
// ) -> NifResult<(Atom, (thumbelina::Image))> {

//     match image::load_from_memory(bin.as_slice()) {
//         Ok(image) => {
//             let image = thumbelina::Image {
//                 extension: opts.extension,
//                 path: opts.path,
//                 height: image.height(),
//                 width: image.width(),
//                 bytes: bin,
//                 size: bin.len(),
//             };

//             return Ok((image));
//         }

//         Err(_) => Err(Error::BadArg),
//     }
// }

// #[rustler::nif]
// pub fn serialize<'a>(
//     extension: &'a str,
//     path: String,
//     bin: Binary<'a>,
//     width: i32,
//     height: i32
// ) -> NifResult<(Atom, (thumbelina::Image, Vec<u8>))> {
//     let format = match extension {
//         ".png" => ImageFormat::Png,
//         ".jpg" | ".jpeg" => ImageFormat::Jpeg,
//         ".webp" => ImageFormat::WebP,
//         ".gif" => ImageFormat::Gif,
//         // todo: non-exhaustive
//         _ => ImageFormat::Png,
//     };

//     let img_buffer = bin.as_slice();
//     let img = image::load_from_memory_with_format(img_buffer, format).unwrap();
//     let img = img.resize_to_fill(width, height, Nearest);

//     let image = thumbelina::Image {
//         extension: String::from(extension),
//         path,
//         height: img.height(),
//         width: img.width(),
//     };

//     let mut result = Cursor::new(Vec::new());

//     match img.write_to(&mut result, format) {
//         Ok(_) => Ok((atoms::ok(), (image, result.get_ref().to_owned()))),
//         Err(_) => Err(Error::BadArg),
//     }
// }
