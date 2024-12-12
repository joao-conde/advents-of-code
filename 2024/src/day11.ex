defmodule Day11 do
  def solve do
    stones_count = parse_stones("input/day11")

    p1 = stones_count |> blinks(25) |> total_stones()
    IO.puts("Part1: #{p1}")

    p2 = stones_count |> blinks(75) |> total_stones()
    IO.puts("Part2: #{p2}")
  end

  def total_stones(stones_count) do
    stones_count
    |> Map.values()
    |> Enum.sum()
  end

  def blinks(stones_count, times) do
    Enum.reduce(1..times, stones_count, fn _, counts -> blink(counts) end)
  end

  def blink(stones_count) do
    Enum.reduce(stones_count, %{}, fn {stone, count}, counts ->
      next_stones(stone)
      |> Enum.reduce(counts, fn s, acc -> Map.update(acc, s, count, fn c -> c + count end) end)
    end)
  end

  def next_stones(stone) do
    cond do
      stone == 0 -> [1]
      halves = split_half(stone) -> Tuple.to_list(halves)
      true -> [stone * 2024]
    end
  end

  def split_half(stone) do
    string = Integer.to_string(stone)
    digit_count = String.length(string)

    if rem(digit_count, 2) == 0 do
      {left, right} = String.split_at(string, div(digit_count, 2))
      {String.to_integer(left), String.to_integer(right)}
    else
      nil
    end
  end

  def parse_stones(input) do
    input
    |> File.read!()
    |> String.split(" ")
    |> Enum.map(&String.to_integer/1)
    |> Enum.frequencies()
  end
end

Day11.solve()
