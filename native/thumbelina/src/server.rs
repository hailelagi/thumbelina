// use crate::image::{Direction, Image};
// use crate::operation;
// use crate::task;
// use crate::operation::Operation;
// use rustler::{Env, Error, NifResult, LocalPid, Term, Binary, Encoder};

// mod atoms { 
//     rustler::atoms! {ok, noop, resize, thumbnail, flip_horizontal, flip_vertical, rotate, blur, brighten, greyscale}
// }

// fn process(env: Env, operation: Operation, pid: LocalPid, width: f32, height: f32, extension: &str, buffer: &[u8]) -> () {
//     task::spawn(async move{
//         let result = match operation {
//           Operation::Resize => operation::try_resize(extension, buffer, width as u32, height as u32),
//           Operation::Thumbnail => operation::try_thumbnail(extension, buffer, width as u32, height as u32),
//           Operation::FlipHorizontal => operation::try_flip(extension, buffer, Direction::Horizontal),
//           Operation::FlipVertical => operation::try_flip(extension, buffer, Direction::Vertical),
//           Operation::Rotate => operation::try_rotate(extension,buffer, width as i32),
//           Operation::Blur => operation::try_blur(extension, buffer,  width as f32),
//           Operation::Brighten => operation::try_brighten(extension, buffer, width as i32),
//           Operation::Greyscale => operation::try_greyscale(extension, buffer),
//         };

//         let reply = match result {
//           Ok((image, format)) => Ok((atoms::ok(), Image::build(image, extension, format))),
//           Err(err) => Err((atoms::noop(), Error::Term(Box::new(err.to_string())))),
//         };

//         let Some((image, format)) = reply {
//           env.send(&pid, Binary::to_term(image.bytes, env));
//       };
//     };
//     );
// }
