mod thumbelina;

use rustler::{Env, Term};

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(thumbelina::ImageMetadata, env);
    true
}

rustler::init!(
    "Elixir.Thumbelina.Internal",
    [thumbelina::serialize],
    load = load
);
