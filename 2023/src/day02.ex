defmodule Day02 do
  @alias %{
    "X" => "A",
    "Y" => "B",
    "Z" => "C"
  }

  @weakness %{
    "A" => "B",
    "B" => "C",
    "C" => "A"
  }

  @points %{
    "A" => 1,
    "B" => 2,
    "C" => 3
  }

  def main do
    rounds = get_rounds()
    p1 = Enum.map(rounds, fn [op, me] -> score(@alias[me], op) end) |> Enum.sum()
    p2 = Enum.map(rounds, fn [op, strat] -> score(move(op, strat), op) end) |> Enum.sum()
    IO.puts("Part1: #{p1}")
    IO.puts("Part2: #{p2}")
  end

  def get_rounds() do
    File.read!("input/day02")
    |> String.split("\n")
    |> Enum.map(fn r -> String.split(r, " ") end)
  end

  def score(me, op) do
    @points[me] +
      cond do
        op == me -> 3
        @weakness[me] != op -> 6
        true -> 0
      end
  end

  def move(op, strat) do
    case strat do
      "X" -> Enum.find(@weakness, fn {_, v} -> v == op end) |> elem(0)
      "Y" -> op
      _ -> @weakness[op]
    end
  end
end

Day02.main()
