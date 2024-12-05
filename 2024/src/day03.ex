defmodule Day03 do
  def solve do
    p1 = parse_instructions("input/day03", false) |> process_muls
    IO.puts("Part1: #{p1}")

    p2 = parse_instructions("input/day03", true) |> process_muls
    IO.puts("Part2: #{p2}")
  end

  def parse_instructions(input, include_do) do
    File.read!(input)
    |> parse_matches(include_do)
    |> remove_disabled_instructions()
  end

  def parse_matches(text, include_do) do
    Regex.scan(~r/mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))/, text, capture: :all_but_first)
    |> Enum.map(&filter_matches(&1, include_do))
    |> Enum.reject(&Enum.empty?/1)
  end

  def filter_matches(matches, include_do) do
    Enum.filter(matches, &valid_match?(&1, include_do))
  end

  def valid_match?(match, include_do) do
    case match do
      "" -> false
      "do()" -> include_do
      "don't()" -> include_do
      _ -> true
    end
  end

  def remove_disabled_instructions(instructions) do
    Enum.reduce(instructions, {[], true}, fn match, {muls, enabled} ->
      case match do
        ["do()"] -> {muls, true}
        ["don't()"] -> {muls, false}
        _ when enabled -> {[match | muls], enabled}
        _ -> {muls, enabled}
      end
    end)
    |> elem(0)
  end

  def process_muls(muls) do
    Enum.map(muls, fn operands -> Enum.map(operands, &String.to_integer/1) end)
    |> Enum.map(fn [x, y] -> x * y end)
    |> Enum.sum()
  end
end

Day03.solve()
