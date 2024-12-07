defmodule Day07 do
  def solve do
    equations = parse_equations("input/day07")

    p1 = calibration_total(equations, [&Kernel.*/2, &Kernel.+/2])
    IO.puts("Part1: #{p1}")

    p2 = calibration_total(equations, [&Kernel.*/2, &Kernel.+/2, &concat/2])
    IO.puts("Part2: #{p2}")
  end

  def calibration_total(equations, operators) do
    equations
    |> Enum.filter(fn {result, operands} -> result in solutions(operands, operators) end)
    |> Enum.map(fn {result, _} -> result end)
    |> Enum.sum()
  end

  def solutions([operand], _), do: [operand]

  def solutions([operand | operands], operators) do
    operands
    |> solutions(operators)
    |> Enum.flat_map(fn s ->
      Enum.map(operators, fn operator -> operator.(operand, s) end)
    end)
  end

  def concat(x, y), do: String.to_integer("#{y}#{x}")

  def parse_equations(input) do
    input
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_equation/1)
  end

  def parse_equation(line) do
    [result, operands] = String.split(line, ":", parts: 2)

    result = String.to_integer(result)

    operands =
      operands
      |> String.split(" ", trim: true)
      |> Enum.map(&String.to_integer/1)
      |> Enum.reverse()

    {result, operands}
  end
end

Day07.solve()
