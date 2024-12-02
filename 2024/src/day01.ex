[left, right] =
  File.read!("input/day01")
  |> String.split("\n")
  |> Enum.map(&String.split/1)
  |> Enum.map(fn pair -> Enum.map(pair, &String.to_integer/1) end)
  |> Enum.reduce([[], []], fn [x1, x2], [left, right] -> [[x1 | left], [x2 | right]] end)
  |> Enum.map(fn list -> Enum.sort(list) end)

# part one
Enum.zip(left, right)
|> Enum.map(fn {x1, x2} -> abs(x2 - x1) end)
|> Enum.sum()
|> IO.inspect(label: "Part1")

# part two
frequencies = Enum.frequencies(right)

Enum.map(left, fn x1 -> x1 * Map.get(frequencies, x1, 0) end)
|> Enum.sum()
|> IO.inspect(label: "Part2")
