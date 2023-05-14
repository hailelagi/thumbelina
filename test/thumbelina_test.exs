defmodule ThumbelinaTest do
  use ExUnit.Case, async: true

  doctest Thumbelina

  setup do
    {:ok, image} = Thumbelina.open("./example/abra.png")

    %{image: image}
  end

  test "it reads an image binary into an %Image{}", %{image: image} do
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

  test "it resizes an example image", %{image: image} do
    assert {:ok, {img, resized_binary}} = Thumbelina.resize(image, 50, 50)

    assert img.height == 50
    assert img.width == 50
    refute image.bytes == resized_binary
  end

  test "it flips an example image", %{image: image} do
    assert {:ok, {_img, vertical_binary}} = Thumbelina.flip(image, :vertical)
    assert {:ok, {_img, horizontal_binary}} = Thumbelina.flip(image, :vertical)

    refute image.bytes == vertical_binary
    refute image.bytes == horizontal_binary
    assert vertical_binary == horizontal_binary
  end

  test "it rotates an example image", %{image: image} do
    assert {:ok, {_img, binary_90}} = Thumbelina.rotate(image, 90)
    assert {:ok, {_img, binary_180}} = Thumbelina.rotate(image, 180)
    assert {:ok, {_img, binary_270}} = Thumbelina.rotate(image, 270)
    assert {:ok, {_img, by_pixel_binary}} = Thumbelina.rotate(image, 69.420)

    refute image.bytes == binary_90
    refute image.bytes == binary_180
    refute image.bytes == binary_270
    refute image.bytes == by_pixel_binary
  end
end
