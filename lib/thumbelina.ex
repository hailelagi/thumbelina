defmodule Thumbelina do
  @moduledoc """
  NIF wrapper rust image processing library.
  https://docs.rs/image/latest/image/
  """

  use Rustler, otp_app: :thumbelina, crate: "thumbelina"

  def generate_thumbnail() do
    nil
  end
end
