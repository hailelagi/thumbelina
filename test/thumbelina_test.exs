defmodule ThumbelinaTest do
  use ExUnit.Case
  doctest Thumbelina

  test "it opens and crops the sample image" do
    assert {:ok, _} = Thumbelina.open("./bench/abra.png")
  end
end
