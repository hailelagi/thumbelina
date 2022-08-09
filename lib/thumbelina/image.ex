defmodule Thumbelina.Image do
  @moduledoc false

  @type t :: %{
          extension: :png | :svg | :jpeg,
          path: String.t(),
          height: non_neg_integer(),
          width: non_neg_integer(),
          bytes: [byte()]
        }

  defstruct extension: nil, path: nil, height: nil, width: nil, bytes: []
end
