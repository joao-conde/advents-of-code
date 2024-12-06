defmodule Day06 do
  def solve do
    map = parse_map("input/day06")

    start = find_guard(map)
    {path, _} = guard_path(map, start)

    p1 = length(path)
    IO.puts("Part1: #{p1}")

    p2 = possible_obstructions(path, map, start)
    IO.puts("Part2: #{p2}")
  end

  def find_guard(map) do
    map |> Enum.find_value(fn {k, v} -> if v == "^", do: k end)
  end

  def guard_path(map, {i, j}, dir \\ {-1, 0}, path \\ MapSet.new()) do
    next_pos = next_position(map, {i, j}, dir)
    next_path = MapSet.put(path, {i, j, dir})

    cond do
      MapSet.member?(path, {i, j, dir}) ->
        path = distinct_positions(next_path)
        {path, true}

      next_pos == nil ->
        path = distinct_positions(next_path)
        {path, false}

      true ->
        {ni, nj, ndir} = next_pos
        guard_path(map, {ni, nj}, ndir, next_path)
    end
  end

  def possible_obstructions(path, map, start) do
    path |> Enum.count(fn pos -> Map.put(map, pos, "#") |> path_loops?(start) end)
  end

  def path_loops?(map, start) do
    {_, loops} = guard_path(map, start)
    loops
  end

  def distinct_positions(path) do
    path |> Enum.map(fn {i, j, _} -> {i, j} end) |> Enum.uniq()
  end

  def next_position(map, {i, j}, {di, dj}) do
    ni = i + di
    nj = j + dj

    case Map.get(map, {ni, nj}) do
      nil -> nil
      "#" -> {i, j, rotate_right({di, dj})}
      _ -> {ni, nj, {di, dj}}
    end
  end

  def rotate_right({di, dj}) do
    case {di, dj} do
      {-1, 0} -> {0, 1}
      {0, 1} -> {1, 0}
      {1, 0} -> {0, -1}
      {0, -1} -> {-1, 0}
    end
  end

  def parse_map(input) do
    input
    |> File.read!()
    |> String.split("\n")
    |> Enum.with_index()
    |> Enum.flat_map(fn {line, i} ->
      line
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.map(fn {char, j} -> {{i, j}, char} end)
    end)
    |> Map.new()
  end
end

Day06.solve()
