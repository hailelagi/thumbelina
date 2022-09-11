defmodule Thumbelina.Image do
  @moduledoc false

  alias Thumbelina.Internal

  @type t :: %{
          extension: :png | :svg | :jpeg,
          path: String.t(),
          height: non_neg_integer(),
          width: non_neg_integer(),
          bytes: [byte()]
        }

  defstruct extension: nil, path: nil, height: nil, width: nil, bytes: []

  def new(bytes) do
    nil
  end

  def resize(%__MODULE__{} = image, width, height) do
    Internal.resize(image.bytes, width, height)
  end
end
