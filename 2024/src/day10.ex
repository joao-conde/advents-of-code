defmodule Day10 do
  def solve do
    map = parse_map("input/day10")
    trailheads = trailheads(map)
    map_trails = map_trails(map)

    p1 =
      score_trailheads(trailheads, fn trailhead ->
        height_nine_endings(trailhead, map_trails)
      end)

    IO.puts("Part1: #{p1}")

    map_endings_count = map_endings_count(map_trails)

    p2 =
      score_trailheads(trailheads, fn trailhead ->
        distinct_trails(trailhead, map_endings_count)
      end)

    IO.puts("Part2: #{p2}")
  end

  def trailheads(map) do
    map |> Enum.filter(fn {_, v} -> v == 0 end)
  end

  def map_trails(map) do
    map
    |> Enum.map(fn {{i, j}, _} -> {{i, j}, trails_from(map, i, j)} end)
    |> Map.new()
  end

  def map_endings_count(map_trails) do
    map_trails
    |> Enum.flat_map(fn {_, trails} ->
      Enum.map(trails, fn trail -> trail |> Enum.reverse() |> hd() end)
    end)
    |> Enum.frequencies()
  end

  def trails_from(map, i, j, path \\ [], paths \\ []) do
    case Map.get(map, {i, j}) do
      9 ->
        [[{i, j} | path] | paths]

      _ ->
        neighbors(map, i, j)
        |> Enum.flat_map(fn {ni, nj} ->
          trails_from(map, ni, nj, [{i, j} | path], paths)
        end)
    end
  end

  def neighbors(map, i, j) do
    cur_height = Map.get(map, {i, j})

    [{1, 0}, {-1, 0}, {0, 1}, {0, -1}]
    |> Enum.map(fn {di, dj} -> {i + di, j + dj} end)
    |> Enum.filter(fn {ni, nj} ->
      Map.get(map, {ni, nj}, cur_height) == cur_height + 1
    end)
  end

  def score_trailheads(trailheads, score_fn) do
    trailheads
    |> Enum.map(fn trailhead -> score_fn.(trailhead) end)
    |> Enum.sum()
  end

  def height_nine_endings({{i, j}, _}, map_trails) do
    Map.get(map_trails, {i, j}, [])
    |> Enum.map(fn t -> hd(t) end)
    |> Enum.uniq()
    |> length()
  end

  def distinct_trails({{i, j}, _}, map_endings_count) do
    Map.get(map_endings_count, {i, j}, 0)
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
      |> Enum.map(fn {v, j} -> {{i, j}, String.to_integer(v)} end)
    end)
    |> Map.new()
  end
end

Day10.solve()
