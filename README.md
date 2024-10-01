# Thumbelina

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fhailelagi%2Fthumbelina%2Fbadge%3Fref%3Dmain&style=flat)](https://actions-badge.atrox.dev/hailelagi/thumbelina/goto?ref=main)

Read the introduction article: https://www.hailelagi.com/hacks/thumbelina/

Rust backed erlang NIF for image processing. This is a fun idea/experiment combining the incredible I/O throughput and
concurrency features of the BEAM and the memory safety of rust.

## Alternatives
If running in production, consider an external service like [aws lambda](https://docs.aws.amazon.com/lambda/latest/dg/with-s3-tutorial.html) or:

- [sharp](https://sharp.pixelplumbing.com/)
- [image](https://github.com/kipcole9/image)
- [imaginary](https://github.com/h2non/imaginary)
- [imageflow](https://github.com/imazen/imageflow) (with the elixir binding)
- [vix](https://github.com/akash-akya/vix)
- [mogrify](https://github.com/elixir-mogrify/mogrify)
