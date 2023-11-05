use crate::image::Image;
use crate::operation;
use crate::operation::Operation;
use crate::thumbelina;
use rustler::{Atom, Encoder, LocalPid, NifTuple};

// todo: weird naming on boundary
#[derive(Debug, NifTuple)]
pub struct Success {
    pub op: Atom,
    pub result: Image,
}

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
    buffer: &[u8],
) {
    let result = operation::perform(operation, width, height, extension, buffer);

    // return result after awaiting future
    match result {
        Ok(image) => env.send(
            &pid,
            Success {
                op: thumbelina::atoms::ok(),
                result: image,
            }
            .encode(env),
        ),
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
