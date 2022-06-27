defmodule Thumbelina do
  @moduledoc """
  Documentation for `Thumbelina`.

  todo: scope out api read rust docs
  https://docs.rs/image/latest/image/

  - can process really large image data sets
  - pause/resume processing saving memory
  - lazy and efficient runtime
  """
  alias Thumbelina.{Consumer, Producer}
  use Rustler, otp_app: :thumbelina, crate: "thumbelina"

  # resize an image
  def resize() do
    # File.stream!(path, :line) |> Flow.from_enumerable()
    {:ok, counter} = GenStage.start_link(Prod, nil)
    {:ok, printer} = GenStage.start_link(Consumer, :ok)

    GenStage.sync_subscribe(printer, to: counter)
  end

  # def batch_resize(path) do
  #   File.stream!(path, :line) |> Flow.from_enumerable()
  # end

  # change image format
  def convert() do
    {:ok, counter} = GenStage.start_link(Producer, nil)
    nil
  end

  ## GenStage
  # elixir process that asks for data (message contract)
  # back pressure to producer
  # producer -> producer_consumer -> consumer
  # [event, event, event]
  # demand dispatcher
  # save
  def save() do
    nil
  end
end
