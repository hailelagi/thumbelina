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
    pub size: u32
}

pub struct Options {
    pub extension: Atom,
    pub path: String,
}

#[rustler::nif]
pub fn serialize<'a>(opts: Options, bin: Binary<'a>) -> Result<ThumbelinaImage, Error> {
        let image = ImageReader::new(Cursor::new(bin)).decode()?;
        let image = ThumbelinaImage{
            extension: opts.extension, 
            path: opts.path,
            height: image.height(),
            width: image.width(),
            bytes: bin,
            size: bin.len()
        };

        Ok(ok(), image)
    }

#[rustler::nif(schedule = "DirtyCpu")]
pub fn serialize_dirty(opts: Options, bin: Binary) -> Result<(Atom, ThumbelinaImage), Error> {
    match image::load_from_memory(bin.as_slice()) {
        Ok(image) => {
                let image = ThumbelinaImage {
                    extension: opts.extension, 
                    path: opts.path,
                    height: img.height(),
                    width: img.width(),
                    bytes: bin,
                    size: bin.len()
                }


                return Ok((ok(), image));
            }
    
        Err(_) => Err(Error::BadArg)
    }
}


// todo: Serialize image-rs Errors to ruster::Error
/* 
impl From<ImageError> for Error {

}
*/