defmodule Day03 do
  def solve do
    p1 = parse_instructions("input/day03", false) |> process_muls
    IO.puts("Part1: #{p1}")

    p2 = parse_instructions("input/day03", true) |> process_muls
    IO.puts("Part2: #{p2}")
  end

  def parse_instructions(input, include_do) do
    text = File.read!(input)

    Regex.scan(~r/mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))/, text, capture: :all_but_first)
    |> Enum.map(fn matches ->
      Enum.filter(matches, fn match -> valid_match(match, include_do) end)
    end)
    |> Enum.filter(fn matches -> !Enum.empty?(matches) end)
    |> remove_disabled
  end

  def remove_disabled(instructions) do
    List.foldl(instructions, {[], true}, fn match, {muls, enabled} ->
      case match do
        ["do()"] ->
          {muls, true}

        ["don't()"] ->
          {muls, false}

        _ ->
          cond do
            enabled -> {[match | muls], true}
            !enabled -> {muls, false}
          end
      end
    end)
    |> elem(0)
  end

  def valid_match(match, include_do) do
    case match do
      "" -> false
      "do()" -> include_do
      "don't()" -> include_do
      _ -> true
    end
  end

  def process_muls(muls) do
    Enum.map(muls, fn operands -> Enum.map(operands, &String.to_integer/1) end)
    |> Enum.map(fn [x, y] -> x * y end)
    |> Enum.sum()
  end
end

Day03.solve()
