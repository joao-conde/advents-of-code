#Link to problem: https://adventofcode.com/2017/day/14

from day10 import knot_hash

def dfs_region_explorer(grid, empty, used, x = 0, y = 0):
  if grid[x][y] != empty:
    grid[x][y] = empty

    if x + 1 < len(grid):
      dfs_region_explorer(grid, empty, used, x + 1, y)

    if y + 1 < len(grid[x]):
      dfs_region_explorer(grid, empty, used, x, y + 1)
    
    if x - 1 >= 0:
      dfs_region_explorer(grid, empty, used, x - 1, y)

    if y - 1 >= 0:
      dfs_region_explorer(grid, empty, used, x, y - 1)

puzzle_input = "stpzcrnm"

# PART 1
HASH_LEN = 128
used, grid = 0, []
for r in range(HASH_LEN):
  hex_hash = "".join(knot_hash(puzzle_input + "-" + str(r)))
  bin_hash = bin(int(hex_hash, 16))[2:]
  used += len([b for b in bin_hash if b == "1"])

  prefix = "".join(["0" for _ in range(HASH_LEN - len(bin_hash))])
  grid.append([c for c in prefix + bin_hash])

print(f'(Part1) Used squares: {used}')

# PART 2
regions, EMPTY, USED = 0, "0", "1"
for r in range(len(grid)):
  while USED in grid[r]:
    dfs_region_explorer(grid, x=r, y=grid[r].index(USED), empty = EMPTY, used = USED)
    regions += 1
    
print(f'(Part2) Number of regions: {regions}')
