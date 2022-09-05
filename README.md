# Thumbelina

Rust backed erlang NIF for image processing. This was a fun idea/experiment combining the incredible I/O throughput and
concurrency features of the BEAM and the raw processing efficiency and memory safety of rust, it might turned out to be flawed. Here's why:

1. The most [important reason](https://www.erlang.org/doc/man/erl_nif.html#lengthy_work), although rust is memory safe, and rustler catches panics before they unwind to the C interface, it is _not_ possible to pre-empt scheduling of a native call.
I've thought about hacking around this with [enif_send](https://www.erlang.org/doc/man/erl_nif.html#enif_send) but I have yet to find a good attack on the problem.

2. alternative libraries are good enough (<https://sharp.pixelplumbing.com/>)

3. CPU vs GPU programming
(<https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html>)

4. The erlang vm isn't [bad at all number crunching](https://groups.google.com/g/erlang-programming/c/zsJRI_XzYPE), there's nuanced [fine print](https://www.erlang.org/doc/efficiency_guide/myths.html)

## goals/features

- can process really large image data sets concurrently in batches.
- pause/resume processing.
- lazy and efficient for I/0 or CPU bound tasks.

test data set used: <https://www.kaggle.com/datasets/vishalsubbiah/pokemon-images-and-types>

## caveat

Thumbelina is likely not the right choice for processing small to medium data. It incurs costs on two fronts:

1. Startup overhead [opening a port](https://www.erlang.org/doc/tutorial/c_port.html) to the rust bridge.
2. Interprocess concurrent communication.
3. rustler calls/NIF calls, cannot be pre-emptively scheduled.

Consider using [mogrify](https://github.com/elixir-mogrify/mogrify) or an external service like [aws lambda](https://docs.aws.amazon.com/lambda/latest/dg/with-s3-tutorial.html).
