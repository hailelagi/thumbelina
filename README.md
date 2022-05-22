# Thumbelina
Rust backed erlang NIF for image processing.

## features
- can process really large image data sets concurrently in batches.
- pause/resume processing.
- lazy and efficient for I/0 or CPU bound tasks.

## caveat
Thumbelina is likely not the right choice for processing small to medium data. It incurs costs on two fronts:
1. Startup overhead [opening a port](https://www.erlang.org/doc/tutorial/c_port.html) to the rust bridge.
2. Interprocess concurrent communication.

Consider using [mogrify](https://github.com/elixir-mogrify/mogrify) or an external service like [aws lambda](https://docs.aws.amazon.com/lambda/latest/dg/with-s3-tutorial.html).

## Installation
If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `thumbelina` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:thumbelina, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/thumbelina>.

