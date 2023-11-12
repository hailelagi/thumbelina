defmodule ThumbelinaTest do
  use ExUnit.Case, async: true

  doctest Thumbelina

  setup do
    {:ok, image} = Thumbelina.open("./example/abra.png")

    %{image: image}
  end

  describe "sync api" do
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
      assert {:ok, resized} = Thumbelina.resize(image, 50, 50)

      assert resized.height == 50
      assert resized.width == 50
      refute image.bytes == resized.bytes
    end

    test "it makes a thumbnail!", %{image: image} do
      assert {:ok, thumbnail} = Thumbelina.thumbnail(image, 50, 50)

      assert thumbnail.height == 50
      assert thumbnail.width == 50
      refute image.bytes == thumbnail.bytes
    end

    test "it flips an example image", %{image: image} do
      assert {:ok, vertical} = Thumbelina.flip(image, :vertical)
      assert {:ok, horizontal} = Thumbelina.flip(image, :vertical)

      refute image.bytes == vertical.bytes
      refute image.bytes == horizontal.bytes
      assert vertical.bytes == horizontal.bytes
    end

    test "it rotates an example image", %{image: image} do
      assert {:ok, binary_90} = Thumbelina.rotate(image, 90)
      assert {:ok, binary_180} = Thumbelina.rotate(image, 180)
      assert {:ok, binary_270} = Thumbelina.rotate(image, 270)
      assert {:ok, by_pixel_binary} = Thumbelina.rotate(image, 69.420)

      refute image.bytes == binary_90.bytes
      refute image.bytes == binary_180.bytes
      refute image.bytes == binary_270.bytes
      refute image.bytes == by_pixel_binary.bytes
    end

    test "it adds a guassian blur on an image", %{image: image} do
      assert {:ok, blur} = Thumbelina.blur(image, 69.420)

      refute image.bytes == blur.bytes
    end

    test "it brigtens an image", %{image: image} do
      assert {:ok, bright} = Thumbelina.brighten(image, 69.420)

      refute image.bytes == bright.bytes
    end

    test "it blurs an image", %{image: image} do
      assert {:ok, bright} = Thumbelina.blur(image, 69.420)

      refute image.bytes == bright.bytes
    end
  end

  describe "parallel api" do
    test "it synchronously parallelizes resizing many image binaries with the same format" do
      entries = Thumbelina.open_all!("./example/pokemon")

      entries = Enum.map(entries, fn {:ok, e} -> e.bytes end)
      assert {:ok, entries} = Thumbelina.Internal.batch(:resize, entries, "png", 50.00, 50.00)

      assert length(entries) == 3
    end
  end

  describe "async api" do
    test "it asynchronously schedules an image operation", %{image: image} do
      Thumbelina.Internal.cast(
        :resize,
        self(),
        image.bytes,
        image.extension,
        50.0,
        50.0
      )

      assert_receive {:ok, %{__struct__: Thumbelina.Image, bytes: bytes}}, 2000

      assert image.bytes != bytes
    end
  end
end
