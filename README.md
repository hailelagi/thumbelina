# Thumbelina
Rust backed erlang NIF for image processing. This was a fun idea/experiment combining the incredibly I/O throughput and 
concurrency features of the BEAM and the raw processing efficiency and memory safety of rust, it turned out to be flawed. Here's why:

1. CPU vs GPU programming
(https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html)

2. alternative libraries are good enough (https://sharp.pixelplumbing.com/)

3. The erlang vm isn't [bad at all number crunching](https://groups.google.com/g/erlang-programming/c/zsJRI_XzYPE), there's nuanced [fine print](https://www.erlang.org/doc/efficiency_guide/myths.html)

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

