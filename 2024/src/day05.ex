defmodule Day05 do
  def solve do
    {rules, updates} = parse_input("input/day05")

    p1 =
      updates
      |> Enum.filter(fn update -> valid_update?(update, rules) end)
      |> Enum.map(fn update -> Enum.at(update, div(length(update), 2)) end)
      |> Enum.sum()

    IO.puts("Part1: #{p1}")

    p2 =
      updates
      |> Enum.filter(fn update -> not valid_update?(update, rules) end)
      |> Enum.map(fn update -> sort_update(update, rules) end)
      |> Enum.map(fn update -> Enum.at(update, div(length(update), 2)) end)
      |> Enum.sum()

    IO.puts("Part2: #{p2}")
  end

  def sort_update(update, rules) do
    Enum.sort(update, fn a, b ->
      afters = Map.get(rules, a, [])
      b in afters
    end)
  end

  def valid_update?(update, rules) do
    reduced =
      Enum.reduce_while(update, [], fn x, seen ->
        afters = Map.get(rules, x, [])

        if(Enum.any?(afters, fn a -> a in seen end)) do
          {:halt, []}
        else
          {:cont, [x | seen]}
        end
      end)

    length(reduced) == length(update)
  end

  def parse_input(input) do
    File.read!(input)
    |> String.split("\n\n", parts: 2)
    |> List.to_tuple()
    |> then(fn {rules, updates} -> {parse_rules(rules), parse_updates(updates)} end)
  end

  def parse_rules(rules_input) do
    rules_input
    |> String.split()
    |> Enum.map(fn rule -> String.split(rule, "|", parts: 2) end)
    |> Enum.group_by(fn [x, _] -> String.to_integer(x) end, fn [_, y] -> String.to_integer(y) end)
  end

  def parse_updates(updates_input) do
    updates_input
    |> String.split()
    |> Enum.map(fn update -> String.split(update, ",") |> Enum.map(&String.to_integer/1) end)
  end
end

Day05.solve()
