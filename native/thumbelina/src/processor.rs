// use crate::image::{Direction, Image};
// use crate::operation;
// use crate::task;
// use rustler::{Binary, Encoder, Error, LocalPid, OwnedEnv, ResourceArc, Term};
// use tokio::sync::mpsc::{channel, Receiver, Sender};
// use std::sync::Mutex;

// mod atoms {
//     rustler::atoms! {ok, noop, resize, thumbnail, flip_horizontal, flip_vertical, rotate, blur, brighten, greyscale}
// }

// enum Op {
//     Blur(LocalPid),
//     Brighten(LocalPid),
//     FlipHorizontal(LocalPid),
//     FlipVertical(LocalPid),
//     Greyscale(LocalPid),
//     Resize(LocalPid),
//     Thumbnail(LocalPid),
//     Rotate(LocalPid),
// }

// struct Ref(Mutex<Sender<Op>>);

// impl Ref {
//     fn new(tx: Sender<Image>) -> ResourceArc<Ref> {
//         ResourceArc::new(Ref(Mutex::new(tx)))
//     }
// }

// fn process(
//     env: OwnedEnv,
//     operation: Op,
//     pid: &LocalPid,
//     width: f32,
//     height: f32,
//     extension: &str,
//     buffer: &[u8],
// ) {
//     task::spawn(async move {
//         let result = match operation {
//             Op::Resize(pid) => {
//                 (pid, operation::try_resize(extension, buffer, width as u32, height as u32))
//             }
//             Op::Thumbnail(pid) => {
//                 (pid, operation::try_thumbnail(extension, buffer, width as u32, height as u32))
//             }
//             Op::FlipHorizontal(pid) => {
//                 (pid, operation::try_flip(extension, buffer, Direction::Horizontal))
//             }
//             Op::FlipVertical(pid) => (pid, operation::try_flip(extension, buffer, Direction::Vertical)),
//             Op::Rotate(pid) => (pid, operation::try_rotate(extension, buffer, width as i32)),
//             Op::Blur(pid) => (pid, operation::try_blur(extension, buffer, width as f32)),
//             Op::Brighten(pid) => (pid, operation::try_brighten(extension, buffer, width as i32)),
//             Op::Greyscale(pid) => (pid, operation::try_greyscale(extension, buffer)),
//         };

//         let reply = match result {
//             (pid, Ok((image, format))) => (pid, Image::build(image, extension, format)),
//             (pid, Err(err)) => (pid, Err(Error::Term(Box::new(err.to_string())))),
//         };

//         match reply {
//             (pid, Ok(image)) => env.send_and_clear(&pid, Ref::new(atoms::ok(), image)),
//             (pid, Err(err)) => env.send_and_clear(&pid, (atoms::noop(), err.encode(env))),
//         };
//     });
// }
