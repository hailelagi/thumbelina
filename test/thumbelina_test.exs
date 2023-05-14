defmodule ThumbelinaTest do
  use ExUnit.Case, async: true
  alias Thumbelina.Image

  doctest Thumbelina

  test "it reads an image binary into an %Image{}" do
    {:ok, image} = Thumbelina.open("./example/abra.png")
    assert image.height == 0
    assert image.width == 0
    assert is_binary(image.bytes)
  end

  test "it streams the images in a local directory" do
    entries = Thumbelina.open_all!("./example/pokemon")

    assert Enum.all?(entries, fn e ->
             {:ok, img} = e
             assert is_binary(img.bytes)
             assert img.source == :disk
           end)
  end

  test "it streams bytes from a stream on disk" do
    entries = Thumbelina.stream_directory!("./example/pokemon") |> Enum.map(&Enum.to_list/1)

    assert Enum.all?(entries, fn [e] -> is_binary(e) end)
  end

  test "it resizes an example image" do
    {:ok, image} = Thumbelina.open("./example/abra.png")
    assert {:ok, {img, resized_binary}} = Thumbelina.resize(image, 50, 50)

    assert img.height == 50
    assert img.width == 50
    refute image.bytes == resized_binary
  end

  test "it flips an example image" do
    {:ok, image} = Thumbelina.open("./example/abra.png")
    assert is_binary(image.bytes)

    assert {:ok, {img, vertical_binary}} = Thumbelina.flip(image, :vertical)
    assert {:ok, {img, horizontal_binary}} = Thumbelina.flip(image, :vertical)

    refute image.bytes == vertical_binary
    refute image.bytes == horizontal_binary
    assert vertical_binary == horizontal_binary
  end
end
