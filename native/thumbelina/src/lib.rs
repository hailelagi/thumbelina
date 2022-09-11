mod thumbelina;
mod image;

rustler::init!("Elixir.Thumbelina.Internal", [
    thumbelina::add, 
    image::serialize
    ]);
