use image::{imageops::FilterType::Nearest, ImageFormat};
use rustler::{Atom, Binary, Encoder, Env, Error, NifResult, NifStruct};

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
pub fn serialize<'a, 's>(
    env: Env<'a>,
    extension: &'a str,
    path: String,
    bin: Binary<'a>,
) -> NifResult<(Atom, (ImageMetadata, Binary))> {
    let mut resource = bin
        .to_owned()
        .ok_or(Error::Term(Box::new("uh oh! this binary is invalid")))?;

    let format = match extension {
        ".png" => ImageFormat::Png,
        ".jpg" | ".jpeg" => ImageFormat::Jpeg,
        ".webp" => ImageFormat::WebP,
        ".gif" => ImageFormat::Gif,
        _ => ImageFormat::Png,
    };

    let img_buffer = resource.as_mut();
    // FIXME: binary data is zeroed? wtf?
    let img = image::load_from_memory_with_format(img_buffer, format).unwrap();
    let img = img.resize(100, 100, Nearest);

    let meta = ImageMetadata {
        extension: String::from(extension),
        path,
        height: img.height(),
        width: img.width(),
    };

    match Binary::from_term(img.into_bytes().encode(env)) {
        Ok(bin) => Ok((atoms::ok(), (meta, bin))),
        Err(err) => Err(err),
    }
}

// #[rustler::nif(schedule = "DirtyCpu")]
// pub fn serialize_dirty<'a, 's>(
//     env: Env<'a>,
//     extension: &'a str,
//     path: String,
//     bin: Binary<'a>,
// ) -> NifResult<(Atom, (ImageMetadata, Binary))> {

//     match image::load_from_memory(bin.as_slice()) {
//         Ok(image) => {
//             let image = ImageMetadata {
//                 extension: opts.extension,
//                 path: opts.path,
//                 height: image.height(),
//                 width: image.width(),
//                 bytes: bin,
//                 size: bin.len(),
//             };

//             return Ok((atoms::ok(), image));
//         }

//         Err(_) => Err(Error::BadArg),
//     }
// }

/*
impl From<ImageError> for Error {

}
*/
