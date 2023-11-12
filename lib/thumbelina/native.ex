defmodule Thumbelina.Native do
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :thumbelina,
    crate: "thumbelina",
    base_url:
      "https://github.com/hailelagi/thumbelina/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_EXAMPLE_BUILD") in ["1", "true"],
    version: version

  def cast(_, _, _, _, _, _), do: error()
  def resize(_, _, _, _), do: error()
  def thumbnail(_, _, _, _), do: error()
  def flip_horizontal(_, _), do: error()
  def flip_vertical(_, _), do: error()
  def rotate(_, _, _), do: error()
  def crop(_, _, _, _), do: error()
  def blur(_, _, _), do: error()
  def brighten(_, _, _), do: error()
  def greyscale(_, _), do: error()
  def batch(_, _, _, _, _), do: error()

  defp error, do: :erlang.nif_error("image-rs is not available")
end
