defmodule Thumbelina.Internal do
  @moduledoc """
  NIF wrapper/rust bindings for image processing library.
  https://docs.rs/image/latest/image/
  """

  use Rustler, otp_app: :thumbelina, crate: "thumbelina"

  def add(_a, _b), do: :erlang.nif_error("not implemented")
  def echo(_), do: :erlang.nif_error("not implemented")
end
