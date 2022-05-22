defmodule Thumbelina.MixProject do
  use Mix.Project

  @source_url "https://github.com/hailelagi/thumbelina"
  @version "0.1.0"


  def project do
    [
      app: :thumbelina,
      version: @version,
      elixir: "~> 1.13",
      name: "Thumbelina",
      source_url: @source_url,
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.24.0"},
      {:flow, "~> 1.0"},
      {:dialyxir, "~> 1.1", only: :dev, runtime: false},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false},
      {:excoveralls, "~> 0.13", only: :test}
    ]
  end
end
