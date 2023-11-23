defmodule Thumbelina.Handle do
  @moduledoc """
    Abstraction over a Network handle, could possibly
    implement a Reader/Writer interface and be Plug.Conn
    compliant?

    during compression the nif api would have to prediodically yield
    to the parent such that neither the active File Descriptor held
    on the elixir side is blocked, the scheduler async continues to
    stream the current bytes out to the Writer interface
  """

  defstruct [resource: nil, eference: nil]

end
