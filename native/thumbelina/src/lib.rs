// use image::{DynamicImage, GenericImageView, ImageFormat};
// use rustler::{Atom, NifStruct, Term};

mod image;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn echo(s: String) -> String {
    s
}

rustler::init!("Elixir.Thumbelina.Internal", [add, echo]);
