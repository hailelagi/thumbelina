use crate::image::{Direction, Image};
use crate::operation::Operation;
use crate::operation::{
    try_blur, try_brighten, try_flip, try_greyscale, try_resize, try_rotate, try_thumbnail,
};
use crate::thumbelina;
use rustler::{Atom, Binary, Encoder, Error, LocalPid, NifTuple};

// todo: derive api scoped errors properly
#[derive(Debug, NifTuple)]
pub struct Failure {
    pub op: Atom,
    pub reason: String,
}

pub fn background_process(
    env: rustler::Env,
    operation: Operation,
    pid: LocalPid,
    width: f32,
    height: f32,
    extension: &str,
    bin: Binary,
) {
    let buffer = bin.as_slice();
    let job = {
        let transform = match operation {
            Operation::Resize => try_resize(&extension, buffer, width as u32, height as u32),
            Operation::Thumbnail => try_thumbnail(&extension, buffer, width as u32, height as u32),
            Operation::FlipHorizontal => try_flip(&extension, buffer, Direction::Horizontal),
            Operation::FlipVertical => try_flip(&extension, buffer, Direction::Vertical),
            Operation::Rotate => try_rotate(&extension, buffer, width as i32),
            Operation::Blur => try_blur(&extension, buffer, width as f32),
            Operation::Brighten => try_brighten(&extension, buffer, width as i32),
            Operation::Greyscale => try_greyscale(&extension, buffer),
        };

        match transform {
            Ok((image, format)) => Image::build(image, &extension, format),
            Err(err) => Err(Error::BadArg),
        }
    };

    let result = job;

    // return result after awaiting future
    match result {
        Ok(image) => env.send(&pid, image.encode(env)),
        Err(_err) => env.send(
            &pid,
            Failure {
                op: thumbelina::atoms::noop(),
                reason: String::from("operation could not complete"),
            }
            .encode(env),
        ),
    };
}
