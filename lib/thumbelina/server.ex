defmodule Thumbelina.Server do
  @moduledoc """
    Long Running Image GenServers for recieving processed image
    binary from the threaded NIF.
  """
  use GenServer

  require Logger

  def start_link do
    # :global registration if configured
    # else use named local
    {:ok, nil}
  end

  def init(_) do
    {:ok, nil}
  end
end
