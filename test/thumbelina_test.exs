defmodule ThumbelinaTest do
  use ExUnit.Case
  alias Thumbelina.Image
  doctest Thumbelina

  test "it opens and resizes an example image" do
    # read the image data into a binary
    {:ok, image} = Thumbelina.open("./example/abra.png")
    assert image.height == 0
    assert image.width == 0

    # it resizes the image in-memory
    assert {:ok, {img, _resized_binary}} = Thumbelina.resize(image, 50, 50)
    assert img.height == 50
    assert img.width == 50
  end

  test "it opens and resizes all the example images lazily" do
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
end
