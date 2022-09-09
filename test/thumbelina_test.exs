defmodule ThumbelinaTest do
  use ExUnit.Case
  doctest Thumbelina

  test "greets the world" do
    assert Thumbelina.Internal.echo("hello") == "hello"
    assert Thumbelina.Internal.add(1, 2) == 3
  end
end
