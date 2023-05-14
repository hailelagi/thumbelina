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
        // thumbelina::serialize,
        //   thumbelina::serialize_dirty,
        //   thumbelina::server,
        //   thumbelina::blur,
        //   thumbelina::brighten,
        //   thumbelina::contrast
        //   thumbelina::crop,
        //   thumbelina::filter_3x3,
        thumbelina::flip_horizontal,
        thumbelina::flip_vertical,
        //   thumbelina::greyscale,
        //   thumbelina::invert,
        thumbelina::resize,
        // thumbelina::resize_all,
        thumbelina::rotate,
        //   thumbelina::unsharpen
    ],
    load = load
);
