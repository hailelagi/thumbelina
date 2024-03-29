# Thumbelina

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fhailelagi%2Fthumbelina%2Fbadge%3Fref%3Dmain&style=flat)](https://actions-badge.atrox.dev/hailelagi/thumbelina/goto?ref=main)

Read the introduction article: https://www.hailelagi.com/hacks/thumbelina/

Rust backed erlang NIF for image processing. This is a fun idea/experiment combining the incredible I/O throughput and
concurrency features of the BEAM and the memory safety of rust.

Things to consider:

1. The most [important reason](https://www.erlang.org/doc/man/erl_nif.html#lengthy_work), although rust is memory safe, and rustler catches panics before they unwind to the C interface, it is _not_ possible to co-operatively [schedule](https://github.com/erlang/otp/blob/maint/erts/emulator/internal_doc/ThreadProgress.md) a native call.

2. CPU vs GPU programming
(<https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html>)

3. The erlang vm isn't [bad at all number crunching](https://groups.google.com/g/erlang-programming/c/zsJRI_XzYPE), there's nuanced [fine print](https://www.erlang.org/doc/efficiency_guide/myths.html)

## Alternatives

Thanks to the maintainers of [image](https://github.com/kipcole9/image), thumbelina adopts a sub-set of its api design.
If running in production, consider an external service like [aws lambda](https://docs.aws.amazon.com/lambda/latest/dg/with-s3-tutorial.html) or:

- [sharp](https://sharp.pixelplumbing.com/)
- [image](https://github.com/kipcole9/image)
- [imaginary](https://github.com/h2non/imaginary)
- [imageflow](https://github.com/imazen/imageflow) (with the elixir binding)
- [vix](https://github.com/akash-akya/vix)
- [mogrify](https://github.com/elixir-mogrify/mogrify)
