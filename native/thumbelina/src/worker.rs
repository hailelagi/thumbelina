use crate::image::Image;
use crate::operation;
use crate::operation::Operation;
use crate::task;
use crate::thumbelina;

use rustler::{Atom, Encoder, LocalPid, NifTuple, OwnedEnv};

// use tokio::sync::oneshot;

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
    operation: Operation,
    pid: LocalPid,
    width: f32,
    height: f32,
    extension: &str,
    buffer: &[u8],
) {
    // let (tx, rx) = oneshot::channel();
    let mut env = OwnedEnv::new();

    // TODO: BAD BAD - make rust compiler happy first, slow but work
    let extension = extension.to_owned();
    let buffer = buffer.to_owned();

    task::spawn(async move {
        let result = operation::perform(operation, width, height, extension, &buffer);

        match result {
            Ok(image) => env.send_and_clear(&pid, move |env| {
                Success {
                    op: thumbelina::atoms::ok(),
                    result: image,
                }
                .encode(env)
            }),
            Err(_err) => env.send_and_clear(&pid, move |env| {
                Failure {
                    op: thumbelina::atoms::noop(),
                    // todo: error handling messages
                    reason: String::from("operation could not complete"),
                }
                .encode(env)
            }),
        }
    });
}
