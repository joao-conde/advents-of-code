defmodule Day13 do
  @prize_offset 10_000_000_000_000

  def solve do
    machines = parse_input("input/day13")

    p1 = fewest_tokens(machines)
    IO.puts("Part1: #{p1}")

    p2 = machines |> offset_prizes(@prize_offset) |> fewest_tokens
    IO.puts("Part2: #{p2}")
  end

  def fewest_tokens(machines) do
    machines
    |> Enum.map(&compute_pushes/1)
    |> Enum.reject(&is_nil/1)
    |> Enum.map(&token_cost/1)
    |> Enum.sum()
  end

  def compute_pushes({{ax, ay}, {bx, by}, {px, py}}) do
    b = round((py - ay * px / ax) / (by - ay * bx / ax))
    a = round((px - bx * b) / ax)

    total_x = a * ax + b * bx
    total_y = a * ay + b * by

    if total_x == px and total_y == py do
      {a, b}
    else
      nil
    end
  end

  def token_cost({pushes_a, pushes_b}) do
    3 * pushes_a + pushes_b
  end

  def offset_prizes(machines, offset) do
    Enum.map(machines, fn {btn_a, btn_b, {px, py}} ->
      {btn_a, btn_b, {px + offset, py + offset}}
    end)
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
