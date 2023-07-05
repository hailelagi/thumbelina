defmodule Thumbelina do
  @moduledoc """
    Image manipulation api.
  """
  alias Thumbelina.Image
  alias Thumbelina.Internal

  @type result :: {:ok, Image.t()} | {:error, String.t()}

  @spec open(String.t(), map()) :: result()
  def open(path, opts \\ %{}) do
    ext = Path.extname(path)
    source = Map.get(opts, :source, :disk)

    with {:ok, binary} <- File.read(path),
         %Image{} = image <- Image.new(ext, path, binary, source) do
      {:ok, image}
    else
      err -> err
    end
  end

  @spec open_all!(String.t()) :: [Enumerable.t()]
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
  @spec stream_directory!(String.t(), pos_integer()) :: [Enumerable.t()]
  def stream_directory!(path, bytes \\ 65536) do
    path
    |> File.ls!()
    |> Stream.map(fn file -> File.stream!(path <> "/" <> file, [], bytes) end)
    |> Stream.map(fn file -> Stream.into(file, <<>>) end)
  end

  @spec resize(Image.t(), pos_integer(), pos_integer()) :: result()
  def resize(%Image{} = image, width, height) do
    Internal.resize(image.extension, image.bytes, width, height)
  end

  @spec crop(Image.t(), pos_integer(), pos_integer()) :: result()
  def crop(%Image{} = image, width, height) do
    Internal.crop(image.extension, image.bytes, width, height)
  end

  @spec thumbnail(Image.t(), pos_integer(), pos_integer()) :: result()
  def thumbnail(%Image{} = image, new_width, new_height) do
    Internal.thumbnail(image.extension, image.bytes, new_width, new_height)
  end

  @spec flip(Image.t(), :vertical | :horizontal) :: result()
  def flip(%Image{} = image, direction) do
    case direction do
      :vertical -> Internal.flip_vertical(image.extension, image.bytes)
      :horizontal -> Internal.flip_horizontal(image.extension, image.bytes)
      _ -> {:error, "Invalid direction for flip"}
    end
  end

  @spec rotate(Image.t(), pos_integer()) :: result()
  def rotate(%Image{} = image, angle) when angle > 0 and angle <= 360 do
    case angle do
      angle when angle in [90, 180, 270] ->
        Internal.rotate(image.extension, image.bytes, angle)

      angle when is_float(angle) or is_integer(angle) ->
        {angle, _} = Integer.parse("#{angle}")
        Internal.rotate(image.extension, image.bytes, angle)
    end
  end

  def rotate(%Image{}, _), do: {:error, "invalid rotation angle. Must be in range 1..360"}

  @spec blur(Image.t(), float()) :: result()
  def blur(%Image{} = image, sigma) do
    case sigma do
      sigma when is_integer(sigma) ->
        {sigma, _} = Float.parse("#{sigma}")
        Internal.blur(image.extension, image.bytes, sigma)

      sigma when is_float(sigma) ->
        Internal.blur(image.extension, image.bytes, sigma)

      _ ->
        {:error, "gausian blur must be a float"}
    end
  end

  @spec brighten(Image.t(), pos_integer()) :: result()
  def brighten(%Image{} = image, brightness) do
    case brightness do
      brightness when is_float(brightness) ->
        {brightness, _} = Integer.parse("#{brightness}")
        Internal.brighten(image.extension, image.bytes, brightness)

      brightness when is_integer(brightness) ->
        Internal.brighten(image.extension, image.bytes, brightness)

      _ ->
        {:error, "brightness must be an integer"}
    end
  end

  @spec greyscale(Image.t()) :: result()
  def greyscale(%Image{} = image), do: Internal.greyscale(image.extension, image.bytes)
end
