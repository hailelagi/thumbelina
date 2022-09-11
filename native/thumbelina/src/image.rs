// use image::{DynamicImage, GenericImageView, ImageFormat};
use rustler::{Atom, Env, Error, NifStruct, ResourceArc, Term};

mod atoms { rustler::atoms! {ok, error, eof} }

#[derive(NifStruct)]
#[module = "Thumbelina.Image"]
pub struct ThumbelinaImage {
    pub byte_size: usize,
    pub format: Atom,
    pub height: u32,
    pub width: u32,
}
