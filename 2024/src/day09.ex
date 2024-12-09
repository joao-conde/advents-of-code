defmodule Day09 do
  def solve do
    {[file_head | files], spaces} = parse_disk_map("input/day09")

    p1 =
      compact_files({files, spaces}, [file_head])
      |> checksum()

    IO.puts("Part1: #{p1}")

    p2 = 0
    IO.puts("Part2: #{p2}")
  end

  def checksum(file_system) do
    file_system
    |> Enum.flat_map(fn {count, digit} -> List.duplicate(digit, count) end)
    |> Enum.with_index()
    |> Enum.reduce(0, fn {digit, i}, sum -> sum + digit * i end)
  end

  def compact_files({[], _}, final) do
    Enum.reverse(final)
  end

  def compact_files({files, free_space}, final) do
    {files, spaces, final} =
      compact_file({files, free_space}, final)

    compact_files({files, spaces}, final)
  end

  def compact_file({files, free_spaces}, final) do
    {last_file, last_i} = List.last(files)

    [first_file | rest_files] = files

    [first_space | rest_spaces] = free_spaces

    cond do
      first_space == 0 ->
        {rest_files, rest_spaces, [first_file | final]}

      last_file < first_space ->
        next_files = files |> Enum.reverse() |> Enum.drop(1) |> Enum.reverse()
        next_spaces = [first_space - last_file | rest_spaces]
        next_final = [{last_file, last_i} | final]
        {next_files, next_spaces, next_final}

      last_file >= first_space ->
        files_left = last_file - first_space
        next_spaces = [0 | rest_spaces]

        next_files =
          if files_left > 0 do
            List.replace_at(files, length(files) - 1, {files_left, last_i})
          else
            files |> Enum.reverse() |> Enum.drop(1) |> Enum.reverse()
          end

        next_final = [{first_space, last_i} | final]

        {next_files, next_spaces, next_final}
    end
  end

  def parse_disk_map(input) do
    input
    |> File.read!()
    |> String.graphemes()
    |> Enum.reverse()
    |> Enum.with_index()
    |> Enum.reduce({[], []}, fn {digit, i}, {files, free_space} ->
      digit = String.to_integer(digit)

      if rem(i, 2) == 0 do
        {[digit | files], free_space}
      else
        {files, [digit | free_space]}
      end
    end)
    |> then(fn {files, free_space} -> {Enum.with_index(files), free_space} end)
  end
end

Day09.solve()
