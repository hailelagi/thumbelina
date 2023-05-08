use image::{imageops::FilterType::Nearest, ImageFormat};
use rustler::{Atom, Binary, Error, NifResult, NifStruct};
use std::io::Cursor;

mod atoms {
    rustler::atoms! {ok, error, png, jpeg, svg}
}

#[derive(NifStruct)]
#[module = "Thumbelina.Image"]
pub struct ImageMetadata {
    pub extension: String,
    pub path: String,
    pub height: u32,
    pub width: u32,
}

#[rustler::nif]
pub fn serialize<'a>(
    extension: &'a str,
    path: String,
    bin: Binary<'a>,
    width: i32,
    height: i32
) -> NifResult<(Atom, (ImageMetadata, Vec<u8>))> {
    let format = match extension {
        ".png" => ImageFormat::Png,
        ".jpg" | ".jpeg" => ImageFormat::Jpeg,
        ".webp" => ImageFormat::WebP,
        ".gif" => ImageFormat::Gif,
        // todo: non-exhaustive
        _ => ImageFormat::Png,
    };

    let img_buffer = bin.as_slice();
    let img = image::load_from_memory_with_format(img_buffer, format).unwrap();
    let img = img.resize_to_fill(width, height, Nearest);

    let meta = ImageMetadata {
        extension: String::from(extension),
        path,
        height: img.height(),
        width: img.width(),
    };

    let mut result = Cursor::new(Vec::new());

    match img.write_to(&mut result, format) {
        Ok(_) => Ok((atoms::ok(), (meta, result.get_ref().to_owned()))),
        Err(_) => Err(Error::BadArg),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn serialize_dirty<'a, 's>(
    env: Env<'a>,
    extension: &'a str,
    path: String,
    bin: Binary<'a>,
) -> NifResult<(Atom, (ImageMetadata))> {

    match image::load_from_memory(bin.as_slice()) {
        Ok(image) => {
            let image = ImageMetadata {
                extension: opts.extension,
                path: opts.path,
                height: image.height(),
                width: image.width(),
                bytes: bin,
                size: bin.len(),
            };

            return Ok((image));
        }

        Err(_) => Err(Error::BadArg),
    }
}

/*
impl From<ImageError> for Error {

}
*/
