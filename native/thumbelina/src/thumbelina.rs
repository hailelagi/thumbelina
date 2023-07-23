use crate::image::{Direction, Image};
use crate::operation;
use crate::operation::{Operation};
use image::{imageops::FilterType::Nearest, DynamicImage, ImageFormat};
use io::ErrorKind::Unsupported;
use rayon::{prelude::*, ThreadPoolBuilder};
use rustler::{Atom, Binary, Env, Error, LocalPid, NifResult};
use std::{io, sync::{Arc}};
use tokio::sync::mpsc;

mod atoms { 
    rustler::atoms! {ok, error, png, jpeg, svg}
}

// Asynchronously clone erlang owned bytes and write them to a new buffer
// within the managed tokio runtime address space, casting it to a DynamicImage performing an `Operation` 
// and replying back to the client immediately with an :ok or `:noop`.
// This is done to relinquish scheduler time to the caller in erts.
// where in the client's server process mailbox the reply will be delivered with `{:"ok_{operation}", image_bytes}`
//  #[rustler::nif]
// pub fn cast<'a>(
//     env: Env,
//     binaries: Vec<Binary<'a>>,
//     extension: &'a str,
//     width: f32,
//     height: f32,
//     pid: LocalPid,
//     operation: Operation
// ) -> NifResult<Atom> {
//     match operation {
//         Operation::Resize => resize_all_async(env, binaries, extension, width, height, pid),
//         Operation::Thumbnail => thumbnail_all_async(env, binaries, extension, width, height, pid),
//         Operation::FlipHorizontal => flip_all_async(env, binaries, extension, Direction::Horizontal, pid),
//         Operation::FlipVertical => flip_all_async(env, binaries, extension, Direction::Vertical, pid),
//         Operation::Rotate => rotate_all_async(env, binaries, extension, width as i32, pid),
//         Operation::Blur => blur_all_async(env, binaries, extension, width as f32, pid),
//         Operation::Brighten => brighten_all_async(env, binaries, extension, width as i32, pid),
//         Operation::Greyscale => greyscale_all_async(env, binaries, extension, pid),
//         // Operation::Crop => crop_all_async(env, binaries, extension, width, height, pid),
//     }

//     //TODO: decide on an IO schedule message passing strategy
//     let (tx, mut rx) = mpsc::channel(256);

//     //TODO: scedule on tokio and reply back async in the process mailbox
//     let image_buffers = binaries.iter().map(|bin| bin.as_slice());

//     for buffer in image_buffers {
//         tokio::spawn(async move {
//             env.send(
//                 &pid,
//                 match try_resize(extension, buffer, width, height) {
//                     Ok((image, format)) => Image::build(image, extension, format).unwrap().bytes,
//                     Err(err) => Error::Term(Box::new(err.to_string())),
//                 },
//             )
//         });
//     }

//     Ok(atoms::ok())
// }

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
