defmodule Day02 do
  def solve do
    reports = parse_reports("input/day02")

    p1 = safe_reports(reports, 0)
    IO.puts("Part1: #{p1}")

    p2 = safe_reports(reports, 1)
    IO.puts("Part2: #{p2}")
  end

  def safe_reports(reports, corrections) do
    Enum.count(reports, fn report -> safe_report(report, corrections) end)
  end

  def safe_report(report, corrections) do
    mistake_index = find_report_mistake(report)

    cond do
      corrections < 0 ->
        false

      mistake_index == nil ->
        true

      true ->
        removed_left = remove_index(report, mistake_index)
        removed_right = remove_index(report, mistake_index + 1)
        safe_report(removed_left, corrections - 1) || safe_report(removed_right, corrections - 1)
    end
  end

  def find_report_mistake(report) do
    diffs =
      Enum.chunk_every(report, 2, 1, :discard)
      |> Enum.map(fn [x1, x2] -> x1 - x2 end)

    positives = Enum.count(diffs, fn x -> x > 0 end)
    negatives = length(diffs) - positives

    sorted_by =
      if positives > negatives do
        :decreasing
      else
        :increasing
      end

    Enum.find_index(diffs, fn diff -> !valid_diff(diff, sorted_by) end)
  end

  def valid_diff(diff, sorted_by) do
    abs(diff) >= 1 && abs(diff) <= 3 &&
      ((sorted_by == :decreasing && diff > 0) ||
         (sorted_by == :increasing && diff < 0))
  end

  def parse_reports(input) do
    File.read!(input)
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn report -> Enum.map(report, &String.to_integer/1) end)
  end

  def remove_index(list, index) do
    Enum.slice(list, 0, index) ++
      Enum.slice(list, index + 1, length(list) - index)
  end
end

Day02.solve()
