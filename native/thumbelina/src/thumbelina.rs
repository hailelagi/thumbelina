use rustler::{Atom, Binary, Error, NifResult};
use crate::image::{Direction, Image};
use image::{imageops::FilterType::Nearest, DynamicImage, ImageFormat};
use rayon::prelude::*;
use std::io;
use io::ErrorKind::Unsupported;
use std::sync::{RwLock, Arc};

mod atoms {
    rustler::atoms! {ok, error, png, jpeg, svg}
}

// TODO: Serialise any known image operation on dirty CPU
// #[rustler::nif(schedule = "DirtyCpu")]
// pub fn serialize_dirty<'a, 's>(
//     env: Env<'a>,
//     extension: &'a str,
//     path: String,
//     bin: Binary<'a>,
// ) -> NifResult<(Atom, (thumbelina::Image))> {

// }

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

// TODO
#[rustler::nif]
pub fn resize_all<'a>(
    images: Vec<Image>,
    width: u32,
    height: u32,
    extension: &'a str,
) -> NifResult<(Atom, Vec<Image>)> {
    let images = images
        .into_par_iter()
        .map(|image| try_resize(&image.extension, &image.bytes.as_slice(), 300, 500))
        .filter_map(|x| Some(x.unwrap()))
        .filter_map(|(img, format)| Some(Image::build_async(img, Arc::new(Box::new(extension)), format)));

    images.collect();
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
