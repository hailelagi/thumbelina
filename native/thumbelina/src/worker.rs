use rustler::Binary;
use std::sync::{Arc, RwLock};

use crate::image::Image;
use crate::operation;
use crate::operation::{Operation, StreamOperation};
use crate::task;
use crate::thumbelina;

use rustler::{Atom, Encoder, LocalPid, NifTuple, OwnedEnv};

#[derive(Debug, NifTuple)]
pub struct Success {
    pub op: Atom,
    pub result: Image,
}

#[derive(Debug, NifTuple)]
pub struct Failure {
    pub op: Atom,
    pub reason: String,
}

pub fn background_stream(_op: StreamOperation, _pid: LocalPid, _buffer: &[u8]) {
    // let env = OwnedEnv::new();
    // let buffer = buffer.to_owned();

    // todo
}

pub fn background_process(
    operation: Operation,
    pid: LocalPid,
    width: f32,
    height: f32,
    extension: &str,
    binary: Binary,
) {
    let mut env = OwnedEnv::new();
    let extension = extension.to_owned();

    // take ownership of the smart pointer to the image binary at runtime
    // ownership in this instance unbinds the lifetime of Binary<'a> from `Env`
    // and the Arc<T> will increment references to the slice for each thread.
    if let Some(buffer) = binary.to_owned() {
        let buffered_lock = Arc::new(RwLock::new(buffer));

        task::spawn(async move {
            let buffer = Arc::clone(&buffered_lock);
            let buffer = match buffer.read() {
                Ok(buffer) => buffer,
                Err(err) => err.into_inner()
            };

            let result = operation::perform(operation, width, height, extension, &buffer);

            match result {
                Ok(image) => env.send_and_clear(&pid, move |env| {
                    Success {
                        op: thumbelina::atoms::ok(),
                        result: image,
                    }
                    .encode(env)
                }),
                Err(_err) => {
                    report_error(&mut env, pid, String::from("corruption during operation"))
                }
            }
        });
    }
}

fn report_error(
    env: &mut OwnedEnv,
    pid: LocalPid,
    err: String,
) -> Result<(), rustler::env::SendError> {
    env.send_and_clear(&pid, move |env| {
        Failure {
            op: thumbelina::atoms::noop(),
            reason: err,
        }
        .encode(env)
    })
}
