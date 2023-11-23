defmodule Thumbelina.Internal do
  @moduledoc """
  NIF wrapper/rust bindings for image processing library,
  Handles FFI Initialisation, scheduling and encoding/decoding to/from image-rs.
  see: https://docs.rs/image/latest/image/

  `batch` api intended for batching calls of image bytes to
  be transformed on the scheduler's dirty cpu thread.

  `cast` are serialised in a Threaded NIF that
  asynchronously sends a message back to a registered pid caller process.
  """
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :thumbelina,
    crate: "thumbelina",
    base_url: "https://github.com/hailelagi/thumbelina/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_BUILD") in ["1", "true"],
    nif_versions: ["2.16"],
    version: version

  # Asynchronous Image Processing
  @spec cast(atom(), pid(), binary(), String.t(), float(), float()) :: :ok
  def cast(_, _, _, _, _, _), do: error()

  # Sync Image Processing
  def resize(_, _, _, _), do: error()
  def thumbnail(_, _, _, _), do: error()
  def flip_horizontal(_, _), do: error()
  def flip_vertical(_, _), do: error()
  def rotate(_, _, _), do: error()
  def crop(_, _, _, _), do: error()
  def blur(_, _, _), do: error()
  def brighten(_, _, _), do: error()
  def greyscale(_, _), do: error()

  # def contrast(_), do: error()
  # def filter_3x3(_), do: error()
  # def invert(_), do: error()
  # def unsharpen(_), do: error()

  # Batch Parallelised Image Processing (Dirty CPU)
  def batch(_, _, _, _, _), do: error()

  defp error, do: :erlang.nif_error("image-rs is not available")
end
