defmodule Day11 do
  def solve do
    stones_count = parse_stones("input/day11")

    p1 = blinks(stones_count, 25) |> total_stones
    IO.puts("Part1: #{p1}")

    p2 = blinks(stones_count, 75) |> total_stones
    IO.puts("Part2: #{p2}")
  end

  def total_stones(stones_count) do
    stones_count
    |> Map.values()
    |> Enum.sum()
  end

  def blinks(stones_count, times) do
    1..times |> Enum.reduce(stones_count, fn _, counts -> blink(counts) end)
  end

  def blink(stones_count) do
    Enum.reduce(stones_count, %{}, fn {stone, count}, counts ->
      next_stones(stone)
      |> Enum.reduce(counts, fn s, acc -> Map.update(acc, s, count, fn c -> c + count end) end)
    end)
  end

  def next_stones(stone) do
    case split_half(stone) do
      _ when stone == 0 ->
        [1]

      {left, right} ->
        [left, right]

      _ ->
        [stone * 2024]
    end
  end

  def split_half(stone) do
    string = Integer.to_string(stone)
    digit_count = String.length(string)

    if rem(digit_count, 2) == 0 do
      String.split_at(string, div(digit_count, 2))
      |> then(fn {left, right} -> {String.to_integer(left), String.to_integer(right)} end)
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
