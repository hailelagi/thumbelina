#[rustler::nif]
fn image_to_buffer(buffer: u8) {
    ()
}

rustler::init!("Elixir.Thumbelina", [image_to_buffer]);
