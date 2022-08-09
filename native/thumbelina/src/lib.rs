// use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

use image::GenericImageView;

#[rustler::nif]
fn image_to_buffer(buffer: &[u8]) -> i64 {
    image::save_buffer("test_image.png", buffer, 800, 600, image::ColorType::Rgb8).unwrap()
}

rustler::init!("Elixir.Thumbelina", [image_to_buffer]);
