defmodule Thumbelina do
  @moduledoc """
    Library public api
  """

  @path "./archive/images/images"

  alias Thumbelina.Image

  # todo: read file data and pass to bytes
  # @spec image_to_bytes() ::  {:ok, Image.t()} |
  # def image_to_bytes() do
  #   Image.new()
  # end

  @doc """
    Open an image file for processing
  """
  @spec open(String.t()) :: Image.t()
  def open(path) do
    {:ok, binary} = File.read(path)
    binary
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
