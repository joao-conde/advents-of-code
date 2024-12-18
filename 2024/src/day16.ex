defmodule Day16 do
  @infinity :math.pow(10, 100)

  def solve do
    map = parse_map("input/day16")
    {src, _} = Enum.find(map, fn {_, v} -> v == "S" end)
    {dst, _} = Enum.find(map, fn {_, v} -> v == "E" end)

    {p1, shortest_paths} = shortest_paths(map, src, dst)
    IO.puts("Part1: #{p1}")

    p2 = shortest_paths |> List.flatten() |> Enum.uniq() |> length
    IO.puts("Part2: #{p2}")
  end

  def shortest_paths(map, {si, sj}, dst) do
    # {cost, {i, j, di, dj, path}}
    start_state = {0, {si, sj, 0, 1, [{si, sj}]}}
    min_heap = Heap.min() |> Heap.push(start_state)
    shortest_paths(map, dst, min_heap, MapSet.new(), [], @infinity)
  end

  def shortest_paths(map, dst, min_heap, visited, paths, max_cost) do
    {{cost, {i, j, di, dj, path}}, next_heap} = Heap.split(min_heap)

    next_visited = MapSet.put(visited, {i, j, di, dj})

    cond do
      cost > max_cost ->
        {max_cost, paths}

      {i, j} == dst ->
        next_paths = [
          [{i, j} | path] | paths
        ]

        shortest_paths(map, dst, next_heap, next_visited, next_paths, cost)

      true ->
        next_heap =
          neighbors(map, next_visited, i, j, di, dj, cost)
          |> Enum.map(fn {cost, {i, j, di, dj}} -> {cost, {i, j, di, dj, [{i, j} | path]}} end)
          |> Enum.reduce(next_heap, fn state, heap -> Heap.push(heap, state) end)

        shortest_paths(map, dst, next_heap, next_visited, paths, max_cost)
    end
  end

  def neighbors(map, visited, i, j, di, dj, cost) do
    rotations({di, dj})
    |> Enum.map(fn {ri, rj} -> {cost + 1000, {i, j, ri, rj}} end)
    |> then(fn neighbors -> [{cost + 1, {i + di, j + dj, di, dj}} | neighbors] end)
    |> Enum.reject(fn {_, {i, j, di, dj}} ->
      out_of_bounds?(map, i, j) or wall?(map, i, j) or {i, j, di, dj} in visited
    end)
  end

  def rotations(dir) do
    case dir do
      {0, 1} -> [{-1, 0}, {1, 0}]
      {0, -1} -> [{-1, 0}, {1, 0}]
      {1, 0} -> [{0, -1}, {0, 1}]
      {-1, 0} -> [{0, -1}, {0, 1}]
    end
  end

  def out_of_bounds?(map, i, j) do
    Map.get(map, {i, j}) == nil
  end

  def wall?(map, i, j) do
    Map.get(map, {i, j}) == "#"
  end

  def parse_map(input) do
    input
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(fn line -> line |> String.graphemes() |> Enum.with_index() end)
    |> Enum.with_index()
    |> Enum.flat_map(fn {line, i} -> Enum.map(line, fn {char, j} -> {{i, j}, char} end) end)
    |> Map.new()
  end
end

Day16.solve()
