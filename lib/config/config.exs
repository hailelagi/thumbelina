import Config

config :thumbelina,
  workers: 4,
  mode: :global

import_config("#{config_env}.exs")
