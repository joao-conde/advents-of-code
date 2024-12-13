defmodule Day13 do
  def solve do
    machines = parse_input("input/day13")

    p1 =
      machines
      |> Enum.map(fn machine -> solution(machine) end)
      |> Enum.filter(fn sol -> sol != nil end)
      |> Enum.map(fn {pushes_a, pushes_b, _, _} -> token_cost(pushes_a, pushes_b) end)
      |> Enum.sum()

    IO.puts("Part1: #{p1}")

    p2 = 0
    IO.puts("Part2: #{p2}")
  end

  def solution({{ax, ay}, {bx, by}, {px, py}}) do
    Enum.flat_map(1..100, fn i -> Enum.map(1..100, fn j -> {i, j} end) end)
    |> Enum.map(fn {push_a, push_b} ->
      {push_a, push_b, push_a * ax + push_b * bx, push_a * ay + push_b * by}
    end)
    |> Enum.filter(fn {_, _, x, y} -> x == px and y == py end)
    |> Enum.sort(fn {a1, b1, _, _}, {a2, b2, _, _} -> a1 + b1 < a2 + b2 end)
    |> List.first()
  end

  def token_cost(pushes_a, pushes_b) do
    3 * pushes_a + pushes_b
  end

  def parse_input(input) do
    input
    |> File.read!()
    |> String.split("\n\n")
    |> Enum.map(&parse_machine/1)
  end

  def parse_machine(block) do
    [
      ~r/Button A: X\+(\d+), Y\+(\d+)/,
      ~r/Button B: X\+(\d+), Y\+(\d+)/,
      ~r/Prize: X=(\d+), Y=(\d+)/
    ]
    |> Enum.flat_map(fn regex -> Regex.scan(regex, block, capture: :all_but_first) end)
    |> Enum.map(fn [x, y] -> {String.to_integer(x), String.to_integer(y)} end)
    |> List.to_tuple()
  end
end

Day13.solve()
