use crate::image::{Direction, Image, Operation};
use image::{imageops::FilterType::Nearest, DynamicImage, ImageFormat};
use io::ErrorKind::Unsupported;
use rayon::{prelude::*, ThreadPoolBuilder};
use rustler::{Atom, Binary, Env, Error, LocalPid, NifResult};
use std::io;
use tokio::sync::mpsc;
// use std::sync::{Arc, RwLock};

mod atoms {
    rustler::atoms! {ok, error, png, jpeg, svg}
}

#[rustler::nif]
pub fn resize<'a>(
    extension: &'a str,
    bin: Binary<'a>,
    width: u32,
    height: u32,
) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_resize(extension, buffer, width, height) {
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
            let resized = try_resize(extension, i, width, height);
            match resized {
                Ok((image, format)) => Image::build(image, extension, format).ok(),
                Err(_err) => None,
            }
        })
        .collect();

    Ok((atoms::ok(), images))
}

#[rustler::nif]
pub fn resize_cast<'a>(
    env: Env,
    binaries: Vec<Binary<'a>>,
    extension: &'a str,
    width: u32,
    height: u32,
    pid: LocalPid,
) -> NifResult<Atom> {
    let (tx, mut rx) = mpsc::channel(256);

    //TODO: scedule on tokio and reply back async in the process mailbox
    let image_buffers = binaries.iter().map(|bin| bin.as_slice());

    for buffer in image_buffers {
        tokio::spawn(async move {
            env.send(
                &pid,
                match try_resize(extension, buffer, width, height) {
                    Ok((image, format)) => Image::build(image, extension, format).unwrap().bytes,
                    Err(err) => Error::Term(Box::new(err.to_string())),
                },
            )
        });
    }

    Ok(atoms::ok())
}

#[rustler::nif]
pub fn thumbnail<'a>(
    extension: &'a str,
    bin: Binary<'a>,
    nwidth: u32,
    nheight: u32,
) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_thumbnail(extension, buffer, nwidth, nheight) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn flip_horizontal<'a>(extension: &'a str, bin: Binary<'a>) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_flip(extension, buffer, Direction::Horizontal) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn flip_vertical<'a>(extension: &'a str, bin: Binary<'a>) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_flip(extension, buffer, Direction::Vertical) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn rotate<'a>(extension: &'a str, bin: Binary<'a>, angle: i32) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_rotate(extension, buffer, angle) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn blur<'a>(extension: &'a str, bin: Binary<'a>, sigma: f32) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_blur(extension, buffer, sigma) {
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
    match try_brighten(extension, buffer, brightness) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
pub fn greyscale<'a>(extension: &'a str, bin: Binary<'a>) -> NifResult<(Atom, Image)> {
    let buffer = bin.as_slice();
    match try_greyscale(extension, buffer) {
        Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format)?)),
        Err(err) => Err(Error::Term(Box::new(err.to_string()))),
    }
}

fn try_resize<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    width: u32,
    height: u32,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.resize_to_fill(width, height, Nearest);

    Ok((img, format))
}

fn try_thumbnail<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    nwidth: u32,
    nheight: u32,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.thumbnail(nwidth, nheight);

    Ok((img, format))
}

fn try_flip<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    direction: Direction,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = match direction {
        Direction::Vertical => img.flipv(),
        Direction::Horizontal => img.fliph(),
    };

    Ok((img, format))
}

fn try_rotate<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    angle: i32,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = match angle {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img.huerotate(angle),
    };

    Ok((img, format))
}

fn try_blur<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    sigma: f32,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.blur(sigma);

    Ok((img, format))
}

fn try_brighten<'a>(
    extension: &'a str,
    buffer: &'a [u8],
    value: i32,
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.brighten(value);

    Ok((img, format))
}

fn try_greyscale<'a>(
    extension: &'a str,
    buffer: &'a [u8],
) -> Result<(DynamicImage, ImageFormat), image::ImageError> {
    let format = ImageFormat::from_extension(extension)
        .ok_or(io::Error::new(Unsupported, "invalid format provided"))?;
    let img = image::load_from_memory_with_format(buffer, format)?;
    let img = img.grayscale();

    Ok((img, format))
}
