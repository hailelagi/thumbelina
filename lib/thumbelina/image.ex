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
          source: :disk | :in_memory
        }

  defstruct [:extension, :path, :source, bytes: <<0::255>>, height: 0, width: 0]

  def new(ext, path, bytes, src) do
    if supported_extension?(ext) do
      ext = String.replace_prefix(ext, ".", "")

      case src do
        :disk ->
          %Image{extension: ext, path: path, bytes: bytes, source: :disk}

        :in_memory ->
          %Image{extension: ext, path: "./thumbelina/temp/", bytes: bytes, source: :in_memory}

        _ ->
          {:error, "invalid image binary source."}
      end
    else
      {:error, "invalid image format"}
    end
  end

  defp supported_extension?(e), do: Enum.member?([".png", ".jpg", ".jpeg"], e)
end
