defmodule Day07 do
  def solve do
    equations = parse_equations("input/day07")

    p1 = total_calibration(equations, &solutions/1)
    IO.puts("Part1: #{p1}")

    p2 = total_calibration(equations, &solutions2/1)
    IO.puts("Part2: #{p2}")
  end

  def total_calibration(equations, solve) do
    equations
    |> Enum.filter(fn {result, operands} -> result in solve.(operands) end)
    |> Enum.map(fn {result, _} -> result end)
    |> Enum.sum()
  end

  def solutions([operand]), do: [operand]

  def solutions([operand | operands]) do
    solutions(operands)
    |> Enum.flat_map(fn s -> [operand * s, operand + s] end)
  end

  def solutions2([operand]), do: [operand]

  def solutions2([operand | operands]) do
    solutions2(operands)
    |> Enum.flat_map(fn s -> [operand * s, operand + s, String.to_integer("#{s}#{operand}")] end)
  end

  def parse_equations(input) do
    input
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(fn line ->
      [result, operands] = String.split(line, ":", parts: 2)
      result = String.to_integer(result)

      operands =
        operands
        |> String.split(" ", trim: true)
        |> Enum.map(&String.to_integer/1)
        |> Enum.reverse()

      {result, operands}
    end)
  end
end

Day07.solve()
