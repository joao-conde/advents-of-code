defmodule Day01 do
  def solve do
    {left, right} = parse_lists("input/day01")

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
    |> Enum.map(&parse_line/1)
    |> Enum.reduce({[], []}, fn [x1, x2], {left, right} -> {[x1 | left], [x2 | right]} end)
    |> then(fn {left, right} -> {Enum.sort(left), Enum.sort(right)} end)
  end

  def parse_line(line) do
    String.split(line) |> Enum.map(&String.to_integer/1)
  end
end

Day01.solve()
