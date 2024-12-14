defmodule Day14 do
  @seconds 100
  @width 101
  @height 103

  def solve do
    robots = parse_robots("input/day14")

    p1 =
      robot_positions(robots)
      |> Enum.frequencies()
      |> safety_factor()

    IO.puts("Part1: #{p1}")
  end

  def safety_factor(positions) do
    middle_x = div(@width, 2)
    middle_y = div(@height, 2)

    topleft = count_quadrant_robots(positions, 0, middle_x - 1, 0, middle_y - 1)
    topright = count_quadrant_robots(positions, middle_x + 1, @width, 0, middle_y - 1)
    botleft = count_quadrant_robots(positions, 0, middle_x - 1, middle_y + 1, @height)
    botright = count_quadrant_robots(positions, middle_x + 1, @width, middle_y + 1, @height)

    topleft * topright * botleft * botright
  end

  def count_quadrant_robots(positions, from_x, to_x, from_y, to_y) do
    positions
    |> Enum.filter(fn {{x, y}, _} -> x >= from_x and x <= to_x and y >= from_y and y <= to_y end)
    |> Enum.map(fn {_, count} -> count end)
    |> Enum.sum()
  end

  def robot_positions(robots) do
    Enum.map(robots, &robot_position/1)
  end

  def robot_position({x, y, vx, vy}) do
    {Integer.mod(x + vx * @seconds, @width), Integer.mod(y + vy * @seconds, @height)}
  end

  def parse_robots(input) do
    input
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&parse_robot/1)
  end

  def parse_robot(line) do
    Regex.run(~r/p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)/, line, capture: :all_but_first)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end

Day14.solve()
