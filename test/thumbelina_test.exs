defmodule ThumbelinaTest do
  use ExUnit.Case
  doctest Thumbelina

  test "greets the world" do
    assert Thumbelina.hello() == :world
  end
end
