defmodule Day01 do
  def solve do
    [left, right] = parse_lists("input/day01")

    p1 = distance(left, right)
    IO.puts("Part1: #{p1}")

    p2 = similarity(left, right)
    IO.puts("Part2: #{p2}")
  end

  def distance(left, right) do
    Enum.zip(left, right)
    |> Enum.map(fn {x1, x2} -> abs(x2 - x1) end)
    |> Enum.sum()
  end

  def similarity(left, right) do
    frequencies = Enum.frequencies(right)

    Enum.map(left, fn x1 -> x1 * Map.get(frequencies, x1, 0) end)
    |> Enum.sum()
  end

  def parse_lists(input) do
    File.read!(input)
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn pair -> Enum.map(pair, &String.to_integer/1) end)
    |> Enum.reduce([[], []], fn [x1, x2], [left, right] -> [[x1 | left], [x2 | right]] end)
    |> Enum.map(fn list -> Enum.sort(list) end)
  end
end

Day01.solve()
