import Config

config :thumbelina,
  workers: 4,
  mode: :global

config :rustler_precompiled, :force_build, thumbelina: true

import_config("#{config_env}.exs")
