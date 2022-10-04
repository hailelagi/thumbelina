use image::{io::Reader as ImageReader, imageops::FilterType::Nearest};
use rustler::{resource, Binary, Env, Error, NifResult, NifStruct, OwnedBinary};
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
    pub size: usize,
    // TODO: figure out how to embed the byte data into the return struct
    // pub resource: Binary<u8>
}

#[rustler::nif]
pub fn serialize<'a>(
    env: Env<'a>,
    extension: String,
    path: String,
    bin: Binary<'a>,
) -> Result<(Binary<'a>, ImageMetadata), Error> {
    // it works?
    let mut resource = bin.to_owned().ok_or(Error::Term(Box::new("uh oh! :(")))?;

    // FIXME: unnecessary double move
    let img = ImageReader::new(Cursor::new(resource.to_owned()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
        // TODO: impl error propagation traits

    let img = img.resize(50, 25, Nearest);

    // let image = ImageReader::new(Cursor::new(bin)).decode()?;
    let meta = ImageMetadata {
        extension,
        path,
        height: img.height(),
        width: img.width(),
        size: img.into_bytes().capacity(),
    };

    // TODO: serialise Vec<u8> to <<u8>>
    Ok((Binary::to_term(img.into_bytes()), meta))
}

// #[rustler::nif(schedule = "DirtyCpu")]
// pub fn serialize_dirty(opts: Options, bin: Binary) -> Result<(Atom, ThumbelinaImage), Error> {
//     match image::load_from_memory(bin.as_slice()) {
//         Ok(image) => {
//                 let image = ThumbelinaImage {
//                     extension: opts.extension,
//                     path: opts.path,
//                     height: image.height(),
//                     width: image.width(),
//                     bytes: bin,
//                     size: bin.len()
//                 };

//                 return Ok((ok(), image));
//             }

//         Err(_) => Err(Error::BadArg)
//     }
// }

// todo: Serialize image-rs Errors to ruster::Error
/*
impl From<ImageError> for Error {

}
*/
