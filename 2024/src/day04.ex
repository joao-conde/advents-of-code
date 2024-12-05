defmodule Day04 do
  def solve do
    soup = parse_soup("input/day04")

    p1 = count_from_center(soup, "X", &count_xmas/3)
    IO.puts("Part1: #{p1}")

    p2 = count_from_center(soup, "A", &count_x_mas/3)
    IO.puts("Part2: #{p2}")
  end

  def count_from_center(soup, center, count_pattern_fn) do
    Enum.map(soup, &Enum.with_index/1)
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, i} -> Enum.map(row, fn {char, j} -> {char, i, j} end) end)
    |> Enum.filter(fn {char, _, _} -> char == center end)
    |> Enum.map(fn {_, i, j} -> count_pattern_fn.(soup, i, j) end)
    |> Enum.sum()
  end

  def count_xmas(soup, i, j) do
    [
      {-1, 0},
      {-1, 1},
      {0, 1},
      {1, 1},
      {1, 0},
      {1, -1},
      {0, -1},
      {-1, -1}
    ]
    |> Enum.map(fn delta -> get_word(soup, i, j, delta, 4) end)
    |> Enum.count(fn word -> word == "XMAS" end)
  end

  def count_x_mas(soup, i, j) do
    line1 = Enum.map([{-1, -1}, {1, 1}], fn delta -> get_word(soup, i, j, delta, 2) end)
    line2 = Enum.map([{-1, 1}, {1, -1}], fn delta -> get_word(soup, i, j, delta, 2) end)
    if x_mas?(line1, line2), do: 1, else: 0
  end

  def x_mas?(line1, line2) do
    "AM" in line1 and "AS" in line1 and "AM" in line2 and "AS" in line2
  end

  def get_word(soup, i, j, {di, dj}, length) do
    Stream.iterate(i, fn i -> i + di end)
    |> Enum.take(length)
    |> Enum.zip(Stream.iterate(j, fn j -> j + dj end) |> Enum.take(length))
    |> Enum.filter(fn {i, j} -> not out_of_bounds(soup, i, j) end)
    |> Enum.map_join("", fn {i, j} -> matrix_at(soup, i, j) end)
  end

  def parse_soup(input) do
    File.read!(input)
    |> String.split()
    |> Enum.map(&String.graphemes/1)
  end

  def matrix_at(matrix, i, j) do
    matrix |> Enum.at(i) |> Enum.at(j)
  end

  def out_of_bounds(matrix, i, j) do
    i < 0 or j < 0 or i >= length(matrix) or j >= length(Enum.at(matrix, i))
  end
end

Day04.solve()
