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
    map
    |> Enum.find(fn {_, v} -> v == "^" end)
    |> elem(0)
  end

  def guard_path(map, {i, j}, dir \\ {-1, 0}, path \\ MapSet.new()) do
    npos = next_position(map, {i, j}, dir)

    cond do
      MapSet.member?(path, {i, j, dir}) ->
        distinct = distinct_positions(path)
        {distinct, true}

      npos == nil ->
        distinct = MapSet.put(path, {i, j, dir}) |> distinct_positions()
        {distinct, false}

      true ->
        {ni, nj, ndir} = npos
        guard_path(map, {ni, nj}, ndir, MapSet.put(path, {i, j, dir}))
    end
  end

  def possible_obstructions(path, map, start) do
    path |> Enum.count(fn {i, j, _} -> Map.put(map, {i, j}, "#") |> path_loops?(start) end)
  end

  def path_loops?(map, start) do
    map
    |> guard_path(start)
    |> then(fn {_, loops} -> loops end)
  end

  def distinct_positions(path) do
    path |> Enum.uniq_by(fn {i, j, _} -> {i, j} end)
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

  def rotate_right(dir) do
    case dir do
      {-1, 0} -> {0, 1}
      {0, 1} -> {1, 0}
      {1, 0} -> {0, -1}
      {0, -1} -> {-1, 0}
    end
  end

  def parse_map(input) do
    input
    |> File.read!()
    |> String.split()
    |> Enum.map(&String.graphemes/1)
    |> Enum.map(&Enum.with_index/1)
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, i} -> Enum.map(row, fn {char, j} -> {char, i, j} end) end)
    |> Map.new(fn {c, i, j} -> {{i, j}, c} end)
  end
end

Day06.solve()
