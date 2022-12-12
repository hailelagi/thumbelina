defmodule Thumbelina.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    # todo: investigate passing back pressure configuration
    children = [
      {Thumbelina.Broadway, []}
    ]

    Supervisor.start_link(children, strategy: :one_for_one)
  end
end
