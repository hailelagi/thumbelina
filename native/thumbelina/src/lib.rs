#[rustler::nif]
fn image_to_buffer(buffer: u8) {
    ()
}

#[rustler::nif]
fn echo(s: String) -> String {
    s
}

rustler::init!("Elixir.Thumbelina", [image_to_buffer, echo]);
