use rustler::{Env, Term};
pub mod image;
pub mod thumbelina;

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(image::Image, env);
    true
}

rustler::init!(
    "Elixir.Thumbelina.Internal",
    [
        //   thumbelina::serialize_dirty,
        //   thumbelina::server,
        thumbelina::blur,
        thumbelina::brighten,
        thumbelina::flip_horizontal,
        thumbelina::flip_vertical,
        thumbelina::greyscale,
        thumbelina::resize,
        thumbelina::thumbnail,
        thumbelina::rotate,
        //   thumbelina::invert,
        //   thumbelina::unsharpen
        //   thumbelina::contrast
        //   thumbelina::crop,
    ],
    load = load
);
