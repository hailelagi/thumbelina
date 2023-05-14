defmodule Thumbelina do
  @moduledoc """
    Library public api
  """
  alias Thumbelina.Image
  alias Thumbelina.Internal

  @doc """
    Open an image file for processing
  """
  def open(path) do
    ext = Path.extname(path)

    case File.read(path) do
      {:ok, binary} -> {:ok, Image.new(ext, path, binary, :disk)}
      error -> error
    end
  end

  @spec open_directory!(String.t()) :: [Image.t()]
  def open_directory!(path, bytes \\ 2048) do
    files = File.ls!(path)

    Enum.map(files, fn file -> File.stream!(file, [], bytes) end)
  end

  # def write() do
  #   nil
  # end

  # def write_stream() do
  #   nil
  # end

  # def write_file() do
  #   nil
  # end

  # def stream() do
  #   nil
  # end

  def resize(%Image{} = image, width, height) do
    Internal.resize(image.extension, image.bytes, width, height)
  end

  # def resize_all([%Image{}] = images, width, height) do
  #   Internal.resize_all(images, width, height)
  # end

  def flip() do
    nil
  end

  def rotate do
    nil
  end

  def avatar do
    nil
  end

  def crop() do
    nil
  end

  def blur do
    nil
  end

  def brighten do
    nil
  end

  def greyscale do
    nil
  end
end
