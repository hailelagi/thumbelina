defmodule Thumbelina.Internal do
  @moduledoc """
  NIF wrapper/rust bindings for image processing library,
  Handles FFI Initialisation, scheduling and encoding/decoding to/from image-rs.
  see: https://docs.rs/image/latest/image/
  """
  use Rustler, otp_app: :thumbelina, crate: "thumbelina"

  @doc """
    Serialise %Image{} operations as blocking and CPU bound.
    This would be appropriate for batched image files above a
    certain MiB threshold.
  """
  def serialize_dirty(_, _, _), do: error()

  @doc """
    Serialise %Image{} in a Threaded NIF that sends a message back to a
    GenServer caller process.
  """
  def server(_), do: error()

  # Image Processing Functions
  def resize(_, _, _, _), do: error()
  def resize_all(_, _, _, _), do: error()
  def thumbnail(_, _, _, _), do: error()
  def flip_horizontal(_, _), do: error()
  def flip_vertical(_, _), do: error()
  def rotate(_, _, _), do: error()
  def crop(_, _, _, _), do: error()
  def blur(_, _, _), do: error()
  def brighten(_, _, _), do: error()
  def greyscale(_, _), do: error()

  # tbd
  # def resize_all(_, _, _), do: error()
  # def contrast(_), do: error()
  # def filter_3x3(_), do: error()
  # def invert(_), do: error()
  # def unsharpen(_), do: error()

  defp error, do: :erlang.nif_error("image-rs is not available")
end
