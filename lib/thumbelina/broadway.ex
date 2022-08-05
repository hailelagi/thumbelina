defmodule Thumbelina.Broadway do
  use Broadway

  alias Thumbelina.Producer

  def start_link(_opts) do
    Broadway.start_link(Thumbelina.Broadway,
      name: Thumbelina.Broadway,
      producer: [
        module: {Producer, []},
        concurrency: 1
      ],
      processors: [
        default: [concurrency: 2]
      ]
    )
  end

  def handle_message(_, _, _) do
    nil
  end
end
