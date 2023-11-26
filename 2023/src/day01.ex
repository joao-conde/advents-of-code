calories =
  "input/day01"
  |> File.read!()
  |> String.split("\n\n")
  |> Enum.map(fn b -> b |> String.split() |> Enum.map(&String.to_integer/1) |> Enum.sum() end)
  |> Enum.sort()

p1 = calories |> List.last()
p2 = calories |> Enum.take(-3) |> Enum.sum()

IO.puts("Part1: #{p1}")
IO.puts("Part2: #{p2}")
