defmodule Thumbelina.Consumer do
  use GenStage

  @impl true
  def init(:ok) do
    {:consumer, nil}
  end

  @impl true
  def handle_events(events, _from, state) do
    Process.sleep(1000)
    IO.inspect(events)
    {:noreply, [], state}
  end
end
