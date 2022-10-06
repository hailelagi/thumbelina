defmodule Thumbelina.Internal do
  @moduledoc """
  NIF wrapper/rust bindings for image processing library.
  https://docs.rs/image/latest/image/
  """

  use Rustler, otp_app: :thumbelina, crate: "thumbelina"
  def serialize(_, _, _), do: error()

  defp error, do: :erlang.nif_error("not implemented")
end
