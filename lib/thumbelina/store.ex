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

  def thumbnail(image = %Image{}, width, height) do
    GenServer.cast({:global, __MODULE__}, {:thumbnail, image, width, height})
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
  def handle_cast({:thumbnail, image, width, height}, _) do
    Internal.cast(:thumbnail, self(), image.bytes, image.extension, width, height)

    {:noreply, :ok}
  end

  @impl true
  def handle_info({:ok, image}, _state) do
    # simply write the successful serialised image struct to STDOUT
    # this would be some sort api consumer registered to pass along the pipeline
    Logger.info(image)

    {:noreply, {:ok, image}}
  end

  @impl true
  def handle_info({:error, err}, _state) do
    Logger.error(err)

    {:noreply, {:ok, err}}
  end
end
