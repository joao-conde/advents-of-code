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
    case find_report_mistake(report) do
      nil ->
        true

      mistake_index when corrections > 0 ->
        generate_corrections(report, mistake_index)
        |> Enum.any?(fn correction -> safe_report(correction, corrections - 1) end)

      _ ->
        false
    end
  end

  def find_report_mistake(report) do
    diffs = calculate_diffs(report)

    sorted_by =
      if Enum.count(diffs, fn x -> x > 0 end) > length(diffs) / 2 do
        :decreasing
      else
        :increasing
      end

    Enum.find_index(diffs, fn diff -> not valid_diff?(diff, sorted_by) end)
  end

  def calculate_diffs(report) do
    Enum.chunk_every(report, 2, 1, :discard)
    |> Enum.map(fn [x1, x2] -> x1 - x2 end)
  end

  def valid_diff?(diff, :decreasing), do: diff > 0 and abs(diff) in 1..3
  def valid_diff?(diff, :increasing), do: diff < 0 and abs(diff) in 1..3

  def generate_corrections(report, mistake_index) do
    [
      List.delete_at(report, mistake_index),
      List.delete_at(report, mistake_index + 1)
    ]
  end

  def parse_reports(input) do
    File.read!(input)
    |> String.split("\n")
    |> Enum.map(&parse_report/1)
  end

  def parse_report(line) do
    String.split(line) |> Enum.map(&String.to_integer/1)
  end
end

Day02.solve()
