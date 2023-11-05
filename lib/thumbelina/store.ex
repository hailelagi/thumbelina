defmodule Thumbelina.Store do
  @moduledoc """
    Long Running Image GenServers for recieving processed image
    binary from the threaded NIF.
  """
  use GenServer

  require Logger

  def start_link(_) do
    # :global registration default
    GenServer.start_link(__MODULE__, [])
  end

  def init(_) do
    {:ok, nil}
  end

  def handle_info(request, _state) do
    IO.inspect(inspect(request))

    {:noreply, {:ok, request}}
  end
end
