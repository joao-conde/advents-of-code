defmodule Day05 do
  def solve do
    {rules, updates} = parse_input("input/day05")

    p1 = ordered_updates(updates, rules) |> score_updates()
    IO.puts("Part1: #{p1}")

    p2 = unordered_updates(updates, rules) |> sort_updates(rules) |> score_updates()
    IO.puts("Part2: #{p2}")
  end

  def ordered_updates(updates, rules) do
    Enum.filter(updates, fn update -> ordered_update?(update, rules) end)
  end

  def unordered_updates(updates, rules) do
    Enum.filter(updates, fn update -> not ordered_update?(update, rules) end)
  end

  def score_updates(updates) do
    Enum.map(updates, fn update -> Enum.at(update, div(length(update), 2)) end) |> Enum.sum()
  end

  def sort_updates(updates, rules) do
    Enum.map(updates, fn update -> sort_update(update, rules) end)
  end

  def sort_update(update, rules) do
    Enum.sort(update, fn a, b ->
      Map.get(rules, a, []) |> Enum.member?(b)
    end)
  end

  def ordered_update?(update, rules) do
    reduced =
      Enum.reduce_while(update, [], fn x, seen ->
        come_after = Map.get(rules, x, [])

        if Enum.any?(come_after, fn a -> a in seen end) do
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
    |> then(fn [rules, updates] -> {parse_rules(rules), parse_updates(updates)} end)
  end

  def parse_rules(rules_input) do
    rules_input
    |> String.split()
    |> Enum.map(fn rule -> String.split(rule, "|", parts: 2) end)
    |> Enum.group_by(
      fn [x, _] -> String.to_integer(x) end,
      fn [_, y] -> String.to_integer(y) end
    )
  end

  def parse_updates(updates_input) do
    updates_input
    |> String.split()
    |> Enum.map(fn update -> String.split(update, ",") |> Enum.map(&String.to_integer/1) end)
  end
end

Day05.solve()
