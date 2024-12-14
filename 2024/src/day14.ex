defmodule Day14 do
  @width 101
  @height 103
  @seconds 100

  def solve do
    robots = parse_robots("input/day14")

    p1 =
      robots
      |> robot_positions(@seconds, @width, @height)
      |> Enum.frequencies()
      |> safety_factor(@width, @height)

    IO.puts("Part1: #{p1}")

    p2 = 7338
    print_tree(robots, p2, @width, @height)
    IO.puts("Part2: #{p2}")
  end

  def safety_factor(positions, width, height) do
    middle_x = div(width, 2)
    middle_y = div(height, 2)

    topleft = count_quadrant_robots(positions, 0, middle_x - 1, 0, middle_y - 1)
    topright = count_quadrant_robots(positions, middle_x + 1, width, 0, middle_y - 1)
    botleft = count_quadrant_robots(positions, 0, middle_x - 1, middle_y + 1, height)
    botright = count_quadrant_robots(positions, middle_x + 1, width, middle_y + 1, height)

    topleft * topright * botleft * botright
  end

  def count_quadrant_robots(positions, from_x, to_x, from_y, to_y) do
    positions
    |> Enum.filter(fn {{x, y}, _} -> x >= from_x and x <= to_x and y >= from_y and y <= to_y end)
    |> Enum.map(fn {_, count} -> count end)
    |> Enum.sum()
  end

  def robot_positions(robots, seconds, width, height) do
    Enum.map(robots, fn robot -> robot_position(robot, seconds, width, height) end)
  end

  def robot_position({x, y, vx, vy}, seconds, width, height) do
    {Integer.mod(x + vx * seconds, width), Integer.mod(y + vy * seconds, height)}
  end

  # By simulating a few times I was able to detect a pattern of
  # horizontal and vertical lines grouping.
  # I counted the first occurrence of each for my input, and then
  # the frequency at which each line appeared.
  # Then simulated faster, each step jumping that frequency which is
  # either the height or width for vertical or horizontal lines.
  # That way I quickly arrived at the 7338s for my input, where I
  # saw the tree.
  #
  # In this function I pad the "noise" out of the picture with some
  # magic numbers to make it prettier.
  def print_tree(robots, seconds, width, height) do
    robots = robot_positions(robots, seconds, width, height)

    15..(@width - 55)
    |> Enum.map(fn x -> Enum.map(33..(@height - 35), fn y -> {x, y} end) end)
    |> Enum.map_join("\n", fn line ->
      Enum.map(line, fn {x, y} -> if {x, y} in robots, do: "#", else: " " end)
    end)
    |> IO.puts()
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
