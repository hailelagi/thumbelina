use crate::image::{Direction, Image};
use crate::operation;
use rustler::{Atom, Binary, Error, Encoder, NifResult};
use rustler::env::OwnedEnv;
use rustler::types::LocalPid;
use rayon::prelude::*;

mod atoms { 
    rustler::atoms! {ok, noop, error, png, jpeg, svg}
}

// TODO: provide opt-in time outs and cancellations

// Asynchronously clone erlang owned bytes and write them to a new buffer
// within the managed tokio runtime address space, casting it to a `DynamicImage` performing an `Operation` 
// and replying back to the client process immediately with an :ok or `:noop`.
// This is done to relinquish scheduler time to the caller in erts counting as full reduction op.
// where in the client's server process mailbox the reply will be delivered with `{:ok, :"{operation}", image_bytes}`
// provides two flavors `cast` for fire and forget on a single large image `cast_all` for several.
//  #[rustler::nif]
// pub fn cast<'a>(
//     pid: &LocalPid,
//     bin: Binary<'a>,
//     extension: &'a str,
//     width: f32, 
//     height: f32,
//     operation: Operation
// ) -> NifResult<Atom> {
//     let mut env = OwnedEnv::new();
//     let data = "test".to_string();
    
//     env.send_and_clear(pid, |env| data.encode_utf16());

//     Ok(atoms::ok())
// }

//  #[rustler::nif]
// pub fn cast_all<'a>(
//     env: Env,
//     binaries: Vec<Binary<'a>>,
//     extension: &'a str,
//     width: f32,
//     height: f32,
//     pid: LocalPid,
//     operation: Operation
// ) ->

#[rustler::nif]
pub fn resize<'a>(
    bin: Binary<'a>,
    extension: &'a str,
    width: u32,
    height: u32,
) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_resize(extension, buffer, width, height) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn resize_all<'a>(
    binaries: Vec<Binary<'a>>,
    extension: &'a str,
    width: u32,
    height: u32,
) -> NifResult<(Atom, Vec<Image>)> {
    let images = binaries
        .iter()
        .map(|bin| bin.as_slice())
        .collect::<Vec<&[u8]>>()
        .into_par_iter()
        .filter_map(|i| {
            let resized =  operation::try_resize(extension, i, width, height);
            match resized {
                Ok((image, format)) => Image::build(image, extension, format).ok(),
                Err(_err) => None,
            }
        })
        .collect();

    Ok((atoms::ok(), images))
}

#[rustler::nif]
pub fn thumbnail<'a>(
    extension: &'a str,
    bin: Binary<'a>,
    nwidth: u32,
    nheight: u32,
) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_thumbnail(extension, buffer, nwidth, nheight) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn flip_horizontal<'a>(extension: &'a str, bin: Binary<'a>) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_flip(extension, buffer, Direction::Horizontal) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn flip_vertical<'a>(extension: &'a str, bin: Binary<'a>) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_flip(extension, buffer, Direction::Vertical) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn rotate<'a>(extension: &'a str, bin: Binary<'a>, angle: i32) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_rotate(extension, buffer, angle) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn blur<'a>(extension: &'a str, bin: Binary<'a>, sigma: f32) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_blur(extension, buffer, sigma) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn brighten<'a>(
    extension: &'a str,
    bin: Binary<'a>,
    brightness: i32,
) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_brighten(extension, buffer, brightness) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn greyscale<'a>(extension: &'a str, bin: Binary<'a>) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match operation::try_greyscale(extension, buffer) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}
