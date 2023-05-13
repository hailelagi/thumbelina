defmodule Thumbelina.Image do
  @moduledoc """
    An image struct serialised in rust.
  """
  alias Thumbelina.Image

  @type t :: %__MODULE__{
          extension: :png | :svg | :jpeg,
          path: String.t(),
          height: non_neg_integer() | 0,
          width: non_neg_integer() | 0,
          bytes: [byte()],
        }

  defstruct [:extension, :path, bytes: <<0::255>>, height: 0, width: 0]

  def new(ext, path, bytes) do
    if valid_extension?(ext) do
      %Image{extension: ext, path: path, bytes: bytes}
    else
      {:error, "invalid image format"}
    end
  end

  defp valid_extension?(e), do: Enum.member?([".png", ".jpg", ".jpeg"], e)
end
