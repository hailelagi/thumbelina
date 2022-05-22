defmodule Thumbelina do
  @moduledoc """
  Documentation for `Thumbelina`.

  todo: scope out api read rust docs
  https://docs.rs/image/latest/image/

  - can process really large image data sets
  - pause/resume processing saving memory
  - lazy and efficient runtime
  """
  use Rustler, otp_app: :thumbelina, crate: "thumbelina"

  # resize an image
  def resize() do
    nil
  end

  # def batch_resize(path) do
  #   File.stream!(path, :line) |> Flow.from_enumerable()
  # end

  # change image format
  def convert() do
    nil
  end

  # save
  def save() do
    nil
  end
end
