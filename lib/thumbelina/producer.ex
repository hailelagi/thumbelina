defmodule Thumbelina.Producer do
  use GenStage

  @impl true
  def init(counter) do
    {:producer, counter}
  end

  @impl true
  def handle_demand(demand, counter) do
    events = Enum.to_list(counter..counter+demand-1)
    {:noreply, events, counter + demand}
  end
end
