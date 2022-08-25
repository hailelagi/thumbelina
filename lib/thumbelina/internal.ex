defmodule Thumbelina.Internal do
  @moduledoc """
  NIF wrapper/rust bindings for image processing library.
  https://docs.rs/image/latest/image/
  """

  use Rustler, otp_app: :thumbelina, crate: "thumbelina"

  def image_to_buffer() do
    nil
  end
end
