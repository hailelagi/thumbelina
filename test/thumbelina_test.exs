defmodule ThumbelinaTest do
  use ExUnit.Case
  alias Thumbelina.Image
  doctest Thumbelina

  test "it opens and resizes the sample image" do
    # read the image data into a binary
    {:ok, image} = Thumbelina.open("./example/abra.png")
    assert image.height == 0
    assert image.width == 0

    # it resizes the image in-memory
    assert {:ok, {img, _resized_binary}} = Thumbelina.resize(image, 50, 50)
    assert img.height == 50
    assert img.width == 50
  end
end
