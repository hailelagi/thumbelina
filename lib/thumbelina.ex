defmodule Thumbelina do
  @moduledoc """
    Library public api
  """

  @path "./archive/images/images/abra.png"

  alias Thumbelina.Image

  @doc """
    Open an image file for processing
  """
  def open(path \\ @path) do
    ext = Path.extname(path)

    # OPTIMISE: maybe read image resource via rust nif
    # TODO: research is it faster to hold file locks in rust?
    case File.read(path) do
      {:ok, binary} -> Image.new(ext, path, binary)
      error -> error
    end
  end

  def test do
    {:ok, {_, b}} =  open()

    File.write!("./test.png",b)
  end

  @doc """
    Open all the images in fire directory
  """
  @spec open_directory!(String.t()) :: [Image.t()]
  def open_directory!(path, bytes \\ 2048) do
    files = File.ls!(path)

    Enum.map(files, fn file -> File.stream!(file, [], bytes) end)
  end

  def write() do
    nil
  end

  def write_stream() do
    nil
  end

  def write_file() do
    nil
  end

  def stream() do
    nil
  end

  def flip() do
    nil
  end

  def resize do
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
