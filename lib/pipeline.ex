defmodule Thumbelina.Pipeline do
  @moduledoc """
    I/O orchestration and batch processing.
  """

  alias Thumbelina.{Consumer, Producer}

  def process() do
    # File.stream!("./archive/images/images, :line) |> Flow.from_enumerable()
    {:ok, counter} = GenStage.start_link(Producer, 0)
    {:ok, printer} = GenStage.start_link(Consumer, :ok)

    GenStage.sync_subscribe(printer, to: counter)
  end

  # def batch_resize(path) do
  #   File.stream!(path, :line) |> Flow.from_enumerable()
  # end

  # change image format
  def convert() do
    {:ok, counter} = GenStage.start_link(Producer, 0)
    nil
  end

  def save() do
    nil
  end

end
