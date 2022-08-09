defmodule Thumbelina do
  @moduledoc """
    Library public api
  """

  alias Thumbelina.Image

  # todo: read file data and pass to bytes
  @spec image_to_bytes() ::  {:ok, Image.t()} | {:error, String.t()}
  def image_to_bytes() do
    Image.new()
  end
end
