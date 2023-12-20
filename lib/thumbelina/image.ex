defmodule Thumbelina.Image do
  @moduledoc """
    An image struct serialised in rust.
  """
  @type t :: %__MODULE__{
          extension: :png | :svg | :jpeg,
          height: non_neg_integer() | 0,
          width: non_neg_integer() | 0,
          bytes: [byte()],
          compressed: boolean()
        }

  defstruct [
    :extension,
    bytes: <<0::255>>,
    height: 0,
    width: 0,
    compressed: false
  ]

  def new(ext, bytes) do
    if supported_extension?(ext) do
      ext = String.replace_prefix(ext, ".", "")
      %Thumbelina.Image{extension: ext, bytes: bytes}

    else
      {:error, "invalid image format"}
    end
  end

  defp supported_extension?(e),
    do:
      Enum.member?(
        [
          ".png",
          ".jpg",
          ".jpeg",
          ".gif",
          ".pnm",
          ".webp",
          ".tiff",
          ".tga",
          ".dds",
          ".bmp",
          ".ico",
          ".hdr",
          ".openexr",
          ".farbfeld",
          ".avif",
          ".qoi"
        ],
        e
      )
end
