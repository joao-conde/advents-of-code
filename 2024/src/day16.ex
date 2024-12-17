defmodule Day16 do
  def solve do
    map = parse_map("input/day16")
    {src, _} = Enum.find(map, fn {_, v} -> v == "S" end)
    {dst, _} = Enum.find(map, fn {_, v} -> v == "E" end)

    p1 = dijkstra(map, src, dst)
    IO.puts("Part1: #{p1}")

    p2 = dijkstra2(map, src, dst, p1) |> List.flatten() |> Enum.uniq() |> length
    IO.puts("Part2: #{p2}")
  end

  def dijkstra2(map, {si, sj}, dst, max_cost) do
    heap = Heap.min() |> Heap.push({0, {si, sj, 0, 1, [{si, sj}]}})
    dijsktra_iter2(map, heap, dst, MapSet.new(), max_cost, [])
  end

  def dijsktra_iter2(map, heap, dst, visited, max_cost, paths) do
    {{cost, {i, j, di, dj, path}}, next_heap} = Heap.split(heap)

    next_visited = MapSet.put(visited, {i, j, di, dj})

    cond do
      {i, j} == dst ->
        dijsktra_iter2(map, next_heap, dst, next_visited, max_cost, [[{i, j} | path] | paths])

      cost > max_cost ->
        paths

      true ->
        next_heap =
          rotations({di, dj})
          |> Enum.map(fn {ri, rj} -> {cost + 1000, {i, j, ri, rj}} end)
          |> then(fn neighbors -> [{cost + 1, {i + di, j + dj, di, dj}} | neighbors] end)
          |> Enum.reject(fn {_, {i, j, di, dj}} ->
            out_of_bounds?(map, i, j) or wall?(map, i, j) or {i, j, di, dj} in next_visited
          end)
          |> Enum.reduce(next_heap, fn {cost, {i, j, di, dj}}, heap ->
            Heap.push(heap, {cost, {i, j, di, dj, [{i, j} | path]}})
          end)

        dijsktra_iter2(map, next_heap, dst, next_visited, max_cost, paths)
    end
  end

  def dijkstra(map, {si, sj}, dst) do
    heap = Heap.min() |> Heap.push({0, {si, sj, 0, 1}})
    dijsktra_iter(map, heap, dst, MapSet.new())
  end

  def dijsktra_iter(map, heap, dst, visited) do
    {{cost, {i, j, di, dj}}, next_heap} = Heap.split(heap)

    next_visited = MapSet.put(visited, {i, j, di, dj})

    if {i, j} == dst do
      cost
    else
      next_heap =
        rotations({di, dj})
        |> Enum.map(fn {ri, rj} -> {cost + 1000, {i, j, ri, rj}} end)
        |> then(fn neighbors -> [{cost + 1, {i + di, j + dj, di, dj}} | neighbors] end)
        |> Enum.reject(fn {_, {i, j, di, dj}} ->
          out_of_bounds?(map, i, j) or wall?(map, i, j) or {i, j, di, dj} in next_visited
        end)
        |> Enum.reduce(next_heap, fn state, heap -> Heap.push(heap, state) end)

      dijsktra_iter(map, next_heap, dst, next_visited)
    end
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
