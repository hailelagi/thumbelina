defmodule Thumbelina.Thumbnail do
  @moduledoc """
    todo: Convenience ds for manipulating an image
  """
  @type t :: %{
          extension: :png | :svg | :jpeg,
          path: String.t(),
          height: non_neg_integer(),
          width: non_neg_integer()
        }

  defstruct extension: nil, path: nil, height: nil, width: nil
end
