defmodule Thumbelina.Image do
  @moduledoc """
    An image struct serialised in rust
  """
  alias Thumbelina.Internal

  @type t :: %__MODULE__{
          extension: :png | :svg | :jpeg,
          path: String.t(),
          height: non_neg_integer(),
          width: non_neg_integer(),
          bytes: [byte()]
        }

  defstruct [:extension, :path, :height, :width, bytes: []]

  def new(ext, path, bytes) do
    if valid_extension?(ext) do
      Internal.serialize(ext, path, bytes)
    else
      {:error, "invalid image format"}
    end
  end

  # def resize(%__MODULE__{} = image, width, height) do
  #   Internal.resize(image.bytes, width, height)
  # end

  defp valid_extension?(e), do: Enum.member?([".png", ".jpg", ".jpeg"], e)
end
