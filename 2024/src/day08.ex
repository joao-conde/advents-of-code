defmodule Day08 do
  def solve do
    {antennas, width, height} = parse_antennas("input/day08")

    p1 =
      antennas
      |> Map.values()
      |> Enum.flat_map(fn a ->
        a
        |> pairings()
        |> Enum.flat_map(fn {{i1, j1}, {i2, j2}} ->
          {vi, vj} = vector({i1, j1}, {i2, j2})
          [{i1 - vi, j1 - vj}, {i2 + vi, j2 + vj}]
        end)
      end)
      |> Enum.filter(fn location ->
        not out_of_bounds(location, width, height)
      end)
      |> MapSet.new()
      |> MapSet.size()

    IO.puts("Part1: #{p1}")

    p2 =
      antennas
      |> Map.values()
      |> Enum.flat_map(fn a ->
        a
        |> pairings()
        |> Enum.flat_map(fn {{i1, j1}, {i2, j2}} ->
          {vi, vj} = vector({i1, j1}, {i2, j2})

          0..100
          |> Enum.flat_map(fn i -> [{i1 - vi * i, j1 - vj * i}, {i2 + vi * i, j2 + vj * i}] end)
        end)
      end)
      |> Enum.filter(fn location ->
        not out_of_bounds(location, width, height)
      end)
      |> MapSet.new()
      |> MapSet.size()

    IO.puts("Part2: #{p2}")
  end

  def pairings(antennas) do
    antennas
    |> Enum.with_index()
    |> Enum.flat_map(fn {a1, i} ->
      antennas
      |> Enum.drop(i)
      |> Enum.filter(fn a2 -> a1 != a2 end)
      |> Enum.map(fn a2 -> {a1, a2} end)
    end)
  end

  def out_of_bounds({i, j}, width, height) do
    i < 0 || i >= width || j < 0 || j >= height
  end

  def vector({i1, j1}, {i2, j2}) do
    {i2 - i1, j2 - j1}
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
        |> Enum.map(fn {char, j} -> {i, j, char} end)
        |> Enum.filter(fn {_, _, char} -> char != "." end)
      end)
      |> Enum.group_by(fn {_, _, char} -> char end, fn {i, j, _} -> {i, j} end)
      |> Map.new()

    {antennas, width, height}
  end
end

Day08.solve()
