defmodule Day11 do
  def solve do
    input = "8435 234 928434 14 0 7 92446 8992692"

    stones_count =
      input
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)
      |> Enum.frequencies()

    1..25
    |> Enum.reduce(stones_count, fn _, counts ->
      blink(counts)
    end)
    |> Map.values()
    |> Enum.sum()
    |> IO.inspect()

    1..75
    |> Enum.reduce(stones_count, fn _, counts ->
      blink(counts)
    end)
    |> Map.values()
    |> Enum.sum()
    |> IO.inspect()
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
end

Day11.solve()
