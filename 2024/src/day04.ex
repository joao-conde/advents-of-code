defmodule Day04 do
  def solve do
    matrix = parse_matrix("input/day04")
    count_xmas(matrix) |> IO.inspect()
  end

  def count_xmas(matrix) do
    Enum.map(matrix, &Enum.with_index/1)
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, i} -> Enum.map(row, fn {c, j} -> {c, i, j} end) end)
    |> Enum.filter(fn {c, _, _} -> c == "X" end)
    |> Enum.map(fn {_, i, j} -> count_rec(matrix, i, j) end)
    |> Enum.sum()
  end

  def count_rec(matrix, i, j) do
    count_rec_help(matrix, i, j, {-1, 0}) +
      count_rec_help(matrix, i, j, {-1, 1}) +
      count_rec_help(matrix, i, j, {0, 1}) +
      count_rec_help(matrix, i, j, {1, 1}) +
      count_rec_help(matrix, i, j, {1, 0}) +
      count_rec_help(matrix, i, j, {1, -1}) +
      count_rec_help(matrix, i, j, {0, -1}) +
      count_rec_help(matrix, i, j, {-1, -1})
  end

  def count_rec_help(matrix, i, j, dir) do
    current = matrix |> Enum.at(i) |> Enum.at(j)

    cond do
      current == "S" ->
        1

      current in ["X", "M", "A"] ->
        case neighbors(matrix, i, j, current, dir) do
          {ni, nj} -> count_rec_help(matrix, ni, nj, dir)
          nil -> 0
        end

      true ->
        0
    end
  end

  def neighbors(matrix, i, j, c, {di, dj}) do
    ni = i + di
    nj = j + dj

    if ni >= 0 and nj >= 0 and ni < length(matrix) and nj < length(Enum.at(matrix, ni)) and
         Enum.at(Enum.at(matrix, ni), nj) == next_letter(c) do
      {ni, nj}
    else
      nil
    end
  end

  def next_letter(letter) do
    case letter do
      "X" -> "M"
      "M" -> "A"
      "A" -> "S"
    end
  end

  def parse_matrix(input) do
    File.read!(input)
    |> String.split()
    |> Enum.map(&String.graphemes/1)
  end
end

Day04.solve()
