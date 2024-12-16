defmodule MixProject do
  use Mix.Project

  def project do
    [
      app: :aoc2024,
      version: "0.1.0",
      elixir: "~> 1.17.3",
      deps: deps(),
      aliases: aliases()
    ]
  end

  defp deps do
    [
      {:heap, "~> 3.0.0"},
      {:credo, "~> 1.7.10", only: [:dev], runtime: false}
    ]
  end

  defp aliases do
    [
      lint: "credo",
      precommit: [
        "format",
        "lint"
      ]
    ]
  end
end
