defmodule Thumbelina.Store do
  @moduledoc """
    Long Running Image GenServers for recieving processed image
    binary from the threaded NIF.
  """
  use GenServer

  alias Thumbelina.Internal
  alias Thumbelina.Image

  require Logger

  def start_link(_) do
    GenServer.start_link(__MODULE__, [], name: {:global, __MODULE__})
  end

  def resize(image = %Image{}, width, height) do
    GenServer.cast({:global, __MODULE__}, {:resize, image, width, height})
  end

  @impl true
  def init(_) do
    {:ok, []}
  end

  @impl true
  def handle_cast({:resize, image, width, height}, _) do
    Internal.cast(:resize, self(), image.bytes, image.extension, width, height)

    {:noreply, :ok}
  end

  @impl true
  def handle_info(request, _state) do
    Logger.info(inspect(request))
    {:noreply, {:ok, request}}
  end
end
