defmodule Day08 do
  def solve do
    {antennas, width, height} = parse_antennas("input/day08")

    p1 = antinode_locations(antennas, width, height, 1..1)
    IO.puts("Part1: #{p1}")

    max_repeat = max(width, height)
    p2 = antinode_locations(antennas, width, height, 0..max_repeat)
    IO.puts("Part2: #{p2}")
  end

  def antinode_locations(antennas, width, height, repeat_range) do
    antennas
    |> Map.values()
    |> Enum.flat_map(fn locations ->
      locations
      |> pairings()
      |> Enum.flat_map(fn {l1, l2} -> antinodes(l1, l2, repeat_range) end)
    end)
    |> Enum.filter(fn location -> in_bounds?(location, width, height) end)
    |> MapSet.new()
    |> MapSet.size()
  end

  def antinodes({i1, j1}, {i2, j2}, repeat_range) do
    {vi, vj} = {i2 - i1, j2 - j1}

    List.flatten(
      for i <- repeat_range, do: [{i1 - vi * i, j1 - vj * i}, {i2 + vi * i, j2 + vj * i}]
    )
  end

  def pairings(locations) do
    Enum.flat_map(locations, fn l1 ->
      Enum.map(locations -- [l1], fn l2 -> {l1, l2} end)
    end)
  end

  def in_bounds?({i, j}, width, height) do
    i >= 0 && i < width && j >= 0 && j < height
  end

  def parse_antennas(input) do
    grid =
      input
      |> File.read!()
      |> String.split("\n")
      |> Enum.map(&String.graphemes/1)

    width = length(grid)
    height = length(Enum.at(grid, 0))

    antennas =
      grid
      |> Enum.with_index()
      |> Enum.flat_map(fn {line, i} ->
        line
        |> Enum.with_index()
        |> Enum.filter(fn {char, _} -> char != "." end)
        |> Enum.map(fn {char, j} -> {i, j, char} end)
      end)
      |> Enum.group_by(fn {_, _, char} -> char end, fn {i, j, _} -> {i, j} end)
      |> Map.new()

    {antennas, width, height}
  end
end

Day08.solve()
