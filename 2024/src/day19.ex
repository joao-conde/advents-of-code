defmodule Day19 do
  def solve do
    {patterns, designs} = parse_input("input/day19")

    {_, dp} =
      designs
      |> Enum.reduce({0, %{}}, fn d, {cnt, acc} ->
        {possible, dp} = design_possible?(d, patterns, acc)

        if possible do
          {cnt + 1, dp}
        else
          {cnt, dp}
        end
      end)

    p1 = designs |> Enum.reject(&is_nil(dp[&1])) |> Enum.count(&(dp[&1] > 0))
    IO.puts("Part1: #{p1}")
  end

  def design_possible?(design, patterns, dp) do
    cond do
      dp[design] != nil ->
        {dp[design], dp}

      String.length(design) == 1 ->
        if design in patterns do
          {1, Map.put(dp, design, 1)}
        else
          {0, dp}
        end

      design in patterns ->
        {1, Map.put(dp, design, 1)}

      true ->
        {possible_cnt, dp} =
          1..(String.length(design) - 1)
          |> Enum.map(fn i -> String.split_at(design, i) end)
          |> Enum.reduce({0, dp}, fn {left, right}, {possible_cnt, dp} ->
            {left_possible, dp} = design_possible?(left, patterns, dp)
            {right_possible, dp} = design_possible?(right, patterns, dp)
            possible = left_possible > 0 and right_possible > 0

            if possible do
              dp =
                dp
                |> Map.put(left, left_possible)
                |> Map.put(right, right_possible)

              {possible_cnt + 1, dp}
            else
              {possible_cnt, dp}
            end
          end)

        dp = dp |> Map.put(design, possible_cnt)

        {possible_cnt, dp}
    end
  end

  def parse_input(input) do
    [patterns, designs] =
      input
      |> File.read!()
      |> String.split("\n\n", parts: 2)

    patterns = String.split(patterns, ", ") |> MapSet.new()
    designs = String.split(designs, "\n")

    {patterns, designs}
  end
end

Day19.solve()
