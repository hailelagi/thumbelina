defmodule Thumbelina.MixProject do
  use Mix.Project

  @source_url "https://github.com/hailelagi/thumbelina"
  @version "0.0.1"

  def project do
    [
      app: :thumbelina,
      version: @version,
      elixir: "~> 1.13",
      name: "Thumbelina",
      source_url: @source_url,
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: [
        licenses: ["MIT"],
        description:
          "Image processing and compression in the BEAM virtual machine via rust bindings.",
        maintainers: ["hailelagi"],
        licenses: ["MIT"],
        links: %{"GitHub" => @source_url}
      ]
    ]
  end

  def application do
    [
      mod: {Thumbelina.Application, []},
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.24.0", optional: true},
      {:rustler_precompiled, "~> 0.7"},
      {:dialyxir, "~> 1.1", only: :dev, runtime: false},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false},
      {:benchee, "~> 1.0", only: :dev},
      {:excoveralls, "~> 0.13", only: :test}
    ]
  end
end
