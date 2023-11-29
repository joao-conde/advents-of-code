defmodule Day01 do
  def main do
    calories = get_calories()
    p1 = calories |> List.last()
    p2 = calories |> Enum.take(-3) |> Enum.sum()
    IO.puts("Part1: #{p1}")
    IO.puts("Part2: #{p2}")
  end

  def get_calories() do
    File.read!("input/day01")
    |> String.split("\n\n")
    |> Enum.map(fn b -> b |> String.split() |> Enum.map(&String.to_integer/1) |> Enum.sum() end)
    |> Enum.sort()
  end
end

Day01.main()
