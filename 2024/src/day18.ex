defmodule Day18 do
  @kilobyte 1024
  @size 70

  def solve do
    bytes = parse_byte_list("input/day18")

    p1 = min_steps(bytes, @kilobyte, @size)
    IO.puts("Part1: #{p1}")

    p2 = first_blocker(bytes, @size) |> then(fn {x, y} -> "#{x},#{y}" end)
    IO.puts("Part2: #{p2}")
  end

  def first_blocker(bytes, size) do
    index = first_blocker(bytes, size, 0, length(bytes))
    Enum.at(bytes, index)
  end

  def first_blocker(bytes, size, lb, ub) do
    mid = lb + div(ub - lb, 2)

    steps = min_steps(bytes, mid, size)

    cond do
      lb >= ub ->
        mid - 1

      is_nil(steps) ->
        first_blocker(bytes, size, lb, mid - 1)

      lb < ub ->
        first_blocker(bytes, size, mid + 1, ub)
    end
  end

  def min_steps(bytes, byte_count, size) do
    src = {0, 0}
    dst = {size, size}
    corrupted = corrupted_bytes(bytes, byte_count)
    deque = :queue.from_list([{0, src}])
    visited = MapSet.new()
    breadth_first_search(corrupted, src, dst, deque, visited)
  end

  def breadth_first_search(corrupted, src, dst, deque, visited) do
    case :queue.out(deque) do
      {:empty, _} ->
        nil

      {{_, {steps, {x, y}}}, next_deque} ->
        next_visited = MapSet.put(visited, {x, y})

        cond do
          {x, y} == dst ->
            steps

          {x, y} in visited ->
            breadth_first_search(corrupted, src, dst, next_deque, visited)

          true ->
            next_deque =
              next_positions(corrupted, x, y, src, dst)
              |> Enum.reduce(next_deque, fn pos, acc -> :queue.in({steps + 1, pos}, acc) end)

            breadth_first_search(corrupted, src, dst, next_deque, next_visited)
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
      not valid_position?(corrupted, x, y, {src_x, src_y}, {dst_x, dst_y})
    end)
  end

  def valid_position?(corrupted, x, y, {src_x, src_y}, {dst_x, dst_y}) do
    {x, y} not in corrupted and x >= src_x and y >= src_y and x <= dst_x and y <= dst_y
  end

  def corrupted_bytes(bytes, byte_count) do
    bytes |> Enum.take(byte_count) |> MapSet.new()
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
