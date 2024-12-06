defmodule Day06 do
  def solve do
    map = parse_map("input/day06")

    {si, sj} = find_guard(map)

    {path, _} = guard_path(map, [], si, sj, {-1, 0})

    path = path |> then(fn l -> [{si, sj, {-1, 0}} | l] end)

    p1 =
      path
      |> Enum.map(fn {i, j, _} -> {i, j} end)
      |> Enum.uniq()
      |> length()

    IO.puts("Part1: #{p1}")

    p2 = count_loops(map, si, sj)
    IO.puts("Part2: #{p2}")
  end

  def count_loops(map, si, sj) do
    Map.keys(map)
    |> Enum.count(fn {i, j} ->
      Map.put(map, {i, j}, "#")
      |> guard_path([], si, sj, {-1, 0})
      |> then(fn {_, loops} -> loops end)
    end)
  end

  def right_loops?(path, at) do
    {i, j, dir} = Enum.at(path, at)
    next = {i, j, rotate_right(dir)}

    next in path and next != Enum.at(path, at + 1)
  end

  def find_guard(map) do
    map
    |> Enum.find(fn {_, v} -> v == "^" end)
    |> elem(0)
  end

  def guard_path(map, positions, i, j, dir) do
    if {i, j, dir} in positions do
      {positions, true}
    else
      case next_position(map, i, j, dir) do
        {ni, nj, ndir} -> guard_path(map, [{i, j, dir} | positions], ni, nj, ndir)
        nil -> {positions, false}
      end
    end
  end

  def next_position(map, i, j, {di, dj}) do
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
