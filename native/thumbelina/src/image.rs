use std::io::Cursor;
use rustler::{Atom, Error, OwnedBinary, NifStruct};
use image::io::Reader as ImageReader;

mod atoms { rustler::atoms! {ok, error, png, jpeg, svg} }

#[derive(NifStruct)]
#[module = "Thumbelina.Image"]
pub struct ThumbelinaImage {
    pub extension: Atom,
    pub path: String,
    pub height: u32,
    pub width: u32,
    pub bytes: usize,
}

pub struct Options {
    pub extension: Atom,
    pub path: String,
}

#[rustler::nif]
pub fn serialize<'a>(opts: Options, bin: Binary<'a>) -> Result<ThumbelinaImage, Error> {
        let img = ImageReader::new(Cursor::new(bin)).decode()?;
        let thumbelina_image = ThumbelinaImage{
            extension: opts.extension, 
            path: opts.path,
            height: img.height(),
            width: img.width(),
            bytes: bin
        };

        Ok(thumbelina_image)
    }

// todo: Serialize image-rs Errors to ruster::Error
/* 
impl From<ImageError> for Error {

}
*/