defmodule Day18 do
  def solve do
    bytes = parse_byte_list("input/day18")

    {take, src, dst} = {1024, {0, 0}, {70, 70}}

    p1 = corrupted(bytes, take) |> bfs(src, dst)
    IO.puts("Part1: #{p1}")

    {x, y} = blocked(bytes, src, dst)
    IO.puts("Part2: #{x},#{y}")
  end

  def blocked(bytes, src, dst) do
    point = blocked(bytes, src, dst, 0, length(bytes))
    Enum.at(bytes, point - 1)
  end

  def blocked(bytes, src, dst, lb, ub) do
    mid = lb + div(ub - lb, 2)

    p = corrupted(bytes, mid) |> bfs(src, dst)

    cond do
      lb >= ub ->
        mid

      is_nil(p) ->
        blocked(bytes, src, dst, lb, mid - 1)

      lb < ub ->
        blocked(bytes, src, dst, mid + 1, ub)
    end
  end

  def bfs(corrupted, src, dst) do
    deque = :queue.from_list([{0, src}])
    bfs(corrupted, src, dst, deque, MapSet.new())
  end

  def bfs(corrupted, src, dst, deque, visited) do
    if :queue.is_empty(deque) do
      nil
    else
      {steps, {x, y}} = :queue.get(deque)
      next_deque = :queue.drop(deque)
      next_visited = MapSet.put(visited, {x, y})

      cond do
        {x, y} == dst ->
          steps

        {x, y} in visited ->
          bfs(corrupted, src, dst, next_deque, visited)

        true ->
          next_deque =
            next_positions(corrupted, x, y, src, dst)
            |> Enum.reduce(next_deque, fn pos, acc -> :queue.in({steps + 1, pos}, acc) end)

          bfs(corrupted, src, dst, next_deque, next_visited)
      end
    end
  end

  def next_positions(corrupted, x, y, {src_x, src_y}, {dst_x, dst_y}) do
    [
      {x + 1, y},
      {x, y + 1},
      {x - 1, y},
      {x, y - 1}
    ]
    |> Enum.reject(fn {x, y} ->
      not valid_state?(corrupted, x, y, {src_x, src_y}, {dst_x, dst_y})
    end)
  end

  def valid_state?(corrupted, x, y, {src_x, src_y}, {dst_x, dst_y}) do
    {x, y} not in corrupted and x >= src_x and y >= src_y and x <= dst_x and y <= dst_y
  end

  def corrupted(bytes, n) do
    bytes |> Enum.take(n) |> MapSet.new()
  end

  def parse_byte_list(input) do
    input
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&parse_line/1)
  end

  def parse_line(line) do
    line
    |> String.split(",", parts: 2)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end

Day18.solve()
