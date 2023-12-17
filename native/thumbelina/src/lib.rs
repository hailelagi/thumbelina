use rustler::{Env, Term};

pub mod image;
pub mod operation;
pub mod task;
pub mod thumbelina;
pub mod worker;

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(image::Image, env);
    true
}

rustler::init!(
    "Elixir.Thumbelina.Internal",
    [
        thumbelina::cast,
        thumbelina::batch,
        thumbelina::blur,
        thumbelina::brighten,
        thumbelina::flip_horizontal,
        thumbelina::flip_vertical,
        thumbelina::greyscale,
        thumbelina::resize,
        thumbelina::thumbnail,
        thumbelina::rotate,
        thumbelina::block_compress,
        thumbelina::block_decompress,
        thumbelina::stream_compress,
        thumbelina::stream_decompress
        //   thumbelina::crop,
    ],
    load = load
);
