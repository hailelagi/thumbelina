defmodule Thumbelina.Image do
  @moduledoc """
    An image struct serialised in rust
  """
  alias Thumbelina.Internal

  @type t :: %{
          extension: :png | :svg | :jpeg,
          path: String.t(),
          height: non_neg_integer(),
          width: non_neg_integer(),
          bytes: [byte()]
        }

  defstruct extension: nil, path: nil, height: nil, width: nil, bytes: []

  def new(ext, path, bytes) do
    if valid_extension?(ext) do
      Options.new(ext, path, bytes)
      |> Internal.serialize(bytes)
      |> struct!(__MODULE__)
    else
      {:error, "invalid image format"}
    end
  end

  def resize(%__MODULE__{} = image, width, height) do
    Internal.resize(image.bytes, width, height)
  end

  defp valid_extension?(e), do: Enum.member?([:png, :svg, :jpeg], e)

  defmodule Options do
    @type t :: %{
            extension: :png | :svg | :jpeg,
            path: String.t(),
          }

    defstruct extension: nil, path: nil, bytes: nil

    def new(e, p), do: struct!(__MODULE__, extension: e, path: p)
  end
end
