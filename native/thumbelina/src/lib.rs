use rustler::{Env, Term};

pub mod image;
pub mod server;
pub mod operation;
pub mod thumbelina;

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(image::Image, env);
    true
}

rustler::init!(
    "Elixir.Thumbelina.Internal",
    [
        // thumbelina::server,
        // thumbelina::cast,
        thumbelina::blur,
        thumbelina::brighten,
        thumbelina::flip_horizontal,
        thumbelina::flip_vertical,
        thumbelina::greyscale,
        thumbelina::resize,
        thumbelina::resize_all,
        thumbelina::thumbnail,
        thumbelina::rotate,
        //   thumbelina::crop,
    ],
    load = load
);
