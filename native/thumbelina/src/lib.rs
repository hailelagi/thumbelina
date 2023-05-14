mod thumbelina;

use rustler::{Env, Term};

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(thumbelina::Image, env);
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
        //   thumbelina::huerotate,
        //   thumbelina::contrast
        //   thumbelina::crop,
        //   thumbelina::filter_3x3,
        //   thumbelina::flip_horizontal,
        //   thumbelina::flip_vertical,
        //   thumbelina::greyscale,
        //   thumbelina::invert,
        thumbelina::resize,
        // thumbelina::resize_all,
        //   thumbelina::rotate180,
        //   thumbelina::rotate270,
        //   thumbelina::rotate90,
        //   thumbelina::unsharpen
    ],
    load = load
);
