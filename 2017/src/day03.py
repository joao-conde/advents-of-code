import math

input_file = open("input/day03")
puzzle_input = int(input_file.read())
input_file.close()


# PART1
def manhattan_distance(origin_row, origin_col, dest_row, dest_col):
    return abs(dest_row - origin_row) + abs(dest_col - origin_col)


# grid size is always odd so it has a middle for the '1' and
# big enough to contain all numbers
grid_size = math.ceil(math.sqrt(puzzle_input))
if grid_size % 2 == 0:
    grid_size += 1

# create a square grid of grid_size by grid_size filled with 0's
grid = [[0 for element in range(grid_size)] for line in range(grid_size)]

# the filling pattern is as it follows:
# from a starting point, in order to complete one spiral (one cycle)
# you use the following algorithm (found by pattern analysis)
# being R(right), U(p), D(own), L(eft)
# 1xR + (2*i+1)xU + (U+1)xL + (U+1)xD + (U+1)xR

center = math.floor(grid_size / 2)
pos_x = pos_y = center
grid[pos_x][pos_y] = 1

element = 2
control = 0

# grid filling
while element <= puzzle_input:
    up_times = 2 * control + 1

    # one right and fill
    pos_y += 1
    grid[pos_x][pos_y] = element

    element += 1
    if element > puzzle_input:
        break

    # up times and fill for each
    for i in range(up_times):
        pos_x -= 1
        grid[pos_x][pos_y] = element

        element += 1
        if element > puzzle_input:
            break

    if element > puzzle_input:
        break

    # left times and fil for each
    for i in range(up_times + 1):
        pos_y -= 1
        grid[pos_x][pos_y] = element

        element += 1
        if element > puzzle_input:
            break

    if element > puzzle_input:
        break

    # down times and fill for each
    for i in range(up_times + 1):
        pos_x += 1
        grid[pos_x][pos_y] = element

        element += 1
        if element > puzzle_input:
            break

    if element > puzzle_input:
        break

    # right times and fill for each
    for i in range(up_times + 1):
        pos_y += 1
        grid[pos_x][pos_y] = element

        element += 1
        if element > puzzle_input:
            break

    control += 1
    if element > puzzle_input:
        break

# find element coords
elX = elY = 0
for i in range(grid_size):
    for j in range(grid_size):
        if grid[i][j] == puzzle_input:
            elX = i
            elY = j
            break

distance = manhattan_distance(center, center, elX, elY)
print("Distance: " + str(distance))

# PART2


# sum of al surrounding elements
def surround_sum(grid, pos_x, pos_y):
    return (
        grid[pos_x - 1][pos_y]
        + grid[pos_x + 1][pos_y]
        + grid[pos_x][pos_y - 1]
        + grid[pos_x][pos_y + 1]
        + grid[pos_x + 1][pos_y + 1]
        + grid[pos_x + 1][pos_y - 1]
        + grid[pos_x - 1][pos_y - 1]
        + grid[pos_x - 1][pos_y + 1]
    )


# grid_size+1 to ensure grid is big enough for the surround_sum algorithm not to check out of boundaries
# quicker fix rathe than fix surround_sum function
grid2 = [[0 for element in range(grid_size + 1)] for line in range(grid_size + 1)]


pos_x = pos_y = center
grid2[pos_x][pos_y] = 1

el = 0
control2 = 0

# grid filling algorithm
while el <= puzzle_input:
    up_times = 2 * control2 + 1

    # one right and fill
    pos_y += 1
    el = surround_sum(grid2, pos_x, pos_y)

    if el <= puzzle_input:
        grid2[pos_x][pos_y] = el
    else:
        break

    # up times and fill for each
    for i in range(up_times):
        pos_x -= 1
        el = surround_sum(grid2, pos_x, pos_y)

        if el <= puzzle_input:
            grid2[pos_x][pos_y] = el
        else:
            break

    if el > puzzle_input:
        break

    # left times and fil for each
    for i in range(up_times + 1):
        pos_y -= 1
        el = surround_sum(grid2, pos_x, pos_y)

        if el <= puzzle_input:
            grid2[pos_x][pos_y] = el
        else:
            break

    if el > puzzle_input:
        break

    # down times and fill for each
    for i in range(up_times + 1):
        pos_x += 1
        el = surround_sum(grid2, pos_x, pos_y)

        if el <= puzzle_input:
            grid2[pos_x][pos_y] = el
        else:
            break

    if el > puzzle_input:
        break

    # right times and fill for each
    for i in range(up_times + 1):
        pos_y += 1
        el = surround_sum(grid2, pos_x, pos_y)

        if el <= puzzle_input:
            grid2[pos_x][pos_y] = el
        else:
            break

    control2 += 1
    if el > puzzle_input:
        break


# fill the value
grid2[pos_x][pos_y] = el

print("First value bigger than input: " + str(el))
