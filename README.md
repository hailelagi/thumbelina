# Thumbelina

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fhailelagi%2Fthumbelina%2Fbadge%3Fref%3Dmain&style=flat)](https://actions-badge.atrox.dev/hailelagi/thumbelina/goto?ref=main)

Rust backed erlang NIF for image processing. This is a fun idea/experiment combining the incredible I/O throughput and
concurrency features of the BEAM and the memory safety of rust. 

# Performance
This is an incredibly.. not at all well behaved NIF at the moment, especially with the basic `synchronous` api which after some basic benchmarks turns out to take about ~2seconds to process an ~2GiG jpeg stalling work stealing BEAM scheduling. Tldr; this is poorly optimised right now.

```
Operating System: macOS
CPU Information: Apple M1
Number of Available Cores: 8
Available memory: 8 GB
Elixir 1.14.4
Erlang 25.3

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 10 s
memory time: 2 s
reduction time: 0 ns
parallel: 1
inputs: none specified
Estimated total run time: 28 s

Benchmarking async resize 2GiG ...
Benchmarking sync resize 2GiG ...

Name                        ips        average  deviation         median         99th %
async resize 2GiG        351.73      0.00284 s  ±1241.40%      0.00035 s       0.0410 s
sync resize 2GiG           0.48         2.06 s     ±5.65%         2.07 s         2.18 s

Comparison: 
async resize 2GiG        351.73
sync resize 2GiG           0.48 - 726.30x slower +2.06 s
```

## Design
<design_diagram.png>

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
