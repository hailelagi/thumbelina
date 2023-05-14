defmodule Thumbelina do
  @moduledoc """
    Image manipulation api.
  """
  alias Thumbelina.Image
  alias Thumbelina.Internal

  def open(path) do
    ext = Path.extname(path)

    case File.read(path) do
      {:ok, binary} -> {:ok, Image.new(ext, path, binary, :disk)}
      error -> error
    end
  end

  def open_all!(path) do
    path
    |> File.ls!()
    |> Stream.map(fn file -> Thumbelina.open(path <> "/" <> file) end)
  end

  @doc """
    If dealing with a large amount of files on disk and you wouldn't like
    to use `File.open/1` due to the number of open file descriptors and would like control
    over how many bytes are read into memory at a time. Defaults to 2 ** 16 bytes or ~65kb.
  """
  def stream_directory!(path, bytes \\ 65536) do
    path
    |> File.ls!()
    |> Stream.map(fn file -> File.stream!(path <> "/" <> file, [], bytes) end)
    |> Stream.map(fn file -> Stream.into(file, <<>>) end)
  end

  @spec resize(Image.t(), pos_integer(), pos_integer()) ::
          {:ok, {Image.t(), <<_::_*256>>}} | {:error, String.t()}
  def resize(%Image{} = image, width, height) do
    Internal.resize(image.extension, image.bytes, width, height)
  end

  @spec thumbnail(Image.t(), pos_integer()) ::
          {:ok, {Image.t(), <<_::_*256>>}} | {:error, String.t()}
  def thumbnail(%Image{} = image, dimension) do
    Internal.resize(image.extension, image.bytes, dimension, dimension)
  end

  @spec flip(Image.t(), :vertical | :horizontal) ::
          {:ok, {Image.t(), <<_::_*256>>}} | {:error, String.t()}
  def flip(%Image{} = image, direction) do
    case direction do
      :vertical -> Internal.flip_vertical(image.extension, image.bytes)
      :horizontal -> Internal.flip_horizontal(image.extension, image.bytes)
      _ -> {:error, "Invalid direction for flip"}
    end
  end

  def rotate(%Image{} = image, angle) when angle > 0 and angle <= 360 do
    case angle do
      angle when angle in [90, 180, 270] ->
        Internal.rotate(image.extension, image.bytes, angle)

      angle when is_float(angle) ->
        {angle, _} = Integer.parse("#{angle}")
        Internal.rotate(image.extension, image.bytes, angle)
    end
  end

  def rotate(%Image{}, _), do: {:error, "invalid rotation angle. Must be in range 1..360"}

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
